use clap::Parser;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Semaphore;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    target: String,

    #[arg(short, long, default_value_t = String::from("1-1024"))]
    port: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let ports: Vec<u32> = args
        .port
        .split("-")
        .filter_map(|s| s.parse::<u32>().ok())
        .collect();

    let semaphore = Arc::new(Semaphore::new(20));

    let (tx, mut rx) = mpsc::channel(100);
    for port in ports[0]..=ports[1] {
        let tx = tx.clone();
        let semaphore_clone = semaphore.clone();
        let target = args.target.clone();
        tokio::spawn(async move {
            let address: SocketAddr = match format!("{}:{}", target, port).parse() {
                Ok(addr) => addr,
                Err(_) => {
                    eprintln!("[-] Invalid address: {}:{}", target, port);
                    return;
                }
            };

            let _permit = semaphore_clone.acquire().await.unwrap();
            if is_port_open(address).await {
                tx.send(port).await.unwrap();
            }
        });
    }

    drop(tx);

    while let Some(r) = rx.recv().await {
        println!("[+] port {r} -> open")
    }
}

async fn is_port_open(addr: SocketAddr) -> bool {
    let time_duration = Duration::from_secs(5);

    let connection = TcpStream::connect(addr);

    matches!(
        tokio::time::timeout(time_duration, connection).await,
        Ok(Ok(_))
    )
}
