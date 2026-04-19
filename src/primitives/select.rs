//! Select component — maud-ui Wave 3
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub enum Size {
    #[default]
    Default,
    Sm,
}

#[derive(Clone, Debug)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

#[derive(Clone, Debug, Default)]
pub struct SelectGroup {
    pub label: String,
    pub options: Vec<SelectOption>,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub name: String,
    pub id: String,
    pub options: Vec<SelectOption>,
    pub groups: Vec<SelectGroup>,
    pub selected: Option<String>,
    pub placeholder: String,
    pub disabled: bool,
    pub size: Size,
    pub aria_invalid: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: "select".to_string(),
            id: "select".to_string(),
            options: vec![],
            groups: vec![],
            selected: None,
            placeholder: "Select…".to_string(),
            disabled: false,
            size: Size::Default,
            aria_invalid: false,
        }
    }
}

/// SelectScrollUpButton — renders the up-scroll affordance for long dropdowns.
pub fn scroll_up_button() -> Markup {
    html! {
        div class="mui-select__scroll-up" role="presentation" aria-hidden="true" { "▲" }
    }
}

/// SelectScrollDownButton — renders the down-scroll affordance for long dropdowns.
pub fn scroll_down_button() -> Markup {
    html! {
        div class="mui-select__scroll-down" role="presentation" aria-hidden="true" { "▼" }
    }
}

/// SelectSeparator — thin horizontal rule between option groups.
pub fn separator() -> Markup {
    html! {
        div class="mui-select__separator" role="separator" {}
    }
}

fn render_option(
    opt: &SelectOption,
    idx: usize,
    id_prefix: &str,
    selected: Option<&String>,
) -> Markup {
    let is_selected = selected == Some(&opt.value);
    let class = if is_selected {
        "mui-select__option mui-select__option--selected"
    } else {
        "mui-select__option"
    };
    html! {
        @if opt.disabled {
            div class=(class)
                role="option" id=(format!("{}-opt-{}", id_prefix, idx))
                data-value=(opt.value.clone())
                aria-selected=(is_selected)
                aria-disabled="true" {
                span class="mui-select__check" aria-hidden="true" { "\u{2713}" }
                span class="mui-select__option-label" { (opt.label.clone()) }
            }
        } @else {
            div class=(class)
                role="option" id=(format!("{}-opt-{}", id_prefix, idx))
                data-value=(opt.value.clone())
                aria-selected=(is_selected) {
                span class="mui-select__check" aria-hidden="true" { "\u{2713}" }
                span class="mui-select__option-label" { (opt.label.clone()) }
            }
        }
    }
}

