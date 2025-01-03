#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
}
