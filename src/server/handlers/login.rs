use std::sync::Arc;

use anyhow::Result;
use bcrypt::DEFAULT_COST;
use tokio::sync::Mutex;

use crate::{
    client::config::Login,
    server::{userconn::UserConn, Server},
    transport::{conn::ConnWrite as _, request::ErrorLevel, Command, Request},
};

pub async fn handle_login_attempt(
    server: Arc<Mutex<Server>>,
    conn: Arc<UserConn>,
    login: Login,
) -> Result<()> {
    dbg!(&login.password);
    let db = &server.lock().await.db;
    let result = sqlx::query!(
        "SELECT id, password_hash 
                FROM Users
                WHERE username = ?",
        login.username,
    )
    .fetch_one(db)
    .await;

    let user = match result {
        Ok(result) => Some(result),
        Err(sqlx::Error::RowNotFound) => None,
        Err(err) => return Err(err.into()),
    };

    // A user with that username exists...
    if let Some(user) = user {
        if bcrypt::verify(login.password, &user.password_hash)? {
            // ...and the password is the correct
            *conn.id.lock().await = Some(user.id);
            log::info!("user {} logged in", login.username);
            return Ok(());
        } else {
            // ...and the password is incorrect
            log::info!("failed login attempt to user {}", login.username);
            conn.write(Request {
                command: Command::Error(
                    "login: wrong username or password".to_string(),
                    ErrorLevel::Fatal,
                ),
            })
            .await?;
            return Ok(());
        }
    }

    // No user with that password exists...
    let password_hash = bcrypt::hash(login.password, DEFAULT_COST)?;
    let id = sqlx::query!(
        "INSERT INTO Users VALUES (NULL, ?, ?)",
        login.username,
        password_hash
    )
    .execute(db)
    .await?
    .last_insert_rowid();

    *conn.id.lock().await = Some(id);
    log::info!("user {} registered", login.username);

    Ok(())
}
