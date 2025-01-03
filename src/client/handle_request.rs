use anyhow::Result;

use crate::transport::Request;

use super::ServerConn;

pub fn handle_request(conn: &mut ServerConn, request: Result<Option<Request>>) -> Result<()> {
    request?;
    Ok(())
}
