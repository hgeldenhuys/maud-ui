//! Mobile navigation with real links. This is navigation, not an ARIA tablist.
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Item {
    pub label: String,
    /// Optional compact visible label. The full label remains in the accessible name.
    pub short_label: Option<String>,
    pub href: String,
    pub icon: Option<Markup>,
}

#[derive(Clone, Debug)]
pub struct More {
    pub label: String,
    /// ID of a native dialog (including sheet/drawer).
    pub target_id: String,
    /// Reachable navigation page or visible anchor when JS is unavailable.
    pub fallback_href: String,
    pub current: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Position {
    #[default]
    Fixed,
    /// For contained previews or an application that owns positioning.
    Inline,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub items: Vec<Item>,
    /// Exact href match. A current More item takes precedence.
    pub current_href: Option<String>,
    pub more: Option<More>,
    pub aria_label: String,
    pub position: Position,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            items: vec![],
            current_href: None,
            more: None,
            aria_label: "Primary navigation".into(),
            position: Position::Fixed,
        }
    }
}

/// Panics for more than five destinations including More. Never silently drops links.
pub fn render(props: Props) -> Markup {
    assert!(
        props.items.len() + usize::from(props.more.is_some()) <= 5,
        "Bottom tab bar supports at most five items including More"
    );
    let more_current = props.more.as_ref().is_some_and(|m| m.current);
    let current = props
        .items
        .iter()
        .position(|i| Some(&i.href) == props.current_href.as_ref());
    html! {
        nav class=(if props.position == Position::Fixed { "mui-bottom-tab-bar" } else { "mui-bottom-tab-bar mui-bottom-tab-bar--inline" }) aria-label=(props.aria_label)
            data-items=(props.items.len() + usize::from(props.more.is_some())) {
            @for (index, item) in props.items.iter().enumerate() {
                @let short = item.short_label.as_deref().filter(|label| !label.trim().is_empty());
                // Include the visible label in the accessible name for voice control.
                @let accessible = short.filter(|label| *label != item.label).map(|label| format!("{label} — {}", item.label));
                a class="mui-bottom-tab-bar__item" href=(item.href) aria-current=[(!more_current && current == Some(index)).then_some("page")]
                    aria-label=[accessible.as_deref()] title=(item.label) {
                    @if let Some(icon) = &item.icon { span class="mui-bottom-tab-bar__icon" aria-hidden="true" { (icon) } }
                    span class="mui-bottom-tab-bar__label" { (short.unwrap_or(&item.label)) }
                }
            }
            @if let Some(more) = &props.more {
                a class="mui-bottom-tab-bar__item" href=(more.fallback_href) aria-controls=(more.target_id)
                    aria-haspopup="dialog" data-mui="navigation-trigger" aria-current=[more.current.then_some("page")] {
                    span class="mui-bottom-tab-bar__icon" aria-hidden="true" { "•••" }
                    span class="mui-bottom-tab-bar__label" { (more.label) }
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        p class="mui-showcase__caption" { "Four frequent destinations and More. The fixed variant includes bottom safe-area padding; reserve space in your page with --mui-bottom-tab-bar-height." }
        (render(Props {
            items: [("Home", "Home", "⌂"), ("Reservations", "Stays", "▤"), ("Guests", "Guests", "♧"), ("Tasks", "Tasks", "✓")].into_iter().map(|(label, short, icon)| Item {
                label: label.into(), short_label: Some(short.into()), href: format!("?view={label}"), icon: Some(html! { (icon) }),
            }).collect(),
            current_href: Some("?view=Reservations".into()),
            more: Some(More { label: "More".into(), target_id: "tab-bar-more".into(), fallback_href: "#tab-bar-destinations".into(), current: false }),
            position: Position::Inline, ..Default::default()
        }))
        dialog class="mui-navigation-dialog" id="tab-bar-more" aria-label="More destinations" {
            form method="dialog" { button class="mui-btn mui-btn--outline mui-btn--sm" { "Close navigation" } }
            a href="/blocks/shell-sidebar" { "Application shell" }
            a href="/blocks/task-grid" { "Tasks" }
        }
        p class="mui-bottom-tab-bar__destinations" id="tab-bar-destinations" { "More destinations: " a href="/blocks/shell-sidebar" { "Application shell" } " · " a href="/blocks/task-grid" { "Tasks" } }
    }
}
