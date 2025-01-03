pub mod handle_connection;
pub mod handle_request;
pub mod userconn;

use std::{collections::HashMap, sync::Arc};

#[derive(Debug)]
pub struct Server {
    pub db: sqlx::SqlitePool,
    pub conns: HashMap<std::net::SocketAddr, Arc<userconn::UserConn>>,
}
