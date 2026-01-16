# Kora Monitor Bot - Video Walkthrough Script

## Scene 1: Introduction (0:00 - 0:30)
- **Visual**: Show the terminal with the bot logo/startup logs.
- **Audio**: "Hi, this is [Name]. Today I'm demonstrating the Kora Monitor Bot, a tool built to help Solana operators recover rent-locked SOL from inactive sponsored accounts."
- **Action**: Run `cargo run`. Show the "Configuration loaded" message.

## Scene 2: Telegram Interaction - Status (0:30 - 1:00)
- **Visual**: Split screen with Terminal and Telegram Desktop.
- **Action**: Type `/status` in Telegram.
- **Audio**: "First, let's check our status. The bot queries the Solana RPC in real-time to see how many accounts our Node is currently sponsoring."
- **Visual**: Bot replies with "Accounts Monitored: X".

## Scene 3: The Problem - Idle Rent (1:00 - 1:45)
- **Visual**: Show a block explorer (Solscan) with a list of "abandoned" accounts (created by the test script).
- **Audio**: "Here we have several accounts that were created but never used or are now obsolete. Each holds about 0.002 SOL. This adds up."

## Scene 4: Reclaim Workflow (1:45 - 2:30)
- **Action**: Type `/reclaim` in Telegram.
- **Visual**: Bot replies "⚠️ Found 5 accounts... Reply CONFIRM".
- **Audio**: "The bot scans the accounts, filters out any on the whitelist, and identifies those meeting our 'inactive' criteria. It asks for confirmation before doing anything dangerous."

## Scene 5: Execution & Result (2:30 - 3:00)
- **Action**: (Hypothetically) Click confirm / Execute dry run log.
- **Visual**: Terminal logs show "Simulating reclaim...".
- **Audio**: "Once confirmed, the bot would execute the close instructions. For safety, we are running in Dry Run mode today, which you can see in the logs here."

## Scene 6: Conclusion (3:00 - 3:15)
- **Visual**: Display the GitHub repo URL.
- **Audio**: "This tool is open-source and helps operators keep their treasury efficient. Thanks for watching."
