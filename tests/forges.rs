//! Tests for the E1 surface: `api::forges` wire types/list() and the
//! `state::Resource` lifecycle helper.
//!
//! Runs under the D1 wasm-bindgen-test harness
//! (`wasm-pack test --headless --firefox`).
//!
//! The end-to-end `state::forges::use_forges()` flow needs a richer
//! executor harness than D1 currently exposes, so it is exercised by the
//! Onboarding screen at runtime rather than here. B2 will land a proper
//! `Resource<T>` test fixture and that's where the integration test
//! belongs.

use class_forge_web::api::forges::{self, Forge, ForgeKind, ForgeStatus};
use class_forge_web::state::Resource;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
async fn forges_list_returns_at_least_one_live_entry() {
    let list: Vec<Forge> = forges::list().await.expect("forges::list() should succeed");
    assert!(!list.is_empty(), "fixture must not be empty");
    assert!(
        list.iter().any(|f| f.status == ForgeStatus::Live),
        "at least one forge must be Live so onboarding can proceed",
    );
    assert!(
        list.iter().any(|f| matches!(f.kind, ForgeKind::Forgejo)),
        "the self-hosted Forgejo entry is the seeded default",
    );
}

#[wasm_bindgen_test]
async fn forges_list_round_trips_through_serde() {
    let list = forges::list().await.unwrap();
    let json = serde_json::to_string(&list[0]).expect("forge serializes");
    let back: Forge = serde_json::from_str(&json).expect("forge round-trips");
    assert_eq!(back, list[0]);
}

#[wasm_bindgen_test]
fn resource_is_loading_reports_idle_and_loading() {
    let r: Resource<Vec<Forge>> = Resource::Loading;
    assert!(r.is_loading());
    let r: Resource<Vec<Forge>> = Resource::Idle;
    assert!(r.is_loading());
    let r: Resource<Vec<Forge>> = Resource::Ready(vec![]);
    assert!(!r.is_loading());
}
