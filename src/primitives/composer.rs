//! Composer — the multi-state prompt dock for an agent session view.
//!
//! Server-rendered: the writable states are a plain `<form>` wrapping a
//! `<textarea rows="2">`, so the composer submits with JavaScript disabled.
//! Auto-grow, `⌘↵`-to-send and voice capture are a JS layer the consumer adds
//! on top — none of them are required for the control to function.
//!
//! Four states, one anchor line (the dock never moves):
//!   * [`State::Ready`]     — the 84px resting dock.
//!   * [`State::Growing`]   — focused; the field is ringed and grows to a max
//!                            then scrolls internally (the max is a JS/consumer
//!                            concern; the CSS sets the min and the cap).
//!   * [`State::Executing`] — a turn is running. The field stays WRITABLE (not
//!                            disabled) so a follow-up can be queued; an
//!                            `Interrupt` hollow-destructive button appears
//!                            beside the primary.
//!   * [`State::Asleep`]    — a 46px dashed bar with a mono state tag and a
//!                            neutral `Wake` button.

use maud::{html, Markup};

/// Which dock state to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum State {
    /// Resting dock — 84px, placeholder visible.
    #[default]
    Ready,
    /// Focused and typing — ringed field, grows then scrolls.
    Growing,
    /// A turn is running — field writable, `Interrupt` shown.
    Executing,
    /// Session paused — dashed bar, `Wake` button.
    Asleep,
}

impl State {
    fn class(self) -> &'static str {
        match self {
            Self::Ready => "mui-composer--ready",
            Self::Growing => "mui-composer--growing",
            Self::Executing => "mui-composer--executing",
            Self::Asleep => "mui-composer--asleep",
        }
    }
}

/// A leading chip in the composer action row — a mono tag describing something
/// about THIS message (model, auto-accept mode, an attachment).
#[derive(Debug, Clone, Default)]
pub struct Chip {
    /// Chip text, e.g. `sonnet-4.6` or `auto-accept: off`.
    pub label: String,
    /// Accent-tinted — the "this message carries something extra" treatment
    /// used for attachments (`user.rs ×`).
    pub accent: bool,
}

impl Chip {
    /// A plain mono chip.
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), accent: false }
    }
    /// An accent-tinted chip — for attachments and other message payloads.
    pub fn accent(label: impl Into<String>) -> Self {
        Self { label: label.into(), accent: true }
    }
}

/// Composer rendering properties.
#[derive(Debug, Clone)]
pub struct Props {
    /// Which state to render.
    pub state: State,
    /// `<form action>` — where the message posts.
    pub action: String,
    /// `<form method>` — defaults to `post`.
    pub method: String,
    /// `<textarea name>` — defaults to `message`.
    pub field_name: String,
    /// Placeholder shown when the field is empty.
    pub placeholder: String,
    /// Pre-filled field content — the in-progress draft.
    pub value: String,
    /// Leading mono chips (model, auto-accept, attachments).
    pub chips: Vec<Chip>,
    /// Show the voice-capture button before the primary action.
    pub show_voice: bool,
    /// Optional secondary action label — renders a hollow-destructive button
    /// (the `Interrupt` affordance) before the primary. Only meaningful while
    /// executing.
    pub secondary_label: Option<String>,
    /// Primary button label — `Send`, `Queue`, …
    pub primary_label: String,
    /// Optional kbd hint shown inside the primary button (`⌘↵`).
    pub primary_kbd: Option<String>,
    /// Optional mono status line rendered below the field — the "beside" data
    /// (turn count, context, spend, worktree). 11px, muted.
    pub status: Option<Markup>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            state: State::Ready,
            action: String::new(),
            method: "post".into(),
            field_name: "message".into(),
            placeholder: String::new(),
            value: String::new(),
            chips: Vec::new(),
            show_voice: false,
            secondary_label: None,
            primary_label: "Send".into(),
            primary_kbd: None,
            status: None,
        }
    }
}

fn chip_markup(chip: &Chip) -> Markup {
    let class = if chip.accent {
        "mui-composer__chip mui-composer__chip--accent"
    } else {
        "mui-composer__chip"
    };
    html! { span class=(class) { (chip.label) } }
}

