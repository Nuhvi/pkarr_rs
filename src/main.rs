use std::net::UdpSocket;
use std::str::from_utf8;
use std::time::Instant;

enum Router {
    Bittorrent,
    Utorrent,
    Transmissionbt,
}

const fn get_ip(router: Router) -> &'static str {
    match router {
        Router::Bittorrent => "67.215.246.10",
        Router::Utorrent => "82.221.103.244",
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
    let string = match from_utf8(recieved) {
        Ok(s) => s,
        Err(_) => {
            let hex_string: Vec<String> = recieved.iter().map(|b| format!("{:02X}", b)).collect();
            let hex_string = hex_string.join("");
            println!("Invalid UTF-8 response: {}", hex_string);
            ""
        }
    };

    println!("Received: {string}");

    if recieved == b"pong" {
        let elapsed_time = start_time.elapsed();
        println!("Ping time: {:?}", elapsed_time);
    }

    Ok(())
}

fn message() -> &'static [u8] {
    return b"d1:ad2:id20:abcdefghij0123456789e1:q4:ping1:t2:aa1:y1:qe";
}
