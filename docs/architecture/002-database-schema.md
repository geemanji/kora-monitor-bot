# Database Schema (SQLite)

The bot uses an embedded SQLite database (`kora.db`) to persist state. This ensures that historical data—specifically the total amount of SOL recovered—is preserved across restarts.

## Tables

### `recovered_rent`

Records individual reclamation events. This acts as an immutable ledger of the bot's actions.

| Column | Type | Description |
|--------|------|-------------|
| `id` | `INTEGER PRIMARY KEY` | Auto-incrementing unique ID. |
| `pubkey` | `TEXT` | The Solana Public Key of the account that was closed. |
| `amount_lamports` | `INTEGER` | The amount of rent recovered (in lamports). |
| `reclaimed_at` | `TIMESTAMP` | (Default: `CURRENT_TIMESTAMP`) When the action occurred. |

### `meta_state`

A generic key-value store for system-wide configuration or persistent counters that don't fit into the event log.

| Column | Type | Description |
|--------|------|-------------|
| `key` | `TEXT PRIMARY KEY` | Unique identifier (e.g., `last_scan_height`). |
| `value` | `TEXT` | The stored value. |

## Migrations

Database schema changes are managed via `sqlx`.

- **Location:** `migrations/` directory.
- **Execution:** Migrations are applied automatically when the bot starts via `src/db.rs`.

## Connecting Manually

You can inspect the database using the `sqlite3` CLI tool:

```bash
sqlite3 kora.db
> SELECT * FROM recovered_rent;
```
