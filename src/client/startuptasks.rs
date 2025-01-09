use std::sync::Arc;

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt as _, BufReader},
    net::TcpStream,
    sync::mpsc::{Receiver, Sender},
};

use crate::transport::{conn::ConnRead, Request};

use super::{
    config::Config, handle_request::handle_request, handle_terminal::handle_terminal, ServerConn,
};

pub async fn server_conn_task() -> Arc<ServerConn> {
    let stream = TcpStream::connect("localhost:3080")
        .await
        .expect("unable to connect to server");

    Arc::new(ServerConn::new(stream))
}

pub async fn config_read_task() -> Config {
    let file = tokio::fs::read_to_string("hop.toml").await;
    let file = match file {
        Ok(s) => s,
        Err(_) => panic!("file `hop.toml` not found"),
    };
    let config = toml::from_str::<Config>(&file);

    match config {
        Ok(c) => c,
        Err(e) => panic!("unable to parse `hop.toml`: {}", e),
    }
}

pub async fn read_conn_task(conn: Arc<ServerConn>, tx: Sender<Request>) -> Result<()> {
    conn.read(tx).await
}

pub async fn read_and_handle_terminal_task(conn: Arc<ServerConn>) -> Result<()> {
    loop {
        let mut buf = String::new();
        let mut reader = BufReader::new(tokio::io::stdin());
        reader.read_line(&mut buf).await?;
        let line = buf.trim().to_string();
        handle_terminal(conn.clone(), line).await?;
    }
}

pub async fn handle_requests_task(conn: Arc<ServerConn>, mut rx: Receiver<Request>) -> Result<()> {
    while let Some(request) = rx.recv().await {
        handle_request(conn.clone(), request).await?
    }

    Ok(())
}
