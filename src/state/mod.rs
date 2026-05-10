//! Reactive state stores for resources fetched from the backend.
//!
//! Each top-level resource (forges, classrooms, assignments, roster) lives
//! in its own submodule and exposes a `use_*()` accessor that returns a
//! `ReadSignal<Resource<T>>`. Screens subscribe to these signals; load and
//! refresh logic stays inside the state module, not the screen.
//!
//! Today only [`forges`] is implemented — the rest land with their
//! migration cards (E2–E4) on top of B2.

pub mod forges;

/// Lifecycle of a value fetched from the backend.
///
/// See `book/src/architecture/api.md` for the rationale; the variants are
/// shaped so a screen can render every state without needing the underlying
/// resource to also be `Default` or `Clone` until it's `Ready`.
#[derive(Clone, Debug, PartialEq)]
pub enum Resource<T> {
    /// Nothing has been requested yet.
    Idle,
    /// A request is in flight.
    Loading,
    /// The most recent request succeeded.
    Ready(T),
    /// The most recent request failed.
    Failed(crate::api::ApiError),
}

impl<T> Resource<T> {
    pub fn is_loading(&self) -> bool {
        matches!(self, Resource::Loading | Resource::Idle)
    }
}
