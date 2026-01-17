# Onboarding Guide

Welcome to the Kora Monitor Bot project! This guide will help you set up your development environment and understand our production standards.

## Development Environment Setup

### Option 1: Dev Container (Recommended)
The easiest way to get started is using VS Code and Docker.
1. Open the project in VS Code.
2. When prompted, click **"Reopen in Container"**.
3. All dependencies (Rust, Solana CLI, SQLx) will be pre-installed automatically.

### Option 2: Local Setup
1. **Install Prerequisites**:
   - Rust (latest stable)
   - Solana CLI
   - SQLite3
   - Docker

2. **Environment Configuration**:
   - Copy `.env.example` to `.env`.
   - Ensure your `OPERATOR_KEYPAIR_PATH` points to a valid Devnet keypair.

3. **Database Initialization**:
   - The bot will automatically create `kora.db` and run migrations on startup.

## Project Structure

- `src/solana/`: Logic for blockchain interaction.
- `src/bot/`: Telegram bot command handlers and background alerts.
- `src/db.rs`: Database schema and queries (SQLx).
- `src/state.rs`: Shared application state.
- `migrations/`: SQL migration files.

## Coding Standards

- **Error Handling**: Always use `anyhow::Result` for application-level logic.
- **Persistence**: Never store business-critical state in memory or flat files. Use the SQLite database.
- **Safety**: Always include a "Dry Run" mode for destructive actions (like closing accounts).

## Testing

Run unit tests frequently:
```bash
cargo test
```

## Production Deployment

The application is containerized. To build the production image:
```bash
docker build -t kora-monitor-bot .
```
