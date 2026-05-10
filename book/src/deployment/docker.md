# Docker images

We ship one image per process. The frontend image is just a static bundle
behind a tiny web server; it has zero runtime dependency on the Rust toolchain.

## Frontend image — `class-forge-web`

Two-stage build: compile WASM with `trunk`, then copy `dist/` into a Caddy
image that also handles `/api` reverse proxying.

```dockerfile
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

# ── Stage 2: serve dist/ ────────────────────────────────────────────────
FROM caddy:2-alpine
COPY --from=build /app/dist /srv
COPY deploy/Caddyfile /etc/caddy/Caddyfile
EXPOSE 8080
```

`deploy/Caddyfile`:

```
:8080 {
    encode zstd gzip
    handle /api/* {
        reverse_proxy {$BACKEND_URL:http://backend:8080}
    }
    handle {
        root * /srv
        try_files {path} /index.html
        file_server
    }
}
```

Notes:
- `try_files {path} /index.html` is a SPA fallback so deep links to
  `#/classes/cs331/...` work on refresh.
- The frontend and `/api` share an origin → cookies + CSRF stay simple.
- `BACKEND_URL` is overridable so the same image works against staging.

## Backend image — `class-forge`

Lives in the [`bloboss/class-forge`](https://github.com/bloboss/class-forge)
repo. We do not build it here; `docker compose` pulls
`ghcr.io/bloboss/class-forge:latest` (or builds from a sibling checkout when
`BACKEND_LOCAL=1` — see [compose](./compose.md)).

## Image hygiene

- Pin the Rust base image by minor version (`1.83-slim`).
- Use `--locked` for `cargo install` and ensure `Cargo.lock` is committed.
- `.dockerignore` should exclude `target/`, `dist/`, `book/book/`, and
  `node_modules/` if anything ever introduces one.
