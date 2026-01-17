mod config;
mod solana;
mod bot;
mod state;

use config::Config;
use solana::client::SolanaClient;
use solana::reclaim::Reclaimer;
use state::AppState;
use anyhow::Result;
use log::info;
use std::sync::Arc;
use solana_sdk::signature::read_keypair_file;

#[tokio::main]
async fn main() -> Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("Kora Monitor Bot starting...");
    
    let config = Config::load()?;
    
    // Initialize Solana Client
    let client = SolanaClient::new(&config.rpc_url);
    
    // Initialize Reclaimer
    // Note: We need to handle the keypair loading safely
    let operator_keypair = read_keypair_file(&config.operator_keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;
    
    let reclaimer = Reclaimer::new(operator_keypair);

    let state = Arc::new(AppState::new(client, reclaimer, config));

    // Spawn background task
    let bg_state = state.clone();
    tokio::spawn(async move {
        bot::alerts::start_background_monitoring(bg_state).await;
    });

    // Start Telegram bot (blocks main thread)
    bot::start_bot(state).await;

    Ok(())
}
