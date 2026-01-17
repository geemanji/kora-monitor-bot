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

### Option 1: Dev Container (Recommended)

If you use VS Code, you can start developing instantly:

1. Open the project in VS Code.
2. Click **"Reopen in Container"** when prompted.
3. All dependencies (Rust, Solana CLI, SQLite) are pre-configured.

### Option 2: Local Setup

1. **Clone the repository**:

   ```bash
   git clone <repository-url>
   cd kora-monitor-bot
   ```

2. **Configure Environment**:

   ```bash
   cp .env.example .env
   ```

   Edit `.env` with your details (see [Configuration Reference](./docs/guides/configuration.md)).

3. **Build**:

   ```bash
   cargo build --release
   ```

## Running the Bot

Start the bot:

```bash
RUST_LOG=info cargo run
```

## Documentation

For a deeper understanding of the project, please refer to the following:

### 📘 Guides

- [**Project Goals & Charter**](./docs/PROJECT_GOALS.md) - Our mission and success metrics.
- [**Onboarding Guide**](./docs/guides/onboarding.md) - Getting started for developers.
- [**Configuration Reference**](./docs/guides/configuration.md) - Env vars and setup.
- [**Troubleshooting Runbook**](./docs/guides/troubleshooting.md) - Common errors and fixes.

### 🏗 Architecture & API

- [**Bot Command Reference**](./docs/api/commands.md) - List of Telegram commands.
- [**Rent Model Architecture**](./docs/architecture/001-rent-model.md) - How Kora rent reclamation works.
- [**Database Schema**](./docs/architecture/002-database-schema.md) - SQLite data model.

## CI/CD & Deployment

- **CI**: Automated testing and linting via [GitHub Actions](./.github/workflows/ci.yml).
- **Docker**: Fully containerized for production.

  ```bash
  docker build -t kora-monitor-bot .
  ```

- **Persistence**: Uses SQLite. Ensure `kora.db` is mounted to a persistent volume in production.
