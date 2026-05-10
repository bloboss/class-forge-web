# Authentication & sessions

## Goal

Get an instructor signed in, hold a session, hydrate user context on every
load, and surface a logout. The Hi-Fi prototype already shows the user as
"signed in" in the topbar — that string needs to be backed by a real session.

## Decision: cookie session, OAuth-via-forge as the primary login

The product is forge-agnostic, and instructors **already** prove identity to
their forge (Forgejo / GitLab) during onboarding. We reuse that:

1. **Primary path** — OAuth2 Authorization Code flow against the user's
   chosen forge. Backend completes the exchange and issues a session cookie.
2. **Fallback path** — email + password against a backend-managed account
   (for institutions whose forges block OAuth apps). Same cookie afterwards.

Why a cookie, not a bearer token in `localStorage`:
- HTTP-only + `SameSite=Lax` + `Secure` blocks XSS token exfiltration,
  which matters because we render forge-derived strings.
- The Go backend can rotate the session server-side without coordinating
  with the WASM client.
- The frontend never has to read the token, so we don't need
  `wasm-bindgen` storage shims on the hot path.

## Routes (frontend)

Add to `src/router.rs`:

| Hash route                | Variant                          | Notes                          |
| ------------------------- | -------------------------------- | ------------------------------ |
| `/login`                  | `Route::Login`                   | provider picker + email form   |
| `/login/callback?code=…`  | `Route::LoginCallback { code }`  | posts code to backend          |
| (existing) `/onboarding`  | gated: requires session          |                                |
| (existing) `/classes`     | gated: requires session          |                                |

A new `auth_guard` wrapper in `app.rs` redirects unauthenticated users to
`/login` for any gated route.

## Backend contract (assumed; confirm in `class-forge`)

```
POST /auth/login                {email, password}            → 204 + Set-Cookie
GET  /auth/oauth/{forge}/start                               → 302 to forge
GET  /auth/oauth/{forge}/callback?code=…                     → 302 to /#/onboarding + Set-Cookie
POST /auth/logout                                            → 204 + clear cookie
GET  /auth/me                                                → {user, connected_forges}
                                                                or 401 if no session
```

If `class-forge` does not yet expose these, that gap becomes a backend task
referenced from the roadmap; the frontend agent stubs against a fake server
(see [API client](./api.md)) until the real endpoints land.

## Frontend state

```rust
// src/state/session.rs (new)
#[derive(Clone, Debug, PartialEq)]
pub enum Session {
    Loading,            // /auth/me in flight
    Anonymous,          // 401 — show /login
    SignedIn(User),     // hydrated
}

pub fn provide_session() { /* RwSignal<Session> in context */ }
pub fn use_session() -> ReadSignal<Session> { … }
```

Boot sequence inside `App::setup`:

1. `provide_session()` → state starts `Loading`.
2. Spawn an async task that calls `GET /auth/me`.
3. On 200 → `SignedIn(user)`. On 401 → `Anonymous`. On network error →
   `Anonymous` with a non-blocking toast.
4. The router renders `<LoginScreen/>` whenever the route is gated and the
   state is `Anonymous`.

## Login screen (rough)

- Heading: "Sign in to ClassForge"
- Big buttons: "Continue with Forgejo", "Continue with GitLab"
  (using the existing `ForgeMark` component — see `src/components.rs`).
- Disclosure → email + password form (POST `/auth/login`).
- Error band for `401` / `429` / network failures.
- After success: navigate to `/onboarding` if `connected_forges` is empty,
  else `/classes`.

## CSRF

POST endpoints require a `X-CSRF-Token` header that mirrors a non-HTTP-only
`csrf_token` cookie set on session creation. The frontend reads the cookie
once at boot and includes it on every mutating request via the API client.

## Open questions

- Do we want "stay signed in" → 30-day session vs. session-only? Default to
  30-day with absolute timeout.
- Multi-account support (instructor with two forges) — the cookie identifies
  the *user*, not the forge; forge tokens stay on the backend.
