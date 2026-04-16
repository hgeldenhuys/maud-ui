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
                            span class="mui-select__check" aria-hidden="true" { "\u{2713}" }
                            span class="mui-select__option-label" { (opt.label.clone()) }
                        }
                    } @else {
                        div class=(format!("mui-select__option{}", if props.selected.as_ref() == Some(&opt.value) { " mui-select__option--selected" } else { "" }))
                            role="option" id=(format!("{}-opt-{}", props.id, idx))
                            data-value=(opt.value.clone())
                            aria-selected=(props.selected.as_ref() == Some(&opt.value))
                            aria-disabled="false" {
                            span class="mui-select__check" aria-hidden="true" { "\u{2713}" }
                            span class="mui-select__option-label" { (opt.label.clone()) }
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
            // Primary demo: realistic Preferences form
            section {
                h2 { "Preferences" }
                p.mui-showcase__caption { "A realistic settings form with theme, language, and a locked timezone field." }
                div style="display:flex;flex-direction:column;gap:1rem;max-width:24rem;" {
                    div class="mui-field" {
                        label class="mui-field__label" for="pref-theme" { "Theme" }
                        (render(Props {
                            name: "theme".to_string(),
                            id: "pref-theme".to_string(),
                            options: vec![
                                SelectOption { value: "light".to_string(), label: "Light".to_string(), disabled: false },
                                SelectOption { value: "dark".to_string(), label: "Dark".to_string(), disabled: false },
                                SelectOption { value: "system".to_string(), label: "System".to_string(), disabled: false },
                            ],
                            selected: Some("system".to_string()),
                            placeholder: "Choose theme\u{2026}".to_string(),
                            disabled: false,
                        }))
                        p class="mui-field__description" { "Controls the application appearance." }
                    }
                    div class="mui-field" {
                        label class="mui-field__label" for="pref-language" { "Language" }
                        (render(Props {
                            name: "language".to_string(),
                            id: "pref-language".to_string(),
                            options: vec![
                                SelectOption { value: "en".to_string(), label: "English".to_string(), disabled: false },
                                SelectOption { value: "es".to_string(), label: "Spanish".to_string(), disabled: false },
                                SelectOption { value: "fr".to_string(), label: "French".to_string(), disabled: false },
                                SelectOption { value: "de".to_string(), label: "German".to_string(), disabled: false },
                                SelectOption { value: "ja".to_string(), label: "Japanese".to_string(), disabled: false },
                            ],
                            selected: None,
                            placeholder: "Select language\u{2026}".to_string(),
                            disabled: false,
                        }))
                    }
                    div class="mui-field" {
                        label class="mui-field__label mui-label--disabled" for="pref-timezone" {
                            "Timezone"
                            span style="font-weight:400;color:var(--mui-muted-foreground);margin-left:0.5rem;font-size:0.75rem;" { "(locked by admin)" }
                        }
                        (render(Props {
                            name: "timezone".to_string(),
                            id: "pref-timezone".to_string(),
                            options: vec![
                                SelectOption { value: "utc".to_string(), label: "UTC".to_string(), disabled: false },
                                SelectOption { value: "est".to_string(), label: "US Eastern (EST)".to_string(), disabled: false },
                                SelectOption { value: "pst".to_string(), label: "US Pacific (PST)".to_string(), disabled: false },
                                SelectOption { value: "cet".to_string(), label: "Central European (CET)".to_string(), disabled: false },
                                SelectOption { value: "jst".to_string(), label: "Japan Standard (JST)".to_string(), disabled: false },
                            ],
                            selected: Some("utc".to_string()),
                            placeholder: "Select timezone\u{2026}".to_string(),
                            disabled: true,
                        }))
                    }
                }
            }

            // Anatomy: individual select features
            section {
                h2 { "Select Anatomy" }
                p.mui-showcase__caption { "Individual select states shown in isolation." }
                div style="display:grid;grid-template-columns:repeat(auto-fill,minmax(14rem,1fr));gap:1.5rem;" {
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Pre-selected" }
                        (render(Props {
                            name: "status".to_string(),
                            id: "anatomy-preselected".to_string(),
                            options: vec![
                                SelectOption { value: "active".to_string(), label: "Active".to_string(), disabled: false },
                                SelectOption { value: "paused".to_string(), label: "Paused".to_string(), disabled: false },
                                SelectOption { value: "archived".to_string(), label: "Archived".to_string(), disabled: false },
                            ],
                            selected: Some("active".to_string()),
                            placeholder: "Select status\u{2026}".to_string(),
                            disabled: false,
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Placeholder" }
                        (render(Props {
                            name: "priority".to_string(),
                            id: "anatomy-placeholder".to_string(),
                            options: vec![
                                SelectOption { value: "low".to_string(), label: "Low".to_string(), disabled: false },
                                SelectOption { value: "medium".to_string(), label: "Medium".to_string(), disabled: false },
                                SelectOption { value: "high".to_string(), label: "High".to_string(), disabled: false },
                                SelectOption { value: "critical".to_string(), label: "Critical".to_string(), disabled: false },
                            ],
                            selected: None,
                            placeholder: "Set priority\u{2026}".to_string(),
                            disabled: false,
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Disabled option" }
                        (render(Props {
                            name: "plan".to_string(),
                            id: "anatomy-disabled-opt".to_string(),
                            options: vec![
                                SelectOption { value: "free".to_string(), label: "Free".to_string(), disabled: false },
                                SelectOption { value: "pro".to_string(), label: "Pro".to_string(), disabled: false },
                                SelectOption { value: "enterprise".to_string(), label: "Enterprise (contact sales)".to_string(), disabled: true },
                            ],
                            selected: Some("free".to_string()),
                            placeholder: "Choose plan\u{2026}".to_string(),
                            disabled: false,
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Fully disabled" }
                        (render(Props {
                            name: "role".to_string(),
                            id: "anatomy-disabled".to_string(),
                            options: vec![
                                SelectOption { value: "viewer".to_string(), label: "Viewer".to_string(), disabled: false },
                                SelectOption { value: "editor".to_string(), label: "Editor".to_string(), disabled: false },
                                SelectOption { value: "admin".to_string(), label: "Admin".to_string(), disabled: false },
                            ],
                            selected: Some("editor".to_string()),
                            placeholder: "Select role\u{2026}".to_string(),
                            disabled: true,
                        }))
                    }
                }
            }
        }
    }
}
