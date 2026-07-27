//! The landing page at `/` — maud-ui's front door.
//!
//! The gallery lived at `/` until this page took the slot; it now serves from
//! `/gallery`. Only `/` changed meaning: every component page already linked
//! with absolute paths.
//!
//! ## Why this page is built out of the library
//!
//! Every specimen below is a real primitive call — `message::render`,
//! `tool_call::render`, `blocks::dashboard::stats::preview()` — not a
//! hand-written mock of one. A component library whose landing page is made of
//! itself is the strongest available argument for it, and it means the page
//! cannot drift from the components: a regression in `tool_call` shows up on
//! the front door.
//!
//! ## Claims
//!
//! Every number on this page is derived or measured, never asserted:
//!
//! - "72 components" reads `COMPONENT_NAMES.len()` — the same constant the
//!   registration-parity test and the header count use.
//! - "10 block templates" reads `blocks::BLOCK_NAMES.len()`.
//! - "15 integrations" is `INTEGRATIONS.len()`, the list rendered below it.
//! - The runtime size was measured on `dist/maud-ui.min.js` (49,958 bytes raw,
//!   12,273 gzipped, 2026-07-27) and is stated as "~12 kB".
//!
//! Four numbers from the source design were dropped rather than restated,
//! because this repo contradicts them: `v0.3.0` (Cargo.toml says 0.4.1),
//! `MIT / Apache-2.0` (the crate is MIT only), `MSRV 1.79` (no `rust-version`
//! is declared at all), and a `1.4 kB` runtime (the real bundle is ~12 kB
//! gzipped). A landing page that overstates by 8x is a liability, not a pitch.

use maud::{html, Markup, DOCTYPE};

use crate::blocks;
use crate::primitives::{badge, code_block, diff, message, streaming_cursor, tool_call};

use super::{page_head, page_header, showcase_js, COMPONENT_NAMES};

/// Third-party libraries with a worked integration page. The labels and
/// one-liners are the same strings the header's "Advanced" menu uses, so the
/// two never disagree about what an integration is called.
const INTEGRATIONS: &[(&str, &str, &str, &str)] = &[
    ("monaco-editor", "Monaco editor", "Code", "VS Code's editor, embedded"),
    ("tiptap", "TipTap", "Code", "Rich text prose editor"),
    ("xterm", "xterm.js", "Terminal", "Terminal emulator"),
    ("xyflow", "xyflow", "Graph", "React Flow node editor"),
    ("cytoscape", "Cytoscape", "Graph", "Network graph visualisation"),
    ("mermaid", "Mermaid", "Diagram", "Text-to-diagram renderer"),
    ("excalidraw", "Excalidraw", "Canvas", "Sketchy whiteboard canvas"),
    ("threejs", "Three.js", "Canvas", "WebGL 3D scene"),
    ("ag-grid", "AG Grid", "Data", "Enterprise data grid"),
    ("echarts", "Apache ECharts", "Data", "Charting library"),
    ("leaflet", "Leaflet", "Maps", "Interactive maps"),
    ("fullcalendar", "FullCalendar", "Scheduling", "Scheduling, drag-drop events"),
    ("wavesurfer", "Wavesurfer", "Media", "Audio waveform player"),
    ("pdfjs", "PDF.js", "Media", "Inline PDF viewer"),
    ("sortable", "SortableJS", "Drag & drop", "Reorder list, kanban, tile grid"),
];

/// Section heading with its mono ordinal. The source design opened ten
/// sections with uppercase letter-spaced eyebrows; a numbered marker carries
/// the same "step N of an argument" signal without the shout.
fn marker(num: &str, label: &str) -> Markup {
    html! {
        p class="lp__marker" {
            span class="lp__marker-num" { (num) }
            span { (label) }
        }
    }
}

fn cta_link(href: &str, label: &str, variant: &str) -> Markup {
    html! {
        a href=(href) class=(format!("mui-btn mui-btn--{variant} mui-btn--lg")) { (label) }
    }
}

