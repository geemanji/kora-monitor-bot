# Configuration Reference

The Kora Monitor Bot is configured entirely via environment variables. These can be set in a `.env` file for local development or injected into the Docker container in production.

## Core Variables

| Variable | Required | Description | Example |
|----------|:--------:|-------------|---------|
| `RPC_URL` | Yes | The Solana JSON RPC endpoint to query. | `https://api.mainnet-beta.solana.com` |
| `TELOXIDE_TOKEN` | Yes | The API Token for your Telegram Bot (from @BotFather). | `123456:ABC-DEF1234ghIkl-zyx57W2v1u123ew11` |
| `OPERATOR_KEYPAIR_PATH` | Yes | Absolute path to the `.json` keypair file that has authority to close accounts. | `/app/secrets/operator-keypair.json` |
| `KORA_NODE_ID` | Yes | The Public Key (Program ID or Account) used to filter sponsored accounts. | `Bcp3LrVwRFGkkNdVTMmtqv7427BmrXTkrCsGDtHEpvsV` |

## Advanced / Optional

| Variable | Default | Description |
|----------|---------|-------------|
| `DATABASE_URL` | `sqlite:kora.db` | Connection string for the database. Use `sqlite:` prefix. |
| `RUST_LOG` | `info` | Logging verbosity. Options: `error`, `warn`, `info`, `debug`, `trace`. |
| `NETWORK` | `devnet` | Label used in status reports (visual only). |

## Security Notes

- **Secrets:** Never commit your `.env` file or `operator-keypair.json` to version control.
- **RPC URLs:** Avoid using public RPC URLs in production as they have strict rate limits that will cause the bot to crash or lag. Use a dedicated provider (Helius, Alchemy, QuickNode).
