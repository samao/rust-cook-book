use std::{error::Error, io::Read, net::{Ipv4Addr, SocketAddrV4, TcpListener}};

fn main() -> Result<(), Box<dyn Error>> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 6666);
    let listener = TcpListener::bind(socket)?;

    let port = listener.local_addr()?;

    println!("Listening on {}, access this port to end the program", port);

    let (mut tcp_stream, addr) = listener.accept()?;

    println!("Connection received! {:?} is sending data.",  addr);

    let mut input = vec![];
    let _ = tcp_stream.read_to_end(&mut input)?;
    println!("{:?} says {:?}", addr, input);
    println!("remote<{:?}>: {:?}", addr, String::from_utf8_lossy(&input));
    Ok(())
}
