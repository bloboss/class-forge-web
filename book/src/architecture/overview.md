# Architecture overview

## Runtime topology

```
┌──────────────────────────────────┐         ┌────────────────────────────┐
│ Browser                          │  HTTPS  │ Reverse proxy (Caddy/NGINX)│
│  ┌────────────────────────────┐  │ ──────► │  TLS, gzip, /api → backend │
│  │ class-forge-web (WASM)     │  │         │  /         → static WASM   │
│  │  Leptos CSR + hash router  │  │         └─────────────┬──────────────┘
│  └────────────────────────────┘  │                       │
└──────────────────────────────────┘                       │
                                              ┌────────────▼────────────┐
                                              │ class-forge (Go)        │
                                              │  REST API, auth,        │
                                              │  forge integrations     │
                                              └────────────┬────────────┘
                                                           │
                                              ┌────────────▼────────────┐
                                              │ Postgres + Forgejo/...  │
                                              └─────────────────────────┘
```

The frontend is a static bundle (`dist/` from `trunk build --release`). It
lives behind the same origin as the API to avoid CORS and to let us use
HTTP-only session cookies safely.

## Frontend module layout (current + target)

```
src/
├── main.rs            # mount App into #root
├── lib.rs             # crate exports
├── app.rs             # Router dispatch + chrome
├── router.rs          # hash router (extend with /login, /login/callback)
├── data.rs            # mock data — to be split into:
│                      #   ├── api/        (HTTP client, types)
│                      #   └── state/      (signal-backed stores)
├── components.rs      # shared primitives
├── icons.rs           # SVG icon set
└── screens/
    ├── onboarding.rs
    ├── dashboard.rs
    ├── classroom.rs
    ├── assignment_detail.rs
    └── login.rs       # NEW
```

## Guiding principles

- **One source of truth per resource.** A `RwSignal<Vec<Classroom>>` lives in a
  context and screens read from it. No more `data::classes()` calls scattered
  through views.
- **API types are not view types.** `api::Classroom` is the wire model;
  `data::Classroom` may diverge. Translate at the boundary.
- **Errors are values.** API calls return `Result<T, ApiError>`; screens render
  loading / error / data states explicitly.
- **No JS shims unless needed.** Prefer `web-sys` / `gloo-net` over
  hand-written `wasm-bindgen` glue.
