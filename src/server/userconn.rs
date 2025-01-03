use std::net::SocketAddr;

#[derive(Debug)]
pub struct UserConn {
    pub socket: tokio::net::TcpStream,
    pub addr: SocketAddr,
}
