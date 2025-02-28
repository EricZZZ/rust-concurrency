use std::{io, net::SocketAddr};

use anyhow::Result;
use tokio::{io::AsyncWriteExt, net::TcpListener};
use tracing::{info, warn};

const BUF_SIZE: usize = 4096;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    // build a listener
    let addr = "0.0.0.0:6379";
    let listener = TcpListener::bind(addr).await?;
    info!("Mredis Listening on {}", addr);

    loop {
        let (stream, radder) = listener.accept().await?;
        info!("Accepted connection from {}", radder);
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, radder).await {
                info!("Error handling connection: {:?}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: tokio::net::TcpStream, raddr: SocketAddr) -> Result<()> {
    loop {
        stream.readable().await?;
        let mut buf = Vec::with_capacity(BUF_SIZE);

        match stream.try_read_buf(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                info!("read {} bytes", n);
                let line = String::from_utf8_lossy(&buf);
                info!("{:?}", line);
                stream.write_all(b"+OK\r\n").await?;
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    warn!("Connection {} closed", raddr);
    Ok(())
}
