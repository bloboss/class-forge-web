//! Lucide-style stroked icons + the ForgeMark badge.
//!
//! Each icon is a small `view!` snippet returned from a `fn` so callers can
//! drop them inline without juggling component generics.

use crate::data::ForgeKind;
use leptos::prelude::*;

#[component]
pub fn Icon(
    #[prop(into)] name: String,
    #[prop(default = 16)] size: u32,
    #[prop(into, optional)] stroke: String,
) -> impl IntoView {
    let stroke = if stroke.is_empty() {
        "currentColor".to_string()
    } else {
        stroke
    };
    let body = match name.as_str() {
        "search" => view! {
            <circle cx="11" cy="11" r="7"/>
            <path d="m20 20-3.5-3.5"/>
        }.into_any(),
        "plus" => view! { <path d="M12 5v14M5 12h14"/> }.into_any(),
        "chevron-down" => view! { <path d="m6 9 6 6 6-6"/> }.into_any(),
        "chevron-right" => view! { <path d="m9 6 6 6-6 6"/> }.into_any(),
        "chevron-left" => view! { <path d="m15 6-6 6 6 6"/> }.into_any(),
        "check" => view! { <path d="M5 12.5 10 17.5 19 7.5"/> }.into_any(),
        "x" => view! { <path d="M6 6l12 12M18 6 6 18"/> }.into_any(),
        "filter" => view! { <path d="M3 5h18l-7 9v6l-4-2v-4Z"/> }.into_any(),
        "settings" => view! {
            <circle cx="12" cy="12" r="3"/>
            <path d="M19.4 15a1.7 1.7 0 0 0 .3 1.8l.1.1a2 2 0 1 1-2.8 2.8l-.1-.1a1.7 1.7 0 0 0-1.8-.3 1.7 1.7 0 0 0-1 1.5V21a2 2 0 1 1-4 0v-.1a1.7 1.7 0 0 0-1-1.5 1.7 1.7 0 0 0-1.8.3l-.1.1a2 2 0 1 1-2.8-2.8l.1-.1a1.7 1.7 0 0 0 .3-1.8 1.7 1.7 0 0 0-1.5-1H3a2 2 0 1 1 0-4h.1a1.7 1.7 0 0 0 1.5-1 1.7 1.7 0 0 0-.3-1.8l-.1-.1a2 2 0 1 1 2.8-2.8l.1.1a1.7 1.7 0 0 0 1.8.3 1.7 1.7 0 0 0 1-1.5V3a2 2 0 1 1 4 0v.1a1.7 1.7 0 0 0 1 1.5 1.7 1.7 0 0 0 1.8-.3l.1-.1a2 2 0 1 1 2.8 2.8l-.1.1a1.7 1.7 0 0 0-.3 1.8 1.7 1.7 0 0 0 1.5 1H21a2 2 0 1 1 0 4h-.1a1.7 1.7 0 0 0-1.5 1Z"/>
        }.into_any(),
        "home" => view! { <path d="m3 11 9-8 9 8v10a2 2 0 0 1-2 2h-4v-7h-6v7H5a2 2 0 0 1-2-2Z"/> }.into_any(),
        "users" => view! {
            <circle cx="9" cy="8" r="3.2"/>
            <path d="M3 20a6 6 0 0 1 12 0"/>
            <circle cx="17" cy="8" r="2.6"/>
            <path d="M21 19a4.5 4.5 0 0 0-4-2.4"/>
        }.into_any(),
        "clipboard-list" => view! {
            <rect x="6" y="4" width="12" height="17" rx="2"/>
            <path d="M9 4V3a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v1M9 10h6M9 14h6M9 18h4"/>
        }.into_any(),
        "beaker" => view! {
            <path d="M9 3h6M10 3v6L4 19a2 2 0 0 0 1.7 3h12.6A2 2 0 0 0 20 19l-6-10V3M7 14h10"/>
        }.into_any(),
        "calendar" => view! {
            <rect x="3" y="5" width="18" height="16" rx="2"/>
            <path d="M8 3v4M16 3v4M3 11h18"/>
        }.into_any(),
        "bell" => view! { <path d="M6 8a6 6 0 1 1 12 0c0 6 2 7 2 7H4s2-1 2-7M10 21a2 2 0 0 0 4 0"/> }.into_any(),
        "logout" => view! { <path d="M15 17l5-5-5-5M20 12H9M12 3H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h6"/> }.into_any(),
        "mail" => view! {
            <rect x="3" y="5" width="18" height="14" rx="2"/>
            <path d="m3 7 9 7 9-7"/>
        }.into_any(),
        "sparkles" => view! { <path d="M12 3v4M12 17v4M3 12h4M17 12h4M6 6l2 2M16 16l2 2M6 18l2-2M16 8l2-2"/> }.into_any(),
        "git-branch" => view! {
            <circle cx="6" cy="6" r="2.4"/>
            <circle cx="6" cy="18" r="2.4"/>
            <circle cx="18" cy="9" r="2.4"/>
            <path d="M6 8.4v7.2M8.4 9c2 0 3.6 1.5 3.6 3.6V18M15.6 9c0 4-3.6 4.5-3.6 8"/>
        }.into_any(),
        "play" => view! { <path d="M7 4v16l13-8z" fill="currentColor" stroke="none"/> }.into_any(),
        "sliders" => view! { <path d="M4 6h11M19 6h1M4 12h5M13 12h7M4 18h13M21 18h-1"/> }.into_any(),
        "lock" => view! {
            <rect x="4" y="11" width="16" height="10" rx="2"/>
            <path d="M8 11V7a4 4 0 0 1 8 0v4"/>
        }.into_any(),
        "external" => view! { <path d="M14 4h6v6M20 4l-9 9M19 13v6a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1h6"/> }.into_any(),
        "folder" => view! { <path d="M3 7a2 2 0 0 1 2-2h4l2 2h8a2 2 0 0 1 2 2v9a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Z"/> }.into_any(),
        "more" => view! {
            <circle cx="5" cy="12" r="1.2" fill="currentColor"/>
            <circle cx="12" cy="12" r="1.2" fill="currentColor"/>
            <circle cx="19" cy="12" r="1.2" fill="currentColor"/>
        }.into_any(),
        "upload" => view! { <path d="M12 16V4M7 9l5-5 5 5M5 20h14"/> }.into_any(),
        "clock" => view! {
            <circle cx="12" cy="12" r="9"/>
            <path d="M12 7v5l3 2"/>
        }.into_any(),
        "spark" => view! { <path d="M12 2v4M2 12h4M12 22v-4M22 12h-4M5 5l3 3M16 16l3 3M5 19l3-3M16 8l3-3"/> }.into_any(),
        "globe" => view! {
            <circle cx="12" cy="12" r="9"/>
            <path d="M3 12h18M12 3a14 14 0 0 1 0 18M12 3a14 14 0 0 0 0 18"/>
        }.into_any(),
        "database" => view! {
            <ellipse cx="12" cy="5" rx="8" ry="3"/>
            <path d="M4 5v6c0 1.7 3.6 3 8 3s8-1.3 8-3V5M4 11v6c0 1.7 3.6 3 8 3s8-1.3 8-3v-6"/>
        }.into_any(),
        "shield" => view! { <path d="M12 3 4 6v6c0 5 3.5 8 8 9 4.5-1 8-4 8-9V6Z"/> }.into_any(),
        _ => view! { <circle cx="12" cy="12" r="8"/> }.into_any(),
    };
    view! {
        <svg width=size height=size viewBox="0 0 24 24" fill="none" stroke=stroke stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round">
            {body}
        </svg>
    }
}

#[component]
pub fn ForgeMark(kind: ForgeKind, #[prop(default = 22)] size: u32) -> impl IntoView {
    let (bg, fg, letter) = match kind {
        ForgeKind::Forgejo => ("#fbf6ec", "#d97706", "Fj"),
        ForgeKind::Gitlab => ("#fff4ec", "#fc6d26", "GL"),
        ForgeKind::Github => ("#f3f0ec", "#1c1917", "Gh"),
        ForgeKind::Gitea => ("#ecf6f7", "#609926", "Gt"),
        ForgeKind::Bitbucket => ("#eef2ff", "#2684ff", "Bb"),
        ForgeKind::Codeberg => ("#eaf2ff", "#2185d0", "Cb"),
        ForgeKind::Custom => ("#f3ecdc", "#57534e", "··"),
    };
    let font_size = if size <= 22 { 9 } else { 11 };
    let style = format!(
        "width: {size}px; height: {size}px; background: {bg}; color: {fg}; font-size: {font_size}px;"
    );
    view! { <span class="forge-mark" style=style>{letter}</span> }
}
