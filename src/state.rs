use crate::solana::client::SolanaClient;
use crate::solana::reclaim::Reclaimer;
use crate::config::Config;
use crate::db::Db;

// Shared application state
pub struct AppState {
    pub client: SolanaClient,
    pub reclaimer: Reclaimer,
    pub config: Config,
    pub db: Db,
}

impl AppState {
    pub fn new(client: SolanaClient, reclaimer: Reclaimer, config: Config, db: Db) -> Self {
        Self {
            client,
            reclaimer,
            config,
            db,
        }
    }

    /// Fetches the total recovered SOL from the database (source of truth).
    pub async fn get_total_recovered(&self) -> f64 {
        self.db.get_total_recovered_sol().await.unwrap_or(0.0)
    }

    /// Logs a new recovery event.
    pub async fn log_recovery(&self, pubkey: &str, amount_lamports: u64) {
        if let Err(e) = self.db.log_reclaimed_rent(pubkey, amount_lamports).await {
            log::error!("Failed to log recovery to DB: {}", e);
        }
    }
}