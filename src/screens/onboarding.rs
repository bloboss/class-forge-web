//! Onboarding C — multi-forge connect.

use leptos::prelude::*;
use std::collections::HashSet;

use crate::components::ForgeMark;
use crate::data::{self, Forge, ForgeKind, ForgeStatus};
use crate::icons::Icon;
use crate::router::navigate;

#[component]
pub fn OnboardingScreen() -> impl IntoView {
    let user = data::user();
    let all = data::forges();
    let live: Vec<Forge> = all.iter().filter(|f| f.status == ForgeStatus::Live).cloned().collect();
    let soon: Vec<Forge> = all.iter().filter(|f| f.status == ForgeStatus::Soon).cloned().collect();

    let connected = RwSignal::new({
        let mut s: HashSet<String> = HashSet::new();
        s.insert("forgejo-univ".to_string());
        s
    });
    let active_add: RwSignal<Option<String>> = RwSignal::new(None);

    let count = move || connected.get().len();

    view! {
        <div class="app-shell">
            <div class="topbar">
                <div class="brand">
                    <div class="brand-dot">"C"</div>
                    <div class="brand-text">"Class"<em>"Forge"</em></div>
                </div>
                <div style="flex: 1"></div>
                <div style="font-size: 12px; color: var(--ink-3)">
                    "Signed in as "<strong style="color: var(--ink)">{user.email}</strong>
                </div>
            </div>

            <div style="flex: 1; display: flex; justify-content: center; padding: 64px 32px">
                <div style="width: 100%; max-width: 880px">
                    // Stepper pills (account ✓ → connect forges (current) → roster → done)
                    <div style="display: flex; align-items: center; gap: 10px; margin-bottom: 28px; font-size: 12px; color: var(--ink-3)">
                        <span class="pill pill-ok"><Icon name="check" size=11/>" Account"</span>
                        <span style="width: 24px; height: 1px; background: var(--line)"></span>
                        <span class="pill pill-info" style="font-weight: 600">"Connect forges"</span>
                        <span style="width: 24px; height: 1px; background: var(--line)"></span>
                        <span class="pill">"Roster sources"</span>
                        <span style="width: 24px; height: 1px; background: var(--line)"></span>
                        <span class="pill">"Done"</span>
                    </div>

                    <div class="h-display" style="margin-bottom: 14px">
                        "Connect "<em>"any number"</em>" of forges"
                    </div>
                    <div style="font-size: 16px; color: var(--ink-2); line-height: 1.55; max-width: 640px; margin-bottom: 8px">
                        "ClassForge is forge-agnostic — pick whichever your students already use. \
                         You can add and remove forges at any time, and a single classroom can even span more than one."
                    </div>
                    <div style="font-size: 13px; color: var(--ink-3); margin-bottom: 32px">
                        <strong style="color: var(--ink-2)">{move || format!("{} connected", count())}</strong>
                        " · at least one is required to continue"
                    </div>

                    <div style="font-size: 11px; font-weight: 600; letter-spacing: 0.6px; text-transform: uppercase; color: var(--ink-3); margin-bottom: 12px">
                        "Available now"
                    </div>
                    <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 12px; margin-bottom: 28px">
                        {live.into_iter().map(|f| view! { <LiveForgeCard f=f connected=connected active_add=active_add/> }).collect_view()}
                    </div>

                    <div style="font-size: 11px; font-weight: 600; letter-spacing: 0.6px; text-transform: uppercase; color: var(--ink-3); margin-bottom: 12px">
                        "Coming soon · request priority"
                    </div>
                    <div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 10px; margin-bottom: 32px">
                        {soon.into_iter().map(|f| view! {
                            <div style="padding: 12px; border-radius: 8px; border: 1px dashed var(--line); display: flex; align-items: center; gap: 10px; opacity: 0.85">
                                <ForgeMark kind=f.kind size=24/>
                                <div style="flex: 1; min-width: 0">
                                    <div style="font-size: 12px; font-weight: 500">{f.label}</div>
                                    <div style="font-size: 10px; color: var(--ink-4)">{f.note}</div>
                                </div>
                                <button class="btn btn-sm btn-ghost" style="font-size: 11px">"+1"</button>
                            </div>
                        }).collect_view()}
                    </div>

                    <div style="display: flex; justify-content: space-between; align-items: center; padding-top: 24px; border-top: 1px solid var(--line)">
                        <button class="btn btn-ghost"><Icon name="chevron-left" size=14/>" Back"</button>
                        <div style="display: flex; gap: 8px">
                            <button class="btn" on:click=move |_| navigate("/classes")>"Skip for now"</button>
                            <button class="btn btn-accent" prop:disabled=move || count() == 0
                                    on:click=move |_| navigate("/classes")>
                                {move || {
                                    let n = count();
                                    let s = if n == 1 { "" } else { "s" };
                                    format!("Continue with {n} forge{s}")
                                }}
                                " "<Icon name="chevron-right" size=14/>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn LiveForgeCard(f: Forge, connected: RwSignal<HashSet<String>>, active_add: RwSignal<Option<String>>) -> impl IntoView {
    let id: String = f.id.to_string();
    let kind = f.kind;
    let label = f.label;
    let note = f.note;
    let org = f.org;
    let accounts = f.accounts;

    let (default_url, default_org) = match kind {
        ForgeKind::Gitlab => ("https://gitlab.com", "pcu-classes"),
        _ => ("https://forge.cs.pcu.edu", "cs-dept"),
    };
    let detail_blurb = if matches!(kind, ForgeKind::Forgejo) {
        "Self-hosted Forgejo or Codeberg-flavored instance"
    } else {
        "GitLab.com"
    };

    let id_for_on = id.clone();
    let id_for_active = id.clone();
    let is_on = Memo::new(move |_| connected.get().contains(&id_for_on));
    let is_active = Memo::new(move |_| active_add.get().as_ref() == Some(&id_for_active));

    let card_style = move || if is_on.get() {
        "padding: 16px; position: relative; border-color: var(--accent-line); background: var(--accent-soft);"
    } else {
        "padding: 16px; position: relative;"
    };

    let id_toggle = id.clone();
    let id_remove = id.clone();
    let id_save = id;

    view! {
        <div class="card" style=card_style>
            <div style="display: flex; align-items: flex-start; gap: 12px">
                <ForgeMark kind=kind size=32/>
                <div style="flex: 1">
                    <div style="display: flex; align-items: center; gap: 8px">
                        <div style="font-size: 14px; font-weight: 600">{label}</div>
                        {(!note.is_empty()).then(|| view! { <span class="pill" style="font-size: 10px">{note}</span> })}
                    </div>
                    <Show when=move || is_on.get()
                          fallback=move || view! {
                              <div style="font-size: 12px; color: var(--ink-3); margin-top: 4px">{detail_blurb}</div>
                          }>
                        <div style="font-size: 12px; color: var(--ink-3); margin-top: 4px">
                            "Org "<code class="mono" style="background: var(--paper); padding: 1px 5px; border-radius: 3px">{org.unwrap_or("—")}</code>
                            " · "{accounts}" staff accounts linked"
                        </div>
                    </Show>
                </div>
                <Show when=move || is_on.get()
                      fallback=move || {
                          let id_toggle = id_toggle.clone();
                          view! {
                              <button class="btn btn-sm" on:click={
                                  let id_toggle = id_toggle.clone();
                                  move |_| {
                                      let same = active_add.get().as_ref() == Some(&id_toggle);
                                      if same { active_add.set(None); }
                                      else { active_add.set(Some(id_toggle.clone())); }
                                  }
                              }>
                                  {move || if is_active.get() {
                                      view! { <span>"Cancel"</span> }.into_any()
                                  } else {
                                      view! { <><Icon name="plus" size=12/>" Connect"</> }.into_any()
                                  }}
                              </button>
                          }
                      }>
                    {
                        let id_remove = id_remove.clone();
                        view! {
                            <div style="display: flex; gap: 6px">
                                <span class="pill pill-ok"><Icon name="check" size=11/>" Connected"</span>
                                <button class="btn btn-sm btn-ghost" on:click={
                                    let id_remove = id_remove.clone();
                                    move |_| connected.update(|c| { c.remove(&id_remove); })
                                }>"Remove"</button>
                            </div>
                        }
                    }
                </Show>
            </div>

            <Show when=move || is_active.get() fallback=|| ()>
                {
                    let id_save = id_save.clone();
                    view! {
                        <div style="margin-top: 14px; padding: 14px; border-radius: 8px; background: var(--paper); border: 1px dashed var(--line)">
                            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px">
                                <label class="label">
                                    <span>"Instance URL"</span>
                                    <input class="input" prop:value=default_url/>
                                </label>
                                <label class="label">
                                    <span>"Default org / group"</span>
                                    <input class="input" prop:value=default_org/>
                                </label>
                            </div>
                            <label class="label" style="margin-top: 10px">
                                <span>
                                    "API token "
                                    <span style="color: var(--ink-4); font-weight: 400">
                                        "· needs "<code class="mono">"repo"</code>", "<code class="mono">"org:write"</code>
                                    </span>
                                </span>
                                <input class="input mono" type="password" prop:value="ghp_••••••••••••••••"/>
                            </label>
                            <div style="display: flex; justify-content: flex-end; gap: 8px; margin-top: 12px">
                                <button class="btn btn-sm" on:click=move |_| active_add.set(None)>"Cancel"</button>
                                <button class="btn btn-sm btn-primary" on:click={
                                    let id_save = id_save.clone();
                                    move |_| {
                                        connected.update(|c| { c.insert(id_save.clone()); });
                                        active_add.set(None);
                                    }
                                }>
                                    <Icon name="check" size=12/>" Verify & connect"
                                </button>
                            </div>
                        </div>
                    }
                }
            </Show>
        </div>
    }
}
