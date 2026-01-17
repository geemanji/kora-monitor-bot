# Bot Command Reference

The Kora Monitor Bot interacts via Telegram. Below is the full list of supported commands and their behaviors.

## User Commands

### `/start`
**Description:** Initializes the conversation and confirms the bot is responsive.
- **Response:** "Welcome to Kora Monitor Bot! Type /help to see available commands."

### `/help`
**Description:** Lists all available commands and brief descriptions.

### `/status`
**Description:** Provides a real-time snapshot of the Kora Node's footprint.
- **Data Displayed:**
  - **Accounts Monitored:** Total number of accounts found on-chain owned by `KORA_NODE_ID`.
  - **Total SOL Recovered:** Cumulative amount of SOL reclaimed since the database was initialized.
  - **Network:** The configured network (e.g., devnet).
- **Usage:** Run this to verify the bot can connect to the RPC and DB.

### `/reclaim`
**Description:** Triggers a manual scan for "abandoned" accounts and initiates the reclamation flow.
- **Process:**
  1. **Scan:** Queries all sponsored accounts.
  2. **Analyze:** Checks balance and data size against "abandoned" criteria (e.g., small balance, specific data patterns).
  3. **Filter:** Excludes any accounts found in the hardcoded `whitelist`.
  4. **Report:** Returns a summary message: "⚠️ Found X accounts with Y SOL."
  5. **Prompt:** Asks the user to reply with "CONFIRM" (implementation pending) to execute. *Currently runs in Dry Run mode.*

## Background Processes

### Hourly Monitor
**Description:** A background task runs every 60 minutes.
- **Logic:** Performs the same scan as `/reclaim`.
- **Alerting:** If the total potential reclaimable SOL exceeds **0.1 SOL**, it logs a warning (and can be configured to send a proactive Telegram alert).
