use std::{net::SocketAddr, sync::Arc};

use anyhow::Result;
use rmp_serde::Serializer;
use serde::Serialize;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
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
    // TODO: make this into a trait, and also a trait to read
    pub async fn send_request(&self, request: Request) -> Result<()> {
        let w_locked = self.w.clone();
        let mut w = w_locked.lock().await;

        let mut buf = Vec::new();
        request.serialize(&mut Serializer::new(&mut buf))?;

        // TODO: Do this also on the other side
        let conv_len: u64 = buf.len().try_into()?;
        dbg!(buf.len());
        dbg!(conv_len);

        let x = conv_len.to_be_bytes();
        w.write_all(&x).await?;
        dbg!(x);

        // w.write_u64(conv_len).await?;
        //
        // // Just for debugging purposes
        // let mut len_buf = Vec::new();
        // len_buf.write_u64(conv_len).await?;
        //
        // println!(
        //     "{:8b}{:8b}{:8b}{:8b}{:8b}{:8b}{:8b}{:8b}",
        //     len_buf[0],
        //     len_buf[1],
        //     len_buf[2],
        //     len_buf[3],
        //     len_buf[4],
        //     len_buf[5],
        //     len_buf[6],
        //     len_buf[7]
        // );
        //
        // w.write_all(&buf).await?;

        let _n = w.write_all(&buf).await?;
        w.flush().await?;

        Ok(())
    }
}
