use anyhow::Result;
use rmp_serde::Serializer;
use serde::Serialize;
use tokio::{io::AsyncWriteExt, net::TcpStream};

use crate::transport::Request;

#[derive(Debug)]
pub struct ServerConn {
    pub socket: TcpStream,
}

impl ServerConn {
    pub async fn send_request(&mut self, request: Request) -> Result<()> {
        let mut buf = Vec::new();
        request.serialize(&mut Serializer::new(&mut buf))?;

        self.socket.write_all(&buf).await?;
        Ok(())
    }
}
