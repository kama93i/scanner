use std::net::SocketAddr;

use crate::AppError;
use crate::OpenPort;
use std::net::IpAddr;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

pub async fn scan_port(addr: IpAddr, port: u16, dur: Duration) -> Result<OpenPort, AppError> {
    let sock_addr = SocketAddr::new(addr, port);
    timeout(dur, TcpStream::connect(sock_addr))
        .await
        .map_err(|_| AppError::ConnectionError(addr.to_string(), port))? // timeout error
        .map_err(|_| AppError::ConnectionError(addr.to_string(), port))?; // connection error

    Ok(OpenPort {
        addr: addr.to_string(),
        port,
    })
}

//pub async fn stealth_scan(addr: String, port: u32) -> Result<OpenPort, AppError> {todo!("Yet to be implemented")}
