use std::{
    io::{Cursor, Read},
    sync::Arc,
};

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

// TODO: Move this into a function that may be used by server too
pub async fn read_from_server(conn: Arc<ServerConn>) -> Result<Vec<Option<Request>>> {
    let mut r = conn.r.lock().await;

    let mut buf = [0u8; 1024];
    let n: u64 = r.read(&mut buf).await?.try_into()?;

    let mut requests: Vec<Option<Request>> = vec![];

    let mut cursor = Cursor::new(&buf);
    while cursor.position() <= n {
        let expected_n: usize = cursor.read_u64().await?.try_into()?;
        if expected_n == 0 {
            break;
        }

        if expected_n > 1024 {
            println!("Received a HUGE packet! ({expected_n} bytes)");
            continue;
        }

        let mut buf = vec![0u8; expected_n];
        let actual_n = AsyncReadExt::read_exact(&mut cursor, &mut buf).await?;

        if actual_n == 0 {
            break;
        }

        dbg!(expected_n, actual_n);

        if actual_n != expected_n {
            bail!("actual_n ({actual_n}) != expected_n ({expected_n})")
        }

        requests.push(rmp_serde::from_slice(&buf).ok());
    }

    Ok(requests)
}
