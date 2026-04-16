//! Toggle group component — segmented control with roving tabindex
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct GroupItem {
    pub value: String,
    pub label: String,
    pub pressed: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Size {
    #[default]
    Md,
    Sm,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub items: Vec<GroupItem>,
    pub multiple: bool,
    pub disabled: bool,
    pub aria_label: String,
    pub size: Size,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            items: vec![],
            multiple: false,
            disabled: false,
            aria_label: "Toggle group".to_string(),
            size: Size::Md,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let multiple_attr = if props.multiple { "true" } else { "false" };
    let size_cls = match props.size {
        Size::Md => "mui-toggle-group--md",
        Size::Sm => "mui-toggle-group--sm",
    };

    // Find first focusable item: first pressed, or first item if none pressed
    let first_focusable_idx = props
        .items
        .iter()
        .position(|item| item.pressed)
        .unwrap_or(0);

    html! {
        div class={"mui-toggle-group " (size_cls)}
            role="group"
            aria-label=(props.aria_label)
            data-mui="toggle-group"
            data-multiple=(multiple_attr)
            data-disabled=[props.disabled.then(|| "true")]
        {
            @for (idx, item) in props.items.iter().enumerate() {
                @let tabindex = if idx == first_focusable_idx { "0" } else { "-1" };
                @let aria_pressed = if item.pressed { "true" } else { "false" };

                button type="button" class="mui-toggle-group__item"
                    aria-pressed=(aria_pressed)
                    data-value=(item.value)
                    tabindex=(tabindex)
                    disabled[props.disabled]
                {
                    (item.label.clone())
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Text alignment — single select
            section {
                h2 { "Text Alignment" }
                p.mui-showcase__caption { "Single selection" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "left".into(), label: "Left".into(), pressed: true },
                            GroupItem { value: "center".into(), label: "Center".into(), pressed: false },
                            GroupItem { value: "right".into(), label: "Right".into(), pressed: false },
                            GroupItem { value: "justify".into(), label: "Justify".into(), pressed: false },
                        ],
                        aria_label: "Text alignment".into(),
                        ..Default::default()
                    }))
                }
            }

            // View mode — realistic app context
            section {
                h2 { "View Mode" }
                p.mui-showcase__caption { "Single selection" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "list".into(), label: "List".into(), pressed: false },
                            GroupItem { value: "grid".into(), label: "Grid".into(), pressed: true },
                            GroupItem { value: "kanban".into(), label: "Kanban".into(), pressed: false },
                        ],
                        aria_label: "View mode".into(),
                        ..Default::default()
                    }))
                }
            }

            // Size variants side by side
            section {
                h2 { "Sizes" }
                div.mui-showcase__row {
                    span.mui-showcase__label { "md" }
                    (render(Props {
                        items: vec![
                            GroupItem { value: "a".into(), label: "Day".into(), pressed: true },
                            GroupItem { value: "b".into(), label: "Week".into(), pressed: false },
                            GroupItem { value: "c".into(), label: "Month".into(), pressed: false },
                        ],
                        aria_label: "Time range md".into(),
                        ..Default::default()
                    }))
                }
                div.mui-showcase__row {
                    span.mui-showcase__label { "sm" }
                    (render(Props {
                        items: vec![
                            GroupItem { value: "a".into(), label: "Day".into(), pressed: true },
                            GroupItem { value: "b".into(), label: "Week".into(), pressed: false },
                            GroupItem { value: "c".into(), label: "Month".into(), pressed: false },
                        ],
                        size: Size::Sm,
                        aria_label: "Time range sm".into(),
                        ..Default::default()
                    }))
                }
            }

            // Multiple selection
            section {
                h2 { "Multiple Selection" }
                p.mui-showcase__caption { "Text formatting — multi-select" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "bold".into(), label: "B".into(), pressed: true },
                            GroupItem { value: "italic".into(), label: "I".into(), pressed: false },
                            GroupItem { value: "underline".into(), label: "U".into(), pressed: true },
                        ],
                        multiple: true,
                        aria_label: "Text formatting".into(),
                        ..Default::default()
                    }))
                }
            }

            // Disabled
            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "on".into(), label: "Active".into(), pressed: true },
                            GroupItem { value: "off".into(), label: "Inactive".into(), pressed: false },
                        ],
                        disabled: true,
                        aria_label: "Disabled group".into(),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
