use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use rmp_serde::Serializer;
use serde::Serialize;
use tokio::{
    io::AsyncWriteExt,
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::Mutex,
};

use crate::transport::Request;

#[derive(Debug, Clone)]
pub struct UserConn {
    pub r: Arc<Mutex<OwnedReadHalf>>,
    pub w: Arc<Mutex<OwnedWriteHalf>>,
    pub addr: SocketAddr,
}

impl UserConn {
    pub async fn send_request(&self, request: Request) -> Result<()> {
        let w_locked = self.w.clone();
        let mut w = w_locked.lock().await;

        let mut buf = Vec::new();
        request.serialize(&mut Serializer::new(&mut buf))?;

        w.write_all(&buf).await?;

        Ok(())
    }
}
