# ═══════════════════════════════════════════════════════════════════
# Techub Comms — Multi-stage Docker Build
# ═══════════════════════════════════════════════════════════════════

FROM rust:1.87-slim AS builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk@0.21.15 wasm-bindgen-cli@0.2.100

WORKDIR /app
COPY . .

# Build WASM frontend
WORKDIR /app/apps/web_leptos
RUN trunk build --release

# Build Rust backend
WORKDIR /app
RUN cargo build --release --package backend_api

# ═══════════════════════════════════════════════════════════════════
# Production Runtime
# ═══════════════════════════════════════════════════════════════════
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r techub && useradd -r -g techub -d /app techub

WORKDIR /app

COPY --from=builder /app/target/release/backend_api ./server
COPY --from=builder /app/apps/web_leptos/dist/ ./static/

RUN chown -R techub:techub /app
USER techub

ENV RUST_LOG=info
ENV PORT=3039

EXPOSE 3039

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3039/health || exit 1

CMD ["./server"]
