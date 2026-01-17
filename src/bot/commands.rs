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
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome to Kora Monitor Bot! Type /help to see available commands.").await?
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?
        }
        Command::Status => {
            // Fetch total recovered from DB (async)
            let recovered = state.get_total_recovered().await;
            
            let account_count = match state.client.get_sponsored_accounts(&state.config.kora_node_id) {
                Ok(accounts) => accounts.len(),
                Err(_) => 0,
            };

            let response = format!(
                "📊 **Kora Monitor Status**\n\n\u{200b}
                Accounts Monitored: {}\n\u{200b}
                Total SOL Recovered: {:.4} SOL\n\u{200b}
                Network: {}",
                account_count,
                recovered,
                state.config.network
            );
            bot.send_message(msg.chat.id, response).await?
        }
        Command::Reclaim => {
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
                            "⚠️ Found {} accounts with a total of {:.4} SOL.\n\u{200b}
                            Reply with 'CONFIRM' to execute reclaim.",
                            targets.len(),
                            lamports_to_sol(total_lamports)
                        );
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
