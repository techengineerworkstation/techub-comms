# ═══════════════════════════════════════════════════════════════════
# Techub Comms — Multi-stage Docker Build
# Stage 1: Build WASM frontend with Trunk
# Stage 2: Build Rust backend
# Stage 3: Production runtime with both
# ═══════════════════════════════════════════════════════════════════

# ─── Stage 1: Build WASM Frontend ─────────────────────────────────
FROM rust:1.82-slim AS wasm-builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl wget \
    && rm -rf /var/lib/apt/lists/*

# Install wasm-bindgen-cli and trunk
RUN cargo install trunk wasm-bindgen-cli

RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY packages/shared_core/Cargo.toml packages/shared_core/Cargo.toml
COPY apps/web_leptos/Cargo.toml apps/web_leptos/Cargo.toml
COPY apps/backend_api/Cargo.toml apps/backend_api/Cargo.toml

RUN mkdir -p packages/shared_core/src apps/web_leptos/src apps/web_leptos/public apps/backend_api/src && \
    echo "pub fn placeholder(){}" > packages/shared_core/src/lib.rs && \
    echo "pub fn placeholder(){}" > apps/web_leptos/src/lib.rs && \
    echo "fn main(){}" > apps/backend_api/src/main.rs

COPY . .

WORKDIR /app/apps/web_leptos
RUN trunk build --release 2>&1 || (echo "WASM build failed" && exit 1)

# ─── Stage 2: Build Rust Backend ──────────────────────────────────
FROM rust:1.82-slim AS backend-builder

RUN apt-get update && apt-get install -y \
    pkg-config libssl-dev curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY packages/shared_core/Cargo.toml packages/shared_core/Cargo.toml
COPY apps/backend_api/Cargo.toml apps/backend_api/Cargo.toml

RUN mkdir -p packages/shared_core/src apps/backend_api/src && \
    echo "pub fn placeholder(){}" > packages/shared_core/src/lib.rs && \
    echo "fn main(){}" > apps/backend_api/src/main.rs

RUN cargo build --release --package backend_api 2>/dev/null || true

COPY . .
RUN cargo build --release --package backend_api

# ─── Stage 3: Production Runtime ──────────────────────────────────
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates libssl3 curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r techub && useradd -r -g techub -d /app techub

WORKDIR /app

# Copy backend binary
COPY --from=backend-builder /app/target/release/backend_api ./server

# Copy WASM frontend build output
COPY --from=wasm-builder /app/apps/web_leptos/dist/ ./static/

# Copy public assets
COPY apps/web_leptos/public/ ./static/

# Set permissions
RUN chown -R techub:techub /app
USER techub

ENV RUST_LOG=info
ENV PORT=3039

EXPOSE 3039

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:3039/health || exit 1

CMD ["./server"]
