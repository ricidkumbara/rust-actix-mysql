use sqlx::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect(std::env::var("DATABASE_URL").expect("Databse URL must be set").as_str()).await
}