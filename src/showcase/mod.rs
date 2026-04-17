//! Live component gallery. Serves `showcase_page()` as the `showcase` axum example.
//! Each component also has its own route via `component_page_by_name()`.
//! Components are grouped into tiers: Form Controls, Display, Layout, Overlay,
//! Navigation, and Composite.

use maud::{html, Markup, DOCTYPE};

use crate::{blocks, primitives};

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

// ── Blocks ─────────────────────────────────────────────────────────────

/// Catalog of blocks shown on the /blocks index. Each entry = one card
/// on the index page + one route at /blocks/{slug}.
struct BlockEntry {
    slug: &'static str,
    category: &'static str,
    title: &'static str,
    description: &'static str,
    uses: &'static [&'static str],
}

const BLOCK_CATALOG: &[BlockEntry] = &[
    BlockEntry {
        slug: "auth-login",
        category: "Authentication",
        title: "Sign in",
        description: "Centered card with optional OAuth providers, email + password form, forgot-password link, and signup prompt. Drop it behind your POST /login handler.",
        uses: &["card", "input", "button", "alert", "separator"],
    },
];

/// Render the preview for a block by slug.
fn block_content(slug: &str) -> Option<Markup> {
    let markup = match slug {
        "auth-login" => blocks::auth::login::preview(),
        _ => return None,
    };
    Some(markup)
}

/// Hand-written "Usage" snippet for a block. Shown below the preview
/// on each /blocks/{slug} page.
fn block_docs(slug: &str) -> Option<Markup> {
    let code = match slug {
        "auth-login" => r#"use maud_ui::blocks::auth::login;

login::render(login::Props {
    action: "/auth/login".into(),
    heading: "Welcome back".into(),
    subheading: "Sign in to continue".into(),
    oauth_providers: vec![
        login::OAuthProvider {
            id: "google".into(),
            label: "Continue with Google".into(),
            href: "/auth/oauth/google".into(),
            icon: None,
        },
    ],
    forgot_password_url: Some("/auth/forgot".into()),
    signup_url: Some("/auth/signup".into()),
    // Re-render after failed POST to preserve the email + surface error:
    // email_value: submitted_email,
    // error: Some("Invalid credentials".into()),
    ..Default::default()
})"#,
        _ => return None,
    };
    Some(code_example("Usage", code))
}

/// Render the content list of all blocks as a card grid — used by
/// the /blocks index page.
fn blocks_index_grid() -> Markup {
    html! {
        div class="mui-showcase__block-grid" {
            @for entry in BLOCK_CATALOG {
                a class="mui-showcase__block-card" href=(format!("/blocks/{}", entry.slug)) {
                    div class="mui-showcase__block-card-header" {
                        span class="mui-showcase__block-card-category" { (entry.category) }
                        h3 class="mui-showcase__block-card-title" { (entry.title) }
                    }
                    p class="mui-showcase__block-card-desc" { (entry.description) }
                    div class="mui-showcase__block-card-uses" {
                        @for u in entry.uses {
                            span class="mui-showcase__block-card-use" { (u) }
                        }
                    }
                }
            }
            @if BLOCK_CATALOG.len() < 10 {
                div class="mui-showcase__block-card mui-showcase__block-card--placeholder" {
                    div class="mui-showcase__block-card-header" {
                        span class="mui-showcase__block-card-category" { "Coming soon" }
                        h3 class="mui-showcase__block-card-title" { "More blocks on the way" }
                    }
                    p class="mui-showcase__block-card-desc" {
                        "Signup, two-factor, sidebar shell, dashboard stats, settings profile, billing, pricing tiers, data-table with filters. "
                        a href="https://github.com/hgeldenhuys/maud-ui/issues" target="_blank" rel="noopener" style="color:var(--mui-accent-text);" {
                            "Open an issue"
                        }
                        " if you want a specific one prioritised."
                    }
                }
            }
        }
    }
}

