//! Drawer component — sliding panel that anchors to a screen edge using native <dialog> element.
use maud::{html, Markup};

/// Side where the drawer slides in from
#[derive(Clone, Debug)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    fn class_name(&self) -> &'static str {
        match self {
            Side::Left => "mui-drawer--left",
            Side::Right => "mui-drawer--right",
            Side::Top => "mui-drawer--top",
            Side::Bottom => "mui-drawer--bottom",
        }
    }
}

impl Default for Side {
    fn default() -> Self {
        Side::Right
    }
}

/// Drawer rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier for the drawer (used by trigger to open it)
    pub id: String,
    /// Drawer title
    pub title: String,
    /// Optional description text displayed below title
    pub description: Option<String>,
    /// Markup content displayed in drawer body
    pub children: Markup,
    /// Which side the drawer slides from (default Right)
    pub side: Side,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "drawer".to_string(),
            title: "Drawer".to_string(),
            description: None,
            children: html! {},
            side: Side::Right,
        }
    }
}

/// Render a drawer trigger button that opens the drawer with the given target_id
pub fn trigger(target_id: &str, label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-btn mui-btn--default mui-btn--md"
            data-mui="drawer-trigger"
            data-target=(target_id)
        {
            (label)
        }
    }
}

/// Render a close button for use inside the drawer
pub fn close_button(label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-drawer__close"
            data-mui-close
            aria-label=(label)
        {
            "×"
        }
    }
}

/// Render a drawer with the given properties
pub fn render(props: Props) -> Markup {
    let title_id = format!("{}-title", props.id);
    let desc_id = format!("{}-desc", props.id);
    let aria_describedby = if props.description.is_some() { desc_id.clone() } else { String::new() };

    if aria_describedby.is_empty() {
        html! {
            dialog class={"mui-drawer " (props.side.class_name())}
                id=(props.id)
                data-mui="drawer"
                aria-labelledby=(title_id)
            {
                div class="mui-drawer__header" {
                    h2 class="mui-drawer__title" id=(title_id) {
                        (props.title)
                    }
                    (close_button("Close"))
                }
                @if let Some(desc) = props.description {
                    p class="mui-drawer__description" id=(desc_id) {
                        (desc)
                    }
                }
                div class="mui-drawer__body" {
                    (props.children)
                }
            }
        }
    } else {
        html! {
            dialog class={"mui-drawer " (props.side.class_name())}
                id=(props.id)
                data-mui="drawer"
                aria-labelledby=(title_id)
                aria-describedby=(aria_describedby)
            {
                div class="mui-drawer__header" {
                    h2 class="mui-drawer__title" id=(title_id) {
                        (props.title)
                    }
                    (close_button("Close"))
                }
                @if let Some(desc) = props.description {
                    p class="mui-drawer__description" id=(desc_id) {
                        (desc)
                    }
                }
                div class="mui-drawer__body" {
                    (props.children)
                }
            }
        }
    }
}

/// Showcase all drawer use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Right drawer (default)
            {
                (trigger("demo-drawer-1", "Open drawer (right)"))
            }
            {
                (render(Props {
                    id: "demo-drawer-1".to_string(),
                    title: "Settings".to_string(),
                    description: Some("Adjust your preferences here.".to_string()),
                    children: html! {
                        div class="mui-field" {
                            label class="mui-label" { "Theme" }
                            select class="mui-input" {
                                option { "Light" }
                                option { "Dark" }
                                option { "Auto" }
                            }
                        }
                        div class="mui-field" {
                            label class="mui-label" { "Notifications" }
                            input class="mui-input" type="checkbox" {}
                        }
                        div style="display: flex; gap: 0.75rem; margin-top: 1.5rem;" {
                            button class="mui-btn mui-btn--primary" { "Save" }
                            button class="mui-btn mui-btn--secondary" data-mui-close { "Cancel" }
                        }
                    },
                    side: Side::Right,
                }))
            }
            // Left drawer
            {
                (trigger("demo-drawer-2", "Open drawer (left)"))
            }
            {
                (render(Props {
                    id: "demo-drawer-2".to_string(),
                    title: "Navigation".to_string(),
                    description: None,
                    children: html! {
                        nav style="display: flex; flex-direction: column; gap: 0.5rem;" {
                            a class="mui-btn mui-btn--secondary" style="justify-content: flex-start;" href="#" { "Home" }
                            a class="mui-btn mui-btn--secondary" style="justify-content: flex-start;" href="#" { "Products" }
                            a class="mui-btn mui-btn--secondary" style="justify-content: flex-start;" href="#" { "Docs" }
                            a class="mui-btn mui-btn--secondary" style="justify-content: flex-start;" href="#" { "Contact" }
                        }
                    },
                    side: Side::Left,
                }))
            }
        }
    }
}
