use std::sync::Arc;

use anyhow::Result;

use crate::transport::{Command, Request};

use super::ServerConn;

pub async fn handle_request(_conn: Arc<ServerConn>, request: Request) -> Result<()> {
    match request.command {
        Command::NoOp => Ok(()),
        Command::Error(msg, error_level) => {
            println!("{}", error_level.pretty(&msg));
            Ok(())
        }
        Command::ArchaicSendMessage(msg) => {
            println!("{}", &msg);
            Ok(())
        }
        Command::ClientDisconnect => todo!(),
        Command::ClientDisconnectWithReason(_reason) => todo!(),

        _ => Ok(()),
    }
}
