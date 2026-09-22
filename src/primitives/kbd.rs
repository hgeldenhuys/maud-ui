//! Kbd component — keyboard shortcut display.

use maud::{html, Markup};

/// Kbd treatment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// The keycap: bordered chip with a 2px bottom edge and a shadow.
    #[default]
    Default,
    /// Inline code chip: a 24px bordered pill in the mono face that sits
    /// inline in a text line — no keycap bottom border or shadow, and
    /// selectable (Linear's inline code).
    Code,
}

impl Variant {
    fn class(self) -> &'static str {
        match self {
            Variant::Default => "",
            Variant::Code => "mui-kbd--code",
        }
    }
}

/// Keyboard shortcut rendering properties
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct Props {
    /// Keys to display (e.g., ["Ctrl", "S"] or ["⌘", "K"])
    pub keys: Vec<String>,
    /// Treatment. See [`Variant`].
    pub variant: Variant,
}


/// Render keyboard shortcut with key separator
pub fn render(props: Props) -> Markup {
    let key_class = props.variant.class();
    // The keycap group is a control: a wrapper span carries the separators
    // (a div here would make the browser close a surrounding <p> mid-sentence).
    // The code chip is INLINE CONTENT: it renders bare, so a chip sitting in a
    // sentence does not carry a wrapper box that perturbs the line's height.
    if key_class.is_empty() {
        html! {
            span.mui-kbd-container {
                @for (idx, key) in props.keys.iter().enumerate() {
                    @if idx > 0 {
                        span.mui-kbd-separator { "+" }
                    }
                    kbd.mui-kbd { (key) }
                }
            }
        }
    } else {
        html! {
            @for (idx, key) in props.keys.iter().enumerate() {
                @if idx > 0 {
                    span.mui-kbd-separator { "+" }
                }
                kbd class={"mui-kbd " (key_class)} { (key) }
            }
        }
    }
}

/// Showcase keyboard shortcuts in a help card context
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Keyboard shortcuts" }
                div style="border:1px solid var(--mui-border);border-radius: var(--mui-radius-lg);padding: var(--mui-space-lg);background:var(--mui-bg-card);max-width:20rem;" {
                    div style="display:flex;flex-direction:column;gap: var(--mui-inset-cell);" {
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span data-mui-type="small" style="color:var(--mui-text-muted);" { "Save" }
                            (render(Props { keys: vec!["\u{2318}".into(), "S".into()], ..Default::default() }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span data-mui-type="small" style="color:var(--mui-text-muted);" { "Undo" }
                            (render(Props { keys: vec!["\u{2318}".into(), "Z".into()], ..Default::default() }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span data-mui-type="small" style="color:var(--mui-text-muted);" { "Redo" }
                            (render(Props { keys: vec!["\u{21E7}".into(), "\u{2318}".into(), "Z".into()], ..Default::default() }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span data-mui-type="small" style="color:var(--mui-text-muted);" { "Command palette" }
                            (render(Props { keys: vec!["\u{2318}".into(), "K".into()], ..Default::default() }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span data-mui-type="small" style="color:var(--mui-text-muted);" { "Toggle comment" }
                            (render(Props { keys: vec!["\u{2318}".into(), "/".into()], ..Default::default() }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span data-mui-type="small" style="color:var(--mui-text-muted);" { "Inline code chip" }
                            (render(Props { keys: vec!["vehicle_state".into()], variant: Variant::Code }))
                        }
                    }
                }
            }
        }
    }
}
