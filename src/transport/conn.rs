use std::future::Future;

use anyhow::Result;
use serde::Serialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::tcp::{OwnedReadHalf, OwnedWriteHalf},
    sync::MutexGuard,
};

use super::Request;

pub trait ConnRead {
    fn reader(&self) -> impl Future<Output = MutexGuard<'_, OwnedReadHalf>>;

    fn read(&self) -> impl Future<Output = Result<Vec<Request>>> {
        async move {
            let mut reader = self.reader().await;
            let mut requests: Vec<Request> = vec![];

            while let Ok(expected_n) = reader.read_u64().await {
                log::debug!("received something!");
                if expected_n == 0 {
                    log::debug!(".. but expected_n");
                    break;
                }

                let expected_n: usize = expected_n.try_into()?;
                anyhow::ensure!(expected_n <= 1024);
                log::debug!("expected_n is below 1024");

                let mut payload = vec![0u8; expected_n];
                let actual_n = reader.read_exact(&mut payload).await?;
                anyhow::ensure!(expected_n == actual_n);
                log::debug!("expected_n is actual_n");

                let request: Option<Request> = rmp_serde::from_slice(&payload).ok();
                match request {
                    Some(request) => requests.push(request),
                    None => log::warn!("unparsable request"),
                }

                log::debug!("everything good!");
                log::debug!("requests length: {}", requests.len());
            }

            Ok(requests)
        }
    }
}

pub trait ConnWrite {
    fn writer(&self) -> impl Future<Output = MutexGuard<'_, OwnedWriteHalf>>;

    fn write(&self, request: Request) -> impl Future<Output = Result<()>> {
        async move {
            let mut payload = Vec::new();
            request.serialize(&mut rmp_serde::Serializer::new(&mut payload))?;

            let n: u64 = payload.len().try_into()?;

            let mut buf = Vec::new();
            buf.extend_from_slice(&n.to_be_bytes());
            buf.extend_from_slice(&payload);

            let mut writer = self.writer().await;
            writer.write_all(&buf).await?;
            writer.flush().await?;

            Ok(())
        }
    }
}
