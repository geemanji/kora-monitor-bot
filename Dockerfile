# Stage 1: Builder
FROM rust:1.75-bookworm as builder

WORKDIR /usr/src/app
COPY . .

# We need libssl-dev for solana-client/sqlx native-tls
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*

# Build the application
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/local/bin

# Copy the binary from the builder
COPY --from=builder /usr/src/app/target/release/kora-monitor-bot .
COPY --from=builder /usr/src/app/migrations ./migrations

# Expose nothing - this is a bot, not a web server
# But we might want to expose a health check port in the future

CMD ["./kora-monitor-bot"]
