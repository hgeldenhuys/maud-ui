//! Combobox component — maud-ui
//! Extends the Select pattern with text filtering.
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub options: Vec<ComboboxOption>,
    pub selected: Option<String>,
    pub placeholder: String,
    pub search_placeholder: String,
    pub empty_text: String,
    pub disabled: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "combobox".to_string(),
            name: "combobox".to_string(),
            options: vec![],
            selected: None,
            placeholder: "Select\u{2026}".to_string(),
            search_placeholder: "Search\u{2026}".to_string(),
            empty_text: "No results found.".to_string(),
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
        div class="mui-combobox" data-mui="combobox" {
            @if props.disabled {
                button type="button" class="mui-combobox__trigger"
                        role="combobox" aria-expanded="false"
                        aria-haspopup="listbox"
                        aria-controls=(format!("{}-dropdown", props.id))
                        id=(props.id.clone())
                        disabled {
                    span class="mui-combobox__value" { (selected_label) }
                    span class="mui-combobox__chevron" aria-hidden="true" { "\u{25be}" }
                }
            } @else {
                button type="button" class="mui-combobox__trigger"
                        role="combobox" aria-expanded="false"
                        aria-haspopup="listbox"
                        aria-controls=(format!("{}-dropdown", props.id))
                        id=(props.id.clone()) {
                    span class="mui-combobox__value" { (selected_label) }
                    span class="mui-combobox__chevron" aria-hidden="true" { "\u{25be}" }
                }
            }

            div class="mui-combobox__dropdown" id=(format!("{}-dropdown", props.id)) hidden {
                div class="mui-combobox__search-wrap" {
                    span class="mui-combobox__search-icon" aria-hidden="true" { "\u{1f50d}" }
                    input type="text" class="mui-combobox__search"
                          placeholder=(props.search_placeholder)
                          aria-label="Search options"
                          autocomplete="off";
                }
                div class="mui-combobox__list" role="listbox" {
                    @for opt in &props.options {
                        div class=(format!("mui-combobox__option{}", if props.selected.as_ref() == Some(&opt.value) { " mui-combobox__option--selected" } else { "" }))
                            role="option"
                            data-value=(opt.value.clone())
                            aria-selected=(props.selected.as_ref() == Some(&opt.value)) {
                            span class="mui-combobox__check" aria-hidden="true" { "\u{2713}" }
                            span class="mui-combobox__option-label" { (opt.label.clone()) }
                        }
                    }
                }
                div class="mui-combobox__empty" hidden { (props.empty_text) }
            }

            input type="hidden" name=(props.name.clone()) value=(hidden_value) class="mui-combobox__hidden";
        }
    }
}

pub fn showcase() -> Markup {
    let frameworks = vec![
        ComboboxOption { value: "react".to_string(), label: "React".to_string() },
        ComboboxOption { value: "vue".to_string(), label: "Vue".to_string() },
        ComboboxOption { value: "angular".to_string(), label: "Angular".to_string() },
        ComboboxOption { value: "svelte".to_string(), label: "Svelte".to_string() },
        ComboboxOption { value: "nextjs".to_string(), label: "Next.js".to_string() },
        ComboboxOption { value: "nuxt".to_string(), label: "Nuxt".to_string() },
        ComboboxOption { value: "sveltekit".to_string(), label: "SvelteKit".to_string() },
        ComboboxOption { value: "remix".to_string(), label: "Remix".to_string() },
        ComboboxOption { value: "astro".to_string(), label: "Astro".to_string() },
        ComboboxOption { value: "solid".to_string(), label: "Solid".to_string() },
    ];

    html! {
        div.mui-showcase__grid {
            section {
                h2 { "With selection" }
                div.mui-showcase__row {
                    (render(Props {
                        id: "fw-1".to_string(),
                        name: "framework-1".to_string(),
                        options: frameworks.clone(),
                        selected: Some("vue".to_string()),
                        placeholder: "Select framework\u{2026}".to_string(),
                        search_placeholder: "Search\u{2026}".to_string(),
                        empty_text: "No framework found.".to_string(),
                        disabled: false,
                    }))
                }
            }

            section {
                h2 { "With placeholder" }
                div.mui-showcase__row {
                    (render(Props {
                        id: "fw-2".to_string(),
                        name: "framework-2".to_string(),
                        options: frameworks.clone(),
                        selected: None,
                        placeholder: "Select framework\u{2026}".to_string(),
                        search_placeholder: "Search frameworks\u{2026}".to_string(),
                        empty_text: "No framework found.".to_string(),
                        disabled: false,
                    }))
                }
            }

            section {
                h2 { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        id: "fw-3".to_string(),
                        name: "framework-3".to_string(),
                        options: frameworks,
                        selected: Some("react".to_string()),
                        placeholder: "Select framework\u{2026}".to_string(),
                        search_placeholder: "Search\u{2026}".to_string(),
                        empty_text: "No framework found.".to_string(),
                        disabled: true,
                    }))
                }
            }
        }
    }
}
