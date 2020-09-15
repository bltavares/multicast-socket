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
    socket.set_read_timeout(Some(options.read_timeout))?;
    socket.set_multicast_loop_v4(options.loopback)?;
    socket.set_reuse_address(true)?;
    socket.set_reuse_port(true)?;

    sock::setsockopt(socket.as_raw_fd(), sock::sockopt::Ipv4PacketInfo, &true)
        .map_err(nix_to_io_error)?;

    for interface in &interfaces {
        socket.join_multicast_v4(multicast_address.ip(), &interface)?;
    }

    // On Linux we bind to the multicast address, which causes multicast packets to be filtered
    #[cfg(any(target_os = "linux", target_os = "android"))]
    socket.bind(&SocketAddr::from(multicast_address).into())?;
    // Otherwhise we bind to 0.0.0.0
    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    socket.bind(&SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), multicast_address.port()).into())?;

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

#[derive(Debug)]
pub enum Interface {
    Default,
    Ip(Ipv4Addr),
    Index(i32),
}

#[derive(Debug)]
pub struct Message {
    pub data: Vec<u8>,
    pub origin_address: SocketAddrV4,
    pub interface: Interface,
}

/// The crate `get_if_addrs` is reading the bytes of sockets on the wrong endianess on MIPS
/// So the adresses are reversed...
/// The crate `get_if_addrs` is archived and I don't have bandwidth to fork it
/// So this is a hotfix
#[cfg(target_arch = "mips")]
fn reverse_interface(interface: if_addrs::Interface) -> if_addrs::Interface {
    if_addrs::Interface {
        name: interface.name,
        addr: match interface.addr {
            if_addrs::IfAddr::V4(v4) => {
                let reversed = if_addrs::Ifv4Addr {
                    ip: reverse_address(v4.ip),
                    netmask: reverse_address(v4.netmask),
                    broadcast: v4.broadcast.map(reverse_address),
                };
                if_addrs::IfAddr::V4(reversed)
            }
            addr => addr,
        },
    }
}

#[cfg(target_arch = "mips")]
fn reverse_address(v4: Ipv4Addr) -> Ipv4Addr {
    let mut octets = v4.octets();
    octets.reverse();
    octets.into()
}

pub fn all_ipv4_interfaces() -> io::Result<Vec<Ipv4Addr>> {
    #[cfg(not(target_arch = "mips"))]
    let interfaces = if_addrs::get_if_addrs()?.into_iter();
    #[cfg(target_arch = "mips")]
    let interfaces = if_addrs::get_if_addrs()?
        .into_iter()
        .map(reverse_interface);

    let ipv4_interfaces = interfaces
        .filter_map(|i| match i.ip() {
            std::net::IpAddr::V4(v4) if !i.is_loopback() => Some(v4),
            _ => None,
        })
        .collect();
    Ok(ipv4_interfaces)
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
