use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::transport::{Command, Request};

use super::{userconn::UserConn, Server};

pub async fn handle_request(
    _server: Arc<Mutex<Server>>,
    _conn: Arc<UserConn>,
    request: Request,
) -> Result<()> {
    match request.command {
        Command::NoOp => Ok(()),

        Command::ArchaicSendMessage(content) => {
            log::info!("{}", String::from_utf8_lossy(&content));
            Ok(())
        }

        Command::SendMessage {
            channel_id: _,
            content: _,
        } => todo!(),

        _ => Ok(()),
    }
}

// let s = server.lock().await;
// let mut handles = vec![];
//
// let filtered_conns = s
//     .conns
//     .iter()
//     .filter(|(other_addr, _)| **other_addr != addr);
//
// for (other_addr, conn) in filtered_conns {
//     let conn = Arc::clone(conn);
//     let str = format!(
//         "Received msg from {other_addr}: {}",
//         String::from_utf8_lossy(&buf)
//     );
//
//     handles.push(tokio::spawn(async move {
//         let sock = &mut conn.w.lock().await;
//         sock.write_all(str.as_bytes()).await
//     }));
// }
//
// for handle in handles {
//     handle.await??;
// }