/// Render the composer for the given state.
pub fn render(props: Props) -> Markup {
    let class = format!("mui-composer {}", props.state.class());

    // Asleep is structurally its own thing — a dashed bar, no field.
    if props.state == State::Asleep {
        let wake = if props.primary_label.is_empty() {
            "Wake".to_string()
        } else {
            props.primary_label.clone()
        };
        return html! {
            form class=(class) action=(props.action) method=(props.method) {
                div class="mui-composer__sleepbar" {
                    span class="mui-composer__state-tag" { "ASLEEP" }
                    span class="mui-composer__sleep-hint" {
                        @if props.placeholder.is_empty() { "Type to wake" } @else { (props.placeholder) }
                    }
                    span class="mui-composer__spacer" {}
                    button type="submit" class="mui-composer__wake" { (wake) }
                }
            }
        };
    }

    let executing = props.state == State::Executing;

    html! {
        form class=(class) action=(props.action) method=(props.method) {
            div class="mui-composer__field" {
                textarea
                    class="mui-composer__input"
                    name=(props.field_name)
                    rows="2"
                    placeholder=(props.placeholder) {
                    (props.value)
                }
                div class="mui-composer__actions" {
                    @for chip in &props.chips {
                        (chip_markup(chip))
                    }
                    span class="mui-composer__spacer" {}
                    @if props.show_voice {
                        button type="button" class="mui-composer__voice" aria-label="Voice input" { "◉" }
                    }
                    @if executing {
                        @if let Some(label) = props.secondary_label.as_ref() {
                            button type="button" class="mui-composer__interrupt" { (label) }
                        }
                    }
                    button type="submit" class="mui-composer__send" {
                        (props.primary_label)
                        @if let Some(kbd) = props.primary_kbd.as_ref() {
                            span class="mui-composer__kbd" { (kbd) }
                        }
                    }
                }
            }
            @if let Some(status) = props.status.as_ref() {
                div class="mui-composer__status" { (status) }
            }
        }
    }
}

/// Showcase every composer state.
pub fn showcase() -> Markup {
    let status_ready = html! {
        span { "12 turns · 41.2k ctx · $1.84" }
        span { "⇧↵ newline" }
    };
    let status_exec = html! {
        span { "turn 12 · 41s · 18.4k ctx" }
        span { "esc interrupts" }
    };

    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Ready — the 84px dock" }
                p.mui-showcase__caption { "The resting state. Welded to the bottom of the pane; the operator's hands never hunt for it." }
                (render(Props {
                    state: State::Ready,
                    placeholder: "Message refactor-auth-middleware…".into(),
                    chips: vec![Chip::new("sonnet-4.6"), Chip::new("auto-accept: off"), Chip::new("+ attach")],
                    show_voice: true,
                    primary_label: "Send".into(),
                    primary_kbd: Some("⌘↵".into()),
                    status: Some(status_ready),
                    ..Default::default()
                }))
            }

            section {
                h2 { "Growing — focused, ringed, grows then scrolls" }
                p.mui-showcase__caption { "Two rows minimum; grows to a max then scrolls internally. The transcript loses the height, never the composer." }
                (render(Props {
                    state: State::Growing,
                    value: "Two of the auth tests fail now because the extension isn't inserted in the test harness — it only goes in inside the real layer. Add it to test_support::request() rather than making the extractor tolerate a missing handle. I'd rather it panic loudly in prod than silently fall back.".into(),
                    chips: vec![Chip::new("sonnet-4.6"), Chip::new("auto-accept: off"), Chip::accent("user.rs ×")],
                    show_voice: true,
                    primary_label: "Send".into(),
                    primary_kbd: Some("⌘↵".into()),
                    ..Default::default()
                }))
            }

            section {
                h2 { "Executing — writable, not disabled" }
                p.mui-showcase__caption { "A turn is running, but the field stays writable so a follow-up can be queued. Interrupt sits beside the primary as a hollow-destructive." }
                (render(Props {
                    state: State::Executing,
                    placeholder: "Queue a follow-up for turn 13…".into(),
                    chips: vec![Chip::new("sonnet-4.6")],
                    secondary_label: Some("Interrupt".into()),
                    primary_label: "Queue".into(),
                    primary_kbd: Some("⌘↵".into()),
                    status: Some(status_exec),
                    ..Default::default()
                }))
            }

            section {
                h2 { "Asleep — 46px, dashed, same anchor line" }
                p.mui-showcase__caption { "The session is paused. A mono state tag and a neutral Wake button; still the same anchor line." }
                (render(Props {
                    state: State::Asleep,
                    placeholder: "Type to wake and start turn 13".into(),
                    primary_label: "Wake".into(),
                    ..Default::default()
                }))
            }
        }
    }
}
