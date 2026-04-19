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

    fn aria_value(self) -> &'static str {
        match self {
            Orientation::Horizontal => "horizontal",
            Orientation::Vertical => "vertical",
        }
    }

    fn opposite(self) -> Self {
        match self {
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal,
        }
    }
}

/// Renders a visual separator cell inside a button group.
///
/// The `orientation` argument is the separator's OWN orientation, i.e. the
/// axis along which the separator line runs. For a horizontal button group
/// (buttons in a row), pass `Orientation::Vertical` to draw a vertical rule
/// between buttons. When building a separator alongside a known group, prefer
/// [`separator_for_group`] which picks the opposite automatically.
pub fn separator(orientation: Orientation) -> Markup {
    html! {
        div class="mui-button-group__separator" role="separator"
            aria-orientation=(orientation.aria_value()) {}
    }
}

/// Convenience: renders a separator whose orientation is the opposite of the
/// containing group's orientation (the visually-correct default).
pub fn separator_for_group(group_orientation: Orientation) -> Markup {
    separator(group_orientation.opposite())
}

/// Renders a non-interactive text cell inside a button group (e.g. a page
/// indicator "Page 1 of 10" between paging buttons).
pub fn text(children: Markup) -> Markup {
    html! {
        div class="mui-button-group__text" {
            (children)
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
    // Emit aria-orientation only for vertical groups — horizontal is the
    // implicit default for role=group (assistive-tech announces arrow-key
    // behaviour accordingly when the vertical axis is explicit).
    let aria_orientation = if props.orientation == Orientation::Vertical {
        r#" aria-orientation="vertical""#
    } else {
        ""
    };

    html! {
        (PreEscaped(format!(
            r#"<div class="{}" role="group"{}{}{}>"#,
            class, aria_orientation, data_mui, data_mode,
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

    // Pager with separator — [Prev] [|] [Next], sm size.
    let buttons_pager_separator = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--sm" {
            span aria-hidden="true" { "\u{2039}" }
            " Prev"
        }
        (separator_for_group(Orientation::Horizontal))
        button type="button" class="mui-btn mui-btn--outline mui-btn--sm" {
            "Next "
            span aria-hidden="true" { "\u{203a}" }
        }
    };

    // Pager with text cell — [Page 1 of 10] [Prev] [Next].
    let buttons_pager_text = html! {
        (text(html! { "Page 1 of 10" }))
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" {
            span aria-hidden="true" { "\u{2039}" }
            " Prev"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" {
            "Next "
            span aria-hidden="true" { "\u{203a}" }
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
            section {
                h2 { "Pager with separator" }
                p.mui-showcase__caption { "Sm-size pager — [Prev] [separator] [Next]. The separator is a vertical rule between buttons in a horizontal group." }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_pager_separator,
                        orientation: Orientation::Horizontal,
                        size: None,
                        mode: None,
                    }))
                }
            }
            section {
                h2 { "Pager with text indicator" }
                p.mui-showcase__caption { "Non-interactive text cell — [Page 1 of 10] [Prev] [Next]. Useful for status read-outs inside toolbars." }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_pager_text,
                        orientation: Orientation::Horizontal,
                        size: None,
                        mode: None,
                    }))
                }
            }
        }
    }
}
