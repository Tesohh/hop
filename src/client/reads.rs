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
// FIX: Sometimes, may receive a very large number as a expected_n
// FIX: Try to make sure that packets are NEVER "merged", so that you don't even have to deal with
// this bullshit
pub async fn read_from_server(conn: Arc<ServerConn>) -> Result<Vec<Option<Request>>> {
    let mut r = conn.r.lock().await;

    let mut buf = [0u8; 4096];
    let n: u64 = r.read(&mut buf).await?.try_into()?;
    let mut requests: Vec<Option<Request>> = vec![];
    println!(
        "{:?}",
        buf.into_iter().filter(|c| *c != 0).collect::<Vec<u8>>()
    );

    // Change This To while r.read_u64()
    // The problem is that sometimes, the size prefix is sent separately to the rest
    // So the first 8 bytes are realy a string, but get interpreted as a huge numbr
    // Also make this a trait

    let mut cursor = Cursor::new(&buf);
    while cursor.position() <= n {
        let mut len_buf = [0u8; 8];

        AsyncReadExt::read_exact(&mut cursor, &mut len_buf).await?;
        let conv_n = u64::from_be_bytes(len_buf);

        let expected_n: usize = conv_n.try_into()?;
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

        if actual_n != expected_n {
            bail!("actual_n ({actual_n}) != expected_n ({expected_n})")
        }

        requests.push(rmp_serde::from_slice(&buf).ok());
    }

    Ok(requests)
}
