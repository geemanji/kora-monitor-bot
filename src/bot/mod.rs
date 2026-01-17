pub mod commands;
pub mod alerts;

use teloxide::prelude::*;
use crate::state::AppState;
use std::sync::Arc;

pub async fn start_bot(state: Arc<AppState>) {
    let bot = Bot::new(&state.config.teloxide_token);

    log::info!("Starting Telegram bot...");

    let handler = Update::filter_message()
        .filter_command::<Command>()
        .endpoint(commands::handle_command);

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![state])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

#[derive(teloxide::utils::command::BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Kora Monitor Bot Commands:")]
pub enum Command {
    #[command(description = "start the bot.")]
    Start,
    #[command(description = "display this text.")]
    Help,
    #[command(description = "show status of locked vs reclaimed SOL.")]
    Status,
    #[command(description = "trigger a manual reclaim sweep.")]
    Reclaim,
}