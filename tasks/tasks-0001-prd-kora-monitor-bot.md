# Tasks: Kora Monitor Bot

## Relevant Files
- `Cargo.toml` - Project dependencies (solana-client, teloxide, tokio, etc.).
- `src/main.rs` - Entry point, initializes bot and background tasks.
- `src/config.rs` - Environment configuration loader.
- `.env.example` - Template for environment variables.
- `README.md` - Setup and usage instructions.

### Notes
- Ensure you have a Solana CLI keypair or a configured `.env` with a private key for signing transactions.
- Tests should be added in a `tests` module within each file or in a separate `tests/` directory for integration tests.
- Use `cargo test` to run tests.

## Tasks

- [x] 1.0 Project Initialization & Infrastructure Setup
    - [x] 1.1 Initialize a new Rust project (`cargo new kora-monitor-bot`).
    - [x] 1.2 Configure `Cargo.toml` with dependencies: `solana-client`, `solana-sdk`, `teloxide`, `tokio`, `dotenv`, `serde`, `log`, `env_logger`.
    - [x] 1.3 Create `src/config.rs` to load environment variables (RPC URL, Telegram Token, Operator Keypair path, Kora Node ID, Network/Devnet).
    - [x] 1.4 Setup logging infrastructure (`env_logger`) in `src/main.rs`.
    - [x] 1.5 Create a basic `README.md` with prerequisite installation instructions (Rust, Solana CLI).

- [ ] 2.0 Solana Integration & Account Monitoring
    - [ ] 2.1 Implement `src/solana/client.rs` to establish a connection to Solana Devnet RPC.
    - [ ] 2.2 Create a function to fetch all accounts owned by or associated with the configured Kora Node (using `getProgramAccounts` with filters).
    - [ ] 2.3 Implement logic to parse account data (balance, rent-exempt status, data size).
    - [ ] 2.4 Create a dummy/test script to fund and create "abandoned" accounts on Devnet for testing purposes.
    - [ ] 2.5 Write tests to verify account fetching and parsing logic.

- [ ] 3.0 Rent Reclaim Logic & Safety Filters
    - [ ] 3.1 Implement `src/solana/reclaim.rs` to identify "closeable" accounts based on criteria (e.g., zero balance token accounts, specific data patterns indicating inactivity).
    - [ ] 3.2 Implement a "Dry Run" function that simulates the transaction and logs what *would* happen without broadcasting.
    - [ ] 3.3 Implement the actual `close_account` transaction builder to transfer rent back to the operator.
    - [ ] 3.4 Add safety checks: Whitelist specific addresses/programs that should *never* be touched.
    - [ ] 3.5 Write unit tests for the filtering logic to ensure active accounts are NOT flagged for reclaim.

- [ ] 4.0 Telegram Bot Implementation
    - [ ] 4.1 Initialize the `teloxide` bot in `src/bot/mod.rs` and connect it to the main Tokio runtime.
    - [ ] 4.2 Implement `/status` command in `src/bot/commands.rs` to query the Solana module and report locked vs. reclaimed SOL.
    - [ ] 4.3 Implement `/reclaim` command with an interactive confirmation flow ("Are you sure? [Yes/No]").
    - [ ] 4.4 Implement background monitoring task in `src/main.rs` that periodically checks for reclaimable rent and sends an alert via `src/bot/alerts.rs`.
    - [ ] 4.5 Connect the "Dry Run" mode to the bot's initial alert (e.g., "Found 5 accounts with 0.05 SOL. Reclaim?").

- [ ] 5.0 Documentation, Reporting & Deep-Dive Article
    - [ ] 5.1 Implement a simple file-based logger or struct in `src/state.rs` to persist the total amount of SOL reclaimed across restarts.
    - [ ] 5.2 Write `DEEP_DIVE.md`: Explain Kora sponsorship, the rent model, and how this bot solves the leakage problem safely.
    - [ ] 5.3 Finalize `README.md` with clear steps: "How to run on Devnet", "How to configure Telegram", "Safety Warnings".
    - [ ] 5.4 Record a short video walkthrough or script the walkthrough as requested by the bounty (optional but recommended for completeness).