pub fn render(props: Props) -> Markup {
    // Flatten candidate list for selected-label resolution (covers both groups and flat options).
    let selected_label = props
        .selected
        .as_ref()
        .and_then(|sel| {
            let mut label: Option<String> = None;
            if !props.groups.is_empty() {
                for group in &props.groups {
                    for opt in &group.options {
                        if &opt.value == sel {
                            label = Some(opt.label.clone());
                            break;
                        }
                    }
                    if label.is_some() {
                        break;
                    }
                }
            }
            if label.is_none() {
                for opt in &props.options {
                    if &opt.value == sel {
                        label = Some(opt.label.clone());
                        break;
                    }
                }
            }
            label
        })
        .unwrap_or_else(|| props.placeholder.clone());

    let hidden_value = props.selected.clone().unwrap_or_default();

    let trigger_class = match props.size {
        Size::Default => "mui-select__trigger",
        Size::Sm => "mui-select__trigger mui-select--sm",
    };

    let aria_invalid_value: Option<&'static str> = if props.aria_invalid {
        Some("true")
    } else {
        None
    };

    html! {
        div class="mui-select" data-mui="select" data-name=(props.name.clone()) {
            @if props.disabled {
                button type="button" class=(trigger_class) id=(props.id.clone())
                        role="combobox" aria-expanded="false"
                        aria-haspopup="listbox" aria-controls=(format!("{}-listbox", props.id))
                        aria-activedescendant=""
                        aria-label=(selected_label.clone())
                        aria-invalid=[aria_invalid_value]
                        disabled {
                    span class="mui-select__value" { (selected_label) }
                    span class="mui-select__chevron" aria-hidden="true" { "▾" }
                }
            } @else {
                button type="button" class=(trigger_class) id=(props.id.clone())
                        role="combobox" aria-expanded="false"
                        aria-haspopup="listbox" aria-controls=(format!("{}-listbox", props.id))
                        aria-activedescendant=""
                        aria-label=(selected_label.clone())
                        aria-invalid=[aria_invalid_value] {
                    span class="mui-select__value" { (selected_label) }
                    span class="mui-select__chevron" aria-hidden="true" { "▾" }
                }
            }

            div class="mui-select__dropdown" id=(format!("{}-listbox", props.id)) role="listbox"
                    aria-labelledby=(props.id) hidden {
                @if !props.groups.is_empty() {
                    @for (g_idx, group) in props.groups.iter().enumerate() {
                        div role="group" aria-labelledby=(format!("{}-group-{}-label", props.id, g_idx)) {
                            div class="mui-select__group-label" role="presentation"
                                id=(format!("{}-group-{}-label", props.id, g_idx)) {
                                (group.label.clone())
                            }
                            @for (idx, opt) in group.options.iter().enumerate() {
                                (render_option(opt, idx, &format!("{}-g{}", props.id, g_idx), props.selected.as_ref()))
                            }
                        }
                    }
                } @else {
                    @for (idx, opt) in props.options.iter().enumerate() {
                        (render_option(opt, idx, &props.id, props.selected.as_ref()))
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
                            ..Default::default()
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
                            placeholder: "Select language\u{2026}".to_string(),
                            ..Default::default()
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
                            ..Default::default()
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
                            ..Default::default()
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
                            placeholder: "Set priority\u{2026}".to_string(),
                            ..Default::default()
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
                            ..Default::default()
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
                            ..Default::default()
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Small size" }
                        (render(Props {
                            name: "density".to_string(),
                            id: "anatomy-size-sm".to_string(),
                            options: vec![
                                SelectOption { value: "compact".to_string(), label: "Compact".to_string(), disabled: false },
                                SelectOption { value: "normal".to_string(), label: "Normal".to_string(), disabled: false },
                                SelectOption { value: "roomy".to_string(), label: "Roomy".to_string(), disabled: false },
                            ],
                            selected: Some("compact".to_string()),
                            placeholder: "Density\u{2026}".to_string(),
                            size: Size::Sm,
                            ..Default::default()
                        }))
                    }
                    div {
                        h3 style="font-size:0.875rem;margin-bottom:0.5rem;" { "Invalid state" }
                        (render(Props {
                            name: "country".to_string(),
                            id: "anatomy-invalid".to_string(),
                            options: vec![
                                SelectOption { value: "us".to_string(), label: "United States".to_string(), disabled: false },
                                SelectOption { value: "ca".to_string(), label: "Canada".to_string(), disabled: false },
                                SelectOption { value: "mx".to_string(), label: "Mexico".to_string(), disabled: false },
                            ],
                            placeholder: "Select country\u{2026}".to_string(),
                            aria_invalid: true,
                            ..Default::default()
                        }))
                    }
                }
            }

            // Grouped options demo
            section {
                h2 { "Grouped options" }
                p.mui-showcase__caption { "Options organized under labeled groups with separator helper." }
                div style="display:flex;flex-direction:column;gap:1rem;max-width:24rem;" {
                    div class="mui-field" {
                        label class="mui-field__label" for="grouped-timezone" { "Timezone" }
                        (render(Props {
                            name: "timezone_grouped".to_string(),
                            id: "grouped-timezone".to_string(),
                            groups: vec![
                                SelectGroup {
                                    label: "North America".to_string(),
                                    options: vec![
                                        SelectOption { value: "est".to_string(), label: "Eastern".to_string(), disabled: false },
                                        SelectOption { value: "cst".to_string(), label: "Central".to_string(), disabled: false },
                                        SelectOption { value: "pst".to_string(), label: "Pacific".to_string(), disabled: false },
                                    ],
                                },
                                SelectGroup {
                                    label: "Europe".to_string(),
                                    options: vec![
                                        SelectOption { value: "gmt".to_string(), label: "London (GMT)".to_string(), disabled: false },
                                        SelectOption { value: "cet".to_string(), label: "Berlin (CET)".to_string(), disabled: false },
                                    ],
                                },
                                SelectGroup {
                                    label: "Asia".to_string(),
                                    options: vec![
                                        SelectOption { value: "jst".to_string(), label: "Tokyo (JST)".to_string(), disabled: false },
                                        SelectOption { value: "sgt".to_string(), label: "Singapore (SGT)".to_string(), disabled: false },
                                    ],
                                },
                            ],
                            selected: Some("est".to_string()),
                            placeholder: "Select timezone\u{2026}".to_string(),
                            ..Default::default()
                        }))
                    }
                }
            }
        }
    }
}
