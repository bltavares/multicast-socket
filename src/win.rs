use std::io;
use std::mem;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::os::windows::prelude::*;
use std::ptr;

use socket2::{Domain, Protocol, Socket, Type};

use winapi::ctypes::{c_char, c_int};
use winapi::shared::inaddr::*;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::{INT, LPDWORD};
use winapi::shared::ws2def::LPWSAMSG;
use winapi::shared::ws2def::*;
use winapi::shared::ws2ipdef::*;
use winapi::um::mswsock::{LPFN_WSARECVMSG, LPFN_WSASENDMSG, WSAID_WSARECVMSG, WSAID_WSASENDMSG};
use winapi::um::winsock2 as sock;
use winapi::um::winsock2::{LPWSAOVERLAPPED, LPWSAOVERLAPPED_COMPLETION_ROUTINE, SOCKET};

fn last_error() -> io::Error {
    io::Error::from_raw_os_error(unsafe { sock::WSAGetLastError() })
}

unsafe fn setsockopt<T>(socket: RawSocket, opt: c_int, val: c_int, payload: T) -> io::Result<()>
where
    T: Copy,
{
    let payload = &payload as *const T as *const c_char;
    if sock::setsockopt(socket as _, opt, val, payload, mem::size_of::<T>() as c_int) == 0 {
        Ok(())
    } else {
        Err(last_error())
    }
}

type WSARecvMsgExtension = unsafe extern "system" fn(
    s: SOCKET,
    lpMsg: LPWSAMSG,
    lpdwNumberOfBytesRecvd: LPDWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
) -> INT;

fn locate_wsarecvmsg(socket: RawSocket) -> io::Result<WSARecvMsgExtension> {
    let mut fn_pointer: usize = 0;
    let mut byte_len: u32 = 0;

    let r = unsafe {
        sock::WSAIoctl(
            socket as _,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &mut WSAID_WSARECVMSG as *const _ as *mut _,
            mem::size_of_val(&WSAID_WSARECVMSG) as DWORD,
            &mut fn_pointer as *const _ as *mut _,
            mem::size_of_val(&fn_pointer) as DWORD,
            &mut byte_len,
            ptr::null_mut(),
            None,
        )
    };
    if r != 0 {
        return Err(io::Error::last_os_error());
    }

    if mem::size_of::<LPFN_WSARECVMSG>() != byte_len as _ {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Locating fn pointer to WSARecvMsg returned different expected bytes",
        ));
    }
    let cast_to_fn: LPFN_WSARECVMSG = unsafe { mem::transmute(fn_pointer) };

    match cast_to_fn {
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "WSARecvMsg extension not foud",
        )),
        Some(extension) => Ok(extension),
    }
}

type WSASendMsgExtension = unsafe extern "system" fn(
    s: SOCKET,
    lpMsg: LPWSAMSG,
    dwFlags: DWORD,
    lpNumberOfBytesSent: LPDWORD,
    lpOverlapped: LPWSAOVERLAPPED,
    lpCompletionRoutine: LPWSAOVERLAPPED_COMPLETION_ROUTINE,
) -> INT;

fn locate_wsasendmsg(socket: RawSocket) -> io::Result<WSASendMsgExtension> {
    let mut fn_pointer: usize = 0;
    let mut byte_len: u32 = 0;

    let r = unsafe {
        sock::WSAIoctl(
            socket as _,
            SIO_GET_EXTENSION_FUNCTION_POINTER,
            &mut WSAID_WSASENDMSG as *const _ as *mut _,
            mem::size_of_val(&WSAID_WSASENDMSG) as DWORD,
            &mut fn_pointer as *const _ as *mut _,
            mem::size_of_val(&fn_pointer) as DWORD,
            &mut byte_len,
            ptr::null_mut(),
            None,
        )
    };
    if r != 0 {
        return Err(io::Error::last_os_error());
    }

    if mem::size_of::<LPFN_WSASENDMSG>() != byte_len as _ {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Locating fn pointer to WSASendMsg returned different expected bytes",
        ));
    }
    let cast_to_fn: LPFN_WSASENDMSG = unsafe { mem::transmute(fn_pointer) };

    match cast_to_fn {
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            "WSASendMsg extension not foud",
        )),
        Some(extension) => Ok(extension),
    }
}

