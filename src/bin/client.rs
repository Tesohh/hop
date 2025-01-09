use anyhow::Result;

use hop::client::startuptasks;

#[tokio::main]
async fn main() -> Result<()> {
    let (conn, _config) = tokio::join!(
        startuptasks::server_conn_task(),
        startuptasks::config_read_task()
    );

    let (request_tx, request_rx) = tokio::sync::mpsc::channel(4096);

    // Selecting because each of these returns an empty Result and signifies a crash.
    let result = tokio::select! {
        err = startuptasks::read_conn_task(conn.clone(), request_tx) => err,
        err = startuptasks::read_and_handle_terminal_task(conn.clone()) => err,
        err = startuptasks::handle_requests_task(conn.clone(), request_rx) => err
    };

    match result {
        Ok(_) => Err(anyhow::anyhow!(
            "somehow the result of startup tasks is Ok?"
        )),
        Err(err) => Err(err),
    }
}
