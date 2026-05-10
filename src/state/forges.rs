//! Reactive store for the forges list.
//!
//! Screens read [`use_forges`] and render against the returned signal.
//! The first caller in a given owner subtree triggers a fetch; subsequent
//! callers share the same signal via context. Once **B2** lands, the
//! provider call moves to `App` boot and this lazy fallback can be
//! removed — but keeping it here means **E1** can ship without touching
//! `src/app.rs`.

use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::api::forges::{self, Forge};
use crate::state::Resource;

#[derive(Clone, Copy)]
struct ForgesSignal(RwSignal<Resource<Vec<Forge>>>);

/// Read access to the forges resource. Triggers a fetch on first use.
pub fn use_forges() -> RwSignal<Resource<Vec<Forge>>> {
    if let Some(existing) = use_context::<ForgesSignal>() {
        return existing.0;
    }
    let signal = RwSignal::new(Resource::Loading);
    provide_context(ForgesSignal(signal));
    spawn_local(async move {
        match forges::list().await {
            Ok(list) => signal.set(Resource::Ready(list)),
            Err(err) => signal.set(Resource::Failed(err)),
        }
    });
    signal
}

/// Re-fetch the forges list, e.g. after a successful "Connect" action.
pub fn refresh_forges() {
    let signal = use_forges();
    signal.set(Resource::Loading);
    spawn_local(async move {
        match forges::list().await {
            Ok(list) => signal.set(Resource::Ready(list)),
            Err(err) => signal.set(Resource::Failed(err)),
        }
    });
}
