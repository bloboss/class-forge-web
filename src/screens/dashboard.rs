//! Classes dashboard + "new classroom" modal.

use leptos::prelude::*;

use crate::components::{Cover, ForgeMark, TopBar};
use crate::data::{self, ForgeStatus};
use crate::icons::Icon;
use crate::router::navigate;

#[component]
pub fn DashboardScreen() -> impl IntoView {
    let classes = data::classes();
    let total_active: u32 = classes.iter().map(|c| c.active_assignments).sum();
    let total_pending: u32 = classes.iter().map(|c| c.pending_push).sum();
    let class_count = classes.len();

    let filter = RwSignal::new("all");
    let show_new = RwSignal::new(false);

    view! {
        <div class="app-shell">
            <TopBar/>
            <div style="flex: 1; padding: 32px 32px 64px; max-width: 1280px; width: 100%; align-self: center">
                <div style="display: flex; align-items: flex-end; justify-content: space-between; margin-bottom: 8px">
                    <div>
                        <div style="font-size: 12px; color: var(--ink-3); text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 6px">
                            "Spring 2026 · " {class_count} " active classrooms"
                        </div>
                        <div class="h-display">"Welcome back, " <em>"Mira"</em></div>
                    </div>
                    <div style="display: flex; gap: 8px">
                        <button class="btn"><Icon name="upload" size=14/>" Import roster"</button>
                        <button class="btn btn-accent" on:click=move |_| show_new.set(true)>
                            <Icon name="plus" size=14/>" New classroom"
                        </button>
                    </div>
                </div>

                <div style="font-size: 14px; color: var(--ink-2); max-width: 640px; margin-top: 14px; margin-bottom: 32px">
                    "You have "<strong>{total_active}" active assignments"</strong>
                    " across "{class_count}" classrooms, and "
                    <strong>{total_pending}" student repos"</strong>
                    " haven't been pushed to in over a week."
                </div>

                <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px">
                    <div style="display: flex; gap: 6px">
                        {[("all", "All"), ("active", "Active"), ("archived", "Archived")].iter().map(|(k, l)| {
                            let key = *k;
                            let class = move || if filter.get() == key { "btn btn-sm btn-primary" } else { "btn btn-sm" };
                            view! {
                                <button class=class on:click=move |_| filter.set(key)>
                                    {*l}
                                    {(*k == "all").then(|| view! {
                                        <span style="opacity: 0.6; margin-left: 2px">{class_count}</span>
                                    })}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    <div style="display: flex; gap: 8px; align-items: center">
                        <div class="input" style="width: 240px; gap: 8px">
                            <Icon name="search" size=14 stroke="var(--ink-3)".to_string()/>
                            <input class="input-bare" placeholder="Search classes…"/>
                            <span class="kbd">"⌘K"</span>
                        </div>
                        <button class="btn btn-sm btn-ghost"><Icon name="sliders" size=14/>" Sort"</button>
                    </div>
                </div>

                <div class="classroom-cards">
                    {classes.into_iter().map(|c| {
                        let forge = data::forge_by_id(c.forge).expect("forge exists");
                        let cid = c.id.to_string();
                        let go = cid.clone();
                        view! {
                            <div class="card hover-row" style="padding: 14px; cursor: pointer; display: flex; flex-direction: column; gap: 14px"
                                 on:click=move |_| navigate(&format!("/classes/{go}/assignments"))>
                                <Cover kind=c.cover/>
                                <div>
                                    <div style="display: flex; align-items: flex-start; justify-content: space-between; gap: 12px">
                                        <div style="flex: 1; min-width: 0">
                                            <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px">
                                                <span class="mono" style="font-size: 11px; font-weight: 600; color: var(--ink-3)">{c.number}</span>
                                                <span style="font-size: 11px; color: var(--ink-4)">"·"</span>
                                                <span style="font-size: 11px; color: var(--ink-4)">{c.term}</span>
                                            </div>
                                            <div class="h-section" style="font-size: 22px; margin-bottom: 6px">{c.name}</div>
                                        </div>
                                        <ForgeMark kind=forge.kind size=26/>
                                    </div>
                                    <div style="display: flex; align-items: center; gap: 16px; font-size: 12px; color: var(--ink-3); margin-bottom: 12px">
                                        <span>
                                            <Icon name="users" size=12 stroke="var(--ink-3)".to_string()/>
                                            " "{c.students}" students"
                                            {(c.teams > 0).then(|| view! { <>" · "{c.teams}" teams"</> })}
                                        </span>
                                        <span>
                                            <Icon name="clipboard-list" size=12 stroke="var(--ink-3)".to_string()/>
                                            " "{c.assignments_count}" assignments"
                                        </span>
                                    </div>
                                    <div style="border-top: 1px solid var(--line); padding-top: 10px">
                                        <div style="font-size: 11px; color: var(--ink-3); text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 6px">
                                            "Up next"
                                        </div>
                                        <div style="display: flex; align-items: center; justify-content: space-between; gap: 8px">
                                            <div style="font-size: 13px; font-weight: 500; overflow: hidden; text-overflow: ellipsis; white-space: nowrap">
                                                {c.next_due.name}
                                            </div>
                                            <span class="pill pill-warn" style="flex-shrink: 0"><Icon name="clock" size=10/>" "{c.next_due.due_in}</span>
                                        </div>
                                        <div style="display: flex; align-items: center; gap: 6px; font-size: 11px; color: var(--ink-3); margin-top: 8px">
                                            <div class="progress" style="flex: 1">
                                                <div style=format!("width: {}%; background: var(--accent)", (c.passing * 100.0).round() as i32)></div>
                                            </div>
                                            <span class="mono">{(c.passing * 100.0).round() as i32}"% passing"</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect_view()}

                    <div class="card" style="padding: 14px; display: flex; align-items: center; justify-content: center; border: 1px dashed var(--line); background: transparent; min-height: 280px; cursor: pointer"
                         on:click=move |_| show_new.set(true)>
                        <div style="text-align: center; color: var(--ink-3)">
                            <Icon name="plus" size=20 stroke="var(--ink-3)".to_string()/>
                            <div style="font-size: 13px; margin-top: 6px; font-weight: 500">"New classroom"</div>
                            <div style="font-size: 11px; margin-top: 2px">"Bind to a forge org or group"</div>
                        </div>
                    </div>
                </div>
            </div>

            <Show when=move || show_new.get() fallback=|| ()>
                <NewClassroomModal show_new=show_new/>
            </Show>
        </div>
    }
}

#[component]
fn NewClassroomModal(show_new: RwSignal<bool>) -> impl IntoView {
    let live: Vec<_> = data::forges()
        .into_iter()
        .filter(|f| f.status == ForgeStatus::Live)
        .collect();
    let initial = live.first().map(|f| f.id.to_string()).unwrap_or_default();
    let chosen = RwSignal::new(initial);
    let cols = format!("repeat({}, 1fr)", live.len());

    view! {
        <div class="modal-backdrop" on:click=move |_| show_new.set(false)>
            <div class="modal" on:click=move |e| e.stop_propagation()>
                <div style="padding: 18px 22px; border-bottom: 1px solid var(--line); display: flex; align-items: center; justify-content: space-between">
                    <div style="font-family: 'Instrument Serif', serif; font-size: 24px">"New classroom"</div>
                    <button class="btn btn-sm btn-ghost btn-icon-sm" on:click=move |_| show_new.set(false)>
                        <Icon name="x" size=14/>
                    </button>
                </div>
                <div style="padding: 22px; display: flex; flex-direction: column; gap: 14px">
                    <div style="display: grid; grid-template-columns: 110px 1fr; gap: 10px">
                        <label class="label"><span>"Course #"</span><input class="input mono" placeholder="CS 410"/></label>
                        <label class="label"><span>"Course name"</span><input class="input" placeholder="Compilers"/></label>
                    </div>
                    <label class="label"><span>"Term"</span><input class="input" prop:value="Spring 2026"/></label>

                    <div class="label">
                        <span>"Bind to forge"</span>
                        <div style=format!("display: grid; grid-template-columns: {cols}; gap: 8px")>
                            {live.into_iter().map(|f| {
                                let id = f.id.to_string();
                                let id_click = id.clone();
                                let kind = f.kind;
                                let label_short = f.label.split(" · ").next().unwrap_or(f.label).to_string();
                                let org = f.org.unwrap_or("—");
                                let style = move || {
                                    if chosen.get() == id {
                                        "flex-direction: column; height: auto; padding: 12px; align-items: flex-start; gap: 6px; background: var(--accent-soft); border-color: var(--accent-line)".to_string()
                                    } else {
                                        "flex-direction: column; height: auto; padding: 12px; align-items: flex-start; gap: 6px".to_string()
                                    }
                                };
                                view! {
                                    <button class="btn" style=style on:click=move |_| chosen.set(id_click.clone())>
                                        <div style="display: flex; align-items: center; gap: 8px; width: 100%">
                                            <ForgeMark kind=kind size=20/>
                                            <span style="font-size: 12px; font-weight: 600">{label_short}</span>
                                        </div>
                                        <span class="mono" style="font-size: 10px; color: var(--ink-3)">"org: "{org}</span>
                                    </button>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
                <div style="padding: 14px 22px; border-top: 1px solid var(--line); display: flex; justify-content: space-between; align-items: center">
                    <span style="font-size: 11px; color: var(--ink-3)">
                        <Icon name="lock" size=11 stroke="var(--ink-3)".to_string()/>" Visible only to you and TAs you invite"
                    </span>
                    <div style="display: flex; gap: 8px">
                        <button class="btn" on:click=move |_| show_new.set(false)>"Cancel"</button>
                        <button class="btn btn-accent">"Create classroom"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
