//! HTTP client and wire types for the Go backend.
//!
//! [`client`] holds the shared `Client`, `ApiError`, `Method`, and the
//! `Transport` trait that lets tests substitute a mock. Resource-specific
//! modules layer on top:
//!
//! - [`auth`] — A1's stub `login`, `logout`, `me` calls used by the
//!   login screen. A2/A4 will rewrite the bodies to call through `Client`.
//! - [`forges`] — E1's wire types and `list()` against fixtures; the body
//!   moves to `client.get("/forges")` once B2 wires the call site through
//!   the shared `Client`.
//!
//! Production code uses the default [`GlooTransport`] wired to `gloo-net`;
//! tests inject their own [`Transport`] implementation (track B3 ships the
//! fixture-driven `MockTransport`).

pub mod auth;
pub mod client;
pub mod forges;

pub use client::{ApiError, Client, GlooTransport, Method, Response, Transport};