/// The conversation specimen: two real `message::render` bubbles with a real
/// `tool_call::render` carrying a real `diff::render` inside it.
fn agent_specimen() -> Markup {
    let hunk = diff::render(diff::Props {
        file_path: Some("src/primitives/message.rs".into()),
        show_line_numbers: true,
        lines: vec![
            diff::Line {
                kind: diff::LineKind::Context,
                old_line_no: Some(17),
                new_line_no: Some(17),
                text: "pub struct Props {".into(),
            },
            diff::Line {
                kind: diff::LineKind::Remove,
                old_line_no: Some(18),
                new_line_no: None,
                text: "    pub role: String,".into(),
            },
            diff::Line {
                kind: diff::LineKind::Add,
                old_line_no: None,
                new_line_no: Some(18),
                text: "    pub role: Role,".into(),
            },
            diff::Line {
                kind: diff::LineKind::Add,
                old_line_no: None,
                new_line_no: Some(19),
                text: "    pub is_live: bool,".into(),
            },
            diff::Line {
                kind: diff::LineKind::Context,
                old_line_no: Some(19),
                new_line_no: Some(20),
                text: "}".into(),
            },
        ],
        ..Default::default()
    });

    html! {
        (message::render(message::Props {
            role: message::Role::User,
            author: "You".into(),
            avatar_initials: Some("H".into()),
            timestamp: Some("14:32".into()),
            body: html! {
                p { "Rename the " code { "role" } " field to a typed enum and add a live flag." }
            },
            ..Default::default()
        }))

        (message::render(message::Props {
            role: message::Role::Assistant,
            author: "Claude".into(),
            avatar_initials: Some("C".into()),
            timestamp: Some("14:32".into()),
            is_live: true,
            body: html! {
                p {
                    "Swapping " code { "String" } " for " code { "Role" } " now"
                    // No `label`: that slot renders VISIBLY (it is for a word
                    // like "thinking"), and the wrapper already carries
                    // role="status" aria-live="polite" for screen readers.
                    // A bare caret is what a mid-stream reply looks like.
                    (streaming_cursor::render(streaming_cursor::Props {
                        variant: streaming_cursor::Variant::Cursor,
                        label: None,
                    }))
                }
                (tool_call::render(tool_call::Props {
                    id: "lp-edit".into(),
                    kind: tool_call::Kind::Edit,
                    name: "Edit".into(),
                    summary: "src/primitives/message.rs".into(),
                    status: tool_call::Status::Success,
                    result: Some(hunk),
                    open: true,
                    ..Default::default()
                }))
                (tool_call::render(tool_call::Props {
                    id: "lp-bash".into(),
                    kind: tool_call::Kind::Bash,
                    name: "Bash".into(),
                    summary: "cargo check -p maud-ui".into(),
                    status: tool_call::Status::Running,
                    ..Default::default()
                }))
            },
            ..Default::default()
        }))
    }
}

