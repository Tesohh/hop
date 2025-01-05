use std::sync::Arc;

use anyhow::Result;
use hop::client::{
    self,
    handle_request::handle_request,
    handle_terminal::handle_terminal,
    reads::{read_from_server, read_from_terminal},
    ServerConn,
};
use tokio::{net::TcpStream, sync::Mutex};

#[tokio::main]
async fn main() -> Result<()> {
    let server_conn_fut = async {
        let stream = TcpStream::connect("localhost:3080")
            .await
            .expect("Unable to connect to server");

        let (r, w) = stream.into_split();

        Arc::new(ServerConn {
            r: Arc::new(Mutex::new(r)),
            w: Arc::new(Mutex::new(w)),
        })
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

    let (conn, config) = tokio::join!(server_conn_fut, config_read_fut);

    loop {
        println!("waiting for some input");
        let handle = tokio::select! {
            line = read_from_terminal() => handle_terminal(conn.clone(), line).await,
            // TODO: Cleanup
            requests = read_from_server(conn.clone()) => async {
                let requests = requests?;
                for request in requests.into_iter().flatten() {
                    handle_request(conn.clone(), request).await?;
                }
                Ok(())
            }.await
        };

        if let Err(err) = handle {
            println!("{err}");
            break;
        }
    }

    drop(conn);
    std::process::exit(0);
}
