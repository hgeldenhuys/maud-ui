//! Live component gallery. Serves `showcase_page()` as the `showcase` axum example.
//! Each component also has its own route via `component_page_by_name()`.
//! Components are grouped into tiers: Form Controls, Display, Layout, Overlay,
//! Navigation, and Composite.

use maud::{html, Markup, DOCTYPE};

use crate::primitives;

// ── Tier definitions ───────────────────────────────────────────────────

struct Tier {
    slug: &'static str,
    title: &'static str,
    description: &'static str,
    components: &'static [&'static str],
}

const TIERS: &[Tier] = &[
    Tier {
        slug: "form-controls",
        title: "Form Controls",
        description: "Interactive inputs for collecting user data",
        components: &[
            "button",
            "input",
            "textarea",
            "checkbox",
            "radio",
            "select",
            "switch",
            "slider",
            "number_field",
            "field",
            "fieldset",
            "label",
            "native_select",
        ],
    },
    Tier {
        slug: "display",
        title: "Display",
        description: "Visual indicators and content presentation",
        components: &[
            "badge",
            "avatar",
            "separator",
            "progress",
            "meter",
            "kbd",
            "skeleton",
            "spinner",
            "typography",
            "empty_state",
        ],
    },
    Tier {
        slug: "layout",
        title: "Layout",
        description: "Structural components for organizing content",
        components: &[
            "card",
            "accordion",
            "collapsible",
            "tabs",
            "table",
            "data_table",
            "pagination",
            "resizable",
            "scroll_area",
            "aspect_ratio",
        ],
    },
    Tier {
        slug: "overlay",
        title: "Overlay",
        description: "Modal and floating content layers",
        components: &[
            "dialog",
            "alert_dialog",
            "drawer",
            "popover",
            "tooltip",
            "hover_card",
            "toast",
            "alert",
        ],
    },
    Tier {
        slug: "navigation",
        title: "Navigation",
        description: "Menus, breadcrumbs, and wayfinding",
        components: &[
            "menu",
            "context_menu",
            "menubar",
            "navigation_menu",
            "breadcrumb",
            "command",
            "combobox",
        ],
    },
    Tier {
        slug: "composite",
        title: "Composite",
        description: "Multi-part components combining primitives",
        components: &[
            "button_group",
            "toggle",
            "toggle_group",
            "input_group",
            "input_otp",
            "radio_group",
            "calendar",
            "carousel",
            "chart",
            "date_picker",
            "toolbar",
        ],
    },
];

/// All component slug names, used for nav generation and route dispatch.
pub const COMPONENT_NAMES: &[&str] = &[
    "accordion",
    "alert",
    "alert_dialog",
    "aspect_ratio",
    "avatar",
    "badge",
    "breadcrumb",
    "button",
    "button_group",
    "calendar",
    "card",
    "carousel",
    "chart",
    "checkbox",
    "collapsible",
    "combobox",
    "command",
    "context_menu",
    "data_table",
    "date_picker",
    "dialog",
    "drawer",
    "empty_state",
    "field",
    "fieldset",
    "hover_card",
    "input",
    "input_group",
    "input_otp",
    "kbd",
    "label",
    "menu",
    "menubar",
    "meter",
    "native_select",
    "navigation_menu",
    "number_field",
    "pagination",
    "popover",
    "progress",
    "radio",
    "radio_group",
    "resizable",
    "scroll_area",
    "select",
    "separator",
    "skeleton",
    "slider",
    "spinner",
    "switch",
    "table",
    "tabs",
    "textarea",
    "toast",
    "toggle",
    "toggle_group",
    "tooltip",
    "typography",
];

/// Render a code example block with a title.
fn code_example(title: &str, code: &str) -> Markup {
    html! {
        div.mui-showcase__code-section {
            h3.mui-showcase__code-title { (title) }
            pre.mui-showcase__code {
                code { (code) }
            }
        }
    }
}

