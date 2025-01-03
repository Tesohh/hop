use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Command {
    NoOp,
    ArchaicSendMessage(#[serde(with = "serde_bytes")] Vec<u8>),

    SendMessage {
        channel_id: u64,
        #[serde(with = "serde_bytes")]
        content: Vec<u8>,
    },

    ClientDisconnect,
    ClientDisconnectWithReason(#[serde(with = "serde_bytes")] Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub command: Command,
}
