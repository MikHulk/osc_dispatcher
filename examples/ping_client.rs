use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use rosc::{decoder, encoder};
use rosc::{OscMessage, OscPacket, OscType};

use tokio::net::UdpSocket;
use tokio::task::yield_now;
use tokio::time::timeout;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = env::args().collect();
    let addr = &args[1];
    let arc_socket = Arc::new(UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).await?);
    let socket = arc_socket.clone();
    let osc_packet = OscPacket::Message(OscMessage {
        addr: "/ping".to_string(),
        args: vec![OscType::String("42".to_string())],
    });
    let buf = encoder::encode(&osc_packet)?;
    let task = tokio::spawn(async move {
        let socket = arc_socket.clone();
        let mut rbuf = [0u8; 1024];
        println!("waiting for reply on {:?}", socket);
        if let Ok((n, addr)) = socket.recv_from(&mut rbuf).await {
            println!("receive {} from {}", n, addr);
            if let Ok((_, osc_msg)) = decoder::decode_udp(&rbuf[..n]) {
                println!("{:?}", osc_msg);
            } else {
                eprintln!("error decoding incoming message");
            }
        } else {
            eprintln!("error receiving incoming message");
        }
    });
    yield_now().await;
    println!("send packet from to {}", addr);
    println!("{:?}", buf);
    let n = socket.send_to(&buf, addr).await?;
    println!("{} transmitted", n);
    if let Err(_) = timeout(Duration::from_millis(5), task).await {
        eprintln!("no data received");
    }
    Ok(())
}
