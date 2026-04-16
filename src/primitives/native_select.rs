//! NativeSelect component — styled native `<select>` matching Input appearance.
use maud::{html, Markup, PreEscaped};

pub struct NativeOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

pub struct NativeSelectProps {
    pub name: String,
    pub id: String,
    pub options: Vec<NativeOption>,
    pub selected: Option<String>,
    pub disabled: bool,
    pub placeholder: Option<String>,
}

/// SVG chevron-down (lucide icon, 15x15)
const CHEVRON_DOWN: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m6 9 6 6 6-6"/></svg>"#;

pub fn render(props: NativeSelectProps) -> Markup {
    html! {
        div.mui-native-select {
            select.mui-native-select__select
                name=(props.name)
                id=(props.id)
                disabled[props.disabled]
            {
                @if let Some(placeholder) = props.placeholder {
                    option value="" disabled selected hidden { (placeholder) }
                }
                @for option in props.options {
                    option
                        value=(option.value.clone())
                        selected[props.selected.as_ref() == Some(&option.value)]
                        disabled[option.disabled]
                    {
                        (option.label)
                    }
                }
            }
            span.mui-native-select__chevron aria-hidden="true" { (PreEscaped(CHEVRON_DOWN)) }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div {
            h3 { "Basic native select with fruits" }
            (render(NativeSelectProps {
                name: "fruit".to_string(),
                id: "fruit-select".to_string(),
                options: vec![
                    NativeOption {
                        value: "apple".to_string(),
                        label: "Apple".to_string(),
                        disabled: false,
                    },
                    NativeOption {
                        value: "banana".to_string(),
                        label: "Banana".to_string(),
                        disabled: false,
                    },
                    NativeOption {
                        value: "cherry".to_string(),
                        label: "Cherry".to_string(),
                        disabled: false,
                    },
                ],
                selected: None,
                disabled: false,
                placeholder: None,
            }))

            h3 { "With placeholder" }
            (render(NativeSelectProps {
                name: "country".to_string(),
                id: "country-select".to_string(),
                options: vec![
                    NativeOption {
                        value: "us".to_string(),
                        label: "United States".to_string(),
                        disabled: false,
                    },
                    NativeOption {
                        value: "uk".to_string(),
                        label: "United Kingdom".to_string(),
                        disabled: false,
                    },
                    NativeOption {
                        value: "ca".to_string(),
                        label: "Canada".to_string(),
                        disabled: false,
                    },
                ],
                selected: None,
                disabled: false,
                placeholder: Some("Select a country...".to_string()),
            }))

            h3 { "Disabled" }
            (render(NativeSelectProps {
                name: "disabled-select".to_string(),
                id: "disabled-select".to_string(),
                options: vec![
                    NativeOption {
                        value: "opt1".to_string(),
                        label: "Option 1".to_string(),
                        disabled: false,
                    },
                    NativeOption {
                        value: "opt2".to_string(),
                        label: "Option 2".to_string(),
                        disabled: false,
                    },
                ],
                selected: Some("opt1".to_string()),
                disabled: true,
                placeholder: None,
            }))
        }
    }
}
