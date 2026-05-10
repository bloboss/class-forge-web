# API client & state

## What we have today

`src/data.rs` returns hard-coded `Vec<Classroom>`, `Vec<Assignment>`, etc. on
every call. Screens invoke these directly during render. That's fine for a
prototype, but it makes it impossible to:

- show loading / error states,
- propagate updates from one screen to another,
- mock the API in tests.

## Target shape

Split `data.rs` into two layers:

```
src/
├── api/
│   ├── mod.rs         # client::Client, ApiError
│   ├── client.rs      # gloo-net wrapper, base URL, credentials: include
│   ├── classrooms.rs  # list/get/create
│   ├── assignments.rs
│   ├── roster.rs
│   ├── forges.rs
│   └── auth.rs        # me / login / logout / oauth
└── state/
    ├── mod.rs
    ├── session.rs     # see auth.md
    ├── classrooms.rs  # RwSignal<Resource<Vec<Classroom>>>
    └── …
```

`Resource<T>` is a small enum:

```rust
pub enum Resource<T> {
    Idle,
    Loading,
    Ready(T),
    Failed(ApiError),
}
```

Screens render against `Resource<T>`; they no longer call `data::*` directly.

## HTTP client

Use [`gloo-net`](https://docs.rs/gloo-net) (already common in the Leptos
ecosystem). One `Client` instance is provided via context:

```rust
pub struct Client {
    base: String,           // "/api" in prod, "http://localhost:8080/api" in dev
    csrf: Option<String>,   // read from cookie at boot
}
```

Every request:
- sends `credentials: include` (cookies),
- includes `X-CSRF-Token` for non-GET,
- maps `401` to `ApiError::Unauthorized` (which the session layer listens
  for and flips to `Session::Anonymous`).

## Wire types vs. view types

`api::Classroom` mirrors the Go JSON exactly (field names, types). The view
keeps using `data::Classroom` until we are ready to migrate the screens; a
`From<api::Classroom> for data::Classroom` shim bridges them.

This is the only place where wire/view drift is tolerated. Once a screen
moves to the `Resource<T>` flow, it reads `api::Classroom` directly.

## Mock server for development

`docker compose` (see [compose](../deployment/compose.md)) runs the real Go
backend, so most development goes against that. For headless tests
(`wasm-bindgen-test`), use a tiny in-process `MockTransport` that the
`Client` accepts via dependency injection, returning fixture JSON.

## Migration order

The screens migrate one at a time so we can review each diff in isolation:

1. `OnboardingScreen` → `forges::list`, `auth::oauth_start` ✅ **shipped (E1)**
2. `DashboardScreen` → `classrooms::list`
3. `ClassroomShell` → `assignments::list_for_class`, `roster::list`
4. `AssignmentDetail` → `assignments::get`

Each migration is a separate task card in the [roadmap](../roadmap/map.md)
so they can run in parallel once the API layer lands.

## Current status

| Module                  | State                            |
| ----------------------- | -------------------------------- |
| `api::ApiError`         | landed (E1 scaffold)             |
| `api::forges`           | wire types + `list()` against fixtures (E1 scaffold) |
| `api::client::Client`   | not yet — task **B1**            |
| `api::auth`             | not yet — task **A1/A2/A4**      |
| `api::classrooms`       | not yet — task **E2**            |
| `api::assignments`      | not yet — task **E3/E4**         |
| `api::roster`           | not yet — task **E3**            |
| `state::Resource<T>`    | landed (E1 scaffold)             |
| `state::forges`         | landed with lazy provider (E1 scaffold; B2 will tighten) |
| `state::session`        | not yet — task **A2**            |
| `state::{classrooms,…}` | not yet — task **B2**            |

The "scaffold" tag means the public surface is real but the implementation
is a fixture stub; replacing it with the gloo-net call B1 introduces is a
local change inside the corresponding `api::*::*()` function body.
