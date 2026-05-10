# Agent task cards

One card per node in the [parallel task map](./map.md). Each card has the
same shape:

- **Goal** — one sentence.
- **Files** — exhaustive list of paths the agent may create or edit. If
  the agent needs to touch a file outside this list, they pause and open a
  coordination question.
- **Approach** — the steps that lead to a passing test gate.
- **Test gate** — the exact commands that must pass.
- **Out of scope** — guardrails so the PR stays small.

Branch convention: `agent/<track><n>-<slug>`, e.g. `agent/a1-login-route`.

---

## Track A — Auth

### A1 — Login route + screen (stubbed backend)

**Goal.** Add `Route::Login` and `Route::LoginCallback` and a working
`<LoginScreen/>` that renders provider buttons and an email/password form.
The screen calls a stub function — the real HTTP wiring lands when B1 is
available; A1 should not block on B1.

**Files.**
- `src/router.rs` — add `Login` and `LoginCallback { code: String }` variants, parse rules, navigate helper.
- `src/screens/login.rs` — new screen.
- `src/screens/mod.rs` — `pub mod login;`
- `src/app.rs` — render arm for the new routes.
- `book/src/architecture/auth.md` — keep in sync if the route shape changes.

**Approach.**
1. Extend `parse()` in `router.rs` with `["login"]` and `["login","callback"]`. The callback variant reads `?code=…` from `window.location.search`; keep parsing in one place.
2. Build `LoginScreen` mirroring the visual language of `OnboardingScreen` (reuse `.app-shell`, `.topbar`, `.btn`).
3. The form's submit handler calls `crate::api::auth::login(email, pw).await` if the module exists, else a `todo!()`-free stub that logs and navigates to `/onboarding`. Behind a `cfg(feature = "stubs")`? No — just a plain function in `src/api/auth.rs` that B1 will replace; A1 ships the function with a hard-coded `Ok(())`.
4. Provider buttons (`Forgejo`, `GitLab`) navigate to `/api/auth/oauth/forgejo/start` via `window.location.assign` — that's a real navigation, not a fetch.

**Test gate.**
```bash
cargo fmt --all -- --check
cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings
cargo check --target wasm32-unknown-unknown
trunk build
# Manual: visit #/login, see screen renders; submit form → lands on #/onboarding.
```

**Out of scope.** Real HTTP, session hydration, route guarding (those are A2/A3).

---

### A2 — Session context + `/auth/me` hydration

**Goal.** On app boot, fetch `/auth/me`, populate a `Session` signal in
context, and re-export `use_session()`.

**Files.**
- `src/state/mod.rs`, `src/state/session.rs` — new module.
- `src/lib.rs` — `pub mod state;`
- `src/app.rs` — call `provide_session()` and kick off the hydrate task.
- `src/api/auth.rs` — add `me()` function (extends B1).

**Approach.**
1. Define `Session::{Loading, Anonymous, SignedIn(User)}`.
2. `provide_session()` creates an `RwSignal<Session>::new(Loading)`, puts it in context, and `wasm_bindgen_futures::spawn_local`s a task that calls `api::auth::me()`.
3. On `Ok(user)` → `SignedIn`. On `Err(Unauthorized)` → `Anonymous`. On `Err(_)` → `Anonymous` and log.
4. Update `OnboardingScreen` topbar to read `use_session()` instead of `data::user()` (small diff; keeps mock path as fallback for `Loading`).

**Test gate.**
```bash
cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings
cargo check --target wasm32-unknown-unknown
wasm-pack test --headless --firefox  # tests/state_session.rs covers reducer
```
Tests verify the state machine using `MockTransport` from B3.

**Depends on.** A1, B1, B3.

---

### A3 — `auth_guard` for gated routes

**Goal.** Any route that isn't `Login` or `LoginCallback` redirects to
`/login` when `Session == Anonymous`.

**Files.**
- `src/app.rs` — wrap the match arm.
- `src/router.rs` — `Route::is_public(&self) -> bool` helper.

**Approach.** In `App::view`, before the route match, check session: if
`Anonymous` and route is not public, `navigate("/login")`. If `Loading`,
render a centered spinner. Use `Effect::new` so navigation happens once
the signal settles.

