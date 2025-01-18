use std::sync::Arc;

use tokio::{
    net::{
        tcp::{OwnedReadHalf, OwnedWriteHalf},
        TcpStream,
    },
    sync::{Mutex, MutexGuard},
};

use crate::transport::conn::{ConnRead, ConnWrite};

use super::screen::{Mode, Screen};

#[derive(Debug, Clone)]
pub struct ServerConn {
    r: Arc<Mutex<OwnedReadHalf>>,
    w: Arc<Mutex<OwnedWriteHalf>>,
    pub screen: Arc<Mutex<Screen>>, // im so SORRY
}

impl ServerConn {
    pub fn new(stream: TcpStream) -> Self {
        let (r, w) = stream.into_split();
        ServerConn {
            r: Arc::new(Mutex::new(r)),
            w: Arc::new(Mutex::new(w)),
            screen: Arc::new(Mutex::new(Screen { mode: Mode::Normal })),
        }
    }
}

impl ConnRead for ServerConn {
    async fn reader(&self) -> MutexGuard<'_, OwnedReadHalf> {
        self.r.lock().await
    }
}

impl ConnWrite for ServerConn {
    async fn writer(&self) -> MutexGuard<'_, OwnedWriteHalf> {
        self.w.lock().await
    }
}
