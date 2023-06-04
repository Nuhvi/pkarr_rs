use std::net::UdpSocket;
use std::time::Instant;

mod bencode;

use crate::bencode::decode;

enum Router {
    Transmissionbt,
}

const fn get_ip(router: Router) -> &'static str {
    match router {
        Router::Transmissionbt => "87.98.162.88",
    }
}

const HOST: &'static str = get_ip(Router::Transmissionbt);
const PORT: u16 = 6881;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:6881")?;
    let addr = format!("{}:{}", HOST, PORT);
    println!("Connectiong to {}", addr);
    socket.connect(addr)?;

    let mut buf = [0; 1024];
    let start_time = Instant::now();

    socket.send(message())?;
    let (amt, _) = socket.recv_from(&mut buf)?;

    let recieved = &buf[..amt];
    let decoded = decode(recieved);
    println!(
        "Recieved {:?}, time {}",
        decoded,
        start_time.elapsed().as_secs()
    );

    Ok(())
}

fn message() -> &'static [u8] {
    return b"d1:ad2:id20:abcdefghij0123456789e1:q4:ping1:t2:aa1:y1:qe";
}