**Test gate.** Manual: open `#/classes` while unauthenticated → redirect.
Sign in via stub → reach `/classes`. Plus `cargo clippy` clean.

**Depends on.** A2.

---

### A4 — Logout + topbar user menu

**Goal.** Replace the static "Signed in as …" string with a menu that has
"Sign out". Sign out POSTs `/auth/logout`, clears the session signal, and
navigates to `/login`.

**Files.**
- `src/components.rs` — new `<UserMenu/>`.
- `src/screens/onboarding.rs`, `src/screens/dashboard.rs`, `src/screens/classroom.rs` — use `UserMenu` in topbar.
- `src/api/auth.rs` — add `logout()`.

**Test gate.** `cargo clippy` clean; manual sign-out round trip in
`docker compose` stack.

**Depends on.** A3.

---

## Track B — API & state

### B1 — `api::Client` + `ApiError`

**Goal.** Land the HTTP client used by every other API module.

**Files.**
- `Cargo.toml` — add `gloo-net = "0.6"`, `serde_json = "1"`, `wasm-bindgen-futures = "0.4"`.
- `src/api/mod.rs`, `src/api/client.rs` — `Client`, `ApiError`, `Method`, `request()`.
- `src/api/auth.rs` — empty stubs for `me`, `login`, `logout` so A1/A2 can compile.
- `src/lib.rs` — `pub mod api;`

**Approach.**
1. `Client::new(base: &str)` reads the `csrf_token` cookie via `web_sys::HtmlDocument::cookie()`.
2. `request<T: DeserializeOwned, B: Serialize>(method, path, body) -> Result<T, ApiError>` uses `gloo_net::http::Request`. Always sets `credentials: include`.
3. Map status: `401 → Unauthorized`, `4xx → Client(code, msg)`, `5xx → Server(code)`, network → `Network(string)`.
4. Provide a `Transport` trait so B3 can swap a mock in tests.

**Test gate.**
```bash
cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings
cargo check --target wasm32-unknown-unknown
# A unit test that constructs a Client and asserts URL composition.
cargo test --target wasm32-unknown-unknown -p class-forge-web client::
```

**Out of scope.** Resource layer (B2), tests harness (B3), screen wiring.

---

### B2 — `Resource<T>` + state context

**Goal.** Provide reusable `RwSignal<Resource<T>>` stores for the four
top-level resources and a `use_resource!` helper to load/refresh them.

**Files.**
- `src/state/mod.rs` (extend), `src/state/{classrooms, assignments, roster, forges}.rs`.

**Test gate.**
```bash
cargo clippy --target wasm32-unknown-unknown --all-targets -- -D warnings
wasm-pack test --headless --firefox  # tests/state_resource.rs
```

**Depends on.** B1.

---

### B3 — `MockTransport` for tests

**Goal.** A fixture-driven `Transport` impl that returns canned JSON for
specific (method, path) pairs.

**Files.** `src/api/mock.rs` (under `#[cfg(any(test, feature = "mock"))]`),
`tests/fixtures/*.json`.

**Test gate.** A round-trip test that uses `MockTransport` to fetch
`/classrooms` and decodes into `api::Classroom`. `wasm-pack test --headless --firefox` green.

**Depends on.** B1.

---

## Track C — Deployment

### C1 — Dockerfile + Caddyfile + `.dockerignore`

**Goal.** `docker build -t class-forge-web .` produces a runnable image
that serves the WASM bundle and proxies `/api`.

**Files.**
- `Dockerfile` (see [docker.md](../deployment/docker.md) for the template).
- `deploy/Caddyfile`.
- `.dockerignore` — `target/`, `dist/`, `book/book/`, `.git/`, `*.log`.

**Test gate.**
```bash
docker build -t class-forge-web:dev .
docker run --rm -p 8080:8080 -e BACKEND_URL=http://example.invalid class-forge-web:dev &
PID=$!
sleep 2
curl -fsS http://localhost:8080/ | grep -q '<div id="root">'
kill $PID
```

**Out of scope.** Compose, OAuth bootstrap.

---

### C2 — `docker-compose.yml`

**Goal.** `docker compose up --build` brings up `web + backend + db +
forgejo` and the browser reaches the app at `http://localhost:8080`.

**Files.** `docker-compose.yml`, `.env.example`, `deploy/README.md`.

