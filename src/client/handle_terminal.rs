use std::sync::Arc;

use anyhow::Result;

use crate::transport::{conn::ConnWrite, Command, Request};

use super::ServerConn;

pub async fn handle_terminal(conn: Arc<ServerConn>, line: String) -> Result<()> {
    println!("Read something from terminal!");
    conn.write(Request {
        command: Command::ArchaicSendMessage(line),
    })
    .await?;
    Ok(())
}
