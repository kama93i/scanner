use crate::AppError;
use crate::OpenPort;
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

pub async fn scan_port(addr: String, port: u16) -> Result<OpenPort, AppError> {
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
