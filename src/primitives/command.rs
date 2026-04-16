//! Command component — Cmd+K command palette with search, grouped items, and keyboard navigation.
use maud::{html, Markup};

/// A single command item in the palette
#[derive(Clone, Debug)]
pub struct CommandItem {
    /// Display label for the command
    pub label: String,
    /// Optional keyboard shortcut (e.g., "⌘N")
    pub shortcut: Option<String>,
    /// Optional group name for categorization
    pub group: Option<String>,
    /// Whether the item is disabled
    pub disabled: bool,
}

/// Command palette rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier for the command palette dialog
    pub id: String,
    /// List of command items
    pub items: Vec<CommandItem>,
    /// Placeholder text for the search input
    pub placeholder: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "command".to_string(),
            items: vec![],
            placeholder: "Type a command or search\u{2026}".to_string(),
        }
    }
}

/// Render a trigger button that opens the command palette
pub fn trigger(target_id: &str, label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-btn mui-btn--default mui-btn--md"
            data-mui="command-trigger"
            data-target=(target_id)
        {
            (label)
        }
    }
}

/// Render the command palette
pub fn render(props: Props) -> Markup {
    // Collect unique groups in insertion order
    let mut groups: Vec<String> = Vec::new();
    for item in &props.items {
        let group_key = item.group.clone().unwrap_or_default();
        let mut found = false;
        for g in &groups {
            if *g == group_key {
                found = true;
                break;
            }
        }
        if !found {
            groups.push(group_key);
        }
    }

    html! {
        dialog class="mui-command"
            id=(props.id)
            data-mui="command"
            aria-label="Command palette"
        {
            div class="mui-command__search-wrap" {
                span class="mui-command__search-icon" aria-hidden="true" { "\u{2315}" }
                input type="text" class="mui-command__search"
                    placeholder=(props.placeholder)
                    autocomplete="off"
                    aria-label="Search commands";
            }
            div class="mui-command__list" role="listbox" {
                @for group_name in &groups {
                    div class="mui-command__group" {
                        @if !group_name.is_empty() {
                            div class="mui-command__group-label" { (group_name) }
                        }
                        @for item in &props.items {
                            @let item_group = item.group.clone().unwrap_or_default();
                            @if item_group == *group_name {
                                @if item.disabled {
                                    div class="mui-command__item mui-command__item--disabled"
                                        role="option"
                                        tabindex="-1"
                                        aria-disabled="true"
                                        data-label=(item.label)
                                    {
                                        span class="mui-command__item-label" { (item.label) }
                                        @if let Some(shortcut) = &item.shortcut {
                                            kbd class="mui-kbd" { (shortcut) }
                                        }
                                    }
                                } @else {
                                    div class="mui-command__item"
                                        role="option"
                                        tabindex="-1"
                                        data-label=(item.label)
                                    {
                                        span class="mui-command__item-label" { (item.label) }
                                        @if let Some(shortcut) = &item.shortcut {
                                            kbd class="mui-kbd" { (shortcut) }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            div class="mui-command__empty" hidden { "No results found." }
        }
    }
}

/// Showcase the command palette
pub fn showcase() -> Markup {
    let items = vec![
        CommandItem {
            label: "Calendar".to_string(),
            shortcut: None,
            group: Some("Suggestions".to_string()),
            disabled: false,
        },
        CommandItem {
            label: "Search".to_string(),
            shortcut: None,
            group: Some("Suggestions".to_string()),
            disabled: false,
        },
        CommandItem {
            label: "Settings".to_string(),
            shortcut: None,
            group: Some("Suggestions".to_string()),
            disabled: false,
        },
        CommandItem {
            label: "New File".to_string(),
            shortcut: Some("\u{2318}N".to_string()),
            group: Some("Actions".to_string()),
            disabled: false,
        },
        CommandItem {
            label: "Save".to_string(),
            shortcut: Some("\u{2318}S".to_string()),
            group: Some("Actions".to_string()),
            disabled: false,
        },
        CommandItem {
            label: "Export".to_string(),
            shortcut: None,
            group: Some("Actions".to_string()),
            disabled: false,
        },
    ];

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Command palette trigger" }
                div.mui-showcase__row {
                    (trigger("demo-command", "Open command palette"))
                    span.mui-text-muted style="font-size: 0.875rem;" {
                        "Press "
                        kbd.mui-kbd { "\u{2318}K" }
                    }
                }
            }
            div {
                (render(Props {
                    id: "demo-command".to_string(),
                    items,
                    placeholder: "Type a command or search\u{2026}".to_string(),
                }))
            }
        }
    }
}
