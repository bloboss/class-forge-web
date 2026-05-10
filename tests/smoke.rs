//! Smoke test for the `wasm-bindgen-test` harness (Track D1).
//!
//! Running `wasm-pack test --headless --firefox` should pick up this file,
//! launch a browser, and report at least one passing test. Future tracks
//! (D2 router tests, B-track state tests) build on top of this harness.

use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn it_compiles() {
    // The mere fact that this test links and runs in the browser proves
    // that the harness is wired up correctly. No assertion needed.
}
