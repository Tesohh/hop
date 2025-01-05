use std::sync::Arc;

use anyhow::Result;

use crate::transport::{Command, Request};

use super::ServerConn;

pub async fn handle_request(conn: Arc<ServerConn>, request: Request) -> Result<()> {
    // let request = request?.context("tried to handle None request")?;
    //
    dbg!(&request);

    match request.command {
        Command::NoOp => Ok(()),
        Command::Error(msg, error_level) => {
            println!("{}", error_level.pretty(&msg));
            Ok(())
        }
        Command::ArchaicSendMessage(msg) => Ok(()),
        Command::ClientDisconnect => todo!(),
        Command::ClientDisconnectWithReason(reason) => todo!(),

        _ => Ok(()),
    }
}
