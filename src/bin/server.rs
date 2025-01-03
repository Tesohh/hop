use std::{collections::HashMap, sync::Arc};

use hop::server::handle_connection::handle_connection;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;
    pretty_env_logger::init();

    let db = hop::db::connect::connect().await?;

    let s = Arc::new(Mutex::new(hop::server::Server {
        db,
        conns: HashMap::new(),
    }));

    let bind_addr = std::env::var("SERVER_BIND_ADDR")?;
    let listener = tokio::net::TcpListener::bind(bind_addr).await?;

    loop {
        let (socket, addr) = listener.accept().await?;
        tokio::spawn(handle_connection(s.clone(), socket, addr));
    }

    #[allow(unreachable_code)]
    Ok(())
}
