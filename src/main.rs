use futures::future::join_all;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Connection {0}:{1} refused")]
    ConnectionError(String, u32),
}

#[derive(Debug)]
struct OpenPort {
    addr: String,
    port: u32,
}

async fn scan_port(addr: String, port: u32) -> Result<OpenPort, AppError> {
    timeout(
        Duration::from_secs(1),
        TcpStream::connect(format!("{addr}:{port}")),
    )
    .await
    .map_err(|_| AppError::ConnectionError(addr.clone(), port))? // timeout error
    .map_err(|_| AppError::ConnectionError(addr.clone(), port))?; // connection error

    Ok(OpenPort { addr, port })
}

#[tokio::main]
async fn main() {
    let addr = String::from("127.0.0.1");

    let tasks: Vec<_> = (1..=65535)
        .map(|port| scan_port(addr.clone(), port))
        .collect();

    let open_ports: Vec<OpenPort> = join_all(tasks)
        .await
        .into_iter()
        .filter_map(|result| result.ok())
        .collect();

    println!("Found {} open ports:", open_ports.len());
    for p in &open_ports {
        println!("  {}:{}", p.addr, p.port);
    }
}
