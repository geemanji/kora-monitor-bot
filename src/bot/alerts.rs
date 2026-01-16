use teloxide::prelude::*;
use crate::state::AppState;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use solana_sdk::native_token::lamports_to_sol;

pub async fn start_background_monitoring(state: Arc<AppState>) {
    let bot = Bot::new(&state.config.teloxide_token);
    // Hardcoded chat ID for alerts - in a real app, this would be stored/configured
    // For now, we assume the bot messages the user who started it or a config ID.
    // Since we don't have the chat ID from the start, we might skip sending *unsolicited* alerts
    // unless we store a chat_id from the first /start command.
    
    // For this prototype, we'll log to console if we can't send telegram messages,
    // or assume the user has provided a CHAT_ID in env.
    
    log::info!("Background monitoring started...");

    loop {
        sleep(Duration::from_secs(3600)).await; // Check every hour
        
        log::info!("Running hourly rent check...");
        
        // Logic similar to Reclaim command, but automatic
        match state.client.get_sponsored_accounts(&state.config.kora_node_id) {
            Ok(raw_accounts) => {
                let analyses = state.client.analyze_accounts(raw_accounts);
                let targets = state.reclaimer.identify_reclaimable_accounts(&analyses);
                
                if !targets.is_empty() {
                    let total_sol = lamports_to_sol(targets.iter().map(|t| t.reclaimable_lamports).sum());
                    if total_sol > 0.1 { // Threshold for alert
                         log::warn!("High idle rent detected: {} SOL across {} accounts", total_sol, targets.len());
                         // bot.send_message(CHAT_ID, text).await; // Requires known Chat ID
                    }
                }
            }
            Err(e) => log::error!("Background check failed: {}", e),
        }
    }
}