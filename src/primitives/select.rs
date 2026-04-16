//! Select component — maud-ui Wave 3
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub name: String,
    pub id: String,
    pub options: Vec<SelectOption>,
    pub selected: Option<String>,
    pub placeholder: String,
    pub disabled: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: "select".to_string(),
            id: "select".to_string(),
            options: vec![],
            selected: None,
            placeholder: "Select…".to_string(),
            disabled: false,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let selected_label = props
        .selected
        .as_ref()
        .and_then(|sel| {
            props
                .options
                .iter()
                .find(|opt| &opt.value == sel)
                .map(|opt| opt.label.clone())
        })
        .unwrap_or_else(|| props.placeholder.clone());

    let hidden_value = props.selected.clone().unwrap_or_default();

    html! {
        div class="mui-select" data-mui="select" data-name=(props.name.clone()) {
            @if props.disabled {
                button type="button" class="mui-select__trigger" id=(props.id.clone())
                        role="combobox" aria-expanded="false"
                        aria-haspopup="listbox" aria-controls=(format!("{}-listbox", props.id))
                        aria-activedescendant=""
                        disabled {
                    span class="mui-select__value" { (selected_label) }
                    span class="mui-select__chevron" aria-hidden="true" { "▾" }
                }
            } @else {
                button type="button" class="mui-select__trigger" id=(props.id.clone())
                        role="combobox" aria-expanded="false"
                        aria-haspopup="listbox" aria-controls=(format!("{}-listbox", props.id))
                        aria-activedescendant="" {
                    span class="mui-select__value" { (selected_label) }
                    span class="mui-select__chevron" aria-hidden="true" { "▾" }
                }
            }

            div class="mui-select__dropdown" id=(format!("{}-listbox", props.id)) role="listbox"
                    aria-labelledby=(props.id) hidden {
                @for (idx, opt) in props.options.iter().enumerate() {
                    @if opt.disabled {
                        div class=(format!("mui-select__option{}", if props.selected.as_ref() == Some(&opt.value) { " mui-select__option--selected" } else { "" }))
                            role="option" id=(format!("{}-opt-{}", props.id, idx))
                            data-value=(opt.value.clone())
                            aria-selected=(props.selected.as_ref() == Some(&opt.value))
                            aria-disabled="true" {
                            (opt.label.clone())
                        }
                    } @else {
                        div class=(format!("mui-select__option{}", if props.selected.as_ref() == Some(&opt.value) { " mui-select__option--selected" } else { "" }))
                            role="option" id=(format!("{}-opt-{}", props.id, idx))
                            data-value=(opt.value.clone())
                            aria-selected=(props.selected.as_ref() == Some(&opt.value))
                            aria-disabled="false" {
                            (opt.label.clone())
                        }
                    }
                }
            }

            input type="hidden" name=(props.name.clone()) value=(hidden_value) class="mui-select__hidden";
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Basic" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "fruit-1".to_string(),
                        id: "fruit-1".to_string(),
                        options: vec![
                            SelectOption { value: "apple".to_string(), label: "Apple".to_string(), disabled: false },
                            SelectOption { value: "banana".to_string(), label: "Banana".to_string(), disabled: false },
                            SelectOption { value: "cherry".to_string(), label: "Cherry".to_string(), disabled: false },
                            SelectOption { value: "grape".to_string(), label: "Grape".to_string(), disabled: false },
                        ],
                        selected: Some("banana".to_string()),
                        placeholder: "Select…".to_string(),
                        disabled: false,
                    }))
                }
            }

            section {
                h2 { "With placeholder" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "fruit-2".to_string(),
                        id: "fruit-2".to_string(),
                        options: vec![
                            SelectOption { value: "apple".to_string(), label: "Apple".to_string(), disabled: false },
                            SelectOption { value: "banana".to_string(), label: "Banana".to_string(), disabled: false },
                            SelectOption { value: "cherry".to_string(), label: "Cherry".to_string(), disabled: false },
                            SelectOption { value: "grape".to_string(), label: "Grape".to_string(), disabled: false },
                        ],
                        selected: None,
                        placeholder: "Pick a fruit…".to_string(),
                        disabled: false,
                    }))
                }
            }

            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "fruit-3".to_string(),
                        id: "fruit-3".to_string(),
                        options: vec![
                            SelectOption { value: "apple".to_string(), label: "Apple".to_string(), disabled: false },
                            SelectOption { value: "banana".to_string(), label: "Banana".to_string(), disabled: false },
                            SelectOption { value: "cherry".to_string(), label: "Cherry".to_string(), disabled: false },
                            SelectOption { value: "grape".to_string(), label: "Grape".to_string(), disabled: false },
                        ],
                        selected: Some("grape".to_string()),
                        placeholder: "Select…".to_string(),
                        disabled: true,
                    }))
                }
            }

            section {
                h2 { "With disabled option" }
                div.mui-showcase__row {
                    (render(Props {
                        name: "fruit-4".to_string(),
                        id: "fruit-4".to_string(),
                        options: vec![
                            SelectOption { value: "apple".to_string(), label: "Apple".to_string(), disabled: false },
                            SelectOption { value: "banana".to_string(), label: "Banana".to_string(), disabled: false },
                            SelectOption { value: "cherry".to_string(), label: "Cherry".to_string(), disabled: true },
                            SelectOption { value: "grape".to_string(), label: "Grape".to_string(), disabled: false },
                        ],
                        selected: Some("apple".to_string()),
                        placeholder: "Select…".to_string(),
                        disabled: false,
                    }))
                }
            }
        }
    }
}
