use std::sync::Arc;
use tokio::sync::Mutex;
use crate::solana::client::SolanaClient;
use crate::solana::reclaim::Reclaimer;
use crate::config::Config;
use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
struct PersistedState {
    total_recovered_sol: f64,
}

// Shared application state
pub struct AppState {
    pub client: SolanaClient,
    pub reclaimer: Reclaimer,
    pub config: Config,
    pub recovered_sol: Mutex<f64>,
}

impl AppState {
    pub fn new(client: SolanaClient, reclaimer: Reclaimer, config: Config) -> Self {
        let persisted_state: PersistedState = fs::read_to_string("bot_state.json")
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();

        Self {
            client,
            reclaimer,
            config,
            recovered_sol: Mutex::new(persisted_state.total_recovered_sol),
        }
    }

    pub async fn increment_recovered(&self, amount: f64) {
        let mut locked = self.recovered_sol.lock().await;
        *locked += amount;
        
        // Save to disk
        let state = PersistedState { total_recovered_sol: *locked };
        if let Ok(json) = serde_json::to_string(&state) {
            let _ = fs::write("bot_state.json", json);
        }
    }
}

