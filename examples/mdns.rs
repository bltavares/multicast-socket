use std::net::{Ipv4Addr, SocketAddrV4};

use multicast_socket::MulticastSocket;

fn main() {
    let mdns_multicast_address = SocketAddrV4::new(Ipv4Addr::new(224, 0, 0, 251).into(), 5353);
    let socket = MulticastSocket::all_interfaces(mdns_multicast_address)
        .expect("could not create and bind socket");

    let data = vec![1, 2];
    socket
        .broadcast(&data)
        .expect("could not broadcast message to ips being listened");

    loop {
        if let Ok(message) = socket.receive() {
            dbg!(&message.interface);
            dbg!(&message.origin_address);

            let data = vec![9, 8, 7];
            socket
                .send(&data, &message.interface)
                .expect("could not send data");
        };
    }
}
