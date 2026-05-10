//! A1 regression tests — covers only the surface introduced by the
//! `agent/a1-login-route` task: the two new `Route` variants and the
//! `Route::is_public()` helper that A3's auth_guard will lean on.
//!
//! Comprehensive parse() coverage is intentionally left to D2, which owns
//! `tests/router.rs`.

use class_forge_web::router::{parse, Route};
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn parses_login_route() {
    assert_eq!(parse("#/login"), Route::Login);
}

#[wasm_bindgen_test]
fn parses_login_callback_variant() {
    let route = parse("#/login/callback");
    assert!(
        matches!(route, Route::LoginCallback { .. }),
        "expected LoginCallback variant, got {route:?}",
    );
}

#[wasm_bindgen_test]
fn login_routes_are_public() {
    assert!(Route::Login.is_public());
    assert!(Route::LoginCallback {
        code: String::new()
    }
    .is_public());
}

#[wasm_bindgen_test]
fn gated_routes_are_not_public() {
    assert!(!Route::Onboarding.is_public());
    assert!(!Route::Dashboard.is_public());
}