fn set_pktinfo(socket: RawSocket, payload: bool) -> io::Result<()> {
    unsafe { setsockopt(socket, IPPROTO_IP, IP_PKTINFO, payload as c_int) }
}

fn create_on_interfaces(
    options: crate::MulticastOptions,
    interfaces: Vec<Ipv4Addr>,
    multicast_address: SocketAddrV4,
) -> io::Result<MulticastSocket> {
    let socket = Socket::new(Domain::ipv4(), Type::dgram(), Some(Protocol::udp()))?;
    socket.set_read_timeout(Some(options.read_timeout))?;
    socket.set_multicast_loop_v4(options.loopback)?;
    socket.set_reuse_address(true)?;

    // enable fetching interface information and locate the extension function
    set_pktinfo(socket.as_raw_socket(), true)?;
    let wsarecvmsg: WSARecvMsgExtension = locate_wsarecvmsg(socket.as_raw_socket())?;
    let wsasendmsg: WSASendMsgExtension = locate_wsasendmsg(socket.as_raw_socket())?;

    // Join multicast listeners on every interface passed
    for interface in &interfaces {
        socket.join_multicast_v4(multicast_address.ip(), &interface)?;
    }

    // On Windows, unlike all Unix variants, it is improper to bind to the multicast address
    // see https://msdn.microsoft.com/en-us/library/windows/desktop/ms737550(v=vs.85).aspx
    socket.bind(&SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), multicast_address.port()).into())?;

    Ok(MulticastSocket {
        socket,
        wsarecvmsg,
        wsasendmsg,
        interfaces,
        multicast_address,
        buffer_size: options.buffer_size,
    })
}

pub struct MulticastSocket {
    socket: socket2::Socket,
    wsarecvmsg: WSARecvMsgExtension,
    wsasendmsg: WSASendMsgExtension,
    interfaces: Vec<Ipv4Addr>,
    multicast_address: SocketAddrV4,
    buffer_size: usize,
}

#[derive(Debug)]
pub enum Interface {
    Default,
    Ip(Ipv4Addr),
    Index(u32),
}

#[derive(Debug)]
pub struct Message {
    pub data: Vec<u8>,
    pub origin_address: SocketAddrV4,
    pub interface: Interface,
}

const CMSG_HEADER_SIZE: usize = mem::size_of::<WSACMSGHDR>();
const PKTINFO_DATA_SIZE: usize = mem::size_of::<IN_PKTINFO>();
const CONTROL_PKTINFO_BUFFER_SIZE: usize = CMSG_HEADER_SIZE + PKTINFO_DATA_SIZE;

