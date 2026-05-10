# syntax=docker/dockerfile:1.7

# ── Stage 1: build the WASM bundle ───────────────────────────────────────
FROM rust:1.83-slim AS build
WORKDIR /app

RUN rustup target add wasm32-unknown-unknown \
 && cargo install --locked trunk

COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
COPY src ./src
COPY index.html styles.css Trunk.toml ./

RUN trunk build --release --public-url /

# ── Stage 2: serve dist/ ─────────────────────────────────────────────────
FROM caddy:2-alpine
COPY --from=build /app/dist /srv
COPY deploy/Caddyfile /etc/caddy/Caddyfile
EXPOSE 8080
