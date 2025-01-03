use std::borrow::Borrow;

use anyhow::{Context, Result};
use rmp_serde::Serializer;
use serde::Serialize;
use tokio::{io::AsyncWriteExt, net::TcpStream};

#[tokio::main]
async fn main() -> Result<()> {
    let mut stream = TcpStream::connect("localhost:3080")
        .await
        .expect("Unable to connect to server");

    let req = hop::transport::Request {
        command: hop::transport::Command::ArchaicSendMessage {
            content: "Harris".into(),
            broadcast: false,
        },
    };

    let mut buf = Vec::new();
    req.serialize(&mut Serializer::new(&mut buf))?;

    stream.write_all(&buf).await?;

    Ok(())
}
