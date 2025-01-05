use anyhow::Result;
use hop::client::{
    self,
    handle_request::handle_request,
    handle_terminal::handle_terminal,
    reads::{read_from_server, read_from_terminal},
    ServerConn,
};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let server_conn_fut = async {
        let stream = TcpStream::connect("localhost:3080")
            .await
            .expect("Unable to connect to server");

        ServerConn { socket: stream }
    };

    let config_read_fut = async {
        let file = tokio::fs::read_to_string("hop.toml").await;
        let file = match file {
            Ok(s) => s,
            Err(_) => panic!("file `hop.toml` not found"),
        };
        let config = toml::from_str::<client::config::Config>(&file);

        match config {
            Ok(c) => c,
            Err(e) => panic!("unable to parse `hop.toml`: {}", e),
        }
    };

    let (mut conn, config) = tokio::join!(server_conn_fut, config_read_fut);

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
