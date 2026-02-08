# Build Stage
FROM rust:1.80-slim as builder
WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential
COPY . .
RUN cargo build --release

# Final Stage
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y libssl3 ca-certificates curl && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/cyberlab-backend /app/cyberlab-backend
COPY --from=builder /app/savegame.json /app/savegame.json || true

EXPOSE 3000
CMD ["./cyberlab-backend"]
