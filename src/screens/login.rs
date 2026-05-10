//! Login screen — provider buttons + email/password fallback.
//!
//! Roadmap card A1: the screen calls a stub in `crate::api::auth` that B1
//! will replace with a real HTTP request. Provider buttons issue real
//! browser navigations to the backend OAuth start endpoints because the
//! response is a 302 to the forge.

use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::api::auth;
use crate::data::ForgeKind;
use crate::icons::{ForgeMark, Icon};
use crate::router::{navigate, Route};

#[component]
pub fn LoginScreen() -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error: RwSignal<Option<String>> = RwSignal::new(None);
    let pending = RwSignal::new(false);

    let submit = move |_| {
        if pending.get() {
            return;
        }
        let e = email.get();
        let p = password.get();
        if e.is_empty() || p.is_empty() {
            error.set(Some("Email and password are required.".to_string()));
            return;
        }
        error.set(None);
        pending.set(true);
        spawn_local(async move {
            match auth::login(e, p).await {
                Ok(()) => {
                    pending.set(false);
                    navigate("/onboarding");
                }
                Err(_) => {
                    pending.set(false);
                    error.set(Some(
                        "Sign-in failed. Check your credentials and try again.".to_string(),
                    ));
                }
            }
        });
    };

    view! {
        <div class="app-shell">
            <div class="topbar">
                <div class="brand">
                    <div class="brand-dot">"C"</div>
                    <div class="brand-text">"Class"<em>"Forge"</em></div>
                </div>
                <div style="flex: 1"></div>
                <div style="font-size: 12px; color: var(--ink-3)">
                    "New here? "
                    <a class="link" href="#/onboarding">"Take the tour"</a>
                </div>
            </div>

            <div style="flex: 1; display: flex; justify-content: center; align-items: flex-start; padding: 64px 32px">
                <div style="width: 100%; max-width: 420px">
                    <div class="h-display" style="margin-bottom: 12px">
                        "Sign in to "<em>"ClassForge"</em>
                    </div>
                    <div style="font-size: 14px; color: var(--ink-3); margin-bottom: 28px">
                        "Continue with the forge you already use, or use an email "
                        "and password if your institution prefers."
                    </div>

                    <div style="display: flex; flex-direction: column; gap: 10px; margin-bottom: 24px">
                        <ProviderButton kind=ForgeKind::Forgejo
                            label="Continue with Forgejo"
                            href="/api/auth/oauth/forgejo/start"/>
                        <ProviderButton kind=ForgeKind::Gitlab
                            label="Continue with GitLab"
                            href="/api/auth/oauth/gitlab/start"/>
                    </div>

                    <div style="display: flex; align-items: center; gap: 10px; margin: 18px 0; color: var(--ink-4); font-size: 11px; letter-spacing: 0.6px; text-transform: uppercase">
                        <span style="flex: 1; height: 1px; background: var(--line)"></span>
                        "or"
                        <span style="flex: 1; height: 1px; background: var(--line)"></span>
                    </div>

                    <form on:submit=move |ev| { ev.prevent_default(); submit(()); }>
                        <label class="label" style="margin-bottom: 12px">
                            <span>"Email"</span>
                            <input class="input"
                                type="email"
                                autocomplete="username"
                                prop:value=move || email.get()
                                on:input=move |ev| email.set(event_target_value(&ev))/>
                        </label>
                        <label class="label" style="margin-bottom: 16px">
                            <span>"Password"</span>
                            <input class="input"
                                type="password"
                                autocomplete="current-password"
                                prop:value=move || password.get()
                                on:input=move |ev| password.set(event_target_value(&ev))/>
                        </label>

                        <Show when=move || error.get().is_some() fallback=|| ()>
                            <div class="pill pill-err"
                                style="display: block; padding: 10px 12px; margin-bottom: 12px; height: auto; border-radius: 8px; font-size: 12px">
                                {move || error.get().unwrap_or_default()}
                            </div>
                        </Show>

                        <button class="btn btn-accent btn-lg"
                                type="submit"
                                style="width: 100%; justify-content: center"
                                prop:disabled=move || pending.get()>
                            {move || if pending.get() { "Signing in…" } else { "Sign in" }}
                            " "<Icon name="chevron-right" size=14/>
                        </button>
                    </form>

                    <div style="margin-top: 20px; font-size: 12px; color: var(--ink-3); text-align: center">
                        "Trouble signing in? "
                        <a class="link" href="mailto:support@classforge.dev">"Contact support"</a>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ProviderButton(
    kind: ForgeKind,
    #[prop(into)] label: String,
    #[prop(into)] href: String,
) -> impl IntoView {
    let click = {
        let href = href.clone();
        move |_| {
            if let Some(win) = web_sys::window() {
                let _ = win.location().assign(&href);
            }
        }
    };
    view! {
        <button class="btn btn-lg"
                style="width: 100%; justify-content: flex-start; gap: 10px"
                on:click=click>
            <ForgeMark kind=kind size=24/>
            <span style="font-size: 14px">{label}</span>
            <span style="flex: 1"></span>
            <Icon name="chevron-right" size=14/>
        </button>
    }
}

/// Handle the `/login/callback` route. Until B1 lands the real session
/// hydration, we show a holding screen and bounce the user to onboarding.
#[component]
pub fn LoginCallbackScreen(#[allow(unused_variables)] code: String) -> impl IntoView {
    Effect::new(move |_| {
        navigate("/onboarding");
    });
    let _ = Route::Login;
    view! {
        <div class="app-shell">
            <div style="flex: 1; display: flex; align-items: center; justify-content: center">
                <div style="text-align: center; color: var(--ink-3); font-size: 13px">
                    "Completing sign-in…"
                </div>
            </div>
        </div>
    }
}
