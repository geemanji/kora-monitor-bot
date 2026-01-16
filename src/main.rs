mod config;
mod solana;

use config::Config;
use anyhow::Result;
use log::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize env_logger with default level "info" if RUST_LOG is not set
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    info!("Kora Monitor Bot starting...");
    
    match Config::load() {
        Ok(config) => {
            info!("Configuration loaded successfully for network: {}", config.network);
        }
        Err(e) => {
            error!("Failed to load configuration: {}. Please ensure .env is configured.", e);
        }
    }

    Ok(())
}