/// The token specimen. The swatches are painted with `var(--mui-*)` rather
/// than hex literals, so hitting the theme toggle in the header re-paints them
/// live — which is the claim this section makes, demonstrated instead of
/// described.
fn token_specimen() -> Markup {
    // Solid fills only. `--mui-bg-card` was here too and read as a hole in the
    // row rather than a swatch — a surface token is nearly the page colour by
    // definition, which is the one thing a swatch cannot show.
    const SWATCHES: &[(&str, &str)] = &[
        ("--mui-accent", "Accent"),
        ("--mui-success", "Success"),
        ("--mui-warning", "Warning"),
        ("--mui-danger", "Danger"),
        ("--mui-info", "Info"),
        ("--mui-violet", "Violet"),
        ("--mui-rose", "Rose"),
    ];

    html! {
        div class="lp__swatches" {
            @for (token, label) in SWATCHES {
                span class="lp__swatch"
                     style=(format!("background:var({token})"))
                     title=(*token)
                     role="img"
                     aria-label=(format!("{label} token")) {}
            }
        }
        (code_block::render(code_block::Props {
            language: Some("css".into()),
            filename: Some("your-theme.css".into()),
            code: ":root[data-theme=\"light\"] {\n  --mui-bg: #ffffff;\n  --mui-bg-card: #f9fafb;\n  --mui-text: #09090b;\n  --mui-accent: #2563eb;\n\n  /* Decorative edges — cards, dividers. */\n  --mui-border: #e4e4e7;\n  /* A CONTROL's edge. Held at 3:1 for WCAG 1.4.11;\n     a shared border token made unchecked checkboxes\n     invisible at 1.27:1. */\n  --mui-border-control: #71717a;\n}"
                .into(),
            ..Default::default()
        }))
    }
}

/// The install snippet used in the closing section — a real axum handler,
/// because "add it to a handler" is the claim being made next to it.
fn axum_specimen() -> Markup {
    code_block::render(code_block::Props {
        language: Some("rust".into()),
        filename: Some("src/main.rs".into()),
        code: "use axum::{routing::get, Router};\nuse maud::html;\nuse maud_ui::primitives::{button, card};\n\nasync fn index() -> maud::Markup {\n    html! {\n        (card::render(card::Props {\n            title: Some(\"Create project\".into()),\n            description: Some(\"Deploy your new project in one click.\".into()),\n            footer: Some(button::render(button::Props {\n                label: \"Create\".into(),\n                variant: button::Variant::Primary,\n                ..Default::default()\n            })),\n            ..Default::default()\n        }))\n    }\n}\n\n#[tokio::main]\nasync fn main() {\n    let app = Router::new().route(\"/\", get(index));\n    let listener = tokio::net::TcpListener::bind(\"0.0.0.0:3000\").await.unwrap();\n    axum::serve(listener, app).await.unwrap();\n}"
            .into(),
        ..Default::default()
    })
}

