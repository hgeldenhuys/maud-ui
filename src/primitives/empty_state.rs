//! EmptyState component — placeholder for empty content (no results, empty table, etc.).
//!
//! Two composition paths are supported:
//! - [`render`] takes a [`Props`] struct and renders the full empty-state block in one call.
//!   Best for simple cases (icon + title + description + optional action).
//! - Subcomponent helpers ([`compose`], [`header`], [`media`], [`title`], [`description`],
//!   [`content`]) mirror shadcn's `Empty` / `EmptyHeader` / `EmptyMedia` / `EmptyTitle` /
//!   `EmptyDescription` / `EmptyContent` primitives. Use when you need a custom layout, a
//!   non-text media slot (SVG / image), or additional content sections.

use maud::{html, Markup};

/// Variant of the media slot — maps to shadcn's `EmptyMedia.variant` prop.
///
/// - [`MediaVariant::Default`] — larger decorative block (~4rem box); suits illustrations or
///   oversized emoji.
/// - [`MediaVariant::Icon`] — smaller glyph sized for a lucide-style icon (~2rem box).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MediaVariant {
    /// Larger decorative block. Default.
    #[default]
    Default,
    /// Smaller glyph sized for icon-style media.
    Icon,
}

impl MediaVariant {
    /// Returns the modifier class suffix — `default` or `icon`.
    pub fn as_class_suffix(self) -> &'static str {
        match self {
            MediaVariant::Default => "default",
            MediaVariant::Icon => "icon",
        }
    }
}

/// EmptyState rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Optional icon (text or emoji), defaults to 📭
    pub icon: Option<String>,
    /// Main title/heading
    pub title: String,
    /// Optional description text
    pub description: Option<String>,
    /// Optional action markup (e.g., button)
    pub action: Option<Markup>,
}

impl Props {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            icon: None,
            title: title.into(),
            description: None,
            action: None,
        }
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_action(mut self, action: Markup) -> Self {
        self.action = Some(action);
        self
    }
}

/// Render an empty state component with icon, title, description, and optional action.
///
/// Canonical shortcut — use this for the common case. For custom layouts, use the
/// [`compose`] / [`header`] / [`media`] / [`title`] / [`description`] / [`content`]
/// subcomponent helpers.
pub fn render(props: Props) -> Markup {
    let icon = props.icon.unwrap_or_else(|| "📭".to_string());

    html! {
        div.mui-empty-state {
            div.mui-empty-state__icon.mui-empty-state__icon--default aria-hidden="true" { (icon) }
            h3.mui-empty-state__title { (props.title) }
            @if let Some(desc) = props.description {
                p.mui-empty-state__description { (desc) }
            }
            @if let Some(action) = props.action {
                div.mui-empty-state__action { (action) }
            }
        }
    }
}

/// Root wrapper helper — shadcn `Empty` equivalent. Wrap your subcomponent children
/// (header / media / title / description / content) with this.
pub fn compose(children: Markup) -> Markup {
    html! {
        div class="mui-empty-state" {
            (children)
        }
    }
}

/// Header slot — shadcn `EmptyHeader` equivalent. Typically contains media + title +
/// description.
pub fn header(children: Markup) -> Markup {
    html! {
        div class="mui-empty-state__header" {
            (children)
        }
    }
}

/// Media slot — shadcn `EmptyMedia` equivalent. Renders the icon/illustration container
/// with a variant modifier class (`default` for larger decorative blocks, `icon` for
/// smaller glyphs).
pub fn media(children: Markup, variant: MediaVariant) -> Markup {
    let class = format!(
        "mui-empty-state__icon mui-empty-state__icon--{}",
        variant.as_class_suffix()
    );
    html! {
        div class=(class) aria-hidden="true" {
            (children)
        }
    }
}

/// Title heading — shadcn `EmptyTitle` equivalent. Renders an `<h2>`.
pub fn title(text: &str) -> Markup {
    html! {
        h2 class="mui-empty-state__title" { (text) }
    }
}

/// Description paragraph — shadcn `EmptyDescription` equivalent.
pub fn description(text: &str) -> Markup {
    html! {
        p class="mui-empty-state__description" { (text) }
    }
}

/// Content slot — shadcn `EmptyContent` equivalent. Use for actions, secondary text, or
/// any additional body content below the header.
pub fn content(children: Markup) -> Markup {
    html! {
        div class="mui-empty-state__content" {
            (children)
        }
    }
}

/// Showcase all empty state variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // No results
            div {
                p.mui-showcase__caption { "No results" }
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);background:var(--mui-bg-card)" {
                    (render(
                        Props::new("No results found")
                            .with_icon("\u{1F50D}")
                            .with_description("Try adjusting your search query or removing some filters to find what you're looking for.")
                            .with_action(html! {
                                button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Clear filters" }
                            })
                    ))
                }
            }

            // Empty inbox
            div {
                p.mui-showcase__caption { "Empty inbox" }
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);background:var(--mui-bg-card)" {
                    (render(
                        Props::new("Your inbox is empty")
                            .with_icon("\u{2709}\u{FE0F}")
                            .with_description("Messages you receive will appear here.")
                    ))
                }
            }

            // First-run / onboarding
            div {
                p.mui-showcase__caption { "First run" }
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);background:var(--mui-bg-card)" {
                    (render(
                        Props::new("Create your first project")
                            .with_icon("\u{1F680}")
                            .with_description("Projects help you organize your work and collaborate with your team.")
                            .with_action(html! {
                                button type="button" class="mui-btn mui-btn--default mui-btn--md" { "New project" }
                            })
                    ))
                }
            }

            // Minimal
            div {
                p.mui-showcase__caption { "Minimal" }
                (render(Props::new("Nothing here yet")))
            }

            // Compose path — subcomponent helpers with MediaVariant::Default
            div {
                p.mui-showcase__caption { "Composed (Default media)" }
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);background:var(--mui-bg-card)" {
                    (compose(html! {
                        (header(html! {
                            (media(html! { "\u{1F4E6}" }, MediaVariant::Default))
                            (title("No packages installed"))
                            (description("Install a package to start building your app."))
                        }))
                        (content(html! {
                            button type="button" class="mui-btn mui-btn--default mui-btn--md" { "Install package" }
                        }))
                    }))
                }
            }

            // Compose path — subcomponent helpers with MediaVariant::Icon
            div {
                p.mui-showcase__caption { "Composed (Icon media)" }
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);background:var(--mui-bg-card)" {
                    (compose(html! {
                        (header(html! {
                            (media(html! { "\u{1F50E}" }, MediaVariant::Icon))
                            (title("No matches"))
                            (description("No items match your current filters."))
                        }))
                        (content(html! {
                            button type="button" class="mui-btn mui-btn--outline mui-btn--sm" { "Reset filters" }
                        }))
                    }))
                }
            }
        }
    }
}
