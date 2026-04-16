//! EmptyState component — placeholder for empty content (no results, empty table, etc.).

use maud::{html, Markup};

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

/// Render an empty state component with icon, title, description, and optional action
pub fn render(props: Props) -> Markup {
    let icon = props.icon.unwrap_or_else(|| "📭".to_string());

    html! {
        div.mui-empty-state {
            div.mui-empty-state__icon aria-hidden="true" { (icon) }
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

/// Showcase all empty state variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Default with all elements
            div {
                p.mui-showcase__caption { "With description & action" }
                (render(
                    Props::new("No results found")
                        .with_description("Try adjusting your search or filters")
                        .with_action(html! {
                            button.mui-button.mui-button--default { "Clear filters" }
                        })
                ))
            }

            // Minimal
            div {
                p.mui-showcase__caption { "Minimal" }
                (render(Props::new("Nothing here yet")))
            }

            // Custom icon
            div {
                p.mui-showcase__caption { "Custom icon" }
                (render(
                    Props::new("No items")
                        .with_icon("🎯")
                        .with_description("Create your first item to get started")
                ))
            }
        }
    }
}
