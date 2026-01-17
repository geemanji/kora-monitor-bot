# Kora Monitor Bot

A Rust-based background service and Telegram bot designed to help Kora operators track, manage, and reclaim rent-locked SOL from sponsored accounts on Solana.

## Features

- **📊 Status Reporting**: View real-time stats on monitored accounts via Telegram.
- **🛡️ Safe Reclamation**: Identifies idle accounts and allows "Human-in-the-loop" confirmation before reclaiming rent.
- **🤖 Automated Alerts**: Notifies you when a significant amount of rent is sitting idle.
- **🔒 Whitelist Protection**: Ensures critical accounts are never touched.

## Prerequisites

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (latest stable)
- **Solana CLI**: [Install Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-install)
- **Telegram Bot Token**: Get one from [@BotFather](https://t.me/botfather).

## Setup & Configuration

1. **Clone the repository**:

   ```bash
   git clone <repository-url>
   cd kora-monitor-bot
   ```

2. **Configure Environment**:

   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your details:
   - `RPC_URL`: `https://api.devnet.solana.com` (for testing)
   - `TELOXIDE_TOKEN`: Your Telegram Bot Token.
   - `OPERATOR_KEYPAIR_PATH`: Path to the keypair that has authority to close accounts (e.g., `~/.config/solana/id.json`).
   - `KORA_NODE_ID`: The Program ID or Pubkey used to identify your sponsored accounts.

3. **Build**:

   ```bash
   cargo build --release
   ```

## Running the Bot

Start the bot:

```bash
RUST_LOG=info cargo run
```

### Telegram Commands

- `/status`: Check how many accounts are being monitored and total recovered SOL.
- `/reclaim`: Scan for idle accounts and request confirmation to reclaim rent.
- `/help`: Show available commands.

## Safety & Disclaimer

**Use at your own risk.**

- Always run in **Dry Run** mode or on **Devnet** first.
- Ensure your `whitelist` in the code includes any critical PDAs or program accounts.
- The bot performs destructive actions (closing accounts); once closed, data is gone forever.

## Architecture

See [DEEP_DIVE.md](./DEEP_DIVE.md) for a detailed explanation of the rent model and bot logic.
