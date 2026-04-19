//! Combobox component — maud-ui
//! Extends the Select pattern with text filtering.
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct ComboboxOption {
    pub value: String,
    pub label: String,
}

#[derive(Clone, Debug, Default)]
pub struct ComboboxGroup {
    pub label: String,
    pub options: Vec<ComboboxOption>,
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
    /// Multi-select mode. When true, selected values render as chips in the trigger.
    /// TODO: full JS chip interaction is stubbed — struct + rendered attr only.
    pub multiple: bool,
    /// When true, the first filtered option gets `aria-selected="true"` and
    /// `data-highlighted` so keyboard Enter can commit it immediately.
    pub auto_highlight: bool,
    /// Render a small X button inside the trigger to clear the selection.
    pub show_clear: bool,
    /// Sets `aria-invalid="true"` on the trigger for form-validation styling.
    pub aria_invalid: bool,
    /// Grouped options. When non-empty, takes precedence over `options` and
    /// renders group labels as `.mui-combobox__group-label`.
    pub groups: Vec<ComboboxGroup>,
    /// Selected values for multi-select mode (rendered as chips).
    pub selected_values: Vec<String>,
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
            multiple: false,
            auto_highlight: false,
            show_clear: false,
            aria_invalid: false,
            groups: vec![],
            selected_values: vec![],
        }
    }
}

fn lookup_label<'a>(opts: &'a [ComboboxOption], value: &str) -> Option<&'a str> {
    opts.iter()
        .find(|o| o.value == value)
        .map(|o| o.label.as_str())
}

fn flatten_options(props: &Props) -> Vec<ComboboxOption> {
    if !props.groups.is_empty() {
        let mut out = Vec::new();
        for g in &props.groups {
            for o in &g.options {
                out.push(o.clone());
            }
        }
        out
    } else {
        props.options.clone()
    }
}

fn render_option(opt: &ComboboxOption, selected: bool, highlighted: bool) -> Markup {
    let mut class = String::from("mui-combobox__option");
    if selected {
        class.push_str(" mui-combobox__option--selected");
    }
    if highlighted {
        class.push_str(" mui-combobox__option--highlighted");
    }
    html! {
        @if highlighted {
            div class=(class)
                role="option"
                data-value=(opt.value.clone())
                data-highlighted
                aria-selected=(selected.to_string()) {
                span class="mui-combobox__check" aria-hidden="true" { "\u{2713}" }
                span class="mui-combobox__option-label" { (opt.label.clone()) }
            }
        } @else {
            div class=(class)
                role="option"
                data-value=(opt.value.clone())
                aria-selected=(selected.to_string()) {
                span class="mui-combobox__check" aria-hidden="true" { "\u{2713}" }
                span class="mui-combobox__option-label" { (opt.label.clone()) }
            }
        }
    }
}

