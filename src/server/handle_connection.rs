use std::{net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

use crate::transport::{Command, Request};

use super::{handle_request::handle_request, userconn::UserConn, Server};

pub async fn handle_connection(
    server: Arc<Mutex<Server>>,
    socket: TcpStream,
    addr: SocketAddr,
) -> anyhow::Result<()> {
    let (r, w) = socket.into_split();
    let conn = Arc::new(UserConn {
        r: Arc::new(Mutex::new(r)),
        w: Arc::new(Mutex::new(w)),
        addr,
    });

    let mut s = server.lock().await;
    s.conns.entry(addr).or_insert(conn.clone());
    drop(s); // manually unlock the mutex for the server

    let mut w = conn.w.lock().await;
    w.write_all("Welcome to hop\n".as_bytes()).await?;
    drop(w); // manually unlock the writer mutex

    loop {
        let r_locked = conn.r.clone();
        let mut r = r_locked.lock().await;

        let mut buf = [0u8; 1024];
        let n = r.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        let request: Option<Request> = rmp_serde::from_slice(&buf).ok();

        if let Some(request) = request {
            handle_request(server.clone(), conn.clone(), request).await?;
        } else {
            log::warn!("received unparsable data from {addr}");
        }
    }

    Ok(())
}
