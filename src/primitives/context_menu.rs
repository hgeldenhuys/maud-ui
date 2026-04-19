//! Context menu component — right-click menu overlay

use crate::primitives::menu::{render_entry, MenuEntry, MenuItem};
use maud::{html, Markup};

/// Context menu rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Unique identifier for the menu
    pub id: String,
    /// Content that can be right-clicked to trigger the menu
    pub trigger: Markup,
    /// Menu entries (items, separators, and labels)
    pub items: Vec<MenuEntry>,
}

/// Render a context menu with the given properties
pub fn render(props: Props) -> Markup {
    html! {
        div.mui-context-menu data-mui="context-menu" {
            div.mui-context-menu__region {
                (props.trigger)
            }
            div.mui-context-menu__content
                id=(format!("{}-menu", props.id))
                role="menu"
                hidden
                style="position: fixed; top: 0; left: 0;"
            {
                @for entry in &props.items {
                    (render_entry(entry))
                }
            }
        }
    }
}

/// Showcase context menu component
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Right-click the area below" }
                div.mui-showcase__row {
                    (render(Props {
                        id: "demo-ctx-1".into(),
                        trigger: html! {
                            div.mui-context-menu-demo {
                                "Right-click me"
                            }
                        },
                        items: vec![
                            MenuEntry::Item(MenuItem {
                                label: "Cut".into(),
                                action: "cut".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{2318}X".into()),
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Copy".into(),
                                action: "copy".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{2318}C".into()),
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Paste".into(),
                                action: "paste".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{2318}V".into()),
                            }),
                            MenuEntry::Separator,
                            MenuEntry::Item(MenuItem {
                                label: "Delete".into(),
                                action: "delete".into(),
                                disabled: false,
                                destructive: true,
                                shortcut: Some("\u{232b}".into()),
                            }),
                        ],
                    }))
                }
            }
        }
    }
}
