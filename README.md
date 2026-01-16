# Kora Monitor Bot

A Rust-based background service and Telegram bot designed to help Kora operators track, manage, and reclaim rent-locked SOL from sponsored accounts on Solana.

## Prerequisites

Before running the bot, ensure you have the following installed:

- **Rust**: [Install Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- **Solana CLI**: [Install Solana Tool Suite](https://docs.solana.com/cli/install-solana-cli-install)
- **Telegram Bot Token**: Create a bot via [@BotFather](https://t.me/botfather) and obtain your token.

## Setup

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd kora-monitor-bot
   ```

2. **Configure environment variables**:
   Copy the example environment file and fill in your details:
   ```bash
   cp .env.example .env
   ```
   Edit `.env` and provide:
   - `RPC_URL`: Your Solana RPC endpoint (e.g., Devnet).
   - `TELOXIDE_TOKEN`: Your Telegram Bot API token.
   - `OPERATOR_KEYPAIR_PATH`: Path to your Solana operator keypair JSON file.
   - `KORA_NODE_ID`: The ID of your Kora node.
   - `NETWORK`: `devnet` or `mainnet-beta`.

3. **Install Dependencies**:
   ```bash
   cargo build
   ```

## Usage

To start the bot:
```bash
cargo run
```

## Safety Features

- **Dry Run Mode**: By default, the bot can be configured to report potential reclaims without executing them.
- **Whitelist**: Critical accounts can be whitelisted to prevent accidental closure.

## License

MIT / Apache 2.0
