use std::sync::Arc;

use anyhow::Result;
use hop::{
    client::{
        self, handle_request::handle_request, handle_terminal::handle_terminal,
        reads::read_from_terminal, ServerConn,
    },
    transport::conn::ConnRead,
};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let server_conn_fut = async {
        let stream = TcpStream::connect("localhost:3080")
            .await
            .expect("Unable to connect to server");

        Arc::new(ServerConn::new(stream))
    };

    let config_read_fut = async {
        let file = tokio::fs::read_to_string("hop.toml").await;
        let file = match file {
            Ok(s) => s,
            Err(_) => panic!("file `hop.toml` not found"),
        };
        let config = toml::from_str::<client::config::Config>(&file);

        match config {
            Ok(c) => c,
            Err(e) => panic!("unable to parse `hop.toml`: {}", e),
        }
    };

    let (conn, _config) = tokio::join!(server_conn_fut, config_read_fut);
    let (request_tx, mut request_rx) = tokio::sync::mpsc::channel(4096);

    let conn_clone = conn.clone();
    let _read_handle = tokio::spawn(async move {
        let result = conn_clone.read(request_tx).await;
        if let Err(err) = result {
            log::error!("{err}"); // TODO: Add better error handling
        }
    });

    let conn_clone = conn.clone();
    tokio::spawn(async move {
        while let Ok(str) = read_from_terminal().await {
            let result = handle_terminal(conn_clone.clone(), str).await;
            if let Err(err) = result {
                log::error!("{err}"); // TODO: Add better error handling
                break;
            }
        }
    });

    while let Some(request) = request_rx.recv().await {
        dbg!(&request);
        handle_request(conn.clone(), request).await?;
    }

    std::process::exit(0);
}
