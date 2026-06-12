pub mod schema;

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(schema::CREATE_USERS_TABLE).execute(pool).await?;
    sqlx::query(schema::CREATE_SESSIONS_TABLE).execute(pool).await?;
    sqlx::query(schema::CREATE_CALLS_TABLE).execute(pool).await?;
    sqlx::query(schema::CREATE_MESSAGES_TABLE).execute(pool).await?;
    sqlx::query(schema::CREATE_RECORDINGS_TABLE).execute(pool).await?;
    log::info!("Database migrations completed");
    Ok(())
}
