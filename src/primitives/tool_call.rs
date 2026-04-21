//! Tool call — collapsible representation of an agent tool invocation
//! (Edit, Write, Bash, Read, Grep, etc.). Renders a compact trigger row with
//! name + summary + status, and an expandable body with args + result panes.

use maud::{html, Markup};

/// Tool families, each with its own accent colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Kind {
    #[default]
    Custom,
    Edit,
    Write,
    Read,
    Bash,
    Grep,
    Glob,
    Task,
    Agent,
    Search,
}

impl Kind {
    fn class(&self) -> &'static str {
        match self {
            Self::Custom => "mui-tool-call--custom",
            Self::Edit => "mui-tool-call--edit",
            Self::Write => "mui-tool-call--write",
            Self::Read => "mui-tool-call--read",
            Self::Bash => "mui-tool-call--bash",
            Self::Grep => "mui-tool-call--grep",
            Self::Glob => "mui-tool-call--glob",
            Self::Task => "mui-tool-call--task",
            Self::Agent => "mui-tool-call--agent",
            Self::Search => "mui-tool-call--search",
        }
    }

    fn glyph(&self) -> &'static str {
        match self {
            Self::Edit => "\u{270E}",   // ✎
            Self::Write => "\u{2795}",  // ➕
            Self::Read => "\u{1F4D6}",  // 📖
            Self::Bash => "\u{203A}_",  // › _
            Self::Grep => "\u{1F50D}",  // 🔍
            Self::Glob => "\u{2731}",   // ✱
            Self::Task => "\u{2756}",   // ❖
            Self::Agent => "\u{25C6}",  // ◆
            Self::Search => "\u{1F50E}",// 🔎
            Self::Custom => "\u{2022}", // •
        }
    }
}

/// Execution state — drives the status pill colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Status {
    #[default]
    Success,
    Running,
    Error,
    Pending,
}

impl Status {
    fn class(&self) -> &'static str {
        match self {
            Self::Success => "mui-tool-call__status--success",
            Self::Running => "mui-tool-call__status--running",
            Self::Error => "mui-tool-call__status--error",
            Self::Pending => "mui-tool-call__status--pending",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Self::Success => "done",
            Self::Running => "running",
            Self::Error => "error",
            Self::Pending => "queued",
        }
    }
}

/// Tool-call rendering properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// Unique DOM id — used to wire the details disclosure.
    pub id: String,
    /// Tool family (drives accent colour + glyph).
    pub kind: Kind,
    /// Tool name (e.g. "Edit", "Bash").
    pub name: String,
    /// One-line summary shown in the trigger (e.g. file path, command).
    pub summary: String,
    /// Current execution status.
    pub status: Status,
    /// Optional args panel — rendered in the expanded body.
    pub args: Option<Markup>,
    /// Optional result panel — rendered in the expanded body.
    pub result: Option<Markup>,
    /// Initial open state (default false — collapsed).
    pub open: bool,
}

/// Render a single tool call.
pub fn render(props: Props) -> Markup {
    let body_id = format!("{}-body", if props.id.is_empty() { "tool" } else { &props.id });
    let aria_expanded = if props.open { "true" } else { "false" };

    html! {
        div class={"mui-tool-call " (props.kind.class())} data-mui="tool-call" {
            button type="button"
                class="mui-tool-call__trigger"
                aria-expanded=(aria_expanded)
                aria-controls=(body_id)
                onclick=(r#"(function(btn){const body=document.getElementById(btn.getAttribute('aria-controls'));const expanded=btn.getAttribute('aria-expanded')==='true';btn.setAttribute('aria-expanded',(!expanded).toString());if(body){body.hidden=expanded;}})(this)"#)
            {
                span class="mui-tool-call__glyph" aria-hidden="true" { (props.kind.glyph()) }
                span class="mui-tool-call__name" { (props.name) }
                span class="mui-tool-call__summary" { (props.summary) }
                span class={"mui-tool-call__status " (props.status.class())} {
                    (props.status.label())
                }
                span class="mui-tool-call__chevron" aria-hidden="true" { "\u{25BE}" }
            }
            div class="mui-tool-call__body" id=(body_id) hidden[!props.open] {
                @if let Some(args) = props.args {
                    div class="mui-tool-call__section mui-tool-call__section--args" {
                        div class="mui-tool-call__section-label" { "arguments" }
                        div class="mui-tool-call__section-body" { (args) }
                    }
                }
                @if let Some(result) = props.result {
                    div class="mui-tool-call__section mui-tool-call__section--result" {
                        div class="mui-tool-call__section-label" { "result" }
                        div class="mui-tool-call__section-body" { (result) }
                    }
                }
            }
        }
    }
}

/// Showcase of a handful of common tool invocations.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__column style="max-width: 44rem; gap: 0.5rem;" {
            (render(Props {
                id: "s-edit".into(),
                kind: Kind::Edit,
                name: "Edit".into(),
                summary: "src/primitives/message.rs".into(),
                status: Status::Success,
                open: true,
                args: Some(html! {
                    pre { "old_string: \"role: Role::Default\"\nnew_string: \"role: Role::Assistant\"" }
                }),
                result: Some(html! { pre { "Applied edit — 1 replacement." } }),
                ..Default::default()
            }))
            (render(Props {
                id: "s-bash".into(),
                kind: Kind::Bash,
                name: "Bash".into(),
                summary: "cargo check -p maud-ui".into(),
                status: Status::Running,
                ..Default::default()
            }))
            (render(Props {
                id: "s-grep".into(),
                kind: Kind::Grep,
                name: "Grep".into(),
                summary: "pattern: \"fn render\" in **/*.rs".into(),
                status: Status::Success,
                ..Default::default()
            }))
        }
    }
}