pub fn render(props: Props) -> Markup {
    let all_options = flatten_options(&props);

    let selected_label = props
        .selected
        .as_ref()
        .and_then(|sel| lookup_label(&all_options, sel).map(|s| s.to_string()))
        .unwrap_or_else(|| props.placeholder.clone());

    let hidden_value = props.selected.clone().unwrap_or_default();

    // Determine the first-option value to auto-highlight.
    let auto_highlight_value: Option<String> = if props.auto_highlight {
        if !props.groups.is_empty() {
            props
                .groups
                .iter()
                .flat_map(|g| g.options.iter())
                .next()
                .map(|o| o.value.clone())
        } else {
            props.options.first().map(|o| o.value.clone())
        }
    } else {
        None
    };

    let is_selected = |v: &str| -> bool {
        if props.multiple {
            props.selected_values.iter().any(|s| s == v)
        } else {
            props.selected.as_deref() == Some(v)
        }
    };

    html! {
        div class="mui-combobox"
            data-mui="combobox"
            data-multiple[props.multiple]
            data-auto-highlight[props.auto_highlight] {
            @if props.disabled {
                button type="button" class="mui-combobox__trigger"
                        role="combobox" aria-expanded="false"
                        aria-haspopup="listbox"
                        aria-controls=(format!("{}-dropdown", props.id))
                        aria-label=(selected_label.clone())
                        aria-invalid=[props.aria_invalid.then_some("true")]
                        id=(props.id.clone())
                        disabled {
                    @if props.multiple {
                        span class="mui-combobox__chips" {
                            @for v in &props.selected_values {
                                span class="mui-combobox__chip" data-value=(v.clone()) {
                                    span class="mui-combobox__chip-label" {
                                        (lookup_label(&all_options, v).unwrap_or(v.as_str()))
                                    }
                                    span class="mui-combobox__chip-remove" aria-label="Remove" { "\u{00d7}" }
                                }
                            }
                            @if props.selected_values.is_empty() {
                                span class="mui-combobox__value" { (selected_label.clone()) }
                            }
                        }
                    } @else {
                        span class="mui-combobox__value" { (selected_label) }
                    }
                    @if props.show_clear && props.selected.is_some() {
                        span class="mui-combobox__clear" aria-label="Clear" aria-hidden="true" { "\u{00d7}" }
                    }
                    span class="mui-combobox__chevron" aria-hidden="true" { "\u{25be}" }
                }
            } @else {
                button type="button" class="mui-combobox__trigger"
                        role="combobox" aria-expanded="false"
                        aria-haspopup="listbox"
                        aria-controls=(format!("{}-dropdown", props.id))
                        aria-label=(selected_label.clone())
                        aria-invalid=[props.aria_invalid.then_some("true")]
                        id=(props.id.clone()) {
                    @if props.multiple {
                        span class="mui-combobox__chips" {
                            @for v in &props.selected_values {
                                span class="mui-combobox__chip" data-value=(v.clone()) {
                                    span class="mui-combobox__chip-label" {
                                        (lookup_label(&all_options, v).unwrap_or(v.as_str()))
                                    }
                                    button type="button" class="mui-combobox__chip-remove"
                                            aria-label="Remove"
                                            data-value=(v.clone()) { "\u{00d7}" }
                                }
                            }
                            @if props.selected_values.is_empty() {
                                span class="mui-combobox__value" { (selected_label.clone()) }
                            }
                        }
                    } @else {
                        span class="mui-combobox__value" { (selected_label) }
                    }
                    @if props.show_clear && props.selected.is_some() {
                        button type="button" class="mui-combobox__clear" aria-label="Clear" { "\u{00d7}" }
                    }
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
                    @if !props.groups.is_empty() {
                        @for group in &props.groups {
                            div class="mui-combobox__group" role="group" aria-label=(group.label.clone()) {
                                div class="mui-combobox__group-label" { (group.label.clone()) }
                                @for opt in &group.options {
                                    (render_option(
                                        opt,
                                        is_selected(&opt.value),
                                        auto_highlight_value.as_deref() == Some(opt.value.as_str()),
                                    ))
                                }
                            }
                        }
                    } @else {
                        @for opt in &props.options {
                            (render_option(
                                opt,
                                is_selected(&opt.value),
                                auto_highlight_value.as_deref() == Some(opt.value.as_str()),
                            ))
                        }
                    }
                }
                div class="mui-combobox__empty" hidden { (props.empty_text) }
            }

            @if props.multiple {
                @for v in &props.selected_values {
                    input type="hidden" name=(format!("{}[]", props.name)) value=(v.clone()) class="mui-combobox__hidden";
                }
            } @else {
                input type="hidden" name=(props.name.clone()) value=(hidden_value) class="mui-combobox__hidden";
            }
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

    let grouped = vec![
        ComboboxGroup {
            label: "Frontend".to_string(),
            options: vec![
                ComboboxOption {
                    value: "react".to_string(),
                    label: "React".to_string(),
                },
                ComboboxOption {
                    value: "vue".to_string(),
                    label: "Vue".to_string(),
                },
                ComboboxOption {
                    value: "svelte".to_string(),
                    label: "Svelte".to_string(),
                },
            ],
        },
        ComboboxGroup {
            label: "Backend".to_string(),
            options: vec![
                ComboboxOption {
                    value: "rust".to_string(),
                    label: "Rust".to_string(),
                },
                ComboboxOption {
                    value: "go".to_string(),
                    label: "Go".to_string(),
                },
                ComboboxOption {
                    value: "python".to_string(),
                    label: "Python".to_string(),
                },
            ],
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
                            ..Default::default()
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
                            ..Default::default()
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Disabled" }
                        (render(Props {
                            id: "fw-3".to_string(),
                            name: "framework-3".to_string(),
                            options: frameworks.clone(),
                            selected: Some("react".to_string()),
                            placeholder: "Select framework\u{2026}".to_string(),
                            search_placeholder: "Search\u{2026}".to_string(),
                            empty_text: "No framework found.".to_string(),
                            disabled: true,
                            ..Default::default()
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Auto-highlight" }
                        (render(Props {
                            id: "fw-4".to_string(),
                            name: "framework-4".to_string(),
                            options: frameworks.clone(),
                            selected: None,
                            placeholder: "Pick any\u{2026}".to_string(),
                            search_placeholder: "Search\u{2026}".to_string(),
                            empty_text: "No framework found.".to_string(),
                            auto_highlight: true,
                            ..Default::default()
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "With clear button" }
                        (render(Props {
                            id: "fw-5".to_string(),
                            name: "framework-5".to_string(),
                            options: frameworks.clone(),
                            selected: Some("svelte".to_string()),
                            placeholder: "Select\u{2026}".to_string(),
                            search_placeholder: "Search\u{2026}".to_string(),
                            empty_text: "No framework found.".to_string(),
                            show_clear: true,
                            ..Default::default()
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Invalid state" }
                        (render(Props {
                            id: "fw-6".to_string(),
                            name: "framework-6".to_string(),
                            options: frameworks,
                            selected: None,
                            placeholder: "Required\u{2026}".to_string(),
                            search_placeholder: "Search\u{2026}".to_string(),
                            empty_text: "No framework found.".to_string(),
                            aria_invalid: true,
                            ..Default::default()
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
                            ..Default::default()
                        }))
                    }
                    div class="mui-field" {
                        label class="mui-field__label" for="assign-reviewer" { "Reviewer" }
                        (render(Props {
                            id: "assign-reviewer".to_string(),
                            name: "reviewer".to_string(),
                            options: team_members.clone(),
                            selected: None,
                            placeholder: "Assign reviewer\u{2026}".to_string(),
                            search_placeholder: "Search people\u{2026}".to_string(),
                            empty_text: "No team member found.".to_string(),
                            disabled: false,
                            ..Default::default()
                        }))
                        p class="mui-field__description" { "Optional. The reviewer will be notified when the task is ready." }
                    }
                    div class="mui-field" {
                        label class="mui-field__label" for="assign-watchers" { "Watchers (multi)" }
                        (render(Props {
                            id: "assign-watchers".to_string(),
                            name: "watchers".to_string(),
                            options: team_members,
                            selected: None,
                            selected_values: vec!["bob".to_string(), "elena".to_string(), "grace".to_string()],
                            multiple: true,
                            placeholder: "Add watchers\u{2026}".to_string(),
                            search_placeholder: "Search people\u{2026}".to_string(),
                            empty_text: "No team member found.".to_string(),
                            ..Default::default()
                        }))
                    }
                }
            }

            // Third demo: grouped options
            section {
                h2 { "Grouped options" }
                p.mui-showcase__caption { "Options separated into logical groups with uppercase labels." }
                div style="max-width:24rem;" {
                    (render(Props {
                        id: "stack".to_string(),
                        name: "stack".to_string(),
                        groups: grouped,
                        selected: Some("rust".to_string()),
                        placeholder: "Pick a stack\u{2026}".to_string(),
                        search_placeholder: "Search technologies\u{2026}".to_string(),
                        empty_text: "No match.".to_string(),
                        ..Default::default()
                    }))
                }
            }
        }
    }
}
