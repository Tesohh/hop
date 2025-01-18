use std::io::stdout;

use anyhow::Result;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use hop::{
    client::startuptasks,
    transport::{conn::ConnWrite, Command, Request},
};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    enable_raw_mode()?;

    let mut stdout = stdout();
    crossterm::execute!(stdout, DisableMouseCapture)?;

    let (conn, config) = tokio::join!(
        startuptasks::server_conn_task(),
        startuptasks::config_read_task()
    );

    conn.write(Request {
        command: Command::LoginAttempt(config.login),
    })
    .await?;

    let (request_tx, request_rx) = tokio::sync::mpsc::channel(4096);

    // Selecting because each of these returns an empty Result and signifies a crash.
    let result = tokio::select! {
        err = startuptasks::read_conn_task(conn.clone(), request_tx) => err,
        err = startuptasks::handle_requests_task(conn.clone(), request_rx) => err,
        err = startuptasks::read_events(conn.clone()) => err,
    };

    match result {
        Ok(_) => {
            disable_raw_mode()?;
            Err(anyhow::anyhow!(
                "somehow the result of startup tasks is Ok?"
            ))
        }
        Err(err) => {
            disable_raw_mode()?;
            Err(err)
        }
    }
}
