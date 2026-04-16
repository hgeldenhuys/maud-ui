//! Kbd component — keyboard shortcut display.

use maud::{html, Markup};

/// Keyboard shortcut rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Keys to display (e.g., ["Ctrl", "S"] or ["⌘", "K"])
    pub keys: Vec<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            keys: Vec::new(),
        }
    }
}

/// Render keyboard shortcut with key separator
pub fn render(props: Props) -> Markup {
    html! {
        div.mui-kbd-container {
            @for (idx, key) in props.keys.iter().enumerate() {
                @if idx > 0 {
                    span.mui-kbd-separator { "+" }
                }
                kbd.mui-kbd { (key) }
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
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);padding:1rem;background:var(--mui-bg-card);max-width:20rem;" {
                    div style="display:flex;flex-direction:column;gap:0.625rem;" {
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span style="font-size:0.875rem;color:var(--mui-fg-muted);" { "Save" }
                            (render(Props { keys: vec!["\u{2318}".into(), "S".into()] }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span style="font-size:0.875rem;color:var(--mui-fg-muted);" { "Undo" }
                            (render(Props { keys: vec!["\u{2318}".into(), "Z".into()] }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span style="font-size:0.875rem;color:var(--mui-fg-muted);" { "Redo" }
                            (render(Props { keys: vec!["\u{21E7}".into(), "\u{2318}".into(), "Z".into()] }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span style="font-size:0.875rem;color:var(--mui-fg-muted);" { "Command palette" }
                            (render(Props { keys: vec!["\u{2318}".into(), "K".into()] }))
                        }
                        div style="display:flex;justify-content:space-between;align-items:center;" {
                            span style="font-size:0.875rem;color:var(--mui-fg-muted);" { "Toggle comment" }
                            (render(Props { keys: vec!["\u{2318}".into(), "/".into()] }))
                        }
                    }
                }
            }
        }
    }
}
