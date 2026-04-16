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
            section {
                h2 { "Text alignment (single)" }
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
            section {
                h2 { "Text formatting (multiple)" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "bold".into(), label: "Bold".into(), pressed: true },
                            GroupItem { value: "italic".into(), label: "Italic".into(), pressed: false },
                            GroupItem { value: "underline".into(), label: "Underline".into(), pressed: true },
                        ],
                        multiple: true,
                        aria_label: "Text formatting".into(),
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Small size" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "a".into(), label: "A".into(), pressed: true },
                            GroupItem { value: "b".into(), label: "B".into(), pressed: false },
                            GroupItem { value: "c".into(), label: "C".into(), pressed: false },
                        ],
                        size: Size::Sm,
                        aria_label: "Small group".into(),
                        ..Default::default()
                    }))
                }
            }
            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "opt1".into(), label: "Option 1".into(), pressed: true },
                            GroupItem { value: "opt2".into(), label: "Option 2".into(), pressed: false },
                            GroupItem { value: "opt3".into(), label: "Option 3".into(), pressed: false },
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
