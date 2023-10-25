use std::env;
use std::net::SocketAddr;
use std::time;

use rosc::encoder;
use rosc::{OscMessage, OscPacket, OscType};

use tokio::net::UdpSocket;
use tokio::task::JoinSet;
use tokio::time::timeout;

const TIMEOUT: u64 = 300;

async fn send_msg() -> Result<time::Duration, Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<_> = env::args().collect();
    let addr = &args[1];
    let t_out = if args.len() == 5 {
        args[4].parse()?
    } else {
        TIMEOUT
    };
    let osc_packet: OscPacket = OscPacket::Message(OscMessage {
        addr: "/ping".to_string(),
        args: vec![OscType::String("42".to_string())],
    });
    let socket = UdpSocket::bind("0.0.0.0:0".parse::<SocketAddr>().unwrap()).await?;
    let buf = encoder::encode(&osc_packet)?;
    let mut rbuf = [0u8; 1024];
    let now = time::Instant::now();
    timeout(
        time::Duration::from_millis(t_out),
        socket.send_to(&buf, addr),
    )
    .await??;
    timeout(
        time::Duration::from_millis(t_out),
        socket.recv_from(&mut rbuf),
    )
    .await??;
    let end = now.elapsed();
    Ok(end)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("start");
    let args: Vec<_> = env::args().collect();
    let it: i32 = args[2].parse()?;
    let st: i32 = args[3].parse()?;
    let mut durations: Vec<time::Duration> = Vec::new();
    let mut errors = 0;
    for i in 0..it {
        println!("seq {}", i + 1);
        let mut tasks = JoinSet::new();
        for _ in 0..st {
            tasks.spawn(send_msg());
        }
        while let Some(task) = tasks.join_next().await {
            match task? {
                Ok(d) => durations.push(d),
                Err(e) => {
                    eprintln!("ERROR: {:?}", e);
                    errors += 1;
                }
            }
        }
    }
    let total: time::Duration = durations.iter().sum();
    println!("performs {} requests", it * st);
    println!("total: {:?}", total);
    println!("processed: {}", durations.len());
    println!(
        "errors: {}({:0.2}%)",
        errors,
        errors as f64 / (it * st) as f64 * 100.0
    );
    println!("mean: {:?}", total / (it * st - errors).try_into()?);
    Ok(())
}
