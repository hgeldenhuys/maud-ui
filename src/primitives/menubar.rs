//! Menubar component — horizontal menu bar with top-level triggers and dropdown menus

use maud::{html, Markup};
use crate::primitives::menu::{MenuEntry, MenuItem, render_entry};

/// A single menu within the menubar (e.g. "File", "Edit", "View")
#[derive(Debug, Clone)]
pub struct MenubarMenu {
    pub label: String,
    pub items: Vec<MenuEntry>,
}

/// Menubar rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Unique identifier for the menubar
    pub id: String,
    /// Top-level menus
    pub menus: Vec<MenubarMenu>,
}

/// Render a menubar with the given properties
pub fn render(props: Props) -> Markup {
    html! {
        div.mui-menubar data-mui="menubar" role="menubar" id=(props.id) {
            @for (i, menu) in props.menus.iter().enumerate() {
                div.mui-menubar__menu {
                    button.mui-menubar__trigger
                        type="button"
                        role="menuitem"
                        aria-expanded="false"
                        aria-haspopup="menu"
                        aria-controls=(format!("{}-menu-{}", props.id, i))
                        tabindex=(if i == 0 { "0" } else { "-1" })
                    {
                        (menu.label)
                    }
                    div.mui-menubar__content
                        id=(format!("{}-menu-{}", props.id, i))
                        role="menu"
                        hidden
                    {
                        @for entry in &menu.items {
                            (render_entry(entry))
                        }
                    }
                }
            }
        }
    }
}

/// Showcase menubar component
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Classic app menubar" }
                div.mui-showcase__row {
                    (render(Props {
                        id: "demo-menubar-1".into(),
                        menus: vec![
                            MenubarMenu {
                                label: "File".into(),
                                items: vec![
                                    MenuEntry::Item(MenuItem {
                                        label: "New".into(),
                                        action: "new".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: Some("\u{2318}N".into()),
                                    }),
                                    MenuEntry::Item(MenuItem {
                                        label: "Open".into(),
                                        action: "open".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: Some("\u{2318}O".into()),
                                    }),
                                    MenuEntry::Item(MenuItem {
                                        label: "Save".into(),
                                        action: "save".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: Some("\u{2318}S".into()),
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
                            },
                            MenubarMenu {
                                label: "Edit".into(),
                                items: vec![
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
                                        shortcut: Some("\u{2325}\u{2318}Z".into()),
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
                                ],
                            },
                            MenubarMenu {
                                label: "View".into(),
                                items: vec![
                                    MenuEntry::Item(MenuItem {
                                        label: "Zoom In".into(),
                                        action: "zoom-in".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: Some("\u{2318}+".into()),
                                    }),
                                    MenuEntry::Item(MenuItem {
                                        label: "Zoom Out".into(),
                                        action: "zoom-out".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: Some("\u{2318}-".into()),
                                    }),
                                    MenuEntry::Separator,
                                    MenuEntry::Item(MenuItem {
                                        label: "Fullscreen".into(),
                                        action: "fullscreen".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: Some("F11".into()),
                                    }),
                                ],
                            },
                        ],
                    }))
                }
            }
        }
    }
}
