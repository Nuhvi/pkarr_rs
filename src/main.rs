use std::net::UdpSocket;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:9990")?;
    socket.connect("167.86.102.121:9998")?;

    let mut buf = [0; 1024];
    let start_time = Instant::now();

    socket.send(b"ping")?;
    let (amt, _) = socket.recv_from(&mut buf)?;

    if &buf[..amt] == b"pong" {
        let elapsed_time = start_time.elapsed();
        println!("Ping time: {:?}", elapsed_time);
    }

    Ok(())
}
