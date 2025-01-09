use anyhow::Result;
use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn read_from_terminal() -> Result<String> {
    let mut buf = String::new();
    let mut reader = BufReader::new(tokio::io::stdin());
    match reader.read_line(&mut buf).await {
        Ok(_) => Ok(buf.trim().to_string()),
        Err(err) => Err(err.into()),
    }
}
