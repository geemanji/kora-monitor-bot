use kora_monitor_bot::config::Config;
use kora_monitor_bot::solana::client::SolanaClient;
use kora_monitor_bot::solana::reclaim::Reclaimer;
use kora_monitor_bot::state::AppState;
use kora_monitor_bot::bot;
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
    
    let client = SolanaClient::new(&config.rpc_url);
    
    let operator_keypair = read_keypair_file(&config.operator_keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;
    
    let reclaimer = Reclaimer::new(operator_keypair);

    let state = Arc::new(AppState::new(client, reclaimer, config));

    let bg_state = state.clone();
    tokio::spawn(async move {
        bot::alerts::start_background_monitoring(bg_state).await;
    });

    bot::start_bot(state).await;

    Ok(())
}