//! ButtonGroup component — groups multiple buttons with shared borders.
use maud::{html, Markup, PreEscaped};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn class_name(self) -> &'static str {
        match self {
            Orientation::Horizontal => "mui-button-group--horizontal",
            Orientation::Vertical => "mui-button-group--vertical",
        }
    }
}

/// Interactive mode for the group. `None` renders a pure visual
/// group (no JS behaviour attached); the host app is expected to
/// wire its own click handlers. `Some(Exclusive)` renders a
/// radio-like segmented control where exactly one child button
/// carries `aria-pressed="true"` at a time. `Some(Multiple)`
/// renders independent toggles.
///
/// In both interactive modes the group emits a
/// `mui:button-group-change` custom event (bubbles) on activation
/// with `{ value, pressed, target }` in `detail`, so hosts can
/// listen without wiring per-button listeners.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    /// Radio-like. Exactly one pressed button at any time.
    Exclusive,
    /// Toggle-like. Each button flips independently.
    Multiple,
}

impl Mode {
    fn attr(self) -> &'static str {
        match self {
            Mode::Exclusive => "exclusive",
            Mode::Multiple => "multiple",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Props {
    pub children: Markup,
    pub orientation: Orientation,
    pub size: Option<String>,
    /// When `Some`, the group opts into the bundled
    /// `data-mui="button-group"` behaviour that toggles
    /// `aria-pressed` on child buttons.
    pub mode: Option<Mode>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            children: html! {},
            orientation: Orientation::Horizontal,
            size: None,
            mode: None,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let mut class = "mui-button-group".to_string();
    class.push(' ');
    class.push_str(props.orientation.class_name());

    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(&size);
    }

    // We build the opening tag manually so we can conditionally
    // attach `data-mui` / `data-mode` without maud's attribute
    // interpolation syntax complaining about optional keys.
    let data_mui = props
        .mode
        .map(|_| r#" data-mui="button-group""#)
        .unwrap_or("");
    let data_mode = props
        .mode
        .map(|m| format!(r#" data-mode="{}""#, m.attr()))
        .unwrap_or_default();

    html! {
        (PreEscaped(format!(
            r#"<div class="{}" role="group"{}{}>"#,
            class, data_mui, data_mode,
        )))
        (props.children)
        (PreEscaped("</div>"))
    }
}

pub fn showcase() -> Markup {
    // Text alignment — Left/Center/Right with glyph icons
    let buttons_align = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-pressed="true" aria-label="Align left" {
            span aria-hidden="true" { "\u{2630}" }
            " Left"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-label="Align center" {
            span aria-hidden="true" { "\u{2261}" }
            " Center"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-label="Align right" {
            span aria-hidden="true" { "\u{268c}" }
            " Right"
        }
    };

    // Text formatting — Bold/Italic/Underline
    let buttons_format = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-pressed="true" aria-label="Bold" {
            strong { "B" }
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-label="Italic" {
            em { "I" }
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-label="Underline" {
            span style="text-decoration:underline;" { "U" }
        }
    };

    // View switcher — Grid/List/Board
    let buttons_view = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-pressed="true" {
            span aria-hidden="true" { "\u{25a6}" }
            " Grid"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" {
            span aria-hidden="true" { "\u{2630}" }
            " List"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" {
            span aria-hidden="true" { "\u{25a7}" }
            " Board"
        }
    };

    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Text alignment toolbar" }
                p.mui-showcase__caption { "Editor toolbar — exactly one alignment active at a time. Click or use ← / → to switch." }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_align,
                        orientation: Orientation::Horizontal,
                        size: None,
                        mode: Some(Mode::Exclusive),
                    }))
                }
            }
            section {
                h2 { "Text formatting" }
                p.mui-showcase__caption { "Rich-text controls — each toggle independent (Bold currently on). Click any to flip." }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_format,
                        orientation: Orientation::Horizontal,
                        size: None,
                        mode: Some(Mode::Multiple),
                    }))
                }
            }
            section {
                h2 { "View switcher" }
                p.mui-showcase__caption { "Project listing view — Grid / List / Board (Grid active)." }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_view,
                        orientation: Orientation::Horizontal,
                        size: None,
                        mode: Some(Mode::Exclusive),
                    }))
                }
            }
        }
    }
}
