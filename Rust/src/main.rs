use std::thread::sleep;
use std::time;
use mio::{Events, Interest, Poll};

fn main() {

    let mut args = std::env::args();

    let cmd = &args.next().unwrap();

    if args.len() != 1 {
        println!("Usage: {cmd} MulticastGroupAddress");
        return;
    }

    let group_addr = &args.next().unwrap();

    // Create the UDP socket backing the QUIC connection, and register it with
        // the event loop.
    let mut socket =
        mio::net::UdpSocket::bind("0.0.0.0:9000".parse().unwrap()).unwrap();
    std::net::setsockopt(&socket.inner, c::IPPROTO_IPV6, IPV6_ADD_MEMBERSHIP, mreq);

    while true {
        let bytes = socket.send_to("Ping!".as_ref(), group_addr.parse().unwrap()).unwrap();
        println!("Sent {}", bytes);
        sleep(time::Duration::from_secs(1));
        socket.send_to("Pong!".as_ref(), group_addr.parse().unwrap());
        sleep(time::Duration::from_secs(1));
    }
}
