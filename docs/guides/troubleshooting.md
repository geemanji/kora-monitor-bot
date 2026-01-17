# Troubleshooting Runbook

## Common Issues

### 1. RPC Rate Limit / "Too Many Requests"

**Symptoms:**

- Logs show `429 Too Many Requests`.
- Bot fails to start or `/status` command hangs.
- `solana airdrop` fails.

**Cause:**
You are using a public RPC endpoint (like `api.devnet.solana.com`) which has strict request limits. `getProgramAccounts` scans are heavy and often blocked.

**Resolution:**

- **Retry:** Wait 2-3 minutes and try again.
- **Upgrade:** Get a free API key from Helius, Alchemy, or QuickNode and update your `RPC_URL` in `.env`.

### 2. "Attempt to debit an account but found no record of a prior credit"

**Symptoms:**

- Logs show error `-32002`.
- `generate_test_accounts` script fails.
- Reclaim transactions fail.

**Cause:**
The `OPERATOR_KEYPAIR` has **0 SOL**. It cannot pay for transaction fees or account rent.

**Resolution:**

- Run `solana address` to get your operator address.
- Airdrop funds: `solana airdrop 1 <ADDRESS> --url devnet`.
- Or send SOL from another wallet.

### 3. Database Locked / "database is locked"

**Symptoms:**

- Bot crashes with SQLite errors.

**Cause:**
Multiple processes are trying to write to `kora.db` at the same time. This often happens if you have the bot running in a terminal AND you are trying to write to the DB manually or via another script.

**Resolution:**

- Ensure only **one instance** of the bot is running.
- If using `sqlite3` CLI, ensure you exit it properly.

### 4. "Invalid Kora Node ID"

**Symptoms:**

- Bot crashes on startup parsing config.

**Cause:**
The `KORA_NODE_ID` in `.env` is not a valid Base58 Solana Public Key.

**Resolution:**

- Check for extra spaces or typo characters in `.env`.
