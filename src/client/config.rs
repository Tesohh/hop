use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub login: Login,
}

#[derive(Debug, Deserialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}
