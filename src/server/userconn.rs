use std::sync::Arc;

use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::{Mutex, MutexGuard},
};

use crate::transport::conn::{ConnRead, ConnWrite};

#[derive(Debug, Clone)]
pub struct UserConn {
    pub id: Arc<Mutex<Option<i64>>>, // If None, the user is unauthenticated
    r: Arc<Mutex<OwnedReadHalf>>,
    w: Arc<Mutex<OwnedWriteHalf>>,
}

impl UserConn {
    pub fn new(stream: TcpStream) -> Self {
        let (r, w) = stream.into_split();
        UserConn {
            id: Arc::new(Mutex::new(None)),
            r: Arc::new(Mutex::new(r)),
            w: Arc::new(Mutex::new(w)),
        }
    }
}

impl ConnRead for UserConn {
    async fn reader(&self) -> MutexGuard<'_, OwnedReadHalf> {
        self.r.lock().await
    }
}

impl ConnWrite for UserConn {
    async fn writer(&self) -> MutexGuard<'_, OwnedWriteHalf> {
        self.w.lock().await
    }
}
