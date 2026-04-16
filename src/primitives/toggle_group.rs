//! Toggle group component — maud-ui Wave 2
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct GroupItem {
    pub value: String,
    pub label: String,
    pub pressed: bool,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub items: Vec<GroupItem>,
    pub multiple: bool,
    pub disabled: bool,
    pub aria_label: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            items: vec![],
            multiple: false,
            disabled: false,
            aria_label: "Toggle group".to_string(),
        }
    }
}

pub fn render(props: Props) -> Markup {
    let multiple_attr = if props.multiple { "true" } else { "false" };

    // Find first focusable item: first pressed, or first item if none pressed
    let first_focusable_idx = props
        .items
        .iter()
        .position(|item| item.pressed)
        .unwrap_or(0);

    html! {
        @if props.disabled {
            div class="mui-toggle-group" role="group" aria-label=(props.aria_label.clone())
                data-mui="toggle-group"
                data-multiple=(multiple_attr)
                data-disabled="true" {

                @for (idx, item) in props.items.iter().enumerate() {
                    @let tabindex = if idx == first_focusable_idx { "0" } else { "-1" };
                    @let aria_pressed = if item.pressed { "true" } else { "false" };

                    button type="button" class="mui-toggle-group__item"
                        aria-pressed=(aria_pressed)
                        data-value=(item.value.clone())
                        tabindex=(tabindex)
                        disabled {
                        (item.label.clone())
                    }
                }
            }
        } @else {
            div class="mui-toggle-group" role="group" aria-label=(props.aria_label.clone())
                data-mui="toggle-group"
                data-multiple=(multiple_attr) {

                @for (idx, item) in props.items.iter().enumerate() {
                    @let tabindex = if idx == first_focusable_idx { "0" } else { "-1" };
                    @let aria_pressed = if item.pressed { "true" } else { "false" };

                    button type="button" class="mui-toggle-group__item"
                        aria-pressed=(aria_pressed)
                        data-value=(item.value.clone())
                        tabindex=(tabindex) {
                        (item.label.clone())
                    }
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
                            GroupItem {
                                value: "left".to_string(),
                                label: "Left".to_string(),
                                pressed: true,
                            },
                            GroupItem {
                                value: "center".to_string(),
                                label: "Center".to_string(),
                                pressed: false,
                            },
                            GroupItem {
                                value: "right".to_string(),
                                label: "Right".to_string(),
                                pressed: false,
                            },
                            GroupItem {
                                value: "justify".to_string(),
                                label: "Justify".to_string(),
                                pressed: false,
                            },
                        ],
                        multiple: false,
                        disabled: false,
                        aria_label: "Text alignment".to_string(),
                    }))
                }
            }
            section {
                h2 { "Text formatting (multiple)" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem {
                                value: "bold".to_string(),
                                label: "Bold".to_string(),
                                pressed: true,
                            },
                            GroupItem {
                                value: "italic".to_string(),
                                label: "Italic".to_string(),
                                pressed: false,
                            },
                            GroupItem {
                                value: "underline".to_string(),
                                label: "Underline".to_string(),
                                pressed: true,
                            },
                        ],
                        multiple: true,
                        disabled: false,
                        aria_label: "Text formatting".to_string(),
                    }))
                }
            }
            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        items: vec![
                            GroupItem {
                                value: "opt1".to_string(),
                                label: "Option 1".to_string(),
                                pressed: true,
                            },
                            GroupItem {
                                value: "opt2".to_string(),
                                label: "Option 2".to_string(),
                                pressed: false,
                            },
                            GroupItem {
                                value: "opt3".to_string(),
                                label: "Option 3".to_string(),
                                pressed: false,
                            },
                        ],
                        multiple: false,
                        disabled: true,
                        aria_label: "Disabled group".to_string(),
                    }))
                }
            }
        }
    }
}
