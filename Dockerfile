# ── 阶段 1：构建前端 ──────────────────────────────────
FROM node:alpine AS frontend
WORKDIR /app/frontend
COPY frontend/package.json frontend/package-lock.json ./
RUN --mount=type=cache,target=/root/.npm \
    npm ci
COPY frontend/ ./
RUN npm run build

# ── 阶段 2：构建 Rust 后端 ────────────────────────────
FROM rust:slim-bookworm AS builder
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
COPY --from=frontend /app/frontend/dist/ frontend/dist/
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/app/target,sharing=locked \
    cargo build --release && \
    cp /app/target/release/oc-go-switch /usr/local/bin/

# ── 阶段 3：运行时最小镜像 ────────────────────────────
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/bin/oc-go-switch /usr/local/bin/oc-go-switch
EXPOSE 8180
ENTRYPOINT ["oc-go-switch"]
