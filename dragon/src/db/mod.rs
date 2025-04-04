use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub type DbPool = PgPool;

pub async fn create_pool() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool")
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    
    println!("Migrations applied successfully");
    Ok(())
} 