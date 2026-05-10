//! Classroom shell + per-tab views.

use leptos::prelude::*;

use crate::components::{ForgeMark, Stepper, TopBar};
use crate::data::{self, AssignmentKind, AssignmentStatus, Forge, RosterRole, RosterStatus};
use crate::icons::Icon;
use crate::router::navigate;

#[component]
pub fn ClassroomShell(class_id: String, tab: String) -> impl IntoView {
    let Some(klass) = data::class_by_id(&class_id) else {
        return view! { <div style="padding: 40px">"Classroom not found."</div> }.into_any();
    };
    let forge = data::forge_by_id(klass.forge).expect("forge");

    let items: Vec<(&str, &str, &str, Option<u32>)> = vec![
        ("assignments", "Assignments",      "clipboard-list", Some(klass.assignments_count)),
        ("roster",      "Roster",           "users",          Some(klass.students)),
        ("new",         "New assignment",   "plus",           None),
        ("cicd",        "CI / CD & tests",  "beaker",         None),
        ("analytics",   "Analytics",        "spark",          None),
        ("settings",    "Settings",         "settings",       None),
    ];

    let class_id_for_view = class_id.clone();
    let forge_kind = forge.kind;
    let folder_label = format!("{}/{}", forge.org.unwrap_or("—"), klass.number.to_lowercase().replace(' ', "-"));
    let forge_label_short = forge.label.split(" · ").next().unwrap_or(forge.label).to_string();
    let forge_label_short_pill = forge_label_short.clone();

    let tab_for_view = tab.clone();
    let tab_for_sidebar = tab.clone();

    view! {
        <div class="app-shell">
            <TopBar/>

            <div style="border-bottom: 1px solid var(--line); background: var(--paper); padding: 20px 32px">
                <div style="max-width: 1280px; margin: 0 auto; display: flex; align-items: center; gap: 16px">
                    <button class="btn btn-sm btn-ghost" on:click=move |_| navigate("/classes")>
                        <Icon name="chevron-left" size=13/>" All classrooms"
                    </button>
                    <div class="divider-v"></div>
                    <ForgeMark kind=forge_kind size=28/>
                    <div style="flex: 1">
                        <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 2px">
                            <span class="mono" style="font-size: 11px; font-weight: 600; color: var(--ink-3)">{klass.number}</span>
                            <span style="font-size: 11px; color: var(--ink-4)">"·"</span>
                            <span style="font-size: 11px; color: var(--ink-4)">{klass.term}</span>
                            <span class="pill pill-info" style="margin-left: 4px">
                                <ForgeMark kind=forge_kind size=12/>" "{forge_label_short_pill}
                            </span>
                            <span class="pill">
                                <Icon name="folder" size=10 stroke="var(--ink-3)".to_string()/>
                                " "<code class="mono">{folder_label}</code>
                            </span>
                        </div>
                        <div class="h-section">{klass.name}</div>
                    </div>
                    <div style="display: flex; gap: 8px">
                        <button class="btn btn-sm"><Icon name="external" size=13/>" Open on forge"</button>
                        <button class="btn btn-sm btn-ghost btn-icon"><Icon name="more" size=15/></button>
                    </div>
                </div>
            </div>

            <div style="flex: 1; display: flex; max-width: 1280px; margin: 0 auto; width: 100%">
                <div style="width: 220px; padding: 24px 0 24px 16px; flex-shrink: 0">
                    <div style="position: sticky; top: 24px; display: flex; flex-direction: column; gap: 2px">
                        {items.into_iter().map(|(id, label, ico, count)| {
                            let id_owned = id.to_string();
                            let id_click = id_owned.clone();
                            let cid_click = class_id.clone();
                            let active = id_owned == tab_for_sidebar;
                            let class = if active { "sidebar-item active" } else { "sidebar-item" };
                            view! {
                                <button class=class on:click=move |_| navigate(&format!("/classes/{}/{}", cid_click, id_click))>
                                    <span class="ico"><Icon name=ico.to_string() size=15/></span>
                                    <span style="flex: 1">{label}</span>
                                    {count.map(|n| view! { <span style="font-size: 10px; color: var(--ink-4)">{n}</span> })}
                                </button>
                            }
                        }).collect_view()}
                        <div style="height: 1px; background: var(--line); margin: 12px 10px"></div>
                        <button class="sidebar-item"><span class="ico"><Icon name="mail" size=15/></span>"Email roster"</button>
                        <button class="sidebar-item"><span class="ico"><Icon name="database" size=15/></span>"Export grades"</button>
                    </div>
                </div>

                <div style="flex: 1; padding: 24px 32px 64px; min-width: 0">
                    {move || {
                        let cid = class_id_for_view.clone();
                        match tab_for_view.as_str() {
                            "assignments" => view! { <AssignmentsView class_id=cid/> }.into_any(),
                            "roster"      => view! { <RosterView/> }.into_any(),
                            "new"         => view! { <NewAssignmentView class_id=cid/> }.into_any(),
                            "cicd"        => view! { <CICDView/> }.into_any(),
                            "analytics"   => view! { <AnalyticsView/> }.into_any(),
                            "settings"    => view! { <SettingsView class_id=cid/> }.into_any(),
                            _ => view! { <AssignmentsView class_id=cid/> }.into_any(),
                        }
                    }}
                </div>
            </div>
        </div>
    }.into_any()
}

// ─── Assignments tab ─────────────────────────────────────────────────────────

