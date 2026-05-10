//! Auth endpoints.
//!
//! These are placeholder stubs introduced by roadmap card A1 so the login
//! screen has something concrete to call. B1 will replace the bodies with
//! real HTTP requests against the Go backend (`POST /auth/login`,
//! `POST /auth/logout`, `GET /auth/me`).

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthError {
    /// Placeholder until B1 lands `ApiError`.
    Stub,
}

/// Email + password login. Stubbed: always returns `Ok(())`.
pub async fn login(_email: String, _password: String) -> Result<(), AuthError> {
    Ok(())
}

/// Sign out the current session. Stubbed: always returns `Ok(())`.
pub async fn logout() -> Result<(), AuthError> {
    Ok(())
}

/// Hydrate the current user. Stubbed: returns `Err(AuthError::Stub)` so the
/// session reducer treats the boot as anonymous until B1/A2 land.
pub async fn me() -> Result<(), AuthError> {
    Err(AuthError::Stub)
}
