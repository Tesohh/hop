use anyhow::{bail, Result};
use hop::{
    client::{handle_request::handle_request, handle_terminal::handle_terminal, ServerConn},
    transport::Request,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, BufReader},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let stream = TcpStream::connect("localhost:3080")
        .await
        .expect("Unable to connect to server");

    let mut conn = ServerConn { socket: stream };

    loop {
        let handle = tokio::select! {
            line = read_from_terminal() => handle_terminal(&mut conn, line),
            request = read_from_server(&mut conn) => handle_request(&mut conn, request)
        };

        if let Err(err) = handle {
            println!("{err}");
            break;
        }
    }

    drop(conn);
    std::process::exit(0);
}

async fn read_from_terminal() -> String {
    let mut buf = String::new();
    let mut reader = BufReader::new(tokio::io::stdin());
    match reader.read_line(&mut buf).await {
        Ok(_) => buf.trim().to_string(),
        Err(_) => String::from("Error reading from terminal"),
    }
}

async fn read_from_server(conn: &mut ServerConn) -> Result<Option<Request>> {
    let mut buf = [0u8; 1024];
    let n = conn.socket.read(&mut buf).await?;

    if n == 0 {
        bail!("connection closed")
    }

    Ok(rmp_serde::from_slice(&buf).ok())
}
