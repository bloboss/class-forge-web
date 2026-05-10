//! Shared chrome: top bar, classroom cover artwork, etc.

use leptos::prelude::*;
use crate::data::{self, CoverKind};
use crate::icons::Icon;
use crate::router::navigate;

pub use crate::icons::ForgeMark;

#[component]
pub fn TopBar() -> impl IntoView {
    let u = data::user();
    view! {
        <div class="topbar">
            <div class="brand" on:click=move |_| navigate("/classes")>
                <div class="brand-dot">"C"</div>
                <div class="brand-text">"Class"<em>"Forge"</em></div>
            </div>
            <div style="flex: 1"></div>
            <button class="btn btn-sm btn-ghost"><Icon name="sparkles" size=14/>" What's new"</button>
            <button class="btn btn-sm btn-ghost btn-icon-sm"><Icon name="bell" size=15/></button>
            <div class="divider-v"></div>
            <div style="display: flex; align-items: center; gap: 10px; cursor: pointer">
                <div class="avatar">{u.initials}</div>
                <div style="display: flex; flex-direction: column; line-height: 1.2">
                    <div style="font-size: 12px; font-weight: 500">{u.name}</div>
                    <div style="font-size: 10px; color: var(--ink-3)">{u.org}</div>
                </div>
                <Icon name="chevron-down" size=13/>
            </div>
        </div>
    }
}

#[component]
pub fn Cover(kind: CoverKind, #[prop(default = 96)] height: u32) -> impl IntoView {
    let (a, b) = match kind {
        CoverKind::Indigo => ("#4338ca", "#a78bfa"),
        CoverKind::Warm   => ("#b45309", "#fbbf24"),
        CoverKind::Slate  => ("#0f172a", "#475569"),
        CoverKind::Rose   => ("#9d174d", "#f472b6"),
    };
    let style = format!(
        "height: {height}px; background: linear-gradient(135deg, {a} 0%, {b} 100%);"
    );
    let pat_id = format!("p-{:?}-{}", kind, height);
    view! {
        <div class="cover" style=style>
            <svg class="cover-stripe" width="100%" height="100%" viewBox="0 0 240 96" preserveAspectRatio="none">
                <defs>
                    <pattern id=pat_id.clone() width="14" height="14" patternUnits="userSpaceOnUse" patternTransform="rotate(28)">
                        <path d="M0 7h14" stroke="rgba(255,255,255,0.4)" stroke-width="1"/>
                    </pattern>
                </defs>
                <rect width="240" height="96" fill=format!("url(#{})", pat_id)/>
            </svg>
        </div>
    }
}

#[component]
pub fn Stepper(steps: Vec<&'static str>, current: usize) -> impl IntoView {
    let last = steps.len().saturating_sub(1);
    view! {
        <div style="display: flex; align-items: center; gap: 12px; margin-bottom: 28px">
            {steps.into_iter().enumerate().map(|(i, s)| {
                let active = i == current;
                let done = i < current;
                let pill_color = if active { "var(--ink)" } else { "var(--line)" };
                let label_color = if i <= current { "var(--ink)" } else { "var(--ink-4)" };
                let dot_bg = if done { "var(--ok)" } else if active { "var(--ink)" } else { "var(--fill)" };
                let dot_fg = if i <= current { "var(--paper)" } else { "var(--ink-3)" };
                let pill_style = format!(
                    "display: flex; align-items: center; gap: 8px; padding: 6px 12px; border-radius: 20px; \
                     border: 1px solid {pill_color}; cursor: pointer; background: transparent; color: {label_color}; font-family: inherit;"
                );
                let dot_style = format!(
                    "display: inline-flex; align-items: center; justify-content: center; width: 18px; height: 18px; \
                     border-radius: 9px; font-size: 10px; font-weight: 600; background: {dot_bg}; color: {dot_fg};"
                );
                let weight = if active { 600 } else { 400 };
                view! {
                    <>
                        <button style=pill_style>
                            <span class="mono" style=dot_style>
                                {if done { "✓".to_string() } else { (i + 1).to_string() }}
                            </span>
                            <span style=format!("font-size: 12px; font-weight: {weight}")>{s}</span>
                        </button>
                        {(i < last).then(|| view! {
                            <div style="flex: 1; height: 1px; background: var(--line)"></div>
                        })}
                    </>
                }
            }).collect_view()}
        </div>
    }
}
