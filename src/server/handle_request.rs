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
            log::info!("{}", content);
            conn.send_request(Request {
                command: Command::Error("lorem impsum".into(), ErrorLevel::Info),
            })
            .await?;
            conn.send_request(Request {
                command: Command::Error("lorem impsum".into(), ErrorLevel::Warning),
            })
            .await?;
            conn.send_request(Request {
                command: Command::Error("lorem impsum".into(), ErrorLevel::Error),
            })
            .await?;
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
