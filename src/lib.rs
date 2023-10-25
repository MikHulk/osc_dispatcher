use std::future::Future;
use std::net::SocketAddr;

use rosc::{decoder, OscPacket};

use tokio::net::UdpSocket;

pub async fn run<F, Fut>(process: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(OscPacket, SocketAddr) -> Fut + std::marker::Send + Copy + 'static,
    Fut: Future<Output = ()> + std::marker::Send,
{
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:9000").await {
        let mut buf = [0; 1024];
        println!("listening");
        loop {
            match socket.recv_from(&mut buf).await {
                Ok((n, addr)) => {
                    println!("receive {} from {}", n, addr);
                    match decoder::decode_udp(&buf[..n]) {
                        Ok((_, osc_msg)) => {
                            tokio::spawn(async move {
                                process(osc_msg, addr).await;
                            });
                        }
                        _ => {
                            eprintln!("decoding error");
                        }
                    }
                }
                Err(e) => eprintln!("decoding error {:?}", e),
            }
        }
    } else {
        eprintln!("no socket baby");
        Ok(())
    }
}
