# Project Goals & Charter

## Mission Statement

To eliminate silent capital loss for Kora operators by automating the detection and reclamation of rent-locked SOL in obsolete accounts, while prioritizing operational safety and transparency.

## Strategic Objectives

### 1. Capital Efficiency

**Goal:** Recover 100% of rent from accounts deemed "abandoned" or "obsolete" by the operator's criteria.

- **Metric:** Total SOL reclaimed vs. Total SOL locked in inactive accounts.
- **Impact:** Reduces the operational overhead of running high-volume Kora nodes.

### 2. Operational Safety

**Goal:** Zero false positives. No active user account should ever be closed accidentally.

- **Mechanism:** "Human-in-the-loop" confirmation flows, strict whitelists, and mandatory "Dry Run" simulations before execution.
- **Metric:** 0 active accounts affected during any reclamation sweep.

### 3. Enterprise Reliability

**Goal:** The system must run autonomously with 99.9% uptime, surviving restarts and network failures.

- **Mechanism:** SQLite persistence, Docker containerization, and automated background monitoring.
- **Metric:** Successful hourly checks without crash loops.

### 4. Transparency & Auditability

**Goal:** Every action taken by the bot must be traceable.

- **Mechanism:** Comprehensive logging (stdout + database) of every reclaimed lamport.
- **Metric:** 100% of balance changes can be correlated to a specific transaction signature stored in the DB.

## Guiding Principles

1. **Safety Over Speed:** It is better to leave 1 SOL locked than to accidentally close 1 active user account.
2. **Explicit Confirmation:** The bot identifies candidates; the human pulls the trigger.
3. **Production Readiness:** Code is written to be maintained, tested, and deployed via standard DevOps pipelines.
