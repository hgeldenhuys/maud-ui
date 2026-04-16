//! Separator component — visual and semantic divider for organizing content.

use maud::{html, Markup};

/// Separator orientation — horizontal (default) or vertical
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn class(&self) -> &'static str {
        match self {
            Self::Horizontal => "mui-separator--horizontal",
            Self::Vertical => "mui-separator--vertical",
        }
    }

    fn aria_orientation(&self) -> &'static str {
        match self {
            Self::Horizontal => "horizontal",
            Self::Vertical => "vertical",
        }
    }
}

/// Separator rendering properties
#[derive(Debug, Clone, Copy)]
pub struct Props {
    /// Orientation of the separator (default: Horizontal)
    pub orientation: Orientation,
    /// If true, render as purely decorative (aria-hidden). If false, semantic with role="separator"
    pub decorative: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            decorative: false,
        }
    }
}

/// Render a single separator with the given properties
pub fn render(props: Props) -> Markup {
    let class = format!("mui-separator {}", props.orientation.class());
    html! {
        @if props.decorative {
            div class=(class) aria-hidden="true" {}
        } @else {
            div class=(class)
                role="separator"
                aria-orientation=(props.orientation.aria_orientation()) {}
        }
    }
}

/// Showcase all separator variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Horizontal separator — basic
            section {
                h2 { "Horizontal separator" }
                p style="margin: 0 0 0.5rem; color: var(--mui-text-muted); font-size: 0.875rem;" {
                    "A 1px line spanning the full width."
                }
                (render(Props {
                    orientation: Orientation::Horizontal,
                    ..Default::default()
                }))
            }

            // Settings-style list with separators between items
            section {
                h2 { "Between list items" }
                div style="display: flex; flex-direction: column;" {
                    div style="display: flex; justify-content: space-between; align-items: center; padding: 0.75rem 0;" {
                        div {
                            div style="font-weight: 500;" { "Profile" }
                            div style="font-size: 0.8125rem; color: var(--mui-text-muted);" { "Manage your account details" }
                        }
                        span style="color: var(--mui-text-muted); font-size: 0.875rem;" { ">" }
                    }
                    (render(Props { orientation: Orientation::Horizontal, decorative: true }))
                    div style="display: flex; justify-content: space-between; align-items: center; padding: 0.75rem 0;" {
                        div {
                            div style="font-weight: 500;" { "Notifications" }
                            div style="font-size: 0.8125rem; color: var(--mui-text-muted);" { "Configure alert preferences" }
                        }
                        span style="color: var(--mui-text-muted); font-size: 0.875rem;" { ">" }
                    }
                    (render(Props { orientation: Orientation::Horizontal, decorative: true }))
                    div style="display: flex; justify-content: space-between; align-items: center; padding: 0.75rem 0;" {
                        div {
                            div style="font-weight: 500;" { "Privacy" }
                            div style="font-size: 0.8125rem; color: var(--mui-text-muted);" { "Control data sharing" }
                        }
                        span style="color: var(--mui-text-muted); font-size: 0.875rem;" { ">" }
                    }
                }
            }

            // Vertical separators in a toolbar
            section {
                h2 { "Vertical separators (toolbar)" }
                div style="display: flex; align-items: center; gap: 0.75rem; height: 2.5rem;" {
                    button class="mui-btn mui-btn--ghost mui-btn--sm" { "Cut" }
                    button class="mui-btn mui-btn--ghost mui-btn--sm" { "Copy" }
                    button class="mui-btn mui-btn--ghost mui-btn--sm" { "Paste" }
                    (render(Props {
                        orientation: Orientation::Vertical,
                        ..Default::default()
                    }))
                    button class="mui-btn mui-btn--ghost mui-btn--sm" { "Undo" }
                    button class="mui-btn mui-btn--ghost mui-btn--sm" { "Redo" }
                    (render(Props {
                        orientation: Orientation::Vertical,
                        ..Default::default()
                    }))
                    button class="mui-btn mui-btn--ghost mui-btn--sm" { "Settings" }
                }
            }

            // Decorative variant
            section {
                h2 { "Decorative (aria-hidden)" }
                p style="margin: 0 0 0.5rem; color: var(--mui-text-muted); font-size: 0.875rem;" {
                    "Purely visual, hidden from screen readers."
                }
                div style="padding: 0.75rem 0;" {
                    p style="margin: 0;" { "Section one content" }
                }
                (render(Props {
                    orientation: Orientation::Horizontal,
                    decorative: true,
                }))
                div style="padding: 0.75rem 0;" {
                    p style="margin: 0;" { "Section two content" }
                }
            }
        }
    }
}
