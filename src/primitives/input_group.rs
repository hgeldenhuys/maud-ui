//! InputGroup component — input with prefix/suffix/block addons as one visual unit.
//!
//! shadcn Base UI parity (P1 rows from gap list):
//! - [`Align`] — addon placement (inline-start, inline-end, block-start, block-end).
//! - [`addon`]  — wrap arbitrary Markup in an addon slot at a given align.
//! - [`button`] — helper that renders a sized/variant button inside a trailing addon.
//! - [`text`]   — helper for an `InputGroupText` (muted supplementary text) addon.
//! - [`input_el`] / [`textarea`] — render the control with `data-slot="input-group-control"`
//!   so downstream styling / selectors can match shadcn's control hook.
use maud::{html, Markup};

use super::button;
use super::input;
use super::textarea;

/// Addon placement relative to the input control.
///
/// Mirrors shadcn's `InputGroupAddon.align` prop. The default is `InlineStart`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Align {
    /// Leading addon (left in LTR). Rendered BEFORE the control via `order: -1`.
    #[default]
    InlineStart,
    /// Trailing addon (right in LTR). Rendered AFTER the control via `order: 99`.
    InlineEnd,
    /// Addon above the row (full-width banner). Uses `flex-basis: 100%` + `order: -2`.
    BlockStart,
    /// Addon below the row (full-width footer / hint). Uses `flex-basis: 100%` + `order: 100`.
    BlockEnd,
}

impl Align {
    fn modifier(self) -> &'static str {
        match self {
            Self::InlineStart => "mui-input-group__addon--inline-start",
            Self::InlineEnd => "mui-input-group__addon--inline-end",
            Self::BlockStart => "mui-input-group__addon--block-start",
            Self::BlockEnd => "mui-input-group__addon--block-end",
        }
    }
}

pub struct InputGroupProps {
    pub prefix: Option<Markup>,
    pub suffix: Option<Markup>,
    pub children: Markup,
}

pub fn render(props: InputGroupProps) -> Markup {
    html! {
        div.mui-input-group {
            @if let Some(prefix) = props.prefix {
                span.mui-input-group__prefix { (prefix) }
            }
            (props.children)
            @if let Some(suffix) = props.suffix {
                span.mui-input-group__suffix { (suffix) }
            }
        }
    }
}

/// Render an addon at the requested [`Align`]. The addon itself is a plain `<div>`;
/// positioning is driven by the CSS modifier class so flex order and block-wrap
/// behavior is deterministic without re-rendering.
pub fn addon(children: Markup, align: Align) -> Markup {
    let class = format!("mui-input-group__addon {}", align.modifier());
    html! {
        div class=(class) { (children) }
    }
}

/// Render a button sized/variant-styled for use inside an input group addon slot.
/// Wraps the button in an `InlineEnd` addon by default — the most common shadcn usage
/// (password reveal, search submit, copy).
pub fn button(label: &str, size: button::Size, variant: button::Variant) -> Markup {
    let btn = button::render(button::Props {
        label: label.to_string(),
        variant,
        size,
        ..Default::default()
    });
    addon(btn, Align::InlineEnd)
}

/// Render `InputGroupText` — muted supplementary text wrapped as an addon.
/// By default placed at `InlineStart` (shadcn's common use: currency prefix, `@`, `https://`).
pub fn text(children: Markup) -> Markup {
    let inner = html! {
        span.mui-input-group__text { (children) }
    };
    addon(inner, Align::InlineStart)
}

/// Render an [`input::Props`]-configured input control and wrap it with
/// `data-slot="input-group-control"` so shadcn-compatible styling / selectors
/// can match the control inside a group.
pub fn input_el(props: input::Props) -> Markup {
    html! {
        div data-slot="input-group-control" class="mui-input-group__control" {
            (input::render(props))
        }
    }
}

/// Render a [`textarea::Props`]-configured textarea control and wrap it with
/// `data-slot="input-group-control"`. Same rationale as [`input_el`].
pub fn textarea(props: textarea::Props) -> Markup {
    html! {
        div data-slot="input-group-control" class="mui-input-group__control" {
            (textarea::render(props))
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div class="mui-showcase-section" {
            h3 { "Input with $ prefix" }
            (render(InputGroupProps {
                prefix: Some(html! { "$" }),
                suffix: None,
                children: html! { input type="text" placeholder="Amount" {} },
            }))
        }

        div class="mui-showcase-section" {
            h3 { "Input with .com suffix" }
            (render(InputGroupProps {
                prefix: None,
                suffix: Some(html! { ".com" }),
                children: html! { input type="text" placeholder="Domain" {} },
            }))
        }

        div class="mui-showcase-section" {
            h3 { "Input with https:// prefix and Go button" }
            (render(InputGroupProps {
                prefix: Some(html! { "https://" }),
                suffix: Some(html! { button { "Go" } }),
                children: html! { input type="text" placeholder="URL" {} },
            }))
        }

        div class="mui-showcase-section" {
            h3 { "Email with @ prefix" }
            (render(InputGroupProps {
                prefix: Some(html! { "@" }),
                suffix: None,
                children: html! { input type="email" placeholder="username" {} },
            }))
        }

        // P1 parity demo — Align::{InlineStart,InlineEnd,BlockEnd} with helpers.
        div class="mui-showcase-section" {
            h3 { "Addon alignment — prefix text, suffix button, block-end hint" }
            p.mui-showcase__caption { "Uses `addon`, `text`, `button`, and `input_el` helpers. Prefix text is InlineStart, Go is InlineEnd, hint wraps below the row via BlockEnd." }
            div.mui-input-group {
                (text(html! { "https://" }))
                (input_el(input::Props {
                    name: "site-url".into(),
                    id: "ig-site-url".into(),
                    input_type: input::InputType::Url,
                    placeholder: "your-site".into(),
                    aria_describedby: Some("ig-site-hint".into()),
                    ..Default::default()
                }))
                (button("Go", button::Size::Sm, button::Variant::Primary))
                (addon(
                    html! {
                        span id="ig-site-hint" style="font-size:0.75rem;color:var(--mui-text-muted);" {
                            "We'll prepend https:// and check the domain resolves."
                        }
                    },
                    Align::BlockEnd,
                ))
            }
        }
    }
}
