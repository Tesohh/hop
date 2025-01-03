pub mod handle_connection;
pub mod userconn;

use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Server {
    pub db: sqlx::SqlitePool,
    pub conns: HashMap<std::net::SocketAddr, Arc<Mutex<userconn::UserConn>>>,
}
