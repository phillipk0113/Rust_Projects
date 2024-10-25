use sqlx::sqlite::SqlitePool;
use dotenv::dotenv;
use std::env;


pub async fn get_db_pool() -> SqlitePool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqlitePool::connect(&database_url)
        .await
        .expect("Failed to create database pool")
}
