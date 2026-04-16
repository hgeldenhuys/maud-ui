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
                p.mui-showcase__caption { "File menu" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger_label: "File".into(),
                        id: "demo-menu-file".into(),
                        items: vec![
                            MenuEntry::Item(MenuItem {
                                label: "New".into(),
                                action: "new".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{2318}N".into()),
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Open\u{2026}".into(),
                                action: "open".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{2318}O".into()),
                            }),
                            MenuEntry::Separator,
                            MenuEntry::Item(MenuItem {
                                label: "Save".into(),
                                action: "save".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{2318}S".into()),
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Save As\u{2026}".into(),
                                action: "save-as".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{21e7}\u{2318}S".into()),
                            }),
                            MenuEntry::Separator,
                            MenuEntry::Item(MenuItem {
                                label: "Print\u{2026}".into(),
                                action: "print".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: Some("\u{2318}P".into()),
                            }),
                            MenuEntry::Separator,
                            MenuEntry::Item(MenuItem {
                                label: "Exit".into(),
                                action: "exit".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: None,
                            }),
                        ],
                    }))
                }
            }
            div {
                p.mui-showcase__caption { "User menu" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger_label: "My Account".into(),
                        id: "demo-menu-user".into(),
                        items: vec![
                            MenuEntry::Label("Account".into()),
                            MenuEntry::Item(MenuItem {
                                label: "Profile".into(),
                                action: "profile".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: None,
                            }),
                            MenuEntry::Item(MenuItem {
                                label: "Settings".into(),
                                action: "settings".into(),
                                disabled: false,
                                destructive: false,
                                shortcut: None,
                            }),
                            MenuEntry::Separator,
                            MenuEntry::Item(MenuItem {
                                label: "Sign out".into(),
                                action: "sign-out".into(),
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
