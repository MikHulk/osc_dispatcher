use std::net::SocketAddr;

use rosc::{encoder, OscPacket};

use tokio::net::UdpSocket;

use osc_dispatcher::run;

async fn reply(msg: OscPacket, dest: SocketAddr) {
    println!("echo {:?} for {}", msg, dest);
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).await {
        if let Ok(buf) = encoder::encode(&msg) {
            println!("{:?}", buf);
            match socket.send_to(&buf, dest).await {
                Ok(n) => println!("{} transmitted", n),
                Err(e) => eprintln!("transmission error: {}", e),
            }
        } else {
            eprintln!("encode error");
        }
    } else {
        eprintln!("no socket in your basquette!");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run(reply).await
}
