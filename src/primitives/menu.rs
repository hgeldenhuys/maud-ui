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

/// A single radio option inside a [`MenuEntry::RadioGroup`].
#[derive(Debug, Clone, Default)]
pub struct RadioItem {
    pub label: String,
    pub value: String,
    pub checked: bool,
    pub disabled: bool,
    /// Optional keyboard shortcut displayed on the right (e.g. "⌘1")
    pub shortcut: Option<String>,
}

/// Menu entry: item, separator, label, checkbox, radio group, submenu, or group.
///
/// Additive — new variants may be added in minor releases. Existing callers
/// that construct `Item`, `Separator`, or `Label` remain source-compatible.
#[derive(Debug, Clone)]
pub enum MenuEntry {
    Item(MenuItem),
    Separator,
    Label(String),
    /// A menuitemcheckbox with an on/off state.
    CheckboxItem {
        label: String,
        checked: bool,
        disabled: bool,
        shortcut: Option<String>,
    },
    /// A named radio group wrapping a set of [`RadioItem`]s.
    RadioGroup {
        name: String,
        items: Vec<RadioItem>,
    },
    /// A submenu trigger + nested content.
    Sub {
        trigger_label: String,
        items: Vec<MenuEntry>,
    },
    /// Generic grouping wrapper (`role="group"`) for related entries.
    Group {
        items: Vec<MenuEntry>,
    },
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

/// Render a single menu entry (shared between menu, menubar, and context menu).
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
                    data-variant=[item.destructive.then_some("destructive")]
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
            MenuEntry::CheckboxItem { label, checked, disabled, shortcut } => {
                button
                    type="button"
                    role="menuitemcheckbox"
                    class="mui-menu__item"
                    aria-checked=(if *checked { "true" } else { "false" })
                    aria-disabled=(if *disabled { "true" } else { "false" })
                    tabindex="-1"
                    disabled[*disabled]
                {
                    span.mui-menu__checkbox-indicator aria-hidden="true" {}
                    (label.clone())
                    @if let Some(shortcut) = shortcut {
                        span.mui-menu__shortcut { (shortcut) }
                    }
                }
            }
            MenuEntry::RadioGroup { name, items } => {
                div.mui-menu__radio-group role="group" data-radio-group=(name) {
                    @for item in items {
                        button
                            type="button"
                            role="menuitemradio"
                            class="mui-menu__item"
                            aria-checked=(if item.checked { "true" } else { "false" })
                            aria-disabled=(if item.disabled { "true" } else { "false" })
                            data-value=(item.value)
                            data-radio-group=(name)
                            tabindex="-1"
                            disabled[item.disabled]
                        {
                            span.mui-menu__radio-indicator aria-hidden="true" {}
                            (item.label.clone())
                            @if let Some(shortcut) = &item.shortcut {
                                span.mui-menu__shortcut { (shortcut) }
                            }
                        }
                    }
                }
            }
            MenuEntry::Sub { trigger_label, items } => {
                div.mui-menu__sub-wrapper {
                    button
                        type="button"
                        role="menuitem"
                        class="mui-menu__item mui-menu__sub-trigger"
                        aria-haspopup="menu"
                        aria-expanded="false"
                        tabindex="-1"
                    {
                        (trigger_label.clone())
                    }
                    div.mui-menu__sub role="menu" hidden {
                        @for inner in items {
                            (render_entry(inner))
                        }
                    }
                }
            }
            MenuEntry::Group { items } => {
                div.mui-menu__group role="group" {
                    @for inner in items {
                        (render_entry(inner))
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
            div {
                p.mui-showcase__caption { "View menu — checkbox + radio + submenu" }
                div.mui-showcase__row {
                    (render(Props {
                        trigger_label: "View".into(),
                        id: "demo-menu-view".into(),
                        items: vec![
                            MenuEntry::Label("Appearance".into()),
                            MenuEntry::CheckboxItem {
                                label: "Show Bookmarks Bar".into(),
                                checked: true,
                                disabled: false,
                                shortcut: Some("\u{21e7}\u{2318}B".into()),
                            },
                            MenuEntry::CheckboxItem {
                                label: "Show Full URLs".into(),
                                checked: false,
                                disabled: false,
                                shortcut: None,
                            },
                            MenuEntry::Separator,
                            MenuEntry::Label("Panel position".into()),
                            MenuEntry::RadioGroup {
                                name: "panel-position".into(),
                                items: vec![
                                    RadioItem {
                                        label: "Top".into(),
                                        value: "top".into(),
                                        checked: false,
                                        disabled: false,
                                        shortcut: None,
                                    },
                                    RadioItem {
                                        label: "Right".into(),
                                        value: "right".into(),
                                        checked: true,
                                        disabled: false,
                                        shortcut: None,
                                    },
                                    RadioItem {
                                        label: "Bottom".into(),
                                        value: "bottom".into(),
                                        checked: false,
                                        disabled: false,
                                        shortcut: None,
                                    },
                                ],
                            },
                            MenuEntry::Separator,
                            MenuEntry::Sub {
                                trigger_label: "More Tools".into(),
                                items: vec![
                                    MenuEntry::Item(MenuItem {
                                        label: "Save Page As\u{2026}".into(),
                                        action: "save-page".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: Some("\u{2318}S".into()),
                                    }),
                                    MenuEntry::Item(MenuItem {
                                        label: "Create Shortcut\u{2026}".into(),
                                        action: "create-shortcut".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: None,
                                    }),
                                    MenuEntry::Separator,
                                    MenuEntry::Item(MenuItem {
                                        label: "Developer Tools".into(),
                                        action: "devtools".into(),
                                        disabled: false,
                                        destructive: false,
                                        shortcut: Some("\u{2325}\u{2318}I".into()),
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