**Test gate.**
```bash
docker compose up -d --build
sleep 15
curl -fsS http://localhost:8080/ | grep -q ClassForge
curl -fsS http://localhost:8080/api/healthz
docker compose down -v
```

**Depends on.** C1.

---

### C3 — Forgejo OAuth bootstrap

**Goal.** First-time setup script that creates a Forgejo OAuth app inside
the bundled `forgejo` service and writes its credentials into `.env`.

**Files.** `deploy/bootstrap-forgejo-oauth.sh`, update to `deploy/README.md`.

**Test gate.** Running the script against a fresh `docker compose up -d
forgejo` writes a valid `.env`; `docker compose up backend` then completes
its OAuth handshake on a manual login attempt.

**Depends on.** C2.

---

## Track D — Tests

### D1 — `wasm-bindgen-test` harness

**Goal.** `wasm-pack test --headless --firefox` runs at least one passing
test from `tests/`.

**Files.**
- `Cargo.toml` — `[dev-dependencies] wasm-bindgen-test = "0.3"`.
- `.cargo/config.toml` — `[target.wasm32-unknown-unknown] runner = "wasm-bindgen-test-runner"`.
- `tests/smoke.rs` — `#[wasm_bindgen_test] fn it_compiles() {}`.

**Test gate.**
```bash
wasm-pack test --headless --firefox
```

---

### D2 — `router::parse` unit tests

**Goal.** Cover every `Route` variant including malformed inputs.

**Files.** `tests/router.rs`.

**Test gate.** `wasm-pack test --headless --firefox` green; coverage of
all `parse()` branches asserted by reading the test names.

**Depends on.** D1.

---

### D3 — Playwright smoke against compose stack

**Goal.** A 30-second end-to-end check that boots `docker compose`, signs
in with the seeded instructor, and lands on `/classes`.

**Files.** `e2e/playwright.config.ts`, `e2e/smoke.spec.ts`, `e2e/README.md`.
This is the only place TypeScript is allowed; we keep it because Playwright
is the most accurate UI harness and it never ships to users.

**Test gate.**
```bash
docker compose up -d --build
npx playwright install --with-deps chromium
npx playwright test
docker compose down -v
```

**Depends on.** C2.

---

## Track E — Screen migrations

Each card replaces `data::*` calls in a single screen with the matching
`api::*` calls and `state::*` signals.

### E1 — Onboarding → forges API
**Files.** `src/screens/onboarding.rs` only.
**Test gate.** Manual against `docker compose` + `wasm-pack test`.
**Depends on.** A2, B2.

### E2 — Dashboard → classrooms API
**Files.** `src/screens/dashboard.rs` only.
**Depends on.** A2, B2.

### E3 — ClassroomShell → assignments + roster API
**Files.** `src/screens/classroom.rs` only.
**Depends on.** A2, B2.

### E4 — AssignmentDetail → assignments API
**Files.** `src/screens/assignment_detail.rs` only.
**Depends on.** A2, B2.

E1–E4 are mutually parallel: each touches one file.

---

## Track F — Docs & CI

### F1 — mdbook CI build

**Goal.** Every PR builds the book and uploads `book/book/` as an
artifact; pushes to `main` deploy it to GitHub Pages.

**Files.** `.github/workflows/book.yml`.

**Test gate.** PR shows a green "book / build" check; artifact contains
`index.html`.

---

### F2 — ADR template + first ADR

**Goal.** Capture the "stay on Leptos" decision so we don't relitigate it
quarterly.

**Files.**
- `book/src/adr/_template.md`
- `book/src/adr/0001-stay-on-leptos.md`
- `book/src/SUMMARY.md` — link the ADR section.

**Test gate.** `mdbook build` clean.

---

## Coordination rules

1. **One agent per file in flight.** If two cards both list a file (e.g.,
   `src/app.rs`), the later one waits.
2. **Tiny coordination PRs are fine.** If A1 needs a route variant, opening
   a 5-line PR to add it before A2 starts is preferable to hand-merging.
3. **Test gate is the contract.** Don't mark a card done without the gate
   passing. Don't add scope beyond the gate.
4. **Update the map when reality differs.** If a dependency turns out to
   be soft, remove the edge in [map.md](./map.md) so the next agent sees
   accurate parallelism.
