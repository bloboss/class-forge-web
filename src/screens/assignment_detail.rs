//! Assignment drill-down screen.

use leptos::prelude::*;

use crate::components::{ForgeMark, TopBar};
use crate::data::{self, AssignmentKind, AssignmentStatus, RosterRole};
use crate::icons::Icon;
use crate::router::navigate;

#[component]
pub fn AssignmentDetail(class_id: String, asg_id: String) -> impl IntoView {
    let Some(klass) = data::class_by_id(&class_id) else {
        return view! { <div style="padding: 40px">"Classroom not found."</div> }.into_any();
    };
    let assignments = data::assignments_for_class(&class_id);
    let Some(a) = assignments.into_iter().find(|x| x.id == asg_id) else {
        return view! { <div style="padding: 40px">"Assignment not found."</div> }.into_any();
    };
    let forge = data::forge_by_id(klass.forge).expect("forge");
    let cid_back = class_id.clone();
    let klass_name = klass.name;

    let kind_label = match a.kind {
        AssignmentKind::Individual => "individual".to_string(),
        AssignmentKind::Team => {
            let (mn, mx) = a.team_size.unwrap_or((0, 0));
            format!("team · {mn}–{mx}")
        }
    };
    let status_pill_class = match a.status {
        AssignmentStatus::Active => "pill pill-info",
        AssignmentStatus::Graded => "pill",
        AssignmentStatus::Draft  => "pill pill-warn",
    };
    let status_label = match a.status {
        AssignmentStatus::Active => "active",
        AssignmentStatus::Graded => "graded",
        AssignmentStatus::Draft  => "draft",
    };
    let tests_color = if a.passing_tests > 0.7 { "var(--ok)" }
        else if a.passing_tests > 0.4 { "var(--warn)" } else { "var(--err)" };
    let late_color = if a.late > 5 { "var(--warn)" } else { "var(--ink)" };

    view! {
        <div class="app-shell">
            <TopBar/>
            <div style="border-bottom: 1px solid var(--line); background: var(--paper); padding: 20px 32px">
                <div style="max-width: 1280px; margin: 0 auto">
                    <button class="btn btn-sm btn-ghost" on:click=move |_| navigate(&format!("/classes/{cid_back}/assignments"))>
                        <Icon name="chevron-left" size=13/>" "{klass_name}" · Assignments"
                    </button>
                    <div style="display: flex; align-items: flex-end; justify-content: space-between; margin-top: 12px; gap: 24px">
                        <div style="flex: 1">
                            <div style="display: flex; gap: 8px; margin-bottom: 6px">
                                <span class="pill pill-info">{kind_label}</span>
                                <span class=status_pill_class>{status_label}</span>
                                <span class="pill"><Icon name="clock" size=10/>" due "{a.due}</span>
                            </div>
                            <div class="h-section" style="font-size: 26px">{a.title}</div>
                            <div style="display: flex; align-items: center; gap: 12px; font-size: 12px; color: var(--ink-3); margin-top: 6px">
                                <span>
                                    <Icon name="git-branch" size=12 stroke="var(--ink-3)".to_string()/>
                                    " "<code class="mono">{a.template}</code>
                                </span>
                                <ForgeMark kind=forge.kind size=14/>
                            </div>
                        </div>
                        <div style="display: flex; gap: 8px">
                            <button class="btn btn-sm">"Edit"</button>
                            <button class="btn btn-sm"><Icon name="play" size=11/>" Re-grade all"</button>
                            <button class="btn btn-sm btn-primary"><Icon name="external" size=12/>" Open template"</button>
                        </div>
                    </div>
                </div>
            </div>

            <div style="flex: 1; padding: 32px; max-width: 1280px; margin: 0 auto; width: 100%">
                <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; margin-bottom: 24px">
                    {[
                        ("Accepted",      format!("{} / {}", a.accepted, a.total),                "var(--ink)"),
                        ("Submissions",   format!("{}", a.submissions),                            "var(--ink)"),
                        ("Tests passing", format!("{}%", (a.passing_tests * 100.0).round() as i32), tests_color),
                        ("Late",          format!("{}", a.late),                                   late_color),
                    ].into_iter().map(|(l, v, c)| view! {
                        <div class="card" style="padding: 16px">
                            <div style="font-size: 11px; color: var(--ink-3); text-transform: uppercase; letter-spacing: 0.5px">{l}</div>
                            <div class="mono" style=format!("font-size: 28px; font-weight: 600; margin-top: 6px; color: {}; font-variant-numeric: tabular-nums", c)>{v}</div>
                        </div>
                    }).collect_view()}
                </div>

                <div style="display: grid; grid-template-columns: 1fr 320px; gap: 20px">
                    <div class="card" style="padding: 0">
                        <div style="padding: 12px 18px; border-bottom: 1px solid var(--line); display: flex; align-items: center; justify-content: space-between">
                            <div style="font-size: 13px; font-weight: 600">"Submissions"</div>
                            <button class="btn btn-sm btn-ghost"><Icon name="filter" size=12/>" Filter"</button>
                        </div>
                        {data::roster().into_iter()
                            .filter(|s| s.role == RosterRole::Student)
                            .take(8).map(|s| {
                                let initials = data::initials(s.name);
                                let raw = s.id.bytes().nth(3).unwrap_or(0) as i32;
                                let passing = (raw.unsigned_abs() as i32 * 31) % 100;
                                let (state_pill, bar_color, label_view) = if passing > 75 {
                                    ("pill pill-ok", "var(--ok)", view! {
                                        <span class="pill pill-ok"><Icon name="check" size=10/>" passing"</span>
                                    }.into_any())
                                } else if passing > 50 {
                                    ("pill pill-warn", "var(--warn)", view! {
                                        <span class="pill pill-warn">"partial"</span>
                                    }.into_any())
                                } else {
                                    ("pill pill-err", "var(--err)", view! {
                                        <span class="pill pill-err">"failing"</span>
                                    }.into_any())
                                };
                                let _ = state_pill;
                                view! {
                                    <div class="hover-row" style="padding: 12px 18px; border-bottom: 1px solid var(--line); display: grid; grid-template-columns: 28px 1fr 110px 110px 80px; align-items: center; gap: 12px">
                                        <div class="avatar">{initials}</div>
                                        <div style="min-width: 0">
                                            <div style="font-size: 13px; font-weight: 500">{s.name}</div>
                                            <code class="mono" style="font-size: 11px; color: var(--ink-3)">"@"{s.handle.unwrap_or("—")}</code>
                                        </div>
                                        <div>{label_view}</div>
                                        <div style="display: flex; align-items: center; gap: 6px">
                                            <div class="progress" style="flex: 1">
                                                <div style=format!("width: {}%; background: {}", passing, bar_color)></div>
                                            </div>
                                            <span class="mono" style="font-size: 11px; color: var(--ink-3)">{passing}"%"</span>
                                        </div>
                                        <button class="btn btn-sm btn-ghost">"Logs"</button>
                                    </div>
                                }
                            }).collect_view()}
                    </div>

                    <div style="display: flex; flex-direction: column; gap: 12px">
                        <div class="card" style="padding: 16px">
                            <div style="font-size: 13px; font-weight: 600; margin-bottom: 10px">"Configuration"</div>
                            {[("Type", kind_label_for(&a)),
                              ("Template", a.template.to_string()),
                              ("Branch", a.branch.to_string()),
                              ("Assigned", a.assigned.to_string()),
                              ("Deadline", a.due.to_string())].into_iter().map(|(k, v)| view! {
                                <div style="display: flex; justify-content: space-between; padding: 6px 0; font-size: 12px">
                                    <span style="color: var(--ink-3)">{k}</span>
                                    <code class="mono" style="color: var(--ink-2)">{v}</code>
                                </div>
                            }).collect_view()}
                        </div>
                        <div class="card" style="padding: 16px">
                            <div style="font-size: 13px; font-weight: 600; margin-bottom: 8px">"Danger"</div>
                            <button class="btn btn-sm" style="width: 100%; justify-content: flex-start">"Close submissions"</button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }.into_any()
}

fn kind_label_for(a: &data::Assignment) -> String {
    match a.kind {
        AssignmentKind::Individual => "individual".to_string(),
        AssignmentKind::Team => {
            let (mn, mx) = a.team_size.unwrap_or((0, 0));
            format!("team · {mn}–{mx}")
        }
    }
}
