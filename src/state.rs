use std::sync::Arc;
use tokio::sync::Mutex;
use crate::solana::client::SolanaClient;
use crate::solana::reclaim::Reclaimer;
use crate::config::Config;

// Shared application state
pub struct AppState {
    pub client: SolanaClient,
    pub reclaimer: Reclaimer,
    pub config: Config,
    pub recovered_sol: Mutex<f64>, // Simple tracker for now
}

impl AppState {
    pub fn new(client: SolanaClient, reclaimer: Reclaimer, config: Config) -> Self {
        Self {
            client,
            reclaimer,
            config,
            recovered_sol: Mutex::new(0.0),
        }
    }
}
