use owo_colors::OwoColorize as _;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
    Fatal,
}

impl ErrorLevel {
    pub fn pretty(&self, msg: &str) -> String {
        match self {
            ErrorLevel::Info => format!("Info: {msg}").blue().to_string(),
            ErrorLevel::Warning => format!("Warning: {msg}").yellow().to_string(),
            ErrorLevel::Error => format!("Error: {msg}").red().to_string(),
            ErrorLevel::Fatal => panic!("Received fatal Error message from server: {msg}"),
        }
    }
}

/// Commands that start with `Client` are
/// **ALWAYS** sent from the server to the client
#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    NoOp,
    Error(String, ErrorLevel),
    ArchaicSendMessage(String),
    SendMessage { channel_id: u64, content: String },
    ClientDisconnect,
    ClientDisconnectWithReason(String),
    LoginAttempt(crate::client::config::Login),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub command: Command,
}
