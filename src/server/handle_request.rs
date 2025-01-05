use std::sync::Arc;

use anyhow::Result;
use tokio::sync::Mutex;

use crate::transport::{request::ErrorLevel, Command, Request};

use super::{userconn::UserConn, Server};

pub async fn handle_request(
    _server: Arc<Mutex<Server>>,
    conn: Arc<UserConn>,
    request: Request,
) -> Result<()> {
    match request.command {
        Command::NoOp => Ok(()),

        Command::ArchaicSendMessage(content) => {
            log::info!("{}", String::from_utf8_lossy(&content));
            Ok(())
        }

        _ => {
            conn.send_request(Request {
                command: Command::Error("unknown command".into(), ErrorLevel::Error),
            })
            .await
        }
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
