#![allow(unreachable_code)]
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let _db = hop::db::connect::connect().await?;

    let bind_addr = std::env::var("SERVER_BIND_ADDR")?;
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;

    loop {
        let (mut socket, addr) = listener.accept().await?;
        let _ = socket.write_all("Welcome to hop".as_bytes()).await;

        tokio::spawn(async move {
            loop {
                let mut buf = [0u8; 1024];

                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    break;
                }

                socket.try_write(&buf).expect("failed to write data");
            }

            println!("Closed connection to {addr}.")
        });
    }

    Ok(())
}
