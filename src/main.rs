use thiserror::Error;
use tokio::net::TcpStream;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Connection {0} refused")]
    ConnectionError(String, u32),
}

#[derive(Debug)]
struct OpenPort {
    addr: String,
    port: u32,
}

async fn scan_port(addr: String, port: u32) -> Result<OpenPort, AppError> {
    let stream = TcpStream::connect(format!("{addr}:{port}"))
        .await
        .map_err(|_| AppError::ConnectionError(addr.clone(), port))?;

    Ok(OpenPort { addr, port })
}

#[tokio::main]
async fn main() {
    let mut open_ports: Vec<OpenPort> = vec![];
    let addr: String = String::from("127.0.0.1");
    for port in 1..65000 {
        let response = scan_port(addr.clone(), port).await;
        let _ = match response {
            Ok(m) => open_ports.push(m),
            Err(_) => (),
        };
    }
    dbg!(open_ports);
}
