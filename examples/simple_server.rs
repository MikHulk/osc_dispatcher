use std::net::SocketAddr;
use std::time;

use rosc::OscPacket;

use tokio::time::sleep;

use osc_dispatcher::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("start");
    let processor = |msg: OscPacket, addr: SocketAddr| async move {
        let pause = time::Duration::from_millis(rand::random::<u8>() as u64 * 100);
        println!("{} sleep for {:?}", addr, pause);
        sleep(pause).await;
        println!("{}: {:?}", addr, msg);
    };
    run(processor).await
}
