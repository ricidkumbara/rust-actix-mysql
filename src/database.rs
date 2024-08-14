use sqlx::MySqlPool;

pub async fn database_connection() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://root:root@localhost:3306/rnd").await
}