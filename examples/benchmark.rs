use std::env;
use std::net::SocketAddr;
use std::{thread, time};

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};

use tokio::net::UdpSocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let addr = &args[1];
    let iteration: i32 = args[2].parse()?;
    let pause: time::Duration = time::Duration::from_millis(args[3].parse::<u64>()?);
    let socket = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).await?;
    socket.connect(addr).await?;
    for _ in 0..iteration {
        let osc_packet = OscPacket::Message(OscMessage {
            addr: "/greet/me".to_string(),
            args: vec![OscType::String("hi!".to_string())],
        });
        if let Ok(buf) = encoder::encode(&osc_packet) {
            println!(
                "send packet from {} to {}",
                socket.local_addr()?,
                socket.peer_addr()?
            );
            println!("{:?}", buf);
            let n = socket.send(&buf).await?;
            println!("{} transmitted", n);
        } else {
            eprintln!("encode error");
        }
        thread::sleep(pause);
    }
    Ok(())
}
