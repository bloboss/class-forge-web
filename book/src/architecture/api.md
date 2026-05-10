# API client & state

## What we have today

`src/data.rs` returns hard-coded `Vec<Classroom>`, `Vec<Assignment>`, etc.
on every call. Screens invoke these directly during render. That's fine
for a prototype, but it makes it impossible to:

- show loading / error states,
- propagate updates from one screen to another,
- mock the API in tests.

**Status (2026-05): track B1 has landed atop A1 + E1.** `src/api/client.rs`
now contains the shared `Client`, `ApiError`, `Method`, `Response`, and
`Transport` trait described below; `src/api/mod.rs` re-exports them so
existing call sites (`api::ApiError`, used by `state::Resource`) keep
working without churn. `src/api/auth.rs` keeps the A1 stub signatures
(`login(String, String)`, `logout()`, `me()`) so the login screen
continues to work — A2 will rewrite those bodies to call through
`Client`. `api::forges::list()` similarly keeps its fixture body until
a call site is wired through `Client`. The full `Resource<T>` state
context for the remaining resources still belongs to track B2.

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

Built on [`gloo-net`](https://docs.rs/gloo-net) (already common in the
Leptos ecosystem). One `Client` instance is provided via Leptos context:

```rust
pub struct Client {
    base: String,                  // "/api" in prod, "http://localhost:8080/api" in dev
    csrf: Option<String>,          // read from `csrf_token` cookie at boot
    transport: Box<dyn Transport>, // GlooTransport in prod, MockTransport in tests
}
```

Every request:
- sends `credentials: include` (cookies),
- includes `X-CSRF-Token` on every mutating method (anything other than
  `GET`),
- maps `401 → ApiError::Unauthorized` (which the session layer listens
  for and flips to `Session::Anonymous`),
- maps remaining `4xx → ApiError::Client(code, body)`,
- maps `5xx → ApiError::Server(code)`,
- maps transport / parse failures to `ApiError::Network(_)` /
  `ApiError::Decode(_)`.

The `Transport` trait isolates the `gloo-net` dependency so unit tests
can substitute an in-memory implementation. Track B3 ships the
fixture-driven `MockTransport`; the `Client` unit tests in
`src/api/client.rs` use a tiny recording transport to assert URL
composition and CSRF behaviour. Those tests are written with
`#[wasm_bindgen_test]` so they run under the D1 harness and inside the
G1 CI workflow's `wasm-pack test --headless --firefox` job.

URL composition rules (`Client::url(path)`):
- Trailing `/` on the base is stripped at construction.
- A leading `/` on the path is optional — both `"/auth/me"` and
  `"auth/me"` produce `"<base>/auth/me"`.
- Empty path returns the base unchanged.

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
| `api::ApiError`         | landed (B1 — moved into `client.rs`, re-exported through `api::*`) |
| `api::client::Client`   | landed (B1 — `Client`, `Method`, `Response`, `Transport`, `GlooTransport`) |
| `api::forges`           | wire types + `list()` against fixtures (E1 scaffold; body swaps to `Client::get` once a call site is wired) |
| `api::auth`             | A1 stubs (`login`, `logout`, `me`); bodies move to `Client` calls in **A2/A4** |
| `api::classrooms`       | not yet — task **E2**            |
| `api::assignments`      | not yet — task **E3/E4**         |
| `api::roster`           | not yet — task **E3**            |
| `state::Resource<T>`    | landed (E1 scaffold)             |
| `state::forges`         | landed with lazy provider (E1 scaffold; B2 will tighten) |
| `state::session`        | not yet — task **A2**            |
| `state::{classrooms,…}` | not yet — task **B2**            |

The "scaffold" tag means the public surface is real but the implementation
is a fixture stub; now that B1 has landed, replacing each scaffolded
body with a `Client::get` call is a local change inside the
corresponding `api::*::*()` function.
