use tokio::io::{AsyncBufReadExt, BufReader};

pub async fn read_from_terminal() -> String {
    let mut buf = String::new();
    let mut reader = BufReader::new(tokio::io::stdin());
    match reader.read_line(&mut buf).await {
        Ok(_) => buf.trim().to_string(),
        Err(_) => String::from("Error reading from terminal"),
    }
}