/// Return a usage code example for a component by slug name.
fn component_docs(name: &str) -> Option<Markup> {
    let code = match name {
        "button" => r#"use maud_ui::primitives::button;

let html = button::render(button::Props {
    label: "Save changes".into(),
    variant: button::Variant::Primary,
    size: button::Size::Md,
    disabled: false,
    button_type: "submit",
});"#,
        "input" => r#"use maud_ui::primitives::input;

let html = input::render(input::Props {
    name: "email".into(),
    input_type: input::InputType::Email,
    placeholder: "you@example.com".into(),
    id: "email-field".into(),
    required: true,
    ..Default::default()
});"#,
        "textarea" => r#"use maud_ui::primitives::textarea;

let html = textarea::render(textarea::Props {
    name: "bio".into(),
    placeholder: "Tell us about yourself...".into(),
    rows: 6,
    id: "bio-field".into(),
    resize: textarea::Resize::Vertical,
    ..Default::default()
});"#,
        "checkbox" => r#"use maud_ui::primitives::checkbox;

let html = checkbox::render(checkbox::Props {
    name: "terms".into(),
    value: "accepted".into(),
    label: "I agree to the terms".into(),
    id: "terms-cb".into(),
    description: Some("Required to continue.".into()),
    ..Default::default()
});"#,
        "radio" => r#"use maud_ui::primitives::radio;

let html = radio::render(radio::Props {
    name: "plan".into(),
    value: "pro".into(),
    label: "Pro plan".into(),
    id: "plan-pro".into(),
    checked: true,
    description: Some("Unlimited projects".into()),
    ..Default::default()
});"#,
        "select" => r#"use maud_ui::primitives::select;

let html = select::render(select::Props {
    name: "country".into(),
    id: "country-select".into(),
    placeholder: "Choose a country".into(),
    selected: Some("us".into()),
    options: vec![
        select::SelectOption { value: "us".into(), label: "United States".into(), disabled: false },
        select::SelectOption { value: "gb".into(), label: "United Kingdom".into(), disabled: false },
    ],
    ..Default::default()
});"#,
        "switch" => r#"use maud_ui::primitives::switch;

let html = switch::render(switch::Props {
    name: "dark-mode".into(),
    id: "dark-mode-switch".into(),
    label: "Dark mode".into(),
    checked: true,
    disabled: false,
});"#,
        "dialog" => r#"use maud_ui::primitives::dialog;
use maud::html;

// Render the trigger button
let trigger = dialog::trigger("confirm-dlg", "Open dialog");

// Render the dialog itself
let dlg = dialog::render(dialog::Props {
    id: "confirm-dlg".into(),
    title: "Confirm action".into(),
    description: Some("This cannot be undone.".into()),
    children: html! { p { "Are you sure?" } },
    footer: Some(html! {
        button.mui-btn.mui-btn--danger.mui-btn--md { "Confirm" }
    }),
    open: false,
});"#,
        "tabs" => r#"use maud_ui::primitives::tabs;
use maud::html;

let html = tabs::render(tabs::Props {
    tabs: vec![
        tabs::Tab { id: "overview".into(), label: "Overview".into(), content: html! { p { "Overview content" } } },
        tabs::Tab { id: "settings".into(), label: "Settings".into(), content: html! { p { "Settings content" } } },
    ],
    default_active: 0,
    aria_label: "Account tabs".into(),
});"#,
        "accordion" => r#"use maud_ui::primitives::accordion;
use maud::html;

let html = accordion::render(accordion::Props {
    items: vec![
        accordion::Item { id: "faq-1".into(), trigger: "What is maud-ui?".into(), content: html! { p { "A component library for maud + htmx." } }, open: true },
        accordion::Item { id: "faq-2".into(), trigger: "Is it free?".into(), content: html! { p { "Yes, MIT licensed." } }, open: false },
    ],
    multiple: false,
});"#,
        "card" => r#"use maud_ui::primitives::card;
use maud::html;

let html = card::render(card::Props {
    title: Some("Project stats".into()),
    description: Some("Overview of your project.".into()),
    children: html! { p { "Content goes here." } },
    footer: Some(html! {
        button.mui-btn.mui-btn--primary.mui-btn--sm { "View details" }
    }),
});"#,
        "table" => r#"use maud_ui::primitives::table;

let html = table::render(table::Props {
    headers: vec!["Name".into(), "Role".into(), "Status".into()],
    rows: vec![
        vec!["Alice".into(), "Admin".into(), "Active".into()],
        vec!["Bob".into(), "Editor".into(), "Inactive".into()],
    ],
    striped: true,
    hoverable: true,
    compact: false,
    caption: Some("Team members".into()),
});"#,
        "badge" => r#"use maud_ui::primitives::badge;

let html = badge::render(badge::Props {
    label: "New".into(),
    variant: badge::Variant::Success,
});"#,
        "alert" => r#"use maud_ui::primitives::alert;

let html = alert::render(alert::Props {
    title: "Deployment complete".into(),
    description: Some("All services are running.".into()),
    variant: alert::Variant::Success,
    icon: true,
});"#,
        "toast" => r#"use maud_ui::primitives::toast;

// Add the viewport container once in your layout
let vp = toast::viewport();

// Render a toast notification
let html = toast::render(toast::Props {
    title: "Changes saved".into(),
    description: Some("Your profile was updated.".into()),
    variant: toast::Variant::Success,
    duration_ms: 5000,
    id: "save-toast".into(),
});

// Trigger from JS: muiToast({ title, description, variant })"#,
        "field" => r#"use maud_ui::primitives::field;
use maud::html;

let html = field::render(field::Props {
    label: "Email".into(),
    id: "signup-email".into(),
    description: Some("We will never share your email.".into()),
    error: None,
    required: true,
    children: html! {
        input.mui-input type="email" id="signup-email" name="email" placeholder="you@example.com";
    },
});"#,
        "calendar" => r#"use maud_ui::primitives::calendar;

let html = calendar::render(calendar::Props {
    id: "booking-cal".into(),
    year: 2026,
    month: 4,
    selected: Some((2026, 4, 15)),
    min_date: Some((2026, 4, 1)),
    max_date: Some((2026, 12, 31)),
    show_outside_days: true,
});"#,
        "combobox" => r#"use maud_ui::primitives::combobox;

let html = combobox::render(combobox::Props {
    id: "lang-combo".into(),
    name: "language".into(),
    placeholder: "Select language".into(),
    selected: Some("rust".into()),
    options: vec![
        combobox::ComboboxOption { value: "rust".into(), label: "Rust".into() },
        combobox::ComboboxOption { value: "ts".into(), label: "TypeScript".into() },
    ],
    ..Default::default()
});"#,
        "menu" => r#"use maud_ui::primitives::menu;

let html = menu::render(menu::Props {
    trigger_label: "Actions".into(),
    id: "row-menu".into(),
    items: vec![
        menu::MenuEntry::Label("Edit".into()),
        menu::MenuEntry::Item(menu::MenuItem {
            label: "Rename".into(), action: "rename".into(),
            disabled: false, destructive: false, shortcut: Some("F2".into()),
        }),
        menu::MenuEntry::Separator,
        menu::MenuEntry::Item(menu::MenuItem {
            label: "Delete".into(), action: "delete".into(),
            disabled: false, destructive: true, shortcut: None,
        }),
    ],
});"#,
        "slider" => r#"use maud_ui::primitives::slider;

let html = slider::render(slider::Props {
    name: "volume".into(),
    id: "volume-slider".into(),
    value: 75.0,
    min: 0.0,
    max: 100.0,
    step: 1.0,
    label: "Volume".into(),
    show_value: true,
    disabled: false,
});"#,
        "spinner" => r#"use maud_ui::primitives::spinner;

let html = spinner::render(spinner::Props {
    size: spinner::Size::Md,
    label: Some("Loading data...".into()),
});"#,
        "skeleton" => r#"use maud_ui::primitives::skeleton;

let html = skeleton::render(skeleton::Props {
    variant: skeleton::Variant::Text,
    width: Some("200px".into()),
    height: Some("1rem".into()),
});"#,
        "data_table" => r#"use maud_ui::primitives::data_table;

let html = data_table::render(data_table::Props {
    id: "users-table".into(),
    columns: vec![
        data_table::Column { key: "name".into(), label: "Name".into(), sortable: true },
        data_table::Column { key: "email".into(), label: "Email".into(), sortable: true },
    ],
    rows: vec![
        vec!["Alice".into(), "alice@example.com".into()],
        vec!["Bob".into(), "bob@example.com".into()],
    ],
    page_size: 10,
    searchable: true,
    ..Default::default()
});"#,
        "date_picker" => r#"use maud_ui::primitives::date_picker;

let html = date_picker::render(date_picker::Props {
    id: "start-date".into(),
    name: "start_date".into(),
    selected: Some((2026, 4, 15)),
    placeholder: "Pick a start date".into(),
    disabled: false,
    min_date: Some((2026, 1, 1)),
    max_date: None,
});"#,
        "command" => r#"use maud_ui::primitives::command;

// Render the trigger button
let trigger = command::trigger("cmd-palette", "Command palette");

// Render the palette
let html = command::render(command::Props {
    id: "cmd-palette".into(),
    placeholder: "Type a command...".into(),
    items: vec![
        command::CommandItem { label: "New file".into(), shortcut: Some("Cmd+N".into()), group: Some("File".into()), disabled: false },
        command::CommandItem { label: "Search".into(), shortcut: Some("Cmd+K".into()), group: Some("General".into()), disabled: false },
    ],
});"#,
        _ => return None,
    };
    Some(code_example("Usage", code))
}

