use std::{sync::Arc, time::Duration};

use hop::{
    client::{config::Login, ServerConn},
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
        command: Command::LoginAttempt(Login {
            username: "kalewi".into(),
            password: "kalewi".into(),
        }),
    })
    .await?;
    conn.write(Request {
        command: Command::ArchaicSendMessage("harri".into()),
    })
    .await?;

    tokio::time::sleep(Duration::from_millis(2000)).await;

    Ok(())
}
