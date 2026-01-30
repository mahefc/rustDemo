use sqlx::{postgres::PgPoolOptions, PgPool};
use std::sync::OnceLock;
use std::time::Duration;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub async fn get_pool() -> Result<&'static PgPool, sqlx::Error> {
    if let Some(pool) = DB_POOL.get() {
        return Ok(pool);
    }
    let dialect = "postgres";
    let host = std::env::var("PG_HOST").expect("PG_HOST must be set");
    let username = std::env::var("PG_USERNAME").expect("PG_USERNAME must be set");
    let password = std::env::var("PG_PASSWORD").expect("PG_PASSWORD must be set");
    let schema = std::env::var("PG_SCHEMA").expect("PG_SCHEMA must be set");

    let database_url = format!("{}://{}:{}@{}/{}", dialect, username, password, host, schema);

   let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await?;
    println!("✅ Connected to PostgreSQL at {}", database_url);
    DB_POOL.set(pool).ok();
    Ok(DB_POOL.get().unwrap())
}