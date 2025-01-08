use std::{net::SocketAddr, sync::Arc};

use tokio::{net::TcpStream, sync::Mutex};

use crate::transport::{
    conn::{ConnRead, ConnWrite},
    Command, Request,
};

use super::{handle_request::handle_request, userconn::UserConn, Server};

pub async fn handle_connection(
    server: Arc<Mutex<Server>>,
    socket: TcpStream,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let conn = Arc::new(UserConn::new(socket));

    let mut s = server.lock().await;
    s.conns.entry(addr).or_insert(conn.clone());
    drop(s); // manually unlock the mutex for the server

    conn.write(Request {
        command: Command::ArchaicSendMessage("Welcome".into()),
    })
    .await?;

    let (tx, mut rx) = tokio::sync::mpsc::channel(4096);

    let conn_clone = conn.clone();
    let _read_handle = tokio::spawn(async move {
        let result = conn_clone.read(tx).await;
        if let Err(err) = result {
            log::error!("{err}");
            // FIX: Handle properly
            // FIX: And disconnect the tx on EOF
        }
    });

    while let Some(request) = rx.recv().await {
        handle_request(server.clone(), conn.clone(), request).await?;
    }

    // TEMP:
    #[allow(unreachable_code)]
    Ok(())
}
