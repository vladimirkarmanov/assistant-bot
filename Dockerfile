FROM rust:1.91.1 AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# Build dependencies, avoiding rebuilding if they haven't changed
RUN mkdir src && echo "fn main() {println!(\"Hello, world!\");}" > src/main.rs && cargo build --release --target-dir /tmp/target
# Remove the dummy src/main.rs
RUN rm -rf src

# Copy the actual source code
COPY . .

# Build the release binary
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/assistant-bot ./bot

CMD ["./bot"]
