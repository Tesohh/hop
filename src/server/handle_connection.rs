use std::{net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

use super::{userconn::UserConn, Server};

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
        let mut buf = [0u8; 1024];

        let r_locked = conn.r.clone();
        let mut r = r_locked.lock().await;

        let n = r.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        let s = server.lock().await;
        let mut handles = vec![];

        let filtered_conns = s
            .conns
            .iter()
            .filter(|(other_addr, _)| **other_addr != addr);

        for (other_addr, conn) in filtered_conns {
            let conn = Arc::clone(conn);
            let str = format!(
                "Received msg from {other_addr}: {}",
                String::from_utf8_lossy(&buf)
            );

            handles.push(tokio::spawn(async move {
                let sock = &mut conn.w.lock().await;
                sock.write_all(str.as_bytes()).await
            }));
        }

        for handle in handles {
            handle.await??;
        }
    }

    Ok(())
}
