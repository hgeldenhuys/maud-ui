//! Menu component — dropdown menu for actions

use maud::{html, Markup};

/// A single menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: String,
    pub disabled: bool,
    pub destructive: bool,
}

/// Menu entry: item or separator
#[derive(Debug, Clone)]
pub enum MenuEntry {
    Item(MenuItem),
    Separator,
}

/// Menu rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Text displayed on the trigger button
    pub trigger_label: String,
    /// Unique identifier for the menu
    pub id: String,
    /// Menu entries (items and separators)
    pub items: Vec<MenuEntry>,
}

/// Render a dropdown menu with the given properties
pub fn render(props: Props) -> Markup {
    html! {
        div.mui-menu data-mui="menu" {
            button.mui-menu__trigger.mui-btn.mui-btn--default.mui-btn--md
                type="button"
                aria-expanded="false"
                aria-haspopup="menu"
                aria-controls=(format!("{}-items", props.id))
            {
                (props.trigger_label)
                span.mui-menu__chevron aria-hidden="true" { "▾" }
            }
            div.mui-menu__content id=(format!("{}-items", props.id)) role="menu" hidden {
                @for entry in &props.items {
                    @match entry {
                        MenuEntry::Item(item) => {
                            button.mui-menu__item
                                type="button"
                                role="menuitem"
                                data-action=(item.action.clone())
                                tabindex="-1"
                                class={
                                    @if item.destructive { "mui-menu__item--danger" } @else { "" }
                                }
                                disabled[item.disabled]
                            {
                                (item.label.clone())
                            }
                        }
                        MenuEntry::Separator => {
                            div.mui-menu__separator role="separator" {}
                        }
                    }
                }
            }
        }
    }
}

/// Showcase menu component
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Dropdown menu" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger_label: "Actions".into(),
                        id: "demo-menu-1".into(),
                        items: vec![
                            MenuEntry::Item(MenuItem {
                                label: "Edit".into(),
                                action: "edit".into(),
                                disabled: false,
                                destructive: false,
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Duplicate".into(),
                                action: "duplicate".into(),
                                disabled: false,
                                destructive: false,
                            }),
                            MenuEntry::Separator,
                            MenuEntry::Item(MenuItem {
                                label: "Archive".into(),
                                action: "archive".into(),
                                disabled: false,
                                destructive: false,
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Delete".into(),
                                action: "delete".into(),
                                disabled: false,
                                destructive: true,
                            }),
                        ],
                    }))
                }
            }
        }
    }
}
