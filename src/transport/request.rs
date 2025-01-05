use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    NoOp,
    Error(#[serde(with = "serde_bytes")] Vec<u8>, ErrorLevel),
    ArchaicSendMessage(#[serde(with = "serde_bytes")] Vec<u8>),
    SendMessage {
        channel_id: u64,
        #[serde(with = "serde_bytes")]
        content: Vec<u8>,
    },
    ClientDisconnect,
    ClientDisconnectWithReason(#[serde(with = "serde_bytes")] Vec<u8>),
    Register,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub command: Command,
}
