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
                        aria-label=(selected_label.clone())
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
                        aria-label=(selected_label.clone())
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
        ComboboxOption {
            value: "react".to_string(),
            label: "React".to_string(),
        },
        ComboboxOption {
            value: "vue".to_string(),
            label: "Vue".to_string(),
        },
        ComboboxOption {
            value: "angular".to_string(),
            label: "Angular".to_string(),
        },
        ComboboxOption {
            value: "svelte".to_string(),
            label: "Svelte".to_string(),
        },
        ComboboxOption {
            value: "nextjs".to_string(),
            label: "Next.js".to_string(),
        },
        ComboboxOption {
            value: "nuxt".to_string(),
            label: "Nuxt".to_string(),
        },
        ComboboxOption {
            value: "sveltekit".to_string(),
            label: "SvelteKit".to_string(),
        },
        ComboboxOption {
            value: "remix".to_string(),
            label: "Remix".to_string(),
        },
        ComboboxOption {
            value: "astro".to_string(),
            label: "Astro".to_string(),
        },
        ComboboxOption {
            value: "solid".to_string(),
            label: "Solid".to_string(),
        },
    ];

    let team_members = vec![
        ComboboxOption {
            value: "alice".to_string(),
            label: "Alice Johnson".to_string(),
        },
        ComboboxOption {
            value: "bob".to_string(),
            label: "Bob Smith".to_string(),
        },
        ComboboxOption {
            value: "carol".to_string(),
            label: "Carol Williams".to_string(),
        },
        ComboboxOption {
            value: "david".to_string(),
            label: "David Brown".to_string(),
        },
        ComboboxOption {
            value: "elena".to_string(),
            label: "Elena Garcia".to_string(),
        },
        ComboboxOption {
            value: "frank".to_string(),
            label: "Frank Miller".to_string(),
        },
        ComboboxOption {
            value: "grace".to_string(),
            label: "Grace Lee".to_string(),
        },
        ComboboxOption {
            value: "henry".to_string(),
            label: "Henry Chen".to_string(),
        },
    ];

    html! {
        div.mui-showcase__grid {
            // Primary demo: framework picker
            section {
                h2 { "Framework picker" }
                p.mui-showcase__caption { "Search and select from a list of frontend frameworks." }
                div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(14rem,1fr));gap:1.5rem;" {
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Pre-selected" }
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
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Placeholder" }
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
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Disabled" }
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

            // Second demo: assign team member (realistic search-to-select)
            section {
                h2 { "Assign team member" }
                p.mui-showcase__caption { "A realistic search-to-select pattern for assigning people to a task." }
                div style="display:flex;flex-direction:column;gap:1rem;max-width:24rem;" {
                    div class="mui-field" {
                        label class="mui-field__label" for="assign-lead" { "Lead" }
                        (render(Props {
                            id: "assign-lead".to_string(),
                            name: "lead".to_string(),
                            options: team_members.clone(),
                            selected: Some("alice".to_string()),
                            placeholder: "Assign lead\u{2026}".to_string(),
                            search_placeholder: "Search people\u{2026}".to_string(),
                            empty_text: "No team member found.".to_string(),
                            disabled: false,
                        }))
                    }
                    div class="mui-field" {
                        label class="mui-field__label" for="assign-reviewer" { "Reviewer" }
                        (render(Props {
                            id: "assign-reviewer".to_string(),
                            name: "reviewer".to_string(),
                            options: team_members,
                            selected: None,
                            placeholder: "Assign reviewer\u{2026}".to_string(),
                            search_placeholder: "Search people\u{2026}".to_string(),
                            empty_text: "No team member found.".to_string(),
                            disabled: false,
                        }))
                        p class="mui-field__description" { "Optional. The reviewer will be notified when the task is ready." }
                    }
                }
            }
        }
    }
}
