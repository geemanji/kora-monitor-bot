use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use anyhow::Result;
use std::str::FromStr;

pub struct Db {
    pool: Pool<Sqlite>,
}

impl Db {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        
        // Run migrations programmatically
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await?;

        Ok(Self { pool })
    }

    pub async fn log_reclaimed_rent(&self, pubkey: &str, amount_lamports: u64) -> Result<()> {
        sqlx::query!(
            "INSERT INTO recovered_rent (pubkey, amount_lamports) VALUES (?, ?)",
            pubkey,
            amount_lamports as i64
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_total_recovered_sol(&self) -> Result<f64> {
        let total_lamports: Option<i64> = sqlx::query_scalar!(
            "SELECT SUM(amount_lamports) FROM recovered_rent"
        )
        .fetch_one(&self.pool)
        .await?;

        // 10^9 lamports = 1 SOL
        Ok(total_lamports.unwrap_or(0) as f64 / 1_000_000_000.0)
    }

    pub async fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query!(
            "INSERT INTO meta_state (key, value) VALUES (?, ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            key,
            value
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_meta(&self, key: &str) -> Result<Option<String>> {
        let value = sqlx::query_scalar!(
            "SELECT value FROM meta_state WHERE key = ?",
            key
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(value)
    }
}
