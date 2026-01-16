# 0001-prd-kora-monitor-bot.md

## Introduction / Overview
Kora enables Solana apps to sponsor transaction fees and account creation. However, this often results in SOL being locked as rent in accounts that eventually become inactive or are no longer needed. The **Kora Monitor Bot** is a Rust-based background service and Telegram bot designed to help Kora operators track, manage, and reclaim this "rent-locked" SOL, preventing silent capital loss.

## Goals
- **Automated Monitoring:** Continuously track accounts sponsored by a specific Kora node.
- **Rent Recovery:** Identify and reclaim SOL from accounts that are eligible for closure or cleanup.
- **Operational Transparency:** Provide real-time alerts and status reports via Telegram.
- **Safety:** Implement filters and thresholds to ensure active accounts are never prematurely reclaimed.

## User Stories
1. **As an Operator**, I want to receive a Telegram notification when a significant amount of rent becomes eligible for reclaim so that I can take action.
2. **As an Operator**, I want the bot to automatically reclaim rent for specific low-risk account types (e.g., failed initializations) to save time.
3. **As an Operator**, I want to send a `/status` command to the bot to see a summary of locked vs. reclaimed SOL.
4. **As an Operator**, I want to see a clear audit trail of why an account was closed and where the SOL was sent.

## Functional Requirements
1. **Account Monitoring:**
   - Fetch sponsored accounts using Kora node metadata and Solana RPC.
   - Monitor account state (balance, owner, data size).
2. **Reclaim Logic:**
   - Detect "closeable" accounts based on Kora-specific triggers or inactivity.
   - Execute `close_account` or similar instructions to return SOL to the operator's treasury.
   - Provide "Dry Run" mode to simulate reclaims without executing transactions.
3. **Telegram Interface:**
   - **Alerts:** Notify on large idle rent detection.
   - **Commands:** `/status` (current stats), `/reclaim` (trigger manual sweep), `/config` (view current thresholds).
   - **Confirmation:** Interactive buttons for "Human-in-the-loop" reclaim approvals.
4. **Reporting:**
   - Log all reclaim events with transaction signatures.
   - Maintain a local or simple persistent state of total SOL recovered.
5. **Documentation & Content:**
   - **Deep-dive Article:** A comprehensive Markdown file in the repository explaining:
     - How Kora sponsorship works.
     - Where rent locking happens.
     - The bot's architectural approach and safety mechanisms.
   - **README:** Detailed setup instructions for operators.

## Non-Goals
- Building a comprehensive web-based dashboard (focus is on CLI/Telegram).
- Supporting non-Kora sponsored accounts.
- Mainnet-ready production deployment (focus is Devnet for the bounty submission).

## Technical Considerations
- **Language:** Rust (utilizing `solana-client` and `solana-sdk`).
- **Telegram:** `teloxide` crate for the bot framework.
- **Kora Integration:** Use Kora Operator docs to identify sponsorship patterns and metadata.
- **Safety:** Implement a whitelist to protect critical program accounts.

## Success Metrics
- Successfully detect and reclaim rent from a dummy sponsored account on Devnet.
- Telegram bot correctly reports total SOL reclaimed vs. total SOL currently locked.
- Zero "false positive" reclaims of active accounts during testing.

## Open Questions
- What are the specific criteria Kora uses to mark an account as "closed" or "orphaned"?
- Does the Kora API provide a direct endpoint for listing all currently sponsored accounts for a given node?
