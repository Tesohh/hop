use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub login: Login,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
