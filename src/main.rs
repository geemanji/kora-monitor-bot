use kora_monitor_bot::config::Config;
use kora_monitor_bot::solana::client::SolanaClient;
use kora_monitor_bot::solana::reclaim::Reclaimer;
use kora_monitor_bot::state::AppState;
use kora_monitor_bot::db::Db;
use kora_monitor_bot::bot;
use anyhow::Result;
use log::{info, error};
use std::sync::Arc;
use solana_sdk::signature::read_keypair_file;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("Kora Monitor Bot starting...");
    
    let config = Config::load()?;
    
    // Initialize Database
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:kora.db".to_string());
    // Create the DB file if it doesn't exist (sqlx requires the file to exist for connection)
    if !std::path::Path::new("kora.db").exists() && database_url == "sqlite:kora.db" {
        std::fs::File::create("kora.db")?;
    }

    let db = Db::new(&database_url).await?;
    info!("Database connection established.");

    let client = SolanaClient::new(&config.rpc_url);
    
    let operator_keypair = read_keypair_file(&config.operator_keypair_path)
        .map_err(|e| anyhow::anyhow!("Failed to read keypair file: {}", e))?;
    
    let reclaimer = Reclaimer::new(operator_keypair);

    let state = Arc::new(AppState::new(client, reclaimer, config, db));

    let bg_state = state.clone();
    tokio::spawn(async move {
        bot::alerts::start_background_monitoring(bg_state).await;
    });

    bot::start_bot(state).await;

    Ok(())
}