#[component]
fn AssignmentsView(class_id: String) -> impl IntoView {
    let all = data::assignments_for_class(&class_id);
    let total = all.len();
    let n_active = all.iter().filter(|a| a.status == AssignmentStatus::Active).count();
    let n_graded = all.iter().filter(|a| a.status == AssignmentStatus::Graded).count();
    let n_draft  = all.iter().filter(|a| a.status == AssignmentStatus::Draft).count();

    let filter = RwSignal::new("all".to_string());
    let q = RwSignal::new(String::new());

    let class_id_new = class_id.clone();
    let class_id_filter = class_id.clone();

    let view_rows = move || {
        let f = filter.get();
        let qq = q.get().to_lowercase();
        let cid = class_id_filter.clone();
        data::assignments_for_class(&cid).into_iter().filter(|a| {
            (f == "all" || matches!((f.as_str(), a.status),
                ("active", AssignmentStatus::Active) |
                ("graded", AssignmentStatus::Graded) |
                ("draft",  AssignmentStatus::Draft)))
            && (qq.is_empty() || a.title.to_lowercase().contains(&qq))
        }).collect::<Vec<_>>()
    };

    view! {
        <div>
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px">
                <div class="h-section" style="font-size: 28px">"Assignments"</div>
                <div style="display: flex; gap: 8px">
                    <button class="btn"><Icon name="upload" size=13/>" Import"</button>
                    <button class="btn btn-accent" on:click=move |_| navigate(&format!("/classes/{class_id_new}/new"))>
                        <Icon name="plus" size=13/>" New assignment"
                    </button>
                </div>
            </div>

            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 16px; gap: 12px">
                <div style="display: flex; gap: 4px">
                    {[("all","All", total), ("active","Active", n_active), ("graded","Graded", n_graded), ("draft","Draft", n_draft)]
                        .into_iter().map(|(k, l, n)| {
                            let key = k.to_string();
                            let key_click = key.clone();
                            let class = move || if filter.get() == key { "btn btn-sm btn-primary" } else { "btn btn-sm btn-ghost" };
                            view! {
                                <button class=class on:click=move |_| filter.set(key_click.clone())>
                                    {l}" "<span style="opacity: 0.6; margin-left: 4px">{n}</span>
                                </button>
                            }
                        }).collect_view()}
                </div>
                <div class="input" style="width: 280px; gap: 8px">
                    <Icon name="search" size=14 stroke="var(--ink-3)".to_string()/>
                    <input class="input-bare" placeholder="Search assignments…"
                           prop:value=move || q.get()
                           on:input=move |e| q.set(event_target_value(&e))/>
                </div>
            </div>

            <div class="card" style="padding: 0; overflow: hidden">
                <div class="assn-head">
                    <div>"Assignment"</div>
                    <div>"Accepted"</div>
                    <div>"Tests passing"</div>
                    <div>"Due"</div>
                    <div></div>
                </div>
                <For
                    each=move || view_rows()
                    key=|a| a.id.to_string()
                    children=move |a| {
                        let cid_click = class_id.clone();
                        let asg_id = a.id.to_string();
                        let is_team = a.kind == AssignmentKind::Team;
                        let team_blurb = if is_team {
                            let (mn, mx) = a.team_size.unwrap_or((0, 0));
                            format!("team · {mn}–{mx}")
                        } else { "individual".to_string() };
                        let accepted_pct = if a.total > 0 { a.accepted as f32 / a.total as f32 } else { 0.0 };
                        let tests_color = if a.passing_tests > 0.7 { "var(--ok)" }
                            else if a.passing_tests > 0.4 { "var(--warn)" } else { "var(--err)" };
                        let due_in_color = if a.due_in == "closed" || a.due_in == "draft" { "var(--ink-4)" } else { "var(--warm)" };
                        view! {
                            <div class="hover-row assn-row"
                                 on:click=move |_| navigate(&format!("/classes/{cid_click}/a/{asg_id}"))>
                                <div style="min-width: 0">
                                    <div style="display: flex; align-items: center; gap: 8px; margin-bottom: 4px">
                                        <div style="font-size: 13px; font-weight: 600; color: var(--ink); overflow: hidden; text-overflow: ellipsis; white-space: nowrap">
                                            {a.title}
                                        </div>
                                        {match a.status {
                                            AssignmentStatus::Active => view! { <span class="pill pill-info">"active"</span> }.into_any(),
                                            AssignmentStatus::Graded => view! { <span class="pill">"graded"</span> }.into_any(),
                                            AssignmentStatus::Draft  => view! { <span class="pill pill-warn">"draft"</span> }.into_any(),
                                        }}
                                    </div>
                                    <div style="display: flex; align-items: center; gap: 10px; font-size: 11px; color: var(--ink-3)">
                                        <span>
                                            <Icon name="git-branch" size=11 stroke="var(--ink-3)".to_string()/>
                                            " "{a.template}
                                        </span>
                                        <span>"·"</span>
                                        <span>{team_blurb}</span>
                                    </div>
                                </div>
                                <div>
                                    <div style="display: flex; align-items: baseline; gap: 4px; margin-bottom: 4px">
                                        <span class="mono" style="font-size: 14px; font-weight: 600; font-variant-numeric: tabular-nums">{a.accepted}</span>
                                        <span class="mono" style="font-size: 11px; color: var(--ink-3)">
                                            "/ "{a.total}{is_team.then_some(" teams")}
                                        </span>
                                    </div>
                                    <div class="progress" style="width: 100px">
                                        <div style=format!("width: {}%; background: {}",
                                            (accepted_pct * 100.0).round() as i32,
                                            if accepted_pct > 0.9 { "var(--ok)" } else { "var(--ink)" })></div>
                                    </div>
                                </div>
                                <div>
                                    {match a.status {
                                        AssignmentStatus::Draft => view! {
                                            <span style="font-size: 12px; color: var(--ink-4)">"—"</span>
                                        }.into_any(),
                                        _ => view! {
                                            <>
                                                <div class="mono" style="font-size: 14px; font-weight: 600; font-variant-numeric: tabular-nums; margin-bottom: 4px">
                                                    {(a.passing_tests * 100.0).round() as i32}"%"
                                                </div>
                                                <div class="progress" style="width: 100px">
                                                    <div style=format!("width: {}%; background: {}",
                                                        (a.passing_tests * 100.0).round() as i32, tests_color)></div>
                                                </div>
                                            </>
                                        }.into_any(),
                                    }}
                                </div>
                                <div style="font-size: 12px">
                                    <div style="color: var(--ink); font-weight: 500">{a.due}</div>
                                    <div style=format!("color: {}; font-size: 11px; margin-top: 2px", due_in_color)>
                                        {a.due_in}
                                    </div>
                                </div>
                                <div style="display: flex; justify-content: flex-end">
                                    <button class="btn btn-sm btn-ghost btn-icon-sm" on:click=move |e| e.stop_propagation()>
                                        <Icon name="more" size=15/>
                                    </button>
                                </div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

// ─── Roster tab ──────────────────────────────────────────────────────────────

#[component]
fn RosterView() -> impl IntoView {
    let all = data::roster();
    let role = RwSignal::new("all".to_string());
    let status_f = RwSignal::new("all".to_string());
    let section = RwSignal::new("all".to_string());
    let chip: RwSignal<Option<String>> = RwSignal::new(None);
    let q = RwSignal::new(String::new());

    let total_count = all.len();
    let filtered = move || {
        let r = role.get(); let s = status_f.get(); let sec = section.get();
        let c = chip.get(); let qq = q.get().to_lowercase();
        data::roster().into_iter().filter(|s_e| {
            let role_ok = match r.as_str() {
                "all" => true,
                "student" => s_e.role == RosterRole::Student,
                "TA" => s_e.role == RosterRole::Ta,
                _ => true,
            };
            let status_ok = match s.as_str() {
                "all" => true,
                "linked" => s_e.status == RosterStatus::Linked,
                "pending" => s_e.status == RosterStatus::Pending,
                "invited" => s_e.status == RosterStatus::Invited,
                _ => true,
            };
            let sec_ok = sec == "all" || s_e.section == sec;
            let chip_ok = match c.as_deref() {
                Some("low-grade") => s_e.grade.map(|g| g < 0.7).unwrap_or(false),
                Some("stale-push") => {
                    let lp = s_e.last_push;
                    if let Some(num) = lp.split('d').next().and_then(|x| x.trim().parse::<u32>().ok()) {
                        num >= 7 && lp.contains('d')
                    } else { false }
                },
                _ => true,
            };
            let q_ok = qq.is_empty()
                || s_e.name.to_lowercase().contains(&qq)
                || s_e.email.to_lowercase().contains(&qq)
                || s_e.handle.map(|h| h.to_lowercase().contains(&qq)).unwrap_or(false);
            role_ok && status_ok && sec_ok && chip_ok && q_ok
        }).collect::<Vec<_>>()
    };

    view! {
        <div>
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 20px">
                <div class="h-section" style="font-size: 28px">"Roster"</div>
                <div style="display: flex; gap: 8px">
                    <button class="btn"><Icon name="upload" size=13/>" Import CSV"</button>
                    <button class="btn"><Icon name="globe" size=13/>" Sync from Canvas"</button>
                    <button class="btn btn-accent"><Icon name="plus" size=13/>" Invite"</button>
                </div>
            </div>

            <div style="display: flex; gap: 8px; margin-bottom: 12px; flex-wrap: wrap; align-items: center">
                <Select sig=role options=vec![("all","All roles"),("student","Students"),("TA","TAs")]/>
                <Select sig=status_f options=vec![("all","Any status"),("linked","Linked"),("pending","Pending"),("invited","Invited")]/>
                <Select sig=section options=vec![("all","All sections"),("A","Section A"),("B","Section B")]/>
                <div class="divider-v"></div>
                <ChipBtn label="Below 70%".to_string() key="low-grade".to_string() chip=chip/>
                <ChipBtn label="Not pushed in 7d".to_string() key="stale-push".to_string() chip=chip/>
                <div style="flex: 1"></div>
                <div class="input" style="width: 260px; gap: 8px">
                    <Icon name="search" size=14 stroke="var(--ink-3)".to_string()/>
                    <input class="input-bare" placeholder="Search name, email, handle…"
                           prop:value=move || q.get()
                           on:input=move |e| q.set(event_target_value(&e))/>
                </div>
            </div>

            <div style="font-size: 12px; color: var(--ink-3); margin-bottom: 10px">
                "Showing "<strong style="color: var(--ink)">{move || filtered().len()}</strong>
                " of "{total_count}
            </div>

            <div class="card" style="padding: 0; overflow: hidden">
                <div class="roster-head">
                    <div></div><div>"Student"</div><div>"Forge handle"</div><div>"Section"</div>
                    <div>"Role"</div><div>"Status"</div><div>"Grade"</div><div>"Last push"</div><div></div>
                </div>
                <For
                    each=move || filtered()
                    key=|s| s.id.to_string()
                    children=move |s| {
                        let forge: Forge = data::forge_by_id(s.forge).expect("forge");
                        let initials = data::initials(s.name);
                        let stale = s.last_push.contains('d')
                            && s.last_push.split('d').next()
                                .and_then(|x| x.trim().parse::<u32>().ok())
                                .map(|n| n >= 7).unwrap_or(false);
                        let grade_color = match s.grade {
                            Some(g) if g >= 0.85 => "var(--ok)",
                            Some(g) if g >= 0.7  => "var(--ink)",
                            Some(_) => "var(--err)",
                            None => "var(--ink-4)",
                        };
                        view! {
                            <div class="hover-row roster-row">
                                <div><div class="avatar">{initials}</div></div>
                                <div style="min-width: 0">
                                    <div style="font-size: 13px; font-weight: 500">{s.name}</div>
                                    <div class="mono" style="font-size: 11px; color: var(--ink-3); overflow: hidden; text-overflow: ellipsis; white-space: nowrap">
                                        {s.email}{(s.sid != "—").then(|| format!(" · {}", s.sid))}
                                    </div>
                                </div>
                                <div style="display: flex; align-items: center; gap: 6px">
                                    {match s.handle {
                                        Some(h) => view! {
                                            <>
                                                <ForgeMark kind=forge.kind size=16/>
                                                <span class="mono" style="font-size: 12px">"@"{h}</span>
                                            </>
                                        }.into_any(),
                                        None => view! {
                                            <span style="font-size: 11px; color: var(--ink-4); font-style: italic">"not linked"</span>
                                        }.into_any(),
                                    }}
                                </div>
                                <div style="font-size: 12px; color: var(--ink-2)">{s.section}</div>
                                <div>
                                    <span class="pill" style=match s.role {
                                        RosterRole::Ta => "background: var(--accent-soft); color: var(--accent); border-color: var(--accent-line)",
                                        RosterRole::Student => "background: var(--fill); color: var(--ink-3); border-color: var(--line)",
                                    }>{match s.role { RosterRole::Ta => "TA", RosterRole::Student => "student" }}</span>
                                </div>
                                <div>
                                    {match s.status {
                                        RosterStatus::Linked => view! { <span class="pill pill-ok"><Icon name="check" size=10/>" Linked"</span> }.into_any(),
                                        RosterStatus::Pending => view! { <span class="pill pill-warn">"Pending link"</span> }.into_any(),
                                        RosterStatus::Invited => view! { <span class="pill">"Invited"</span> }.into_any(),
                                    }}
                                </div>
                                <div>
                                    {match s.grade {
                                        Some(g) => view! {
                                            <span class="mono" style=format!("font-size: 13px; font-weight: 600; font-variant-numeric: tabular-nums; color: {}", grade_color)>
                                                {(g * 100.0).round() as i32}"%"
                                            </span>
                                        }.into_any(),
                                        None => view! { <span style="font-size: 11px; color: var(--ink-4)">"—"</span> }.into_any(),
                                    }}
                                </div>
                                <div class="mono" style=format!("font-size: 11px; color: {}",
                                        if stale { "var(--warn)" } else { "var(--ink-3)" })>
                                    {s.last_push}
                                </div>
                                <div><button class="btn btn-sm btn-ghost btn-icon-sm"><Icon name="more" size=14/></button></div>
                            </div>
                        }
                    }
                />
            </div>
        </div>
    }
}

#[component]
fn Select(sig: RwSignal<String>, options: Vec<(&'static str, &'static str)>) -> impl IntoView {
    view! {
        <div class="select-wrap">
            <select class="select-native"
                    prop:value=move || sig.get()
                    on:change=move |e| sig.set(event_target_value(&e))>
                {options.into_iter().map(|(v, l)| view! { <option value=v>{l}</option> }).collect_view()}
            </select>
            <div class="select-arrow"><Icon name="chevron-down" size=12/></div>
        </div>
    }
}

#[component]
fn ChipBtn(label: String, key: String, chip: RwSignal<Option<String>>) -> impl IntoView {
    let key_click = key.clone();
    let key_active = key.clone();
    let active = move || chip.get().as_ref() == Some(&key_active);
    let style = move || {
        if active() {
            "cursor: pointer; height: 26px; padding: 0 10px; background: var(--ink); color: var(--paper); border-color: var(--ink)".to_string()
        } else {
            "cursor: pointer; height: 26px; padding: 0 10px; background: var(--paper); color: var(--ink-3); border-color: var(--line)".to_string()
        }
    };
    view! {
        <button class="pill" style=style on:click=move |_| {
            chip.update(|c| {
                if c.as_ref() == Some(&key_click) { *c = None; } else { *c = Some(key_click.clone()); }
            });
        }>{label}</button>
    }
}

// ─── New assignment tab ──────────────────────────────────────────────────────

#[derive(Clone)]
struct NewAssignmentForm {
    title: String,
    kind: AssignmentKind,
    team_min: u32,
    team_max: u32,
    template: String,
    deadline: String,
    timezone: String,
    visibility: String,
    run_student_tests: bool,
    run_hidden_tests: bool,
    framework: String,
    image: String,
}

impl Default for NewAssignmentForm {
    fn default() -> Self {
        Self {
            title: String::new(),
            kind: AssignmentKind::Individual,
            team_min: 2, team_max: 3,
            template: String::new(),
            deadline: "2026-05-22".to_string(),
            timezone: "America/Los_Angeles".to_string(),
            visibility: "private-fork".to_string(),
            run_student_tests: true,
            run_hidden_tests: true,
            framework: "pytest".to_string(),
            image: "python:3.12-slim".to_string(),
        }
    }
}

#[component]
fn NewAssignmentView(class_id: String) -> impl IntoView {
    let step = RwSignal::new(0usize);
    let form = RwSignal::new(NewAssignmentForm::default());
    let cid_discard = class_id.clone();
    let cid_publish = class_id.clone();
    let cid_step3 = class_id.clone();
    let steps = vec!["Basics", "Source", "Tests & CI", "Distribution"];

    view! {
        <div>
            <div style="display: flex; align-items: center; justify-content: space-between; margin-bottom: 4px">
                <div class="h-section" style="font-size: 28px">"New assignment"</div>
                <button class="btn btn-ghost" on:click=move |_| navigate(&format!("/classes/{cid_discard}/assignments"))>
                    <Icon name="x" size=13/>" Discard"
                </button>
            </div>
            <div style="font-size: 13px; color: var(--ink-3); margin-bottom: 28px">
                "We'll create a per-student fork of your template repo, wire it to CI, and notify your roster when you publish."
            </div>

            {move || view! { <Stepper steps=steps.clone() current=step.get()/> }}

            <div style="display: grid; grid-template-columns: 1fr 320px; gap: 24px">
                <div class="card" style="padding: 28px">
                    {move || match step.get() {
                        0 => view! { <StepBasics form=form/> }.into_any(),
                        1 => view! { <StepSource form=form/> }.into_any(),
                        2 => view! { <StepTests form=form/> }.into_any(),
                        _ => view! { <StepDistribution form=form class_id=cid_step3.clone()/> }.into_any(),
                    }}
                    <div style="display: flex; justify-content: space-between; margin-top: 28px; padding-top: 20px; border-top: 1px solid var(--line)">
                        <button class="btn" prop:disabled=move || step.get() == 0
                                on:click=move |_| step.update(|s| if *s > 0 { *s -= 1 })>
                            <Icon name="chevron-left" size=13/>" Back"
                        </button>
                        {move || {
                            let cur = step.get();
                            let cid = cid_publish.clone();
                            if cur < 3 {
                                view! {
                                    <button class="btn btn-accent" on:click=move |_| step.update(|s| *s += 1)>
                                        "Continue "<Icon name="chevron-right" size=13/>
                                    </button>
                                }.into_any()
                            } else {
                                view! {
                                    <div style="display: flex; gap: 8px">
                                        <button class="btn">"Save as draft"</button>
                                        <button class="btn btn-accent" on:click=move |_| navigate(&format!("/classes/{cid}/assignments"))>
                                            <Icon name="sparkles" size=13/>" Publish to roster"
                                        </button>
                                    </div>
                                }.into_any()
                            }
                        }}
                    </div>
                </div>

                <div style="position: sticky; top: 20px; align-self: flex-start">
                    <div style="font-size: 11px; font-weight: 600; color: var(--ink-3); text-transform: uppercase; letter-spacing: 0.5px; margin-bottom: 10px">
                        "Preview"
                    </div>
                    <div class="card" style="padding: 16px">
                        <div style="font-size: 11px; color: var(--ink-3); margin-bottom: 4px">"Student will see:"</div>
                        <div style="font-size: 14px; font-weight: 600; margin-bottom: 10px">
                            {move || { let f = form.get(); if f.title.is_empty() { "Untitled assignment".to_string() } else { f.title } }}
                        </div>
                        <div style="display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 14px">
                            <span class="pill pill-info">{move || {
                                let f = form.get();
                                match f.kind {
                                    AssignmentKind::Individual => "individual".to_string(),
                                    AssignmentKind::Team => format!("team · {}–{}", f.team_min, f.team_max),
                                }
                            }}</span>
                            <span class="pill"><Icon name="clock" size=10/>" due "{move || form.get().deadline}</span>
                            {move || form.get().run_student_tests.then(|| view! {
                                <span class="pill pill-ok"><Icon name="check" size=10/>" tests on push"</span>
                            })}
                            {move || form.get().run_hidden_tests.then(|| view! {
                                <span class="pill"><Icon name="lock" size=10 stroke="var(--ink-3)".to_string()/>" hidden grader"</span>
                            })}
                        </div>
                        <div style="border-top: 1px solid var(--line); padding-top: 10px; font-size: 11px; color: var(--ink-3)">
                            <div style="display: flex; justify-content: space-between; margin-bottom: 4px">
                                <span>"Template"</span>
                                <span class="mono" style="color: var(--ink-2)">
                                    {move || { let t = form.get().template; if t.is_empty() { "—".to_string() } else { t } }}
                                </span>
                            </div>
                            <div style="display: flex; justify-content: space-between; margin-bottom: 4px">
                                <span>"Image"</span>
                                <span class="mono" style="color: var(--ink-2)">{move || form.get().image}</span>
                            </div>
                            <div style="display: flex; justify-content: space-between">
                                <span>"Framework"</span>
                                <span class="mono" style="color: var(--ink-2)">{move || form.get().framework}</span>
                            </div>
                        </div>
                    </div>
                    <div style="margin-top: 12px; padding: 12px; border-radius: 8px; background: var(--accent-soft); font-size: 11px; color: var(--accent); line-height: 1.5">
                        <Icon name="sparkles" size=12 stroke="var(--accent)".to_string()/>
                        " "<strong>"Tip:"</strong>" Click step pills to jump back & forth. Nothing is created until you publish."
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StepBasics(form: RwSignal<NewAssignmentForm>) -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 18px">
            <div class="h-section" style="font-size: 22px">"Basics"</div>
            <label class="label">
                <span>"Title"</span>
                <input class="input" placeholder="Homework 5 — Graph algorithms"
                       prop:value=move || form.get().title
                       on:input=move |e| form.update(|f| f.title = event_target_value(&e))/>
            </label>
            <div class="label">
                <span>"Type"</span>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 10px">
                    {[(AssignmentKind::Individual, "Individual", "One repo per student"),
                      (AssignmentKind::Team, "Team", "One repo per team")].iter().map(|(k, l, sub)| {
                        let kind = *k; let label = *l; let s = *sub;
                        let style = move || {
                            let active = form.get().kind == kind;
                            if active {
                                "flex-direction: column; align-items: flex-start; height: auto; padding: 14px; gap: 4px; text-align: left; background: var(--accent-soft); border-color: var(--accent-line)".to_string()
                            } else {
                                "flex-direction: column; align-items: flex-start; height: auto; padding: 14px; gap: 4px; text-align: left".to_string()
                            }
                        };
                        view! {
                            <button class="btn" style=style on:click=move |_| form.update(|f| f.kind = kind)>
                                <span style="font-size: 13px; font-weight: 600">{label}</span>
                                <span style="font-size: 11px; color: var(--ink-3)">{s}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
            {move || (form.get().kind == AssignmentKind::Team).then(|| view! {
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px">
                    <label class="label">
                        <span>"Min team size"</span>
                        <input class="input mono" type="number"
                               prop:value=move || form.get().team_min.to_string()
                               on:input=move |e| form.update(|f| f.team_min = event_target_value(&e).parse().unwrap_or(2))/>
                    </label>
                    <label class="label">
                        <span>"Max team size"</span>
                        <input class="input mono" type="number"
                               prop:value=move || form.get().team_max.to_string()
                               on:input=move |e| form.update(|f| f.team_max = event_target_value(&e).parse().unwrap_or(3))/>
                    </label>
                </div>
            })}
            <div style="display: grid; grid-template-columns: 1fr 200px; gap: 12px">
                <label class="label">
                    <span>"Deadline"</span>
                    <input class="input mono" type="date"
                           prop:value=move || form.get().deadline
                           on:input=move |e| form.update(|f| f.deadline = event_target_value(&e))/>
                </label>
                <label class="label">
                    <span>"Timezone"</span>
                    <select class="input"
                            prop:value=move || form.get().timezone
                            on:change=move |e| form.update(|f| f.timezone = event_target_value(&e))>
                        <option>"America/Los_Angeles"</option>
                        <option>"America/New_York"</option>
                        <option>"Europe/Berlin"</option>
                        <option>"UTC"</option>
                    </select>
                </label>
            </div>
        </div>
    }
}

#[component]
fn StepSource(form: RwSignal<NewAssignmentForm>) -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 18px">
            <div class="h-section" style="font-size: 22px">"Starter code & template"</div>
            <label class="label">
                <span>"Template repository "
                    <span style="color: var(--ink-4); font-weight: 400">"· will be forked per student"</span>
                </span>
                <div style="display: flex; gap: 8px">
                    <input class="input mono" style="flex: 1"
                           placeholder="cs331-staff/hw05-template"
                           prop:value=move || form.get().template
                           on:input=move |e| form.update(|f| f.template = event_target_value(&e))/>
                    <button class="btn"><Icon name="search" size=13/>" Browse"</button>
                </div>
            </label>
            <div class="label">
                <span>"Visibility"</span>
                <div style="display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 10px">
                    {[("private-fork","Private fork","Only the student & TAs"),
                      ("private-shared","Private + roster","Whole class can read"),
                      ("public","Public","Open repos")].iter().map(|(v, l, sub)| {
                        let vv = v.to_string(); let vv_click = vv.clone();
                        let label = *l; let s = *sub;
                        let style = move || {
                            let active = form.get().visibility == vv;
                            if active {
                                "flex-direction: column; align-items: flex-start; height: auto; padding: 12px; gap: 4px; text-align: left; background: var(--accent-soft); border-color: var(--accent-line)".to_string()
                            } else {
                                "flex-direction: column; align-items: flex-start; height: auto; padding: 12px; gap: 4px; text-align: left".to_string()
                            }
                        };
                        view! {
                            <button class="btn" style=style on:click=move |_| form.update(|f| f.visibility = vv_click.clone())>
                                <span style="font-size: 13px; font-weight: 600">{label}</span>
                                <span style="font-size: 11px; color: var(--ink-3)">{s}</span>
                            </button>
                        }
                    }).collect_view()}
                </div>
            </div>
            <div style="padding: 12px; background: var(--fill); border-radius: 7px; font-size: 12px; color: var(--ink-3); line-height: 1.6">
                <strong style="color: var(--ink)">"Heads-up:"</strong>
                " the template must already live on the forge bound to this classroom (Forgejo · forge.cs.pcu.edu). Cross-forge templates aren't supported yet."
            </div>
        </div>
    }
}

#[component]
fn StepTests(form: RwSignal<NewAssignmentForm>) -> impl IntoView {
    let frameworks = vec!["pytest", "JUnit", "Jest", "cargo test", "go test", "Catch2", "Custom command"];
    view! {
        <div style="display: flex; flex-direction: column; gap: 18px">
            <div class="h-section" style="font-size: 22px">"Tests & CI"</div>

            <div class="label">
                <span>"What runs on every student push?"</span>
                <div class="card" style="padding: 14px">
                    <ToggleRow on=Memo::new(move |_| form.get().run_student_tests)
                               set_on=move |v| form.update(|f| f.run_student_tests = v)
                               title="Student-visible tests"
                               sub="Pass/fail visible in repo. Encourages early debugging."/>
                    <div style="height: 1px; background: var(--line); margin: 12px 0"></div>
                    <ToggleRow on=Memo::new(move |_| form.get().run_hidden_tests)
                               set_on=move |v| form.update(|f| f.run_hidden_tests = v)
                               title="Hidden grader tests"
                               sub="Run separately on a private branch — students can't see fixtures."/>
                </div>
            </div>

            <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px">
                <label class="label">
                    <span>"Framework"</span>
                    <select class="input"
                            prop:value=move || form.get().framework
                            on:change=move |e| form.update(|f| f.framework = event_target_value(&e))>
                        {frameworks.into_iter().map(|f| view! { <option>{f}</option> }).collect_view()}
                    </select>
                </label>
                <label class="label">
                    <span>"Container image"</span>
                    <input class="input mono"
                           prop:value=move || form.get().image
                           on:input=move |e| form.update(|f| f.image = event_target_value(&e))/>
                </label>
            </div>

            <div class="label">
                <span>"Run command preview"</span>
                <div class="mono" style="padding: 12px; border-radius: 7px; background: var(--paper-2); border: 1px solid var(--line); font-size: 12px; color: var(--ink-2)">
                    <span style="color: var(--ink-4)">"$ "</span>
                    {move || match form.get().framework.as_str() {
                        "pytest"          => "pytest tests/ --junit-xml=results.xml",
                        "JUnit"           => "mvn test",
                        "Jest"            => "npm test -- --reporters=jest-junit",
                        "cargo test"      => "cargo test --message-format=json",
                        "go test"         => "go test ./... -json",
                        "Catch2"          => "cmake --build build && ./build/tests",
                        "Custom command"  => "make grade",
                        _ => "",
                    }}
                </div>
            </div>
        </div>
    }
}

#[component]
fn ToggleRow<F>(
    #[prop(into)] on: Memo<bool>,
    set_on: F,
    title: &'static str,
    sub: &'static str,
) -> impl IntoView
where F: Fn(bool) + 'static + Copy {
    view! {
        <div style="display: flex; align-items: center; gap: 12px">
            <div style="flex: 1">
                <div style="font-size: 13px; font-weight: 500">{title}</div>
                <div style="font-size: 12px; color: var(--ink-3); margin-top: 2px">{sub}</div>
            </div>
            <button class=move || if on.get() { "toggle on" } else { "toggle" }
                    on:click=move |_| set_on(!on.get())></button>
        </div>
    }
}

#[component]
fn StepDistribution(form: RwSignal<NewAssignmentForm>, class_id: String) -> impl IntoView {
    let klass = data::class_by_id(&class_id).expect("class");
    let class_number = klass.number.to_string();
    let class_students = klass.students;
    let class_slug = class_number.to_lowercase().replace(' ', "-");
    view! {
        <div style="display: flex; flex-direction: column; gap: 18px">
            <div class="h-section" style="font-size: 22px">"Ready to publish"</div>
            <div class="card" style="padding: 18px; background: var(--paper-2); border: none">
                <div style="font-size: 13px; color: var(--ink-2); line-height: 1.6">
                    "When you publish, ClassForge will:"
                    <ol style="margin: 10px 0 0; padding-left: 22px; line-height: 1.8">
                        <li>"Fork "
                            <code class="mono">{move || {
                                let t = form.get().template;
                                if t.is_empty() { "<template>".to_string() } else { t }
                            }}</code>
                            " into "<strong>{class_students}</strong>
                            " per-student repos under "
                            <code class="mono">{format!("cs-dept/{}-hw/", class_slug)}</code>
                        </li>
                        <li>"Install CI workflow ("
                            {move || form.get().framework}
                            ", image "
                            <code class="mono">{move || form.get().image}</code>
                            ")"
                        </li>
                        <li>"Email every student in the roster with their repo URL"</li>
                        <li>"Open a Slack/email digest for TAs"</li>
                    </ol>
                </div>
            </div>
            <ToggleRow on=Memo::new(|_| true) set_on=|_| {}
                       title="Notify roster on publish"
                       sub="Send email + Canvas announcement (if connected)"/>
        </div>
    }
}

// ─── CI/CD tab ───────────────────────────────────────────────────────────────

#[component]
fn CICDView() -> impl IntoView {
    let tab = RwSignal::new("runners".to_string());
    view! {
        <div>
            <div class="h-section" style="font-size: 28px; margin-bottom: 6px">"CI / CD & tests"</div>
            <div style="font-size: 13px; color: var(--ink-3); margin-bottom: 20px">
                "Defaults applied to every assignment in this classroom. Overridable per assignment."
            </div>

            <div class="tabs" style="margin-bottom: 20px">
                {[("runners","Runners & images"),("templates","Test templates"),("secrets","Secrets"),("logs","Recent runs")]
                    .into_iter().map(|(k, l)| {
                        let key = k.to_string(); let key_click = key.clone();
                        let class = move || if tab.get() == key { "tab active" } else { "tab" };
                        view! { <button class=class on:click=move |_| tab.set(key_click.clone())>{l}</button> }
                    }).collect_view()}
            </div>

            {move || match tab.get().as_str() {
                "runners"   => view! { <CICDRunners/> }.into_any(),
                "templates" => view! { <CICDTemplates/> }.into_any(),
                "secrets"   => view! { <CICDSecrets/> }.into_any(),
                "logs"      => view! { <CICDLogs/> }.into_any(),
                _ => view! { <></> }.into_any(),
            }}
        </div>
    }
}

#[component]
fn CICDRunners() -> impl IntoView {
    view! {
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 14px">
            <div class="card" style="padding: 18px">
                <div style="font-size: 13px; font-weight: 600; margin-bottom: 10px">"Default container image"</div>
                <input class="input mono" prop:value="ghcr.io/cs331-staff/runtime:2026.04"/>
                <div style="font-size: 11px; color: var(--ink-3); margin-top: 8px">
                    "Pinned per term so a re-grade in 2027 still produces identical output."
                </div>
            </div>
            <div class="card" style="padding: 18px">
                <div style="font-size: 13px; font-weight: 600; margin-bottom: 10px">"Resource limits"</div>
                <div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 10px">
                    {[("CPU","2 vCPU"),("RAM","4 GB"),("Timeout","10 min"),("Concurrency","20 jobs")].iter().map(|(k, v)| view! {
                        <div style="padding: 10px; border: 1px solid var(--line); border-radius: 6px">
                            <div style="font-size: 10px; color: var(--ink-3); text-transform: uppercase; letter-spacing: 0.5px">{*k}</div>
                            <div class="mono" style="font-size: 14px; font-weight: 600; margin-top: 2px">{*v}</div>
                        </div>
                    }).collect_view()}
                </div>
                <div style="font-size: 11px; color: var(--warm); margin-top: 10px">
                    <Icon name="shield" size=11 stroke="var(--warm)".to_string()/>
                    " Forgejo runners only — GitLab Cloud falls back to its own CI defaults."
                </div>
            </div>
        </div>
    }
}

#[component]
fn CICDTemplates() -> impl IntoView {
    let rows = vec![
        ("python-pytest", "Python · pytest", "12 assignments", "pytest tests/ --junit-xml=results.xml"),
        ("java-junit",    "Java · JUnit 5",  "3 assignments",  "mvn -B test"),
        ("rust-cargo",    "Rust · cargo",    "1 assignment",   "cargo test --release"),
    ];
    view! {
        <div class="card" style="padding: 0">
            {rows.into_iter().map(|(_id, name, used, cmd)| view! {
                <div class="hover-row" style="padding: 16px; border-bottom: 1px solid var(--line); display: flex; align-items: center; gap: 16px">
                    <Icon name="beaker" size=20 stroke="var(--ink-3)".to_string()/>
                    <div style="flex: 1">
                        <div style="font-size: 13px; font-weight: 600">{name}</div>
                        <div class="mono" style="font-size: 11px; color: var(--ink-3); margin-top: 2px">{cmd}</div>
                    </div>
                    <div style="font-size: 11px; color: var(--ink-3)">"used by "{used}</div>
                    <button class="btn btn-sm">"Edit"</button>
                </div>
            }).collect_view()}
            <div style="padding: 16px; display: flex; justify-content: center">
                <button class="btn btn-sm btn-ghost"><Icon name="plus" size=12/>" Add template"</button>
            </div>
        </div>
    }
}

#[component]
fn CICDSecrets() -> impl IntoView {
    view! {
        <div class="card" style="padding: 18px">
            <div style="font-size: 13px; color: var(--ink-3); margin-bottom: 12px">
                "Secrets are injected as env vars during grader runs. Students never see values."
            </div>
            {[("POSTGRES_URL","set"),("REDIS_URL","set"),("GRADER_TOKEN","set")].iter().map(|(k, v)| view! {
                <div style="display: flex; align-items: center; padding: 10px 0; border-bottom: 1px solid var(--line); gap: 12px">
                    <code class="mono" style="font-size: 12px; font-weight: 600; flex: 1">{*k}</code>
                    <span class="pill pill-ok"><Icon name="check" size=10/>" "{*v}</span>
                    <button class="btn btn-sm">"Rotate"</button>
                </div>
            }).collect_view()}
            <button class="btn btn-sm" style="margin-top: 12px"><Icon name="plus" size=12/>" Add secret"</button>
        </div>
    }
}

#[component]
fn CICDLogs() -> impl IntoView {
    let rows = vec![
        ("apatel", "hw04",   "pass", "12s",      "2 min ago"),
        ("cyrus",  "hw04",   "fail", "47s",      "5 min ago"),
        ("fwong",  "proj01", "pass", "3m 12s",   "14 min ago"),
        ("hpark",  "hw04",   "pass", "11s",      "22 min ago"),
        ("gabea",  "hw04",   "fail", "9s",       "1h ago"),
    ];
    view! {
        <div class="card" style="padding: 0">
            {rows.into_iter().map(|(who, asg, state, dur, when)| {
                let dot_color = if state == "pass" { "var(--ok)" } else { "var(--err)" };
                let pill_class = if state == "pass" { "pill pill-ok" } else { "pill pill-err" };
                view! {
                    <div class="hover-row" style="padding: 12px 18px; border-bottom: 1px solid var(--line); display: grid; grid-template-columns: 24px 1fr 100px 80px 100px 80px; align-items: center; gap: 12px">
                        <span style=format!("width: 8px; height: 8px; border-radius: 4px; background: {}", dot_color)></span>
                        <code class="mono" style="font-size: 12px">"@"{who}" "<span style="color: var(--ink-4)">"pushed to"</span>" "{asg}</code>
                        <span class=pill_class>{state}</span>
                        <span class="mono" style="font-size: 11px; color: var(--ink-3)">{dur}</span>
                        <span style="font-size: 11px; color: var(--ink-3)">{when}</span>
                        <button class="btn btn-sm btn-ghost">"Logs "<Icon name="external" size=11/></button>
                    </div>
                }
            }).collect_view()}
        </div>
    }
}

// ─── Analytics tab ───────────────────────────────────────────────────────────

#[component]
fn AnalyticsView() -> impl IntoView {
    view! {
        <div>
            <div class="h-section" style="font-size: 28px; margin-bottom: 6px">"Analytics"</div>
            <div style="font-size: 13px; color: var(--ink-3); margin-bottom: 24px">
                "Class-wide health — derived from CI runs, push activity, and roster events."
            </div>
            <div style="display: grid; grid-template-columns: repeat(4, 1fr); gap: 12px; margin-bottom: 20px">
                {[("Avg passing tests","74%","var(--ok)","+3% vs last week"),
                  ("Stale repos (7d)","12","var(--warn)","−4 vs last week"),
                  ("Active students","128 / 142","var(--ink)","+6 since Mon"),
                  ("Median time-to-pass","2.4 days","var(--ink)","−0.3 vs hw03")].iter().map(|(l, v, c, sub)| view! {
                    <div class="card" style="padding: 16px">
                        <div style="font-size: 11px; color: var(--ink-3); text-transform: uppercase; letter-spacing: 0.5px">{*l}</div>
                        <div class="mono" style=format!("font-size: 28px; font-weight: 600; margin-top: 6px; color: {}; font-variant-numeric: tabular-nums", c)>{*v}</div>
                        <div style="font-size: 11px; color: var(--ink-3); margin-top: 4px">{*sub}</div>
                    </div>
                }).collect_view()}
            </div>
            <div class="card" style="padding: 20px">
                <div style="font-size: 13px; font-weight: 600; margin-bottom: 14px">"Passing rate over the term"</div>
                <SparkChart/>
            </div>
        </div>
    }
}

#[component]
fn SparkChart() -> impl IntoView {
    let data = [42i32, 51, 58, 60, 62, 67, 71, 73, 70, 74, 76, 74];
    let w = 880.0_f32; let h = 180.0_f32; let pad = 12.0_f32;
    let n = data.len();
    let pts: Vec<(f32, f32)> = data.iter().enumerate().map(|(i, v)| {
        let x = pad + (i as f32 / (n as f32 - 1.0)) * (w - pad * 2.0);
        let y = h - pad - (*v as f32 / 100.0) * (h - pad * 2.0);
        (x, y)
    }).collect();
    let path = pts.iter().enumerate().map(|(i, (x, y))|
        format!("{}{:.1} {:.1}", if i == 0 { "M" } else { "L" }, x, y)
    ).collect::<Vec<_>>().join(" ");
    let fill_path = format!("{} L {:.1} {:.1} L {:.1} {:.1} Z", path, w - pad, h - pad, pad, h - pad);
    let grid_lines: Vec<f32> = vec![0.0, 25.0, 50.0, 75.0, 100.0];
    view! {
        <svg viewBox=format!("0 0 {} {}", w, h) style="width: 100%; height: 180px">
            {grid_lines.into_iter().map(|y| {
                let yy = h - pad - (y / 100.0) * (h - pad * 2.0);
                view! {
                    <line x1=pad x2=w - pad y1=yy y2=yy stroke="var(--line)" stroke-dasharray="2 4"/>
                }
            }).collect_view()}
            <path d=fill_path fill="var(--accent-soft)"/>
            <path d=path stroke="var(--accent)" stroke-width="2" fill="none"/>
            {pts.into_iter().map(|(x, y)| view! {
                <circle cx=x cy=y r="3" fill="var(--paper)" stroke="var(--accent)" stroke-width="1.5"/>
            }).collect_view()}
        </svg>
    }
}

// ─── Settings tab ────────────────────────────────────────────────────────────

#[component]
fn SettingsView(class_id: String) -> impl IntoView {
    let klass = data::class_by_id(&class_id).expect("class");
    let forge = data::forge_by_id(klass.forge).expect("forge");
    view! {
        <div style="display: flex; flex-direction: column; gap: 20px">
            <div>
                <div class="h-section" style="font-size: 28px; margin-bottom: 6px">"Settings"</div>
                <div style="font-size: 13px; color: var(--ink-3)">"Classroom-level configuration and forge binding."</div>
            </div>
            <div class="card" style="padding: 20px">
                <div style="font-size: 13px; font-weight: 600; margin-bottom: 14px">"General"</div>
                <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 14px">
                    <label class="label"><span>"Course #"</span><input class="input mono" prop:value=klass.number/></label>
                    <label class="label"><span>"Term"</span><input class="input" prop:value=klass.term/></label>
                    <label class="label" style="grid-column: 1 / -1"><span>"Course name"</span><input class="input" prop:value=klass.name/></label>
                </div>
            </div>
            <div class="card" style="padding: 20px">
                <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 12px">
                    <ForgeMark kind=forge.kind size=28/>
                    <div style="flex: 1">
                        <div style="font-size: 13px; font-weight: 600">"Forge binding"</div>
                        <div class="mono" style="font-size: 11px; color: var(--ink-3)">
                            {forge.label}" · org "<strong>{forge.org.unwrap_or("—")}</strong>
                        </div>
                    </div>
                    <button class="btn btn-sm">"Re-bind"</button>
                </div>
                <div style="font-size: 11px; color: var(--ink-3)">
                    "Re-binding moves all student repos under a new org. Roster, grades, and CI history are preserved."
                </div>
            </div>
            <div class="card" style="padding: 20px; border-color: color-mix(in srgb, var(--err) 30%, transparent)">
                <div style="font-size: 13px; font-weight: 600; margin-bottom: 8px; color: var(--err)">"Danger zone"</div>
                <div style="display: flex; justify-content: space-between; align-items: center; gap: 12px">
                    <div>
                        <div style="font-size: 13px; font-weight: 500">"Archive classroom"</div>
                        <div style="font-size: 12px; color: var(--ink-3)">"Read-only after archive. Repos and grades are preserved."</div>
                    </div>
                    <button class="btn">"Archive"</button>
                </div>
            </div>
        </div>
    }
}
