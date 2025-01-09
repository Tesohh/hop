#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub hashed_password: String,
}
