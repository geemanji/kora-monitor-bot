# Deep Dive: Kora Rent Reclamation

## The Hidden Cost of Sponsorship

Kora simplifies the user experience on Solana by allowing operators to "sponsor" accounts—paying the rent-exempt minimum so users don't have to. While this lowers barriers to entry, it introduces a subtle operational inefficiency: **Rent Leakage**.

Every account on Solana requires a small amount of SOL (approx. 0.00089 SOL for empty accounts) to remain open. When a Kora operator sponsors thousands of accounts, and a percentage of those users churn or the accounts become obsolete, that SOL remains locked on-chain. It is effectively lost capital.

## The Solution: Kora Monitor Bot

This bot provides an automated, safety-first mechanism to identify and recover this capital.

### Architecture

The bot operates as a hybrid background service and interactive tool:

1. **Observer**: It periodically queries the Solana RPC using `getProgramAccounts` to find all accounts associated with the Kora Node ID.
2. **Analyzer**: It inspects account state (data size, balance, owner) to determine if an account is "closeable."
3. **Reclaimer**: It constructs transactions to close these accounts and return the rent to the operator's treasury.

### Safety Mechanisms

Automated reclamation carries the risk of closing active user accounts. We mitigate this via:

- **Whitelists**: Critical program accounts are explicitly excluded.
- **Dry Run Mode**: Operators can scan for targets and see a report without executing transactions.
- **Human-in-the-Loop**: The Telegram interface requires explicit confirmation before any destructive action (closing accounts) is taken.

### Rent Mechanics

Solana accounts are rent-exempt if they hold 2 years' worth of rent. Closing an account removes it from the state trie and transfers the entire lamport balance to a destination address. This bot leverages this protocol feature to "garbage collect" the Kora operator's sponsored footprint.

## Future Improvements

- **Activity Indexing**: integrating with a Geyser plugin to track "last accessed" timestamps for more accurate inactivity detection.
- **Batched Transactions**: Grouping multiple closures into single transactions to save on fee costs.
