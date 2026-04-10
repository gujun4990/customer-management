use serde::{Deserialize, Serialize};
use sqlx::{migrate::{MigrateError, Migrator}, postgres::PgPoolOptions, PgPool};

pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, String> {
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| "DATABASE_URL must be set for PostgreSQL persistence".to_string())?;

        let max_connections = match std::env::var("DATABASE_MAX_CONNECTIONS") {
            Ok(value) => value
                .parse::<u32>()
                .map_err(|_| "DATABASE_MAX_CONNECTIONS must be a valid u32".to_string())?,
            Err(_) => 5,
        };

        Ok(Self {
            database_url,
            max_connections,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SeedSummary {
    pub users: i64,
    pub leads: i64,
    pub customers: i64,
    pub follow_ups: i64,
}

pub async fn connect_pool(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .connect(&config.database_url)
        .await
}

pub async fn migrate_database(pool: &PgPool) -> Result<(), MigrateError> {
    MIGRATOR.run(pool).await
}

pub async fn fetch_seed_summary(pool: &PgPool) -> Result<SeedSummary, sqlx::Error> {
    let users: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    let leads: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM leads")
        .fetch_one(pool)
        .await?;
    let customers: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM customers")
        .fetch_one(pool)
        .await?;
    let follow_ups: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM follow_ups")
        .fetch_one(pool)
        .await?;

    Ok(SeedSummary {
        users,
        leads,
        customers,
        follow_ups,
    })
}
