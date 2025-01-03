use std::{net::SocketAddr, sync::Arc};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::Mutex,
};

use super::{userconn::UserConn, Server};

pub async fn handle_connection(server: Arc<Mutex<Server>>, socket: TcpStream, addr: SocketAddr) {
    let mut s = server.lock().await;

    let conn = Arc::new(Mutex::new(UserConn { socket, addr }));

    // add conn to the server's hashmap
    s.conns.entry(addr).or_insert(conn.clone());
    drop(s); // free mutex to the server so other connections may use it

    let mut conn = conn.lock().await;
    let _ = conn.socket.write_all("Welcome to hop\n".as_bytes()).await;

    loop {
        let mut buf = [0u8; 1024];

        let n = conn
            .socket
            .read(&mut buf)
            .await
            .expect("failed to read data from socket");

        if n == 0 {
            break;
        }

        conn.socket.try_write(&buf).expect("failed to write data");
    }

    println!("Closed connection to {}.", conn.addr)
}
