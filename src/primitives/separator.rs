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
            // Profile card — horizontal separator between Bio and Settings sections
            section {
                h2 { "Profile card sections" }
                p.mui-showcase__caption { "Separator divides the Bio block from account Settings in a user profile." }
                div style="border: 1px solid var(--mui-border); border-radius: 0.5rem; padding: 1.25rem; max-width: 24rem; background: var(--mui-card-bg, var(--mui-bg, transparent));" {
                    div {
                        div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--mui-text-muted); margin-bottom: 0.25rem;" { "Bio" }
                        div style="font-weight: 600; font-size: 1rem;" { "Henry Geldenhuys" }
                        div style="font-size: 0.8125rem; color: var(--mui-text-muted); margin-top: 0.25rem;" {
                            "Staff engineer at Kapable. Building Claude Conductor. Cape Town \u{2192} Remote."
                        }
                    }
                    div style="margin: 1rem 0;" {
                        (render(Props { orientation: Orientation::Horizontal, decorative: true }))
                    }
                    div {
                        div style="font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em; color: var(--mui-text-muted); margin-bottom: 0.25rem;" { "Settings" }
                        div style="display: flex; justify-content: space-between; font-size: 0.875rem; padding: 0.25rem 0;" {
                            span { "Email" }
                            span style="color: var(--mui-text-muted);" { "invoice@geldentech.ca" }
                        }
                        div style="display: flex; justify-content: space-between; font-size: 0.875rem; padding: 0.25rem 0;" {
                            span { "Two-factor" }
                            span style="color: var(--mui-text-muted);" { "Enabled" }
                        }
                    }
                }
            }

            // "OR" divider — between Google sign-in and email form
            section {
                h2 { "Auth methods" }
                p.mui-showcase__caption { "\u{201c}OR\u{201d} label separates social sign-in from email + password." }
                div style="display: flex; flex-direction: column; gap: 0.75rem; max-width: 22rem;" {
                    button class="mui-btn mui-btn--outline mui-btn--md" style="width: 100%;" {
                        span aria-hidden="true" style="margin-right: 0.5rem;" { "G" }
                        "Sign in with Google"
                    }
                    div style="display: flex; align-items: center; gap: 0.75rem; margin: 0.25rem 0;" {
                        div style="flex: 1;" {
                            (render(Props { orientation: Orientation::Horizontal, decorative: true }))
                        }
                        span style="font-size: 0.75rem; font-weight: 500; text-transform: uppercase; color: var(--mui-text-muted); letter-spacing: 0.08em;" { "OR" }
                        div style="flex: 1;" {
                            (render(Props { orientation: Orientation::Horizontal, decorative: true }))
                        }
                    }
                    div class="mui-field" {
                        input type="email" placeholder="you@company.com" class="mui-input" style="width: 100%;";
                    }
                    div class="mui-field" {
                        input type="password" placeholder="Password" class="mui-input" style="width: 100%;";
                    }
                    button class="mui-btn mui-btn--primary mui-btn--md" type="submit" { "Sign in" }
                }
            }

            // Vertical separator inside a nav bar
            section {
                h2 { "Navigation" }
                p.mui-showcase__caption { "Vertical separator between primary nav and user menu." }
                nav style="display: flex; align-items: center; gap: 1rem; padding: 0.5rem 0.75rem; border: 1px solid var(--mui-border); border-radius: 0.5rem; height: 2.75rem;" {
                    a href="#" style="font-weight: 500; text-decoration: none; color: inherit;" { "Dashboard" }
                    a href="#" style="color: var(--mui-text-muted); text-decoration: none;" { "Projects" }
                    a href="#" style="color: var(--mui-text-muted); text-decoration: none;" { "Billing" }
                    a href="#" style="color: var(--mui-text-muted); text-decoration: none;" { "Settings" }
                    div style="flex: 1;" {}
                    (render(Props {
                        orientation: Orientation::Vertical,
                        ..Default::default()
                    }))
                    a href="#" style="color: var(--mui-text-muted); text-decoration: none; font-size: 0.875rem;" { "Docs" }
                    a href="#" style="font-weight: 500; text-decoration: none; color: inherit;" { "HG" }
                }
            }
        }
    }
}
