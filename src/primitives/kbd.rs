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

/// Showcase keyboard shortcuts
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Common shortcuts" }
                div.mui-showcase__row {
                    (render(Props {
                        keys: vec!["Ctrl".into(), "S".into()],
                    }))
                    (render(Props {
                        keys: vec!["⌘".into(), "K".into()],
                    }))
                    (render(Props {
                        keys: vec!["Shift".into(), "Enter".into()],
                    }))
                    (render(Props {
                        keys: vec!["Esc".into()],
                    }))
                }
            }
        }
    }
}