/// The landing page.
pub fn landing_page() -> Markup {
    let component_count = COMPONENT_NAMES.len();
    let block_count = blocks::BLOCK_NAMES.len();
    let integration_count = INTEGRATIONS.len();

    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("maud-ui \u{00b7} Server-rendered UI components for Rust"))
                meta name="description"
                     content=(format!(
                         "{component_count} headless, accessible UI components for Rust web apps. \
                          Built on maud. Ships as one crate — no bundler, no node_modules."));
                style { (maud::PreEscaped(landing_css())) }
            }
            // `lp-page` scopes one override: the header's sidebar filter has
            // nothing to filter here (its JS is guarded on nav items existing),
            // so it is hidden rather than left as a search box that does
            // nothing. The command palette next to it works on any page — it
            // reads an inlined index, not the DOM.
            body class="lp-page" {
                (page_header())

                main class="lp" {

                    // ── Hero ────────────────────────────────────────────
                    section class="lp__hero" {
                        div class="lp__rule" {}
                        h1 class="lp__hero-h1" {
                            "Server-rendered UI components "
                            span { "for Rust." }
                        }
                        p class="lp__hero-lede" {
                            (component_count) " typed " code { "maud" } " components with shadcn "
                            "Base UI parity. Most need no JavaScript at all — and when you do "
                            "need it, " (integration_count) " heavyweight libraries are already "
                            "wired up."
                        }
                        div class="lp__cta" {
                            (cta_link("/gallery", &format!("Browse all {component_count} components"), "primary"))
                            (cta_link("/getting-started", "Get started", "outline"))
                        }
                        div class="lp__install" {
                            span class="lp__install-prompt" { "$" }
                            span { "cargo add maud-ui" }
                        }

                        div class="lp__stats" {
                            div class="lp__stat" {
                                span class="lp__stat-n" { (component_count) }
                                span class="lp__stat-label" { "components" }
                            }
                            div class="lp__stat" {
                                span class="lp__stat-n" { (block_count) }
                                span class="lp__stat-label" { "composed block templates" }
                            }
                            div class="lp__stat" {
                                span class="lp__stat-n" { (integration_count) }
                                span class="lp__stat-label" { "JS libraries pre-wired" }
                            }
                            div class="lp__stat" {
                                span class="lp__stat-n" { "0" }
                                span class="lp__stat-label" { "node dependencies" }
                            }
                        }
                    }

                    // ── 01 · Agent surfaces ─────────────────────────────
                    section class="lp__section" {
                        div class="lp__split lp__split--flip" {
                            div class="lp__split-prose" {
                                (marker("01", "Agent surfaces"))
                                h2 class="lp__h2" { "Agent surfaces, as first-class primitives" }
                                p class="lp__lede" {
                                    "Message, Streaming Cursor, Code Block, Diff and Tool Call. "
                                    "Everything you need to render an LLM conversation from a "
                                    "Rust server — including collapsed tool invocations with "
                                    "arguments and results."
                                }
                                ul class="lp__list" {
                                    li {
                                        span {
                                            b { "message::Role" } " — Assistant, User, System. "
                                            "Each with its own alignment, avatar slot and live-region behaviour."
                                        }
                                    }
                                    li {
                                        span {
                                            b { "tool_call::Status" } " — Pending, Running, Success, Error. "
                                            "Collapsible argument and result panes."
                                        }
                                    }
                                    li {
                                        span {
                                            b { "diff::render" } " — unified hunks with line numbers, "
                                            "rendered on the server. No highlighter in the browser."
                                        }
                                    }
                                }
                            }
                            div {
                                div class="lp__specimen" {
                                    (agent_specimen())
                                }
                                p class="lp__caption" {
                                    "Live output of message::render, tool_call::render, "
                                    "diff::render and streaming_cursor::render."
                                }
                            }
                        }
                    }

                    // ── 02 · Integrations ───────────────────────────────
                    section class="lp__section" {
                        (marker("02", "Escape hatches"))
                        div class="lp__section-head" {
                            h2 class="lp__h2" { "When you do need JavaScript, it's already wired up" }
                            p class="lp__lede" {
                                (integration_count) " heavyweight libraries, each with a worked "
                                "integration page: a maud-ui shell around the widget, the mount and "
                                "teardown, and the widget's own theme bound to "
                                code { "<html data-theme>" } " so it flips when the page does."
                            }
                        }
                        div class="lp__int-grid" {
                            @for (slug, label, kind, desc) in INTEGRATIONS {
                                a class="lp__int" href=(format!("/integrations/{slug}")) {
                                    span class="lp__int-head" {
                                        span class="lp__int-name" { (label) }
                                        span class="lp__int-kind" { (kind) }
                                    }
                                    span class="lp__int-desc" { (desc) }
                                }
                            }
                        }
                    }

                    // ── 03 · Tokens ─────────────────────────────────────
                    section class="lp__section" {
                        div class="lp__split" {
                            div class="lp__split-prose" {
                                (marker("03", "Tokens"))
                                h2 class="lp__h2" { "One variable. Every component." }
                                p class="lp__lede" {
                                    "Every component reads " code { "var(--mui-*)" } " at paint "
                                    "time. Switching a theme is a stylesheet swap — no rebuild, "
                                    "no re-render, no flash."
                                }
                                p class="lp__lede" {
                                    "Toggle the theme in the header: the swatches beside this "
                                    "paragraph are painted from those variables, so they "
                                    "re-paint with the rest of the page."
                                }
                                div class="lp__cta" style="margin-top:1.75rem;margin-bottom:0;" {
                                    a href="/theme" class="mui-btn mui-btn--outline mui-btn--sm" {
                                        "Open the theme customiser"
                                    }
                                }
                            }
                            div {
                                div class="lp__specimen" { (token_specimen()) }
                                p class="lp__caption" { "Painted from live custom properties, not hex literals." }
                            }
                        }
                    }

                    // ── 04 · Composition ────────────────────────────────
                    section class="lp__section" {
                        (marker("04", "Blocks"))
                        div class="lp__section-head" {
                            h2 class="lp__h2" { "Parts are easy. Composition is the proof." }
                            p class="lp__lede" {
                                (block_count) " block templates ship pre-composed — an auth screen, a "
                                "settings page, a full data table, this dashboard. Each is ordinary "
                                "primitives inside a " code { "render(Props)" } " function you can "
                                "call as-is or fork."
                            }
                        }
                        div class="lp__specimen lp__specimen--flush" style="margin-top:2rem;" {
                            div class="lp__frame" {
                                (blocks::dashboard::stats::preview())
                            }
                        }
                        p class="lp__caption" {
                            "Live output of blocks::dashboard::stats::preview() — the same "
                            "function a consumer calls."
                        }
                        div class="lp__cta" style="margin-top:1.75rem;margin-bottom:0;" {
                            a href="/blocks" class="mui-btn mui-btn--outline mui-btn--sm" {
                                (format!("See all {block_count} blocks"))
                            }
                        }
                    }

                    // ── 05 · The index ──────────────────────────────────
                    section class="lp__section" {
                        (marker("05", "The index"))
                        div class="lp__section-head" {
                            h2 class="lp__h2" {
                                (component_count) " components. None of them undocumented."
                            }
                            p class="lp__lede" {
                                "Registering a component means touching eight files, and nothing but "
                                "a test can notice when you miss one. So a test does: every "
                                "primitive here has a module, a tier, a page, an API doc file, an "
                                "imported stylesheet and a render test — or " code { "cargo test" }
                                " goes red."
                            }
                        }
                        ul class="lp__index" {
                            @for name in COMPONENT_NAMES {
                                li {
                                    a href=(format!("/{name}")) { (super::display_name(name)) }
                                }
                            }
                        }
                        div class="lp__cta" style="margin-bottom:0;" {
                            (cta_link("/gallery", &format!("Browse all {component_count} components"), "primary"))
                        }
                    }

                    // ── 06 · Close ──────────────────────────────────────
                    section class="lp__close" {
                        h2 class="lp__h2" { "Write Rust. Ship HTML. Skip the build step." }
                        p class="lp__lede" {
                            "One crate, no bundler, no " code { "node_modules" } ". Add it to an "
                            "axum handler and you have a working page in a minute."
                        }
                        div style="margin-top:2.5rem;text-align:left;" {
                            (axum_specimen())
                        }
                        div class="lp__cta" {
                            (cta_link("/getting-started", "Get started", "primary"))
                            (cta_link("/gallery", "Browse components", "outline"))
                        }
                    }

                    footer class="lp__footer" {
                        span {
                            "maud-ui "
                            (badge::render(badge::Props {
                                label: format!("v{}", env!("CARGO_PKG_VERSION")),
                                variant: badge::Variant::Outline,
                                mono: true,
                                ..Default::default()
                            }))
                            " \u{00b7} MIT"
                        }
                        nav {
                            a href="/gallery" { "Components" }
                            a href="/blocks" { "Blocks" }
                            a href="/theme" { "Theme" }
                            a href="https://docs.rs/maud-ui" target="_blank" rel="noopener" { "docs.rs" }
                            a href="https://crates.io/crates/maud-ui" target="_blank" rel="noopener" { "crates.io" }
                            a href="https://github.com/hgeldenhuys/maud-ui" target="_blank" rel="noopener" { "GitHub" }
                        }
                    }
                }

                script src=(format!("/js/maud-ui.js?v={}", super::JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn landing_css() -> &'static str {
    include_str!("landing.css")
}
