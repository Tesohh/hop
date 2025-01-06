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

    loop {
        let requests = conn.read().await?;
        log::debug!("Requests to handle: {requests:?}");
        for request in requests {
            log::debug!("attempting to handle {request:?}");
            handle_request(server.clone(), conn.clone(), request).await?
        }
    }

    // TEMP:
    #[allow(unreachable_code)]
    Ok(())
}
