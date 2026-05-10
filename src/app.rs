//! Top-level app: router dispatch + global chrome (jump-to nav, theme tweaks).

use leptos::prelude::*;
use wasm_bindgen::JsCast;

use crate::icons::Icon;
use crate::router::{navigate, provide_router, use_route, Route};
use crate::screens::{
    assignment_detail::AssignmentDetail,
    classroom::ClassroomShell,
    dashboard::DashboardScreen,
    login::{LoginCallbackScreen, LoginScreen},
    onboarding::OnboardingScreen,
};

#[component]
pub fn App() -> impl IntoView {
    provide_router();
    let route = use_route();
    let theme = RwSignal::new("cream".to_string());

    Effect::new(move |_| {
        let t = theme.get();
        if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
            if let Some(el) = doc.document_element() {
                let html: web_sys::HtmlElement = el.unchecked_into();
                let _ = html.dataset().set("theme", &t);
            }
        }
    });

    let tweaks_open = RwSignal::new(false);

    view! {
        <>
            {move || match route.get() {
                Route::Login => view! { <LoginScreen/> }.into_any(),
                Route::LoginCallback { code } =>
                    view! { <LoginCallbackScreen code=code/> }.into_any(),
                Route::Onboarding => view! { <OnboardingScreen/> }.into_any(),
                Route::Dashboard => view! { <DashboardScreen/> }.into_any(),
                Route::Classroom { class_id, tab } =>
                    view! { <ClassroomShell class_id=class_id tab=tab/> }.into_any(),
                Route::AssignmentDetail { class_id, asg_id } =>
                    view! { <AssignmentDetail class_id=class_id asg_id=asg_id/> }.into_any(),
            }}
            <NavChrome/>
            <button class="btn btn-sm" style="position: fixed; right: 16px; bottom: 16px; z-index: 49; background: var(--paper); box-shadow: 0 4px 12px -2px rgba(0,0,0,0.15)"
                    on:click=move |_| tweaks_open.update(|v| *v = !*v)>
                <Icon name="settings" size=13/>" Tweaks"
            </button>
            <Show when=move || tweaks_open.get() fallback=|| ()>
                <TweaksPanel theme=theme tweaks_open=tweaks_open/>
            </Show>
        </>
    }
}

#[component]
fn NavChrome() -> impl IntoView {
    let open = RwSignal::new(false);
    let items: Vec<(&'static str, &'static str, &'static str)> = vec![
        ("Onboarding (option C)", "/onboarding", "sparkles"),
        ("Classes dashboard", "/classes", "home"),
        (
            "Algorithms · Assignments (default)",
            "/classes/cs331/assignments",
            "clipboard-list",
        ),
        ("Algorithms · Roster", "/classes/cs331/roster", "users"),
        ("Algorithms · New assignment", "/classes/cs331/new", "plus"),
        ("Algorithms · CI / CD", "/classes/cs331/cicd", "beaker"),
        (
            "Algorithms · Analytics",
            "/classes/cs331/analytics",
            "spark",
        ),
        (
            "Algorithms · Settings",
            "/classes/cs331/settings",
            "settings",
        ),
    ];
    view! {
        <div class="nav-chrome">
            <Show when=move || open.get() fallback=|| ()>
                <div class="nav-chrome-popover">
                    <div style="font-size: 10px; font-weight: 600; color: var(--ink-3); text-transform: uppercase; letter-spacing: 0.5px; padding: 6px 10px 4px">
                        "Hi-fi v1 · jump to"
                    </div>
                    {items.iter().map(|(label, path, ico)| {
                        let path_owned = path.to_string();
                        let path_click = path_owned.clone();
                        let label = *label;
                        let ico = ico.to_string();
                        view! {
                            <button class="sidebar-item" on:click=move |_| {
                                navigate(&path_click);
                                open.set(false);
                            }>
                                <span class="ico"><Icon name=ico.clone() size=14/></span>
                                <span style="font-size: 12px">{label}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </Show>
            <button class="btn btn-sm" style="background: var(--paper); box-shadow: 0 4px 12px -2px rgba(0,0,0,0.15)"
                    on:click=move |_| open.update(|o| *o = !*o)>
                <Icon name="globe" size=13/>" "
                {move || if open.get() { "Close" } else { "Jump to…" }}
            </button>
        </div>
    }
}

#[component]
fn TweaksPanel(theme: RwSignal<String>, tweaks_open: RwSignal<bool>) -> impl IntoView {
    let themes: Vec<(&str, &str, &str, &str)> = vec![
        ("cream", "Warm cream", "#fbf7f0", "#4338ca"),
        ("dark", "Studio dark", "#161310", "#a78bfa"),
        ("slate", "Cool slate", "#f4f6f8", "#0891b2"),
    ];
    view! {
        <div class="tweaks-panel">
            <div style="display: flex; align-items: center; justify-content: space-between; padding: 12px 16px; border-bottom: 1px solid var(--line)">
                <div style="font-family: 'Instrument Serif', serif; font-size: 18px">"Tweaks"</div>
                <button class="btn btn-sm btn-ghost btn-icon-sm" on:click=move |_| tweaks_open.set(false)>
                    <Icon name="x" size=13/>
                </button>
            </div>
            <div style="padding: 16px">
                <div style="font-size: 11px; font-weight: 600; color: var(--ink-3); text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 8px">
                    "Theme"
                </div>
                <div style="display: flex; flex-direction: column; gap: 6px">
                    {themes.into_iter().map(|(id, label, sw1, sw2)| {
                        let id_owned = id.to_string();
                        let id_click = id_owned.clone();
                        let id_active = id_owned.clone();
                        let style = move || {
                            let active = theme.get() == id_active;
                            if active {
                                "height: auto; padding: 10px 12px; justify-content: flex-start; background: var(--accent-soft); border-color: var(--accent-line)".to_string()
                            } else {
                                "height: auto; padding: 10px 12px; justify-content: flex-start".to_string()
                            }
                        };
                        let id_check = id_owned.clone();
                        view! {
                            <button class="btn" style=style on:click=move |_| theme.set(id_click.clone())>
                                <span style="display: flex">
                                    <span style=format!("width: 14px; height: 14px; border-radius: 7px; background: {}; border: 1px solid var(--line)", sw1)></span>
                                    <span style=format!("width: 14px; height: 14px; border-radius: 7px; background: {}; margin-left: -5px; border: 1px solid var(--line)", sw2)></span>
                                </span>
                                <span style="font-size: 13px; margin-left: 8px">{label}</span>
                                <Show when=move || theme.get() == id_check fallback=|| ()>
                                    <span style="margin-left: auto"><Icon name="check" size=13/></span>
                                </Show>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}
