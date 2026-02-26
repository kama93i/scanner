mod error;
mod models;
mod scan;

use std::{sync::Arc, time::Duration};

use clap::Parser;
use error::AppError;
use futures::future::join_all;
use models::OpenPort;
use scan::scan_port;

#[derive(Parser, Debug)]
#[command(name = "scanner")]
#[command(about= "A fast async port scanner", long_about=None)]
struct Args {
    #[arg(short, long)]
    address: String,

    #[arg(short, long, default_value = "1-65535", value_parser = port_parser )]
    ports: (u16, u16),

    #[arg(short, long, default_value = "200")]
    timeout: u64,

    #[arg(short, long, default_value = "1000")]
    batch_size: usize,
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
#[tokio::main]
async fn main() {
    let args = Args::parse();
    let (start, end) = args.ports;
    let addr = Arc::new(args.address);
    let dur = Duration::from_millis(args.timeout);

    eprintln!("[*] Scanning {}:{}-{}", addr, start, end);

    let mut open_ports: Vec<OpenPort> = vec![];
    let ports: Vec<u16> = (start..=end).collect();

    for chunk in ports.chunks(args.batch_size) {
        let tasks: Vec<_> = chunk
            .iter()
            .map(|&port| scan_port(Arc::clone(&addr), port, dur))
            .collect();

        let results: Vec<OpenPort> = join_all(tasks)
            .await
            .into_iter()
            .filter_map(|r| r.ok())
            .collect();

        for open in &results {
            eprintln!("[*] Found port {} open", open.port);
        }

        open_ports.extend(results);
    }

    eprintln!("[*] Scan complete. Found {} open port(s)", open_ports.len());

    for p in &open_ports {
        println!("{}:{}", p.addr, p.port);
    }
}
