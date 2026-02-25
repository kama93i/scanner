use clap::Parser;
use futures::future::join_all;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

#[derive(Parser, Debug)]
#[command(name = "scanner")]
#[command(about= "A fast async port scanner", long_about=None)]
struct Args {
    #[arg(short, long)]
    address: String,

    #[arg(short, long, default_value = "1-65535", value_parser = port_parser )]
    ports: (u16, u16),
}

fn port_parser(s: &str) -> Result<(u16, u16), AppError> {
    let split: Vec<&str> = s.split("-").collect();

    if split.len() != 2 {
        return Err(AppError::InputError(String::from("Not a valid range")));
    }
    let start: u16 = split[0]
        .parse::<u16>()
        .map_err(|_| AppError::InputError(String::from("Start value is not valid")))?;

    let end: u16 = split[1]
        .parse::<u16>()
        .map_err(|_| AppError::InputError(String::from("End value is not valid")))?;

    if start > end {
        return Err(AppError::InputError(String::from("Not a valid range")));
    }
    Ok((start, end))
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Connection {0}:{1} refused")]
    ConnectionError(String, u16),

    #[error("Invalid input {0}")]
    InputError(String),
}

#[derive(Debug)]
struct OpenPort {
    addr: String,
    port: u16,
}

async fn scan_port(addr: String, port: u16) -> Result<OpenPort, AppError> {
    timeout(
        Duration::from_secs(1),
        TcpStream::connect(format!("{addr}:{port}")),
    )
    .await
    .map_err(|_| AppError::ConnectionError(addr.clone(), port))? // timeout error
    .map_err(|_| AppError::ConnectionError(addr.clone(), port))?; // connection error

    Ok(OpenPort { addr, port })
}

// async fn stealth_scan(addr: String, port: u32) -> Result<OpenPort, AppError> {}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let (start, end) = args.ports;

    let tasks: Vec<_> = (start..=end)
        .map(|port| scan_port(args.address.clone(), port))
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