/// Convert a slug like "toggle_group" to a display name like "Toggle Group".
fn display_name(slug: &str) -> String {
    slug.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{}{}", upper, chars.collect::<String>())
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Render the showcase content for a component by slug name.
fn component_content(name: &str) -> Option<Markup> {
    let markup = match name {
        "accordion" => primitives::accordion::showcase(),
        "alert" => primitives::alert::showcase(),
        "alert_dialog" => primitives::alert_dialog::showcase(),
        "aspect_ratio" => primitives::aspect_ratio::showcase(),
        "avatar" => primitives::avatar::showcase(),
        "badge" => primitives::badge::showcase(),
        "breadcrumb" => primitives::breadcrumb::showcase(),
        "button" => primitives::button::showcase(),
        "button_group" => primitives::button_group::showcase(),
        "calendar" => primitives::calendar::showcase(),
        "card" => primitives::card::showcase(),
        "carousel" => primitives::carousel::showcase(),
        "chart" => primitives::chart::showcase(),
        "checkbox" => primitives::checkbox::showcase(),
        "collapsible" => primitives::collapsible::showcase(),
        "combobox" => primitives::combobox::showcase(),
        "command" => primitives::command::showcase(),
        "context_menu" => primitives::context_menu::showcase(),
        "data_table" => primitives::data_table::showcase(),
        "date_picker" => primitives::date_picker::showcase(),
        "dialog" => primitives::dialog::showcase(),
        "drawer" => primitives::drawer::showcase(),
        "empty_state" => primitives::empty_state::showcase(),
        "field" => primitives::field::showcase(),
        "fieldset" => primitives::fieldset::showcase(),
        "hover_card" => primitives::hover_card::showcase(),
        "input" => primitives::input::showcase(),
        "input_group" => primitives::input_group::showcase(),
        "input_otp" => primitives::input_otp::showcase(),
        "kbd" => primitives::kbd::showcase(),
        "label" => primitives::label::showcase(),
        "menu" => primitives::menu::showcase(),
        "menubar" => primitives::menubar::showcase(),
        "meter" => primitives::meter::showcase(),
        "native_select" => primitives::native_select::showcase(),
        "navigation_menu" => primitives::navigation_menu::showcase(),
        "number_field" => primitives::number_field::showcase(),
        "pagination" => primitives::pagination::showcase(),
        "popover" => primitives::popover::showcase(),
        "progress" => primitives::progress::showcase(),
        "radio" => primitives::radio::showcase(),
        "radio_group" => primitives::radio_group::showcase(),
        "resizable" => primitives::resizable::showcase(),
        "scroll_area" => primitives::scroll_area::showcase(),
        "select" => primitives::select::showcase(),
        "separator" => primitives::separator::showcase(),
        "skeleton" => primitives::skeleton::showcase(),
        "slider" => primitives::slider::showcase(),
        "spinner" => primitives::spinner::showcase(),
        "switch" => primitives::switch::showcase(),
        "table" => primitives::table::showcase(),
        "tabs" => primitives::tabs::showcase(),
        "textarea" => primitives::textarea::showcase(),
        "toast" => primitives::toast::showcase(),
        "toggle" => primitives::toggle::showcase(),
        "toggle_group" => primitives::toggle_group::showcase(),
        "tooltip" => primitives::tooltip::showcase(),
        "typography" => primitives::typography::showcase(),
        _ => return None,
    };
    Some(markup)
}

/// Shared HTML head used by both the gallery and individual component pages.
fn page_head(title: &str) -> Markup {
    html! {
        meta charset="utf-8";
        meta name="viewport" content="width=device-width, initial-scale=1";
        title { (title) }
        link rel="stylesheet" href="/css/maud-ui.css";
        style { (showcase_css()) }
    }
}

/// Shared header with theme toggle.
fn page_header() -> Markup {
    html! {
        header.mui-showcase__header {
            div style="display:flex;justify-content:space-between;align-items:center;" {
                div {
                    h1 { a href="/" style="color:inherit;text-decoration:none;" { "maud-ui" } }
                    p.mui-text-muted { (format!("{} components for maud + htmx", COMPONENT_NAMES.len())) }
                }
                div style="display:flex;gap:0.75rem;align-items:center;" {
                    span.mui-text-subtle style="font-size:0.8125rem;" { "Theme:" }
                    button type="button" class="mui-btn mui-btn--outline mui-btn--sm" data-mui="theme-toggle" id="theme-toggle" {
                        "Toggle theme"
                    }
                }
            }
        }
    }
}

/// Sticky sidebar with grouped component navigation.
fn sidebar_nav() -> Markup {
    html! {
        aside class="mui-gallery__sidebar" {
            nav class="mui-gallery__nav" {
                @for tier in TIERS {
                    div class="mui-gallery__nav-group" {
                        a class="mui-gallery__nav-tier" href=(format!("#{}", tier.slug)) {
                            (tier.title)
                        }
                        div class="mui-gallery__nav-items" {
                            @for comp in tier.components {
                                @if component_content(comp).is_some() {
                                    a class="mui-gallery__nav-item" href=(format!("#{}", comp)) {
                                        (display_name(comp))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Main gallery page at `/`. Shows grouped components with sidebar navigation.
pub fn showcase_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("maud-ui \u{00b7} Component Gallery"))
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        @for tier in TIERS {
                            div class="mui-gallery__tier" id=(tier.slug) {
                                div class="mui-gallery__tier-header" {
                                    h2 class="mui-gallery__tier-title" { (tier.title) }
                                    p class="mui-gallery__tier-desc" { (tier.description) }
                                }
                                @for comp in tier.components {
                                    @if let Some(content) = component_content(comp) {
                                        section class="mui-gallery__component" id=(comp) {
                                            h3 class="mui-gallery__component-name" {
                                                a href=(format!("/{}", comp)) style="color:inherit;text-decoration:none;" {
                                                    (display_name(comp))
                                                }
                                                a href=(format!("#{}", comp)) class="mui-gallery__anchor" { "#" }
                                            }
                                            (content)
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                script src="/js/maud-ui.js" defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

/// Wraps a single component's showcase in the full page shell.
pub fn component_page(name: &str, content: Markup) -> Markup {
    let title = format!("{} \u{2014} maud-ui", display_name(name));
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head(&title))
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" }
                            span { " / " }
                            span { (display_name(name)) }
                        }
                        section class="mui-gallery__component" id=(name) {
                            h3 class="mui-gallery__component-name" { (display_name(name)) }
                            (content)
                            @if let Some(docs) = component_docs(name) {
                                (docs)
                            }
                        }
                        div class="mui-gallery__back" {
                            a href="/" class="mui-btn mui-btn--outline mui-btn--sm" {
                                "\u{2190} Back to Gallery"
                            }
                        }
                    }
                }
                script src="/js/maud-ui.js" defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

/// Dynamic dispatch: renders the individual page for a component by slug.
/// Returns a 404-style page if the name is not recognized.
pub fn component_page_by_name(name: &str) -> Markup {
    match component_content(name) {
        Some(content) => component_page(name, content),
        None => {
            html! {
                (DOCTYPE)
                html lang="en" data-theme="dark" {
                    head {
                        (page_head("Not Found \u{2014} maud-ui"))
                    }
                    body {
                        (page_header())
                        main class="mui-gallery__main" style="padding:2rem;" {
                            section class="mui-gallery__component" {
                                h3 { "Component not found" }
                                p { "No component named \"" (name) "\" exists." }
                                a href="/" class="mui-btn mui-btn--outline mui-btn--sm" {
                                    "\u{2190} Back to Gallery"
                                }
                            }
                        }
                        script src="/js/maud-ui.js" defer {}
                    }
                }
            }
        }
    }
}

// ── Embedded CSS for the gallery layout ─────────────────────────────────

fn showcase_css() -> &'static str {
    r#"
/* Gallery layout — sidebar + main */
.mui-gallery {
    display: grid;
    grid-template-columns: 240px 1fr;
    min-height: calc(100vh - 80px);
}

/* Sticky sidebar */
.mui-gallery__sidebar {
    position: sticky;
    top: 0;
    height: 100vh;
    overflow-y: auto;
    border-right: 1px solid var(--mui-border);
    padding: 1rem 0;
    scrollbar-width: thin;
    scrollbar-color: var(--mui-border) transparent;
}

.mui-gallery__nav {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
}

.mui-gallery__nav-group {
    padding: 0 0 0.25rem;
}

.mui-gallery__nav-tier {
    display: block;
    padding: 0.5rem 1rem;
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--mui-text-muted);
    text-decoration: none;
    transition: color var(--mui-transition);
}
.mui-gallery__nav-tier:hover { color: var(--mui-text); }

.mui-gallery__nav-items {
    display: flex;
    flex-direction: column;
}

.mui-gallery__nav-item {
    display: block;
    padding: 0.25rem 1rem 0.25rem 1.5rem;
    font-size: 0.8125rem;
    color: var(--mui-text-subtle);
    text-decoration: none;
    border-left: 2px solid transparent;
    transition: all var(--mui-transition);
}
.mui-gallery__nav-item:hover {
    color: var(--mui-text);
    background: var(--mui-bg-input);
    border-left-color: var(--mui-border-hover);
}
.mui-gallery__nav-item--active {
    color: var(--mui-text);
    border-left-color: var(--mui-accent);
    background: var(--mui-bg-input);
}

/* Main content */
.mui-gallery__main {
    padding: 2rem;
    max-width: 960px;
}

.mui-gallery__tier {
    margin-bottom: 3rem;
}

.mui-gallery__tier-header {
    margin-bottom: 1.5rem;
    padding-bottom: 0.75rem;
    border-bottom: 1px solid var(--mui-border);
}

.mui-gallery__tier-title {
    margin: 0 0 0.25rem;
    font-size: 1.5rem;
    font-weight: 700;
}

.mui-gallery__tier-desc {
    margin: 0;
    color: var(--mui-text-muted);
    font-size: 0.875rem;
}

.mui-gallery__component {
    padding: 1.5rem;
    margin-bottom: 1rem;
    background: var(--mui-bg-card);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-lg);
}

.mui-gallery__component-name {
    margin: 0 0 1rem;
    font-size: 1.125rem;
    font-weight: 600;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.mui-gallery__anchor {
    color: var(--mui-text-subtle);
    text-decoration: none;
    font-weight: 400;
    opacity: 0;
    transition: opacity var(--mui-transition);
}
.mui-gallery__component:hover .mui-gallery__anchor {
    opacity: 1;
}
.mui-gallery__anchor:hover { color: var(--mui-accent); }

.mui-gallery__breadcrumb {
    display: flex;
    align-items: center;
    gap: 0.25rem;
    font-size: 0.875rem;
    color: var(--mui-text-muted);
    margin-bottom: 1rem;
}
.mui-gallery__breadcrumb a {
    color: var(--mui-text-subtle);
    text-decoration: none;
}
.mui-gallery__breadcrumb a:hover {
    color: var(--mui-text);
    text-decoration: underline;
}

.mui-gallery__back { padding-top: 1rem; }

/* Responsive: collapse sidebar on narrow screens */
@media (max-width: 768px) {
    .mui-gallery {
        grid-template-columns: 1fr;
    }
    .mui-gallery__sidebar {
        position: static;
        height: auto;
        border-right: none;
        border-bottom: 1px solid var(--mui-border);
        padding: 0.75rem 0;
    }
    .mui-gallery__nav-items {
        flex-direction: row;
        flex-wrap: wrap;
        gap: 0.125rem;
    }
    .mui-gallery__nav-item {
        border-left: none;
        padding: 0.25rem 0.5rem;
        border-radius: var(--mui-radius-sm);
    }
}

/* Smooth scrolling */
html { scroll-behavior: smooth; }
"#
}

// ── Embedded JS for the gallery ─────────────────────────────────────────

fn showcase_js() -> &'static str {
    r#"
(function() {
    // Theme toggle is handled by dist/behaviors/theme.js (via data-mui="theme-toggle").
    // Sidebar active state on scroll
    var navItems = document.querySelectorAll('.mui-gallery__nav-item');
    if (navItems.length > 0) {
        var observer = new IntersectionObserver(function(entries) {
            for (var i = 0; i < entries.length; i++) {
                var entry = entries[i];
                if (entry.isIntersecting) {
                    var id = entry.target.id;
                    for (var j = 0; j < navItems.length; j++) {
                        var item = navItems[j];
                        if (item.getAttribute('href') === '#' + id) {
                            item.classList.add('mui-gallery__nav-item--active');
                        } else {
                            item.classList.remove('mui-gallery__nav-item--active');
                        }
                    }
                }
            }
        }, { rootMargin: '-20% 0px -60% 0px' });

        var sections = document.querySelectorAll('.mui-gallery__component');
        for (var k = 0; k < sections.length; k++) {
            observer.observe(sections[k]);
        }
    }
})();
"#
}
