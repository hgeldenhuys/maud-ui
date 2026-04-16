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
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Country" }
                div class="mui-field" {
                    label class="mui-label" for="country-select" { "Country" }
                    (render(NativeSelectProps {
                        name: "country".to_string(),
                        id: "country-select".to_string(),
                        options: vec![
                            NativeOption { value: "us".to_string(), label: "United States".to_string(), disabled: false },
                            NativeOption { value: "gb".to_string(), label: "United Kingdom".to_string(), disabled: false },
                            NativeOption { value: "ca".to_string(), label: "Canada".to_string(), disabled: false },
                            NativeOption { value: "de".to_string(), label: "Germany".to_string(), disabled: false },
                            NativeOption { value: "fr".to_string(), label: "France".to_string(), disabled: false },
                            NativeOption { value: "jp".to_string(), label: "Japan".to_string(), disabled: false },
                            NativeOption { value: "au".to_string(), label: "Australia".to_string(), disabled: false },
                            NativeOption { value: "za".to_string(), label: "South Africa".to_string(), disabled: false },
                        ],
                        selected: None,
                        disabled: false,
                        placeholder: Some("Select a country\u{2026}".to_string()),
                    }))
                }
            }

            div {
                p.mui-showcase__caption { "Currency" }
                div class="mui-field" {
                    label class="mui-label" for="currency-select" { "Currency" }
                    (render(NativeSelectProps {
                        name: "currency".to_string(),
                        id: "currency-select".to_string(),
                        options: vec![
                            NativeOption { value: "USD".to_string(), label: "USD \u{2014} US Dollar".to_string(), disabled: false },
                            NativeOption { value: "EUR".to_string(), label: "EUR \u{2014} Euro".to_string(), disabled: false },
                            NativeOption { value: "GBP".to_string(), label: "GBP \u{2014} British Pound".to_string(), disabled: false },
                            NativeOption { value: "JPY".to_string(), label: "JPY \u{2014} Japanese Yen".to_string(), disabled: false },
                            NativeOption { value: "ZAR".to_string(), label: "ZAR \u{2014} South African Rand".to_string(), disabled: false },
                        ],
                        selected: Some("USD".to_string()),
                        disabled: false,
                        placeholder: None,
                    }))
                }
            }

            div {
                p.mui-showcase__caption { "Disabled" }
                div class="mui-field" {
                    label class="mui-label mui-label--disabled" for="timezone-select" { "Timezone" }
                    (render(NativeSelectProps {
                        name: "timezone".to_string(),
                        id: "timezone-select".to_string(),
                        options: vec![
                            NativeOption { value: "utc".to_string(), label: "UTC +00:00".to_string(), disabled: false },
                            NativeOption { value: "est".to_string(), label: "EST -05:00".to_string(), disabled: false },
                            NativeOption { value: "pst".to_string(), label: "PST -08:00".to_string(), disabled: false },
                        ],
                        selected: Some("utc".to_string()),
                        disabled: true,
                        placeholder: None,
                    }))
                }
            }
        }
    }
}
