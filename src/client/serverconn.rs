use std::sync::Arc;

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
pub struct ServerConn {
    pub r: Arc<Mutex<OwnedReadHalf>>,
    pub w: Arc<Mutex<OwnedWriteHalf>>,
}

impl ServerConn {
    pub async fn send_request(&self, request: Request) -> Result<()> {
        let mut buf = Vec::new();
        request.serialize(&mut Serializer::new(&mut buf))?;

        self.w.lock().await.write_all(&buf).await?;
        Ok(())
    }
}
