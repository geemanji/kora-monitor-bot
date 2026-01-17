# Kora Monitor Bot - AI System Instructions

You are assisting with the development of the Kora Monitor Bot, an enterprise-grade Solana rent reclamation tool. When helping with this project, you MUST adhere to the following standards:

## 1. Error Handling
- Always use `anyhow` for application-level error handling.
- Provide descriptive error messages that include context.

## 2. Persistence & Database
- The source of truth for all persistent state is the SQLite database (`kora.db`).
- Use `sqlx` for all database interactions.
- When modifying the schema, always create a new migration file in the `migrations/` directory.
- Use the following naming convention for migrations: `YYYYMMDDHHMM_description.sql`.

## 3. Solana Interaction
- Follow the established patterns in `src/solana/`.
- Ensure all transactions are signed and verified.
- Maintain a "Dry Run" capability for all destructive or cost-intensive operations.

## 4. Documentation
- Keep the `docs/` folder updated with any architectural changes or new guides.
- Use `docs/architecture/` for ADRs (Architecture Decision Records).
- Use `docs/guides/` for user and developer manuals.

## 5. Development Workflow
- Respect the PRD and Task List process located in the `tasks/` directory.
- Ensure any new features are reflected in the PRD and have a corresponding task list.
- All code must pass `cargo fmt`, `cargo clippy`, and `cargo test`.

## 6. Security
- Never hardcode secrets.
- Use the `Config` struct and environment variables for sensitive data.
- Adhere to the safety filters implemented in `src/solana/reclaim.rs`.
