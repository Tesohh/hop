use std::{env, process, sync::Arc};

use anyhow::Result;
use tokio::sync::Mutex;

use crate::transport::{conn::ConnWrite, request::ErrorLevel, Command, Request};

use super::{handlers::login::handle_login_attempt, userconn::UserConn, Server};

// FIX: Errors disappearing (not handled, just ignored in server.rs)
pub async fn handle_request(
    server: Arc<Mutex<Server>>,
    conn: Arc<UserConn>,
    request: Request,
) -> Result<()> {
    match request.command {
        Command::NoOp => Ok(()),

        Command::ArchaicSendMessage(content) => {
            if !is_auth(&conn).await {
                return Ok(());
            }
            log::info!("{}", content);
            conn.write(Request {
                command: Command::Error("lorem impsum".into(), ErrorLevel::Info),
            })
            .await?;
            conn.write(Request {
                command: Command::Error("lorem impsum".into(), ErrorLevel::Warning),
            })
            .await?;
            conn.write(Request {
                command: Command::Error("lorem impsum".into(), ErrorLevel::Error),
            })
            .await?;
            Ok(())
        }

        Command::LoginAttempt(login) => handle_login_attempt(server, conn, login).await,

        _ => {
            conn.write(Request {
                command: Command::Error("unknown command".into(), ErrorLevel::Error),
            })
            .await
        }
    }
}

async fn is_auth(conn: &UserConn) -> bool {
    if conn.id.lock().await.is_none() {
        log::info!("connection tried to do something while not authed");
        let _ = conn // Don't really care if there's an error
            .write(Request {
                command: Command::Error("You must be logged in for this".into(), ErrorLevel::Fatal),
            })
            .await;
        return false;
    }
    true
}
