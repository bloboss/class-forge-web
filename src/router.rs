//! Tiny hash-based router. Mirrors the original prototype's `parseRoute`.

use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Route {
    Onboarding,
    Dashboard,
    Classroom { class_id: String, tab: String },
    AssignmentDetail { class_id: String, asg_id: String },
}

pub fn parse(hash: &str) -> Route {
    let h = hash.trim_start_matches('#');
    let h = if h.is_empty() { "/onboarding" } else { h };
    let parts: Vec<&str> = h.split('/').filter(|s| !s.is_empty()).collect();
    match parts.as_slice() {
        ["onboarding"] => Route::Onboarding,
        ["classes"] => Route::Dashboard,
        ["classes", id, "a", asg] => Route::AssignmentDetail {
            class_id: (*id).to_string(),
            asg_id: (*asg).to_string(),
        },
        ["classes", id, tab] => Route::Classroom {
            class_id: (*id).to_string(),
            tab: (*tab).to_string(),
        },
        ["classes", id] => Route::Classroom {
            class_id: (*id).to_string(),
            tab: "assignments".to_string(),
        },
        _ => Route::Onboarding,
    }
}

pub fn navigate(path: &str) {
    if let Some(win) = web_sys::window() {
        let _ = win.location().set_hash(path);
    }
}

#[derive(Clone, Copy)]
pub struct RouteSignal(pub ReadSignal<Route>);

pub fn provide_router() {
    let win = web_sys::window().expect("window");
    let initial_hash = win.location().hash().unwrap_or_default();
    if initial_hash.is_empty() {
        let _ = win.location().set_hash("/onboarding");
    }
    let initial = parse(&win.location().hash().unwrap_or_default());
    let (route, set_route) = signal(initial);
    provide_context(RouteSignal(route));

    // Listen for hashchange events
    let cb = Closure::<dyn FnMut()>::new(move || {
        if let Some(win) = web_sys::window() {
            let h = win.location().hash().unwrap_or_default();
            set_route.set(parse(&h));
        }
    });
    let _ = win.add_event_listener_with_callback("hashchange", cb.as_ref().unchecked_ref());
    cb.forget();
}

pub fn use_route() -> ReadSignal<Route> {
    use_context::<RouteSignal>().expect("router not provided").0
}