pub fn all_ipv4_interfaces() -> io::Result<Vec<Ipv4Addr>> {
    let interfaces = get_if_addrs::get_if_addrs()?
        .into_iter()
        .filter_map(|i| match i.ip() {
            std::net::IpAddr::V4(v4) => Some(v4),
            _ => None,
        })
        .collect();
    Ok(interfaces)
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

impl MulticastSocket {
    pub fn receive(&self) -> io::Result<Message> {
        let mut data_buffer = vec![0; self.buffer_size];
        let mut data = WSABUF {
            buf: data_buffer.as_mut_ptr(),
            len: data_buffer.len() as u32,
        };

        let mut control_buffer = [0; CONTROL_PKTINFO_BUFFER_SIZE];
        let control = WSABUF {
            buf: control_buffer.as_mut_ptr(),
            len: control_buffer.len() as u32,
        };

        let mut origin_address: SOCKADDR = unsafe { mem::zeroed() };
        let mut wsa_msg = WSAMSG {
            name: &mut origin_address,
            namelen: mem::size_of_val(&origin_address) as i32,
            lpBuffers: &mut data,
            Control: control,
            dwBufferCount: 1,
            dwFlags: 0,
        };

        let mut read_bytes = 0;
        let r = {
            unsafe {
                (self.wsarecvmsg)(
                    self.socket.as_raw_socket() as _,
                    &mut wsa_msg,
                    &mut read_bytes,
                    ptr::null_mut(),
                    None,
                )
            }
        };

        if r != 0 {
            return Err(io::Error::last_os_error());
        }

        let origin_address = unsafe {
            socket2::SockAddr::from_raw_parts(
                &origin_address,
                mem::size_of_val(&origin_address) as i32,
            )
        }
        .as_std();

        let origin_address = match origin_address {
            Some(SocketAddr::V4(v4)) => v4,
            _ => SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0),
        };

        let mut interface = Interface::Default;
        // Ensures that the control buffer is the size of the CSMG_HEADER + the pkinto data
        if control.len as usize == CONTROL_PKTINFO_BUFFER_SIZE {
            let cmsg_header: WSACMSGHDR = unsafe { ptr::read_unaligned(control.buf as *const _) }; // TODO fix clippy warning without breaking the code
            if cmsg_header.cmsg_level == IPPROTO_IP && cmsg_header.cmsg_type == IP_PKTINFO {
                let interface_info: IN_PKTINFO =
                    unsafe { ptr::read_unaligned(control.buf.add(CMSG_HEADER_SIZE) as *const _) }; // TODO fix clippy warning without breaking the code
                interface = Interface::Index(interface_info.ipi_ifindex);
            };
        };

        Ok(Message {
            data: data_buffer[0..read_bytes as _]
                .iter()
                .map(|i| *i as u8)
                .collect(),
            origin_address,
            interface,
        })
    }

    pub fn send(&self, buf: &[u8], interface: &Interface) -> io::Result<usize> {
        let mut pkt_info: IN_PKTINFO = unsafe { mem::zeroed() };
        match interface {
            Interface::Default => {}
            Interface::Ip(address) => {
                pkt_info.ipi_addr = IN_ADDR {
                    S_un: to_s_addr(address),
                };
            }
            Interface::Index(index) => {
                pkt_info.ipi_ifindex = *index;
            }
        };

        let mut data = WSABUF {
            buf: buf.as_ptr() as *mut _,
            len: buf.len() as _,
        };

        let mut control_buffer = [0; CONTROL_PKTINFO_BUFFER_SIZE];
        let hdr = CMSGHDR {
            cmsg_len: CONTROL_PKTINFO_BUFFER_SIZE,
            cmsg_level: IPPROTO_IP,
            cmsg_type: IP_PKTINFO,
        };
        unsafe {
            ptr::copy(
                &hdr as *const _ as *const _,
                control_buffer.as_mut_ptr(),
                CMSG_HEADER_SIZE,
            );
            ptr::copy(
                &pkt_info as *const _ as *const _,
                control_buffer.as_mut_ptr().add(CMSG_HEADER_SIZE),
                PKTINFO_DATA_SIZE,
            )
        };
        let control = WSABUF {
            buf: control_buffer.as_mut_ptr(),
            len: control_buffer.len() as _,
        };

        let destination = socket2::SockAddr::from(self.multicast_address);
        let destination_address = destination.as_ptr();
        let mut wsa_msg = WSAMSG {
            name: destination_address as *mut _,
            namelen: destination.len(),
            lpBuffers: &mut data,
            Control: control,
            dwBufferCount: 1,
            dwFlags: 0,
        };

        let mut sent_bytes = 0;
        let r = unsafe {
            (self.wsasendmsg)(
                self.socket.as_raw_socket() as _,
                &mut wsa_msg,
                0,
                &mut sent_bytes,
                ptr::null_mut(),
                None,
            )
        };
        if r != 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(sent_bytes as _)
    }

    pub fn broadcast(&self, buf: &[u8]) -> io::Result<()> {
        for interface in &self.interfaces {
            self.send(buf, &Interface::Ip(*interface))?;
        }
        Ok(())
    }
}

fn to_s_addr(addr: &Ipv4Addr) -> in_addr_S_un {
    let octets = addr.octets();
    let res = u32::from_ne_bytes(octets);
    let mut new_addr: in_addr_S_un = unsafe { mem::zeroed() };
    unsafe { *(new_addr.S_addr_mut()) = res };
    new_addr
}
