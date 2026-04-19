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

/// Render a context menu with the given properties.
///
/// The content element emits `data-side="inline-end"` so RTL (right-to-left)
/// locales can flip submenu / popover alignment without JS locale detection.
/// Mirrors Radix/shadcn's `data-side` convention.
pub fn render(props: Props) -> Markup {
    html! {
        div.mui-context-menu data-mui="context-menu" {
            div.mui-context-menu__region {
                (props.trigger)
            }
            div.mui-context-menu__content
                id=(format!("{}-menu", props.id))
                role="menu"
                aria-orientation="vertical"
                data-side="inline-end"
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

/// Build a `MenuEntry::Item` pre-set to destructive (for symmetry with
/// shadcn's `<ContextMenuItem variant="destructive">`).
///
/// The `destructive` flag on [`MenuItem`] is the Maud UI equivalent of
/// shadcn/Radix's `variant="destructive"` — it applies the danger styling
/// (`mui-menu__item--danger`) used for irreversible actions like Delete,
/// Sign out, or Discard changes. Prefer this helper over constructing a
/// `MenuItem` by hand when the action is destructive; it keeps callsites
/// declarative and matches the shadcn API surface exactly.
///
/// # Example
/// ```ignore
/// use maud_ui::primitives::context_menu::destructive_item;
///
/// let entry = destructive_item("delete", "Delete", false, Some("\u{232b}"));
/// ```
pub fn destructive_item(
    action: &str,
    label: &str,
    disabled: bool,
    shortcut: Option<&str>,
) -> MenuEntry {
    MenuEntry::Item(MenuItem {
        label: label.into(),
        action: action.into(),
        disabled,
        destructive: true,
        shortcut: shortcut.map(|s| s.into()),
    })
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
                            destructive_item("delete", "Delete", false, Some("\u{232b}")),
                        ],
                    }))
                }
            }
        }
    }
}
