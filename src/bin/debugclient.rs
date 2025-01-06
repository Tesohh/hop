use std::{sync::Arc, time::Duration};

use hop::{
    client::ServerConn,
    transport::{conn::ConnWrite, Command, Request},
};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let stream = TcpStream::connect("localhost:3080")
        .await
        .expect("Unable to connect to server");

    let conn = Arc::new(ServerConn::new(stream));

    conn.write(Request {
        command: Command::ArchaicSendMessage(String::from("HARRIS 1")),
    })
    .await?;
    conn.write(Request {
        command: Command::ArchaicSendMessage(String::from("HARRIS 2")),
    })
    .await?;
    conn.write(Request {
        command: Command::ArchaicSendMessage(String::from("HARRIS 3")),
    })
    .await?;
    conn.write(Request {
        command: Command::ArchaicSendMessage(String::from("HARRIS 4")),
    })
    .await?;
    conn.write(Request {
        command: Command::ArchaicSendMessage(String::from("HARRIS 5")),
    })
    .await?;

    tokio::time::sleep(Duration::from_secs(4)).await;
    Ok(())
}
