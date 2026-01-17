use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use crate::bot::Command;
use crate::state::AppState;
use std::sync::Arc;
use solana_sdk::native_token::lamports_to_sol;

pub async fn handle_command(
    bot: Bot, 
    msg: Message, 
    cmd: Command,
    state: Arc<AppState>
) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
        }
        Command::Status => {
            let recovered = *state.recovered_sol.lock().await;
            
            // In a real app, we'd fetch live data here.
            // For now, we'll try to fetch the sponsored accounts count.
            let account_count = match state.client.get_sponsored_accounts(&state.config.kora_node_id) {
                Ok(accounts) => accounts.len(),
                Err(_) => 0, // Fallback if RPC fails or ID invalid
            };

            let response = format!(
                "📊 **Kora Monitor Status**\n\n"
                "Accounts Monitored: {}\n"
                "Total SOL Recovered: {:.4} SOL\n"
                "Network: {}",
                account_count,
                recovered,
                state.config.network
            );
            bot.send_message(msg.chat.id, response).await?
        }
        Command::Reclaim => {
            // Trigger a manual check
            bot.send_message(msg.chat.id, "🔎 Scanning for reclaimable accounts...").await?;
            
            match state.client.get_sponsored_accounts(&state.config.kora_node_id) {
                Ok(raw_accounts) => {
                    let analyses = state.client.analyze_accounts(raw_accounts);
                    let targets = state.reclaimer.identify_reclaimable_accounts(&analyses);
                    
                    if targets.is_empty() {
                        bot.send_message(msg.chat.id, "✅ No reclaimable accounts found.").await?
                    } else {
                        let total_lamports: u64 = targets.iter().map(|t| t.reclaimable_lamports).sum();
                        let msg_text = format!(
                            "⚠️ Found {} accounts with a total of {:.4} SOL.\n"
                            "Reply with 'CONFIRM' to execute reclaim.",
                            targets.len(),
                            lamports_to_sol(total_lamports)
                        );
                        // In a fuller implementation, we would use a dialogue state machine to handle the confirmation.
                        // For this simplified version, we'll just report the findings (Dry Run).
                        bot.send_message(msg.chat.id, msg_text).await?
                    }
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("❌ Error scanning accounts: {}", e)).await?
                }
            }
        }
    };

    Ok(())
}