use std::{net::SocketAddr, sync::Arc};

use tokio::{
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::Mutex,
};

#[derive(Debug, Clone)]
pub struct UserConn {
    pub r: Arc<Mutex<OwnedReadHalf>>,
    pub w: Arc<Mutex<OwnedWriteHalf>>,
    pub addr: SocketAddr,
}
