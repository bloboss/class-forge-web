//! Frontend HTTP API layer.
//!
//! A1 ships only the auth stubs needed by the login screen so the screen can
//! compile and navigate. B1 replaces these with the real `Client` /
//! `ApiError` / `Transport` types and wires them up to `gloo-net`.

pub mod auth;
