pub async fn connect() -> anyhow::Result<sqlx::sqlite::SqlitePool> {
    let url = std::env::var("DATABASE_URL")?;
    let pool = sqlx::sqlite::SqlitePool::connect(&url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