/// Full page shell for the /blocks index.
pub fn blocks_index_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Blocks \u{2014} maud-ui"))
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" }
                            span { " / " }
                            span { "Blocks" }
                        }
                        section class="mui-gallery__component" id="blocks" {
                            h3 class="mui-gallery__component-name" { "Blocks" }
                            p style="color:var(--mui-text-muted);font-size:0.9375rem;max-width:42rem;margin:0 0 1.5rem;" {
                                "Pre-composed templates built from primitives. Drop into real apps — customize by reading the source and paste-editing into your own module. Each block renders to plain HTML; no framework needed on the client side."
                            }
                            (blocks_index_grid())
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

/// Page shell for an individual /blocks/{slug} route.
pub fn block_page_by_name(slug: &str) -> Markup {
    let content = block_content(slug);
    let display = blocks::display_name(slug);
    let title = format!("{} \u{2014} maud-ui blocks", display);
    match content {
        Some(preview) => html! {
            (DOCTYPE)
            html lang="en" data-theme="dark" {
                head { (page_head(&title)) }
                body {
                    (page_header())
                    div class="mui-gallery" {
                        (sidebar_nav())
                        main class="mui-gallery__main" {
                            nav class="mui-gallery__breadcrumb" {
                                a href="/" { "Gallery" }
                                span { " / " }
                                a href="/blocks" { "Blocks" }
                                span { " / " }
                                span { (display) }
                            }
                            section class="mui-gallery__component" id=(slug) {
                                h3 class="mui-gallery__component-name" { (display) }
                                (preview)
                                @if let Some(docs) = block_docs(slug) { (docs) }
                            }
                            div class="mui-gallery__back" {
                                a href="/blocks" class="mui-btn mui-btn--outline mui-btn--sm" {
                                    "\u{2190} Back to Blocks"
                                }
                            }
                        }
                    }
                    script src="/js/maud-ui.js" defer {}
                    script { (maud::PreEscaped(showcase_js())) }
                }
            }
        },
        None => html! {
            (DOCTYPE)
            html lang="en" data-theme="dark" {
                head { (page_head("Block not found \u{2014} maud-ui")) }
                body {
                    (page_header())
                    main class="mui-gallery__main" style="padding:4rem 2rem;" {
                        h2 { "Block not found: " (slug) }
                        p { a href="/blocks" { "\u{2190} Back to Blocks" } }
                    }
                }
            }
        },
    }
}

// ── Components ─────────────────────────────────────────────────────────

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
            div style="display:flex;justify-content:space-between;align-items:center;flex-wrap:wrap;gap:1rem;" {
                div {
                    h1 { a href="/" style="color:inherit;text-decoration:none;" { "maud-ui" } }
                    p.mui-text-muted { (format!("{} components for maud + htmx", COMPONENT_NAMES.len())) }
                }
                div style="display:flex;gap:0.75rem;align-items:center;" {
                    a href="/getting-started" class="mui-btn mui-btn--ghost mui-btn--sm" style="text-decoration:none;" {
                        "Get started"
                    }
                    a href="/blocks" class="mui-btn mui-btn--ghost mui-btn--sm" style="text-decoration:none;" {
                        "Blocks"
                    }
                    a href="https://docs.rs/maud-ui" target="_blank" rel="noopener" class="mui-btn mui-btn--ghost mui-btn--sm" style="text-decoration:none;" {
                        "Docs"
                    }
                    a href="https://github.com/hgeldenhuys/maud-ui" target="_blank" rel="noopener" class="mui-btn mui-btn--ghost mui-btn--sm" style="text-decoration:none;" {
                        "GitHub"
                    }
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

/// Onboarding page at /getting-started — install, first paint, theming, runtime.
pub fn getting_started_page() -> Markup {
    use crate::primitives::{alert, badge, button, card, field, input, kbd};

    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Get started \u{2014} maud-ui"))
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" }
                            span { " / " }
                            span { "Get started" }
                        }

                        section class="mui-gallery__component" id="hero" {
                            h3 class="mui-gallery__component-name" { "Welcome to maud-ui" }
                            p style="font-size:1rem;line-height:1.6;color:var(--mui-text-muted);max-width:42rem;" {
                                "58 headless, accessible components for Rust web apps. Drop them into any axum/actix/rocket handler — they render to HTML, ship with pre-built CSS and JS, and work without a JavaScript framework."
                            }
                            div style="display:flex;gap:0.5rem;flex-wrap:wrap;margin-top:0.75rem;" {
                                (badge::render(badge::Props { label: "58 components".into(), variant: badge::Variant::Default }))
                                (badge::render(badge::Props { label: "MIT".into(), variant: badge::Variant::Secondary }))
                                (badge::render(badge::Props { label: "11 KB gzipped".into(), variant: badge::Variant::Success }))
                                (badge::render(badge::Props { label: "WCAG AA".into(), variant: badge::Variant::Outline }))
                            }
                        }

                        section class="mui-gallery__component" id="install" {
                            h3 class="mui-gallery__component-name" { "1. Install" }
                            p.mui-showcase__caption { "Add maud + maud-ui to your Cargo.toml. If you're wiring up a brand new server, grab axum + tokio too." }
                            (code_example("Cargo", r#"cargo new my-app
cd my-app
cargo add maud maud-ui
cargo add axum tokio --features tokio/full
cargo add tower-http --features tower-http/fs
"#))
                        }

                        section class="mui-gallery__component" id="first-paint" {
                            h3 class="mui-gallery__component-name" { "2. First paint" }
                            p.mui-showcase__caption { "A minimal axum server that renders a card with a button. Copy this into src/main.rs and run cargo run." }
                            (code_example("src/main.rs", r##"use axum::{routing::get, Router};
use maud::{html, Markup, DOCTYPE};
use maud_ui::primitives::{button, card};

async fn index() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                meta charset="utf-8";
                link rel="stylesheet" href="/assets/maud-ui.min.css";
                script src="/assets/maud-ui.min.js" defer {}
            }
            body style="padding: 2rem;" {
                (card::render(card::Props {
                    title: Some("Welcome".into()),
                    description: Some("You're running maud-ui.".into()),
                    children: html! {
                        (button::render(button::Props {
                            label: "Ship it".into(),
                            variant: button::Variant::Primary,
                            ..Default::default()
                        }))
                    },
                    ..Default::default()
                }))
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Serve the bundled CSS + JS from maud-ui's dist/ folder.
    let assets = tower_http::services::ServeDir::new(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../maud-ui/dist"),  // or wherever your copy lives
    );

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", assets);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Open http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
"##))
                            p.mui-showcase__caption style="margin-top:1rem;" {
                                "For production you can embed the assets at compile time with include_str! — see the Deployment section below."
                            }
                        }

                        section class="mui-gallery__component" id="your-first-component" {
                            h3 class="mui-gallery__component-name" { "3. Your first component" }
                            p.mui-showcase__caption { "Every component is a Props struct and a render() function. Start typing — the compiler will guide you." }
                            div style="display:grid;grid-template-columns:1fr 1fr;gap:1.5rem;margin-bottom:1rem;" {
                                div {
                                    p.mui-showcase__caption { "You write:" }
                                    (code_example("", r#"button::render(button::Props {
    label: "Invite teammate".into(),
    variant: button::Variant::Primary,
    size: button::Size::Md,
    ..Default::default()
})"#))
                                }
                                div {
                                    p.mui-showcase__caption { "You get:" }
                                    div style="padding:1rem;background:var(--mui-bg-input);border-radius:var(--mui-radius-md);border:1px solid var(--mui-border);" {
                                        (button::render(button::Props {
                                            label: "Invite teammate".into(),
                                            variant: button::Variant::Primary,
                                            size: button::Size::Md,
                                            ..Default::default()
                                        }))
                                    }
                                }
                            }
                        }

                        section class="mui-gallery__component" id="forms" {
                            h3 class="mui-gallery__component-name" { "4. Forms + validation" }
                            p.mui-showcase__caption { "Field wraps label + input + error message with ARIA wiring handled for you." }
                            div style="max-width:28rem;padding:1.25rem;background:var(--mui-bg-input);border-radius:var(--mui-radius-md);border:1px solid var(--mui-border);margin-bottom:1rem;" {
                                (field::render(field::Props {
                                    label: "Work email".into(),
                                    id: "gs-email".into(),
                                    description: Some("We'll send the verification link here.".into()),
                                    required: true,
                                    children: html! {
                                        (input::render(input::Props {
                                            name: "email".into(),
                                            id: "gs-email".into(),
                                            input_type: crate::primitives::input::InputType::Email,
                                            placeholder: "you@company.com".into(),
                                            ..Default::default()
                                        }))
                                    },
                                    ..Default::default()
                                }))
                            }
                            (code_example("", r#"field::render(field::Props {
    label: "Work email".into(),
    id: "email".into(),
    description: Some("We'll send the verification link here.".into()),
    required: true,
    children: html! {
        (input::render(input::Props {
            name: "email".into(),
            id: "email".into(),
            input_type: input::InputType::Email,
            placeholder: "you@company.com".into(),
            ..Default::default()
        }))
    },
    ..Default::default()
})"#))
                        }

                        section class="mui-gallery__component" id="theming" {
                            h3 class="mui-gallery__component-name" { "5. Theming" }
                            p.mui-showcase__caption { "Set data-theme on <html> and every component recolors via CSS variables." }
                            div style="display:flex;gap:1rem;align-items:center;margin-bottom:1rem;flex-wrap:wrap;" {
                                (button::render(button::Props {
                                    label: "Try the theme toggle".into(),
                                    variant: button::Variant::Outline,
                                    size: button::Size::Md,
                                    aria_label: Some("Toggle theme demo".into()),
                                    ..Default::default()
                                }))
                                span.mui-text-muted style="font-size:0.875rem;" {
                                    "Use the toggle at the top-right, or add "
                                    (kbd::render(kbd::Props { keys: vec!["button".into(), "data-mui=\"theme-toggle\"".into()] }))
                                    " anywhere in your app."
                                }
                            }
                            (code_example("Custom palette", r#"[data-theme="dark"] {
    --mui-accent: #8b5cf6;        /* violet */
    --mui-accent-hover: #a78bfa;
    --mui-bg: #0c0a1d;
    --mui-text: #ede9fe;
}"#))
                        }

                        section class="mui-gallery__component" id="js-runtime" {
                            h3 class="mui-gallery__component-name" { "6. JavaScript runtime" }
                            p.mui-showcase__caption { "Components with interactivity (dialogs, dropdowns, carousels) register behaviors under window.MaudUI. The runtime auto-initializes on DOMContentLoaded and after htmx swaps." }
                            (code_example("", r#"// Dropped into your page via:
//   <script src="/assets/maud-ui.min.js" defer></script>

window.MaudUI.init();         // manually re-init (e.g. after a custom swap)
window.MaudUI.init(rootEl);   // re-init just a subtree
"#))
                            (alert::render(alert::Props {
                                title: "Progressive enhancement".into(),
                                description: Some("Tier 1 components (29 of them) render and style correctly with JS disabled. Tier 2 and 3 need the runtime for full keyboard/interaction support.".into()),
                                variant: alert::Variant::Info,
                                ..Default::default()
                            }))
                        }

                        section class="mui-gallery__component" id="htmx" {
                            h3 class="mui-gallery__component-name" { "7. Pair with htmx" }
                            p.mui-showcase__caption { "maud-ui was designed for htmx flows. Return a fresh Markup fragment from any handler and htmx swaps it in — the runtime re-initializes behaviors on the new nodes automatically." }
                            (code_example("Trigger a server fragment swap", r##"// Button that asks the server for a fresh table fragment.
(button::render(button::Props {
    label: "Refresh results".into(),
    variant: button::Variant::Outline,
    ..Default::default()
}))

// Axum handler returns HTML; htmx replaces #results with it.
async fn results() -> Markup {
    html! {
        (table::render(table::Props {
            headers: vec!["Customer".into(), "MRR".into()],
            rows: load_rows().await,
            striped: true,
            ..Default::default()
        }))
    }
}
"##))
                        }

                        section class="mui-gallery__component" id="deployment" {
                            h3 class="mui-gallery__component-name" { "8. Deployment" }
                            p.mui-showcase__caption { "Ship the runtime inside your binary with include_str! so there's nothing to serve from disk." }
                            (code_example("Embed the bundle at compile time", r##"// Instead of nest_service("/assets", ServeDir::new(...)),
// bake the CSS + JS into your binary:
async fn serve_css() -> impl IntoResponse {
    let css = include_str!("../../vendor/maud-ui/dist/maud-ui.min.css");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        css,
    )
}

let app = Router::new()
    .route("/", get(index))
    .route("/assets/maud-ui.min.css", get(serve_css))
    .route("/assets/maud-ui.min.js",  get(serve_js));
"##))
                        }

                        section class="mui-gallery__component" id="next-steps" {
                            h3 class="mui-gallery__component-name" { "Where to next" }
                            div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(16rem,1fr));gap:1rem;" {
                                (card::render(card::Props {
                                    title: Some("Browse the gallery".into()),
                                    description: Some("Every component with code snippets.".into()),
                                    children: html! {
                                        a href="/" class="mui-btn mui-btn--primary mui-btn--sm" style="text-decoration:none;" { "Open gallery" }
                                    },
                                    ..Default::default()
                                }))
                                (card::render(card::Props {
                                    title: Some("Read the API docs".into()),
                                    description: Some("Every Props struct and module, generated from rustdoc.".into()),
                                    children: html! {
                                        a href="https://docs.rs/maud-ui" target="_blank" rel="noopener" class="mui-btn mui-btn--outline mui-btn--sm" style="text-decoration:none;" { "docs.rs" }
                                    },
                                    ..Default::default()
                                }))
                                (card::render(card::Props {
                                    title: Some("Pair with Tailwind".into()),
                                    description: Some("Layer order, Preflight, shared tokens.".into()),
                                    children: html! {
                                        a href="https://github.com/hgeldenhuys/maud-ui/blob/master/docs/TAILWIND.md" target="_blank" rel="noopener" class="mui-btn mui-btn--outline mui-btn--sm" style="text-decoration:none;" { "Tailwind guide" }
                                    },
                                    ..Default::default()
                                }))
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

/* Blocks index grid */
.mui-showcase__block-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(18rem, 1fr));
    gap: 1rem;
    margin-top: 0.5rem;
}
.mui-showcase__block-card {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    padding: 1.25rem;
    background: var(--mui-bg-card);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-lg);
    color: inherit;
    text-decoration: none;
    transition: border-color var(--mui-transition), transform var(--mui-transition);
}
.mui-showcase__block-card:hover {
    border-color: var(--mui-border-hover);
    transform: translateY(-1px);
}
.mui-showcase__block-card--placeholder {
    opacity: 0.55;
    pointer-events: auto;
    background: transparent;
    border-style: dashed;
}
.mui-showcase__block-card-category {
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--mui-text-subtle);
}
.mui-showcase__block-card-title {
    margin: 0.25rem 0 0;
    font-size: 1.0625rem;
    font-weight: 600;
    color: var(--mui-text);
}
.mui-showcase__block-card-desc {
    margin: 0;
    font-size: 0.875rem;
    color: var(--mui-text-muted);
    line-height: 1.5;
    flex: 1;
}
.mui-showcase__block-card-uses {
    display: flex;
    flex-wrap: wrap;
    gap: 0.375rem;
}
.mui-showcase__block-card-use {
    font-size: 0.6875rem;
    font-family: var(--mui-font-mono);
    padding: 0.125rem 0.4375rem;
    background: var(--mui-bg-input);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-sm);
    color: var(--mui-text-muted);
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
