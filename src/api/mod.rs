//! HTTP client and wire types for the Go backend.
//!
//! The full client lands with task **B1** (`api::Client` + `gloo-net`); the
//! module currently exposes only the surface that already-merged screens
//! need:
//!
//! - [`auth`] — stub `login`, `logout`, `me` calls used by the A1 login
//!   screen.
//! - [`forges`] — wire types and `list()` used by the E1 onboarding screen.
//!
//! Other resource modules join as their migration cards (E2–E4) come up.
//! When B1 lands, both submodules collapse onto the shared `Client` and
//! [`ApiError`].

pub mod auth;
pub mod forges;

/// Errors returned from any backend call.
///
/// Mirrors the variants documented in `book/src/architecture/api.md`. Until
/// B1 lands the only producer is the in-process mock used by E1, but the
/// shape is stable so the screens written against it will not need to change
/// when real HTTP is wired in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApiError {
    /// `401` — the session layer flips to `Anonymous` on this.
    Unauthorized,
    /// Any other 4xx response.
    Client(u16, String),
    /// Any 5xx response.
    Server(u16),
    /// Transport-level failure (DNS, TLS, connection reset, …).
    Network(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Unauthorized => write!(f, "unauthorized"),
            ApiError::Client(code, msg) => write!(f, "client error {code}: {msg}"),
            ApiError::Server(code) => write!(f, "server error {code}"),
            ApiError::Network(msg) => write!(f, "network error: {msg}"),
        }
    }
}
