use std::sync::Arc;

use anyhow::{bail, Result};
use crossterm::event::{Event, KeyCode, KeyEvent};

use super::ServerConn;

pub async fn handle_terminal_event(conn: Arc<ServerConn>, event: Event) -> Result<()> {
    match event {
        Event::Key(key) => {
            handle_key(conn, key).await?;
        }
        _ => (),
    }
    Ok(())
}
pub async fn handle_key(conn: Arc<ServerConn>, key: KeyEvent) -> Result<()> {
    Ok(())
}
