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
            // Text alignment picker — single select, editor context
            section {
                h2 { "Text alignment" }
                p.mui-showcase__caption { "Paragraph alignment in the document editor." }
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

            // Text formatting — multi select
            section {
                h2 { "Text formatting" }
                p.mui-showcase__caption { "Bold, italic, underline, strikethrough — multi-select." }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "bold".into(), label: "Bold".into(), pressed: true },
                            GroupItem { value: "italic".into(), label: "Italic".into(), pressed: false },
                            GroupItem { value: "underline".into(), label: "Underline".into(), pressed: true },
                            GroupItem { value: "strike".into(), label: "Strike".into(), pressed: false },
                        ],
                        multiple: true,
                        aria_label: "Text formatting".into(),
                        ..Default::default()
                    }))
                }
            }

            // View mode — calendar picker
            section {
                h2 { "Calendar view" }
                p.mui-showcase__caption { "Switch between Day, Week, and Month layouts." }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem { value: "day".into(), label: "Day".into(), pressed: false },
                            GroupItem { value: "week".into(), label: "Week".into(), pressed: true },
                            GroupItem { value: "month".into(), label: "Month".into(), pressed: false },
                        ],
                        aria_label: "Calendar view".into(),
                        ..Default::default()
                    }))
                }
                div.mui-showcase__row style="margin-top:0.5rem;" {
                    span.mui-showcase__label { "Compact" }
                    (render(Props {
                        items: vec![
                            GroupItem { value: "day".into(), label: "Day".into(), pressed: false },
                            GroupItem { value: "week".into(), label: "Week".into(), pressed: true },
                            GroupItem { value: "month".into(), label: "Month".into(), pressed: false },
                        ],
                        size: Size::Sm,
                        aria_label: "Calendar view compact".into(),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
