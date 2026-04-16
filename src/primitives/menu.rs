//! Menu component — dropdown menu for actions

use maud::{html, Markup};

/// A single menu item
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub action: String,
    pub disabled: bool,
    pub destructive: bool,
    /// Optional keyboard shortcut displayed on the right (e.g. "⌘Z")
    pub shortcut: Option<String>,
}

/// Menu entry: item, separator, or label
#[derive(Debug, Clone)]
pub enum MenuEntry {
    Item(MenuItem),
    Separator,
    Label(String),
}

/// Menu rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Text displayed on the trigger button
    pub trigger_label: String,
    /// Unique identifier for the menu
    pub id: String,
    /// Menu entries (items, separators, and labels)
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
                    (render_entry(entry))
                }
            }
        }
    }
}

/// Render a single menu entry (shared between menu and context menu)
pub fn render_entry(entry: &MenuEntry) -> Markup {
    html! {
        @match entry {
            MenuEntry::Item(item) => {
                @let cls = if item.destructive { "mui-menu__item mui-menu__item--danger" } else { "mui-menu__item" };
                button
                    type="button"
                    role="menuitem"
                    class=(cls)
                    data-action=(item.action)
                    tabindex="-1"
                    disabled[item.disabled]
                {
                    (item.label.clone())
                    @if let Some(shortcut) = &item.shortcut {
                        span.mui-menu__shortcut { (shortcut) }
                    }
                }
            }
            MenuEntry::Separator => {
                div.mui-menu__separator role="separator" {}
            }
            MenuEntry::Label(text) => {
                div.mui-menu__label { (text) }
            }
        }
    }
}

/// Showcase menu component
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Dropdown menu with shortcuts" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger_label: "Actions".into(),
                        id: "demo-menu-1".into(),
                        items: vec![
                            MenuEntry::Label("Edit".into()),
                            MenuEntry::Item(MenuItem {
                                label: "Undo".into(),
                                action: "undo".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{2318}Z".into()),
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Redo".into(),
                                action: "redo".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{21e7}\u{2318}Z".into()),
                            }),
                            MenuEntry::Separator,
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
            div {
                p.mui-showcase__caption { "Simple menu" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger_label: "Options".into(),
                        id: "demo-menu-2".into(),
                        items: vec![
                            MenuEntry::Item(MenuItem {
                                label: "Edit".into(),
                                action: "edit".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: None,
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Duplicate".into(),
                                action: "duplicate".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: None,
                            }),
                            MenuEntry::Separator,
                            MenuEntry::Item(MenuItem {
                                label: "Archive".into(),
                                action: "archive".into(),
                                disabled: true,
                                destructive: false,
                                shortcut: None,
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Delete".into(),
                                action: "delete".into(),
                                disabled: false,
                                destructive: true,
                                shortcut: None,
                            }),
                        ],
                    }))
                }
            }
        }
    }
}
