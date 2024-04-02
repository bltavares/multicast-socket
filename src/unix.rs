use std::collections::HashMap;
use std::io;
use std::mem;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::os::unix::io::AsRawFd;

use socket2::{Domain, Protocol, Socket, Type};

use nix::sys::socket as sock;
use nix::sys::uio::IoVec;

fn create_on_interfaces(
    options: crate::MulticastOptions,
    interfaces: Vec<Ipv4Addr>,
    multicast_address: SocketAddrV4,
) -> io::Result<MulticastSocket> {
    let socket = Socket::new(Domain::ipv4(), Type::dgram(), Some(Protocol::udp()))?;
    socket.set_read_timeout(options.read_timeout)?;
    socket.set_multicast_loop_v4(options.loopback)?;
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;

    // Ipv4PacketInfo translates to `IP_PKTINFO`. Checkout the [ip
    // manpage](https://man7.org/linux/man-pages/man7/ip.7.html) for more details. In summary
    // setting this option allows for determining on which interface a packet was received.
    sock::setsockopt(socket.as_raw_fd(), sock::sockopt::Ipv4PacketInfo, &true)
        .map_err(nix_to_io_error)?;

    for interface in &interfaces {
        socket.join_multicast_v4(multicast_address.ip(), &interface)?;
    }

    socket.bind(&SocketAddr::new(options.bind_address.into(), multicast_address.port()).into())?;

    Ok(MulticastSocket {
        socket,
        interfaces,
        multicast_address,
        buffer_size: options.buffer_size,
    })
}

pub struct MulticastSocket {
    socket: socket2::Socket,
    interfaces: Vec<Ipv4Addr>,
    multicast_address: SocketAddrV4,
    buffer_size: usize,
}

#[derive(Debug, Clone)]
pub enum Interface {
    Default,
    Ip(Ipv4Addr),
    Index(i32),
}

#[derive(Debug, Clone)]
pub struct Message {
    pub data: Vec<u8>,
    pub origin_address: SocketAddrV4,
    pub interface: Interface,
}

pub fn all_ipv4_interfaces() -> io::Result<Vec<Ipv4Addr>> {
    let interfaces = if_addrs::get_if_addrs()?.into_iter();

    // We have to filter the same interface if it has multiple ips
    // https://stackoverflow.com/questions/49819010/ip-add-membership-fails-when-set-both-on-interface-and-its-subinterface-is-that
    let mut collected_interfaces = HashMap::with_capacity(interfaces.len());
    for interface in interfaces {
        if !collected_interfaces.contains_key(&interface.name) {
            match interface.ip() {
                std::net::IpAddr::V4(v4) if !interface.is_loopback() => {
                    collected_interfaces.insert(interface.name, v4);
                }
                _ => {}
            }
        }
    }
    Ok(collected_interfaces.into_iter().map(|(_, ip)| ip).collect())
}

impl MulticastSocket {
    pub fn all_interfaces(multicast_address: SocketAddrV4) -> io::Result<Self> {
        let interfaces = all_ipv4_interfaces()?;
        create_on_interfaces(Default::default(), interfaces, multicast_address)
    }

    pub fn with_options(
        multicast_address: SocketAddrV4,
        interfaces: Vec<Ipv4Addr>,
        options: crate::MulticastOptions,
    ) -> io::Result<Self> {
        create_on_interfaces(options, interfaces, multicast_address)
    }
}

fn nix_to_io_error(e: nix::Error) -> io::Error {
    io::Error::new(io::ErrorKind::Other, e)
}

impl MulticastSocket {
    pub fn receive(&self) -> io::Result<Message> {
        let mut data_buffer = vec![0; self.buffer_size];
        let mut control_buffer = nix::cmsg_space!(libc::in_pktinfo);

        let message = sock::recvmsg(
            self.socket.as_raw_fd(),
            &[IoVec::from_mut_slice(&mut data_buffer)],
            Some(&mut control_buffer),
            sock::MsgFlags::empty(),
        )
        .map_err(nix_to_io_error)?;

        let origin_address = match message.address {
            Some(sock::SockAddr::Inet(v4)) => Some(v4.to_std()),
            _ => None,
        };
        let origin_address = match origin_address {
            Some(SocketAddr::V4(v4)) => v4,
            _ => SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0),
        };

        let mut interface = Interface::Default;

        for cmsg in message.cmsgs() {
            if let sock::ControlMessageOwned::Ipv4PacketInfo(pktinfo) = cmsg {
                interface = Interface::Index(pktinfo.ipi_ifindex as _);
            }
        }

        Ok(Message {
            data: data_buffer[0..message.bytes].to_vec(),
            origin_address,
            interface,
        })
    }

    pub fn send(&self, buf: &[u8], interface: &Interface) -> io::Result<usize> {
        let mut pkt_info: libc::in_pktinfo = unsafe { mem::zeroed() };

        match interface {
            Interface::Default => {}
            Interface::Ip(address) => pkt_info.ipi_spec_dst = sock::Ipv4Addr::from_std(address).0,
            Interface::Index(index) => pkt_info.ipi_ifindex = *index as _,
        };

        let destination = sock::InetAddr::from_std(&self.multicast_address.into());

        sock::sendmsg(
            self.socket.as_raw_fd(),
            &[IoVec::from_slice(&buf)],
            &[sock::ControlMessage::Ipv4PacketInfo(&pkt_info)],
            sock::MsgFlags::empty(),
            Some(&sock::SockAddr::new_inet(destination)),
        )
        .map_err(nix_to_io_error)
    }

    pub fn broadcast(&self, buf: &[u8]) -> io::Result<()> {
        for interface in &self.interfaces {
            self.send(buf, &Interface::Ip(*interface))?;
        }
        Ok(())
    }
}
