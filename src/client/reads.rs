use crate::{client::ServerConn, transport::Request};
use anyhow::{bail, Result};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, BufReader};

pub async fn read_from_terminal() -> String {
    let mut buf = String::new();
    let mut reader = BufReader::new(tokio::io::stdin());
    match reader.read_line(&mut buf).await {
        Ok(_) => buf.trim().to_string(),
        Err(_) => String::from("Error reading from terminal"),
    }
}

pub async fn read_from_server(conn: &mut ServerConn) -> Result<Option<Request>> {
    let mut buf = [0u8; 1024];
    let n = conn.socket.read(&mut buf).await?;

    if n == 0 {
        bail!("connection closed")
    }

    Ok(rmp_serde::from_slice(&buf).ok())
}
