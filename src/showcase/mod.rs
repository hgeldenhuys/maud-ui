//! Live component gallery. Serves `showcase_page()` as the `showcase` axum example.
//! Each component also has its own route via `component_page_by_name()`.
//! Components are grouped into tiers: Form Controls, Display, Layout, Overlay,
//! Navigation, and Composite.

use maud::{html, Markup, DOCTYPE};

use crate::{blocks, primitives};

// Build-time cache busters. Browsers treat `?v=X` as part of the URL,
// so changing X (on every build that touches dist/) guarantees a fresh
// fetch without waiting for `must-revalidate` or manual hard-reloads.
// `.len()` on `&[u8; N]` is a const expression, so these constants
// re-compute automatically each time the embedded files change.
const CSS_VER: usize = include_bytes!("../../dist/maud-ui.css").len();
const JS_VER: usize = include_bytes!("../../dist/maud-ui.js").len();

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
    BlockEntry {
        slug: "auth-signup",
        category: "Authentication",
        title: "Sign up",
        description: "Create-account card. Name + email + password + confirm, optional OAuth, optional Terms/Privacy checkbox. Submits to your POST /signup handler.",
        uses: &["card", "input", "button", "alert", "separator"],
    },
    BlockEntry {
        slug: "auth-two-factor",
        category: "Authentication",
        title: "Two-factor verification",
        description: "OTP confirmation card with email/SMS/authenticator variants. Submits the 6-digit (or 4-digit) code to your verify handler, with resend + cancel links.",
        uses: &["card", "input_otp", "button", "alert"],
    },
    BlockEntry {
        slug: "dashboard-stats",
        category: "Dashboard",
        title: "Stats overview",
        description: "Auto-fit grid of KPI cards with color-coded +/- deltas, plus an optional chart slot and recent-activity feed. Drop it into any dashboard route.",
        uses: &["card"],
    },
    BlockEntry {
        slug: "data-table-full",
        category: "Data",
        title: "Full data table",
        description: "Production-ready table — search, filter dropdowns, bulk-action form, row selection, status badges, row-view links, and pagination summary. All server-rendered.",
        uses: &["card", "input", "button", "badge", "native_select"],
    },
    BlockEntry {
        slug: "pricing-tiers",
        category: "Marketing",
        title: "Pricing tiers",
        description: "3-column pricing grid with a highlighted middle tier (\"Most popular\"), per-tier feature list with check marks, CTA buttons, and fine-print line.",
        uses: &["card", "badge", "button"],
    },
    BlockEntry {
        slug: "settings-billing",
        category: "Settings",
        title: "Billing",
        description: "Current plan card with seat-usage bar + next-charge date, payment method card, and a table of past invoices with download links.",
        uses: &["card", "badge", "button", "table"],
    },
    BlockEntry {
        slug: "settings-profile",
        category: "Settings",
        title: "Profile",
        description: "Edit name, email, bio. Avatar change/remove controls, save actions, and a clearly separated \"Delete account\" danger zone.",
        uses: &["card", "input", "textarea", "button"],
    },
    BlockEntry {
        slug: "settings-team",
        category: "Settings",
        title: "Team management",
        description: "Roster table with per-member role select + remove, status badges, last-active, plus an invite-by-email row at the top.",
        uses: &["card", "input", "button", "badge", "native_select"],
    },
    BlockEntry {
        slug: "shell-sidebar",
        category: "Application shell",
        title: "Sidebar app shell",
        description: "Full app chrome — 16rem vertical nav with grouped items + badges + user footer, plus a sticky topbar and main content slot. Drop your whole app inside.",
        uses: &["button", "card", "badge"],
    },
];

/// Render the preview for a block by slug.
fn block_content(slug: &str) -> Option<Markup> {
    let markup = match slug {
        "auth-login" => blocks::auth::login::preview(),
        "auth-signup" => blocks::auth::signup::preview(),
        "auth-two-factor" => blocks::auth::two_factor::preview(),
        "dashboard-stats" => blocks::dashboard::stats::preview(),
        "data-table-full" => blocks::data::table_full::preview(),
        "pricing-tiers" => blocks::pricing::tiers::preview(),
        "settings-billing" => blocks::settings::billing::preview(),
        "settings-profile" => blocks::settings::profile::preview(),
        "settings-team" => blocks::settings::team::preview(),
        "shell-sidebar" => blocks::shell::sidebar::preview(),
        _ => return None,
    };
    Some(markup)
}

/// Hand-written "Usage" snippet for a block. Shown below the preview
/// on each /blocks/{slug} page.
fn block_docs(slug: &str) -> Option<Markup> {
    let code = match slug {
        "auth-signup" => r#"use maud_ui::blocks::auth::signup;

signup::render(signup::Props {
    action: "/auth/signup".into(),
    heading: "Create your account".into(),
    subheading: "14-day free trial. No credit card required.".into(),
    terms_url: Some("/legal/terms".into()),
    privacy_url: Some("/legal/privacy".into()),
    signin_url: Some("/auth/login".into()),
    // Re-render after failed POST to preserve input + surface error:
    // email_value: submitted_email,
    // name_value: submitted_name,
    // error: Some("Email already in use".into()),
    ..Default::default()
})"#,
        "dashboard-stats" => r#"use maud_ui::blocks::dashboard::stats;

stats::render(stats::Props {
    title: Some("Overview".into()),
    subtitle: Some("Last 30 days".into()),
    cards: vec![
        stats::StatCard {
            label: "MRR".into(),
            value: "$42,310".into(),
            delta: Some(stats::Delta { value: "+12.4%".into(), positive: true }),
            hint: Some("vs last month".into()),
        },
        stats::StatCard {
            label: "Churn".into(),
            value: "2.1%".into(),
            // Set positive:true on a negative number for
            // "lower is better" metrics (churn, error rate).
            delta: Some(stats::Delta { value: "-0.4%".into(), positive: true }),
            hint: Some("lower is better".into()),
        },
    ],
    chart: None,         // Some(html! { (chart::render(...)) })
    activity: None,      // Some(vec![ ActivityItem { .. } ])
})"#,
        "shell-sidebar" => r#"use maud_ui::blocks::shell::sidebar;
use maud::html;

sidebar::render(sidebar::Props {
    brand: html! { span { "Acme" } },
    active_path: "/dashboard".into(),
    nav_groups: vec![
        sidebar::NavGroup {
            label: None,
            items: vec![
                sidebar::NavItem {
                    label: "Dashboard".into(),
                    href: "/dashboard".into(),
                    icon: None,
                    badge: None,
                },
                sidebar::NavItem {
                    label: "Inbox".into(),
                    href: "/inbox".into(),
                    icon: None,
                    badge: Some("12".into()),
                },
            ],
        },
    ],
    user: Some(sidebar::UserBlock {
        name: "Sofia Davis".into(),
        email: "sofia@acme.com".into(),
        avatar_initials: "SD".into(),
        menu_href: "/settings".into(),
    }),
    topbar_title: Some("Overview".into()),
    topbar_actions: html! {
        // any Markup — e.g. your primary action button or search
    },
    children: html! {
        // your route content here
    },
    ..Default::default()
})"#,
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
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
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
                    script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
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
        link rel="stylesheet" href=(format!("/css/maud-ui.css?v={}", CSS_VER));
        style { (showcase_css()) }
    }
}

/// Shared header with theme toggle.
fn page_header() -> Markup {
    html! {
        header.mui-showcase__header {
            div class="mui-showcase__header-inner" {
                a href="/" class="mui-showcase__brand" {
                    span class="mui-showcase__brand-name" { "maud-ui" }
                    span class="mui-showcase__brand-count" {
                        (format!("{} components", COMPONENT_NAMES.len()))
                    }
                }
                // Sidebar search — filters the visible nav list in
                // real time. Pure DOM filtering (no index, no fetch)
                // since the list is a few hundred items at most.
                div class="mui-showcase__search" {
                    span class="mui-showcase__search-icon" aria-hidden="true" {
                        (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="11" cy="11" r="7"/><path d="M21 21l-4.35-4.35"/></svg>"##.to_string()))
                    }
                    input type="search" id="mui-search" class="mui-showcase__search-input"
                          placeholder="Search components, blocks, integrations\u{2026}"
                          aria-label="Search the gallery"
                          spellcheck="false" autocomplete="off";
                    kbd class="mui-showcase__search-hint" aria-hidden="true" { "/" }
                }
                nav class="mui-showcase__nav" {
                    a href="/getting-started" class="mui-btn mui-btn--ghost mui-btn--sm" style="text-decoration:none;" {
                        "Get started"
                    }
                    a href="/blocks" class="mui-btn mui-btn--ghost mui-btn--sm" style="text-decoration:none;" {
                        "Blocks"
                    }
                    // "Advanced" dropdown — groups heavy-weight third-party
                    // integrations (Monaco, xyflow, Excalidraw) under a
                    // single header slot. Uses <details>/<summary> so it
                    // works with zero JS; the inline CSS below restyles
                    // the summary to look like a ghost button and pins
                    // the menu below it.
                    details class="mui-gallery__nav-advanced" {
                        summary class="mui-btn mui-btn--ghost mui-btn--sm mui-gallery__nav-advanced-summary" {
                            "Advanced"
                            span class="mui-gallery__nav-advanced-caret" aria-hidden="true" { "\u{25be}" }
                        }
                        div class="mui-gallery__nav-advanced-menu" role="menu" {
                            div class="mui-gallery__nav-advanced-group" {
                                span class="mui-gallery__nav-advanced-group-label" { "Code & Text" }
                                a href="/integrations/monaco-editor" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "Monaco editor" }
                                    span class="mui-gallery__nav-advanced-sub" { "VS Code's editor, embedded" }
                                }
                                a href="/integrations/tiptap" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "TipTap" }
                                    span class="mui-gallery__nav-advanced-sub" { "Rich text prose editor" }
                                }
                            }
                            div class="mui-gallery__nav-advanced-group" {
                                span class="mui-gallery__nav-advanced-group-label" { "Diagrams & Graphs" }
                                a href="/integrations/xyflow" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "xyflow" }
                                    span class="mui-gallery__nav-advanced-sub" { "React Flow node editor" }
                                }
                                a href="/integrations/cytoscape" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "Cytoscape" }
                                    span class="mui-gallery__nav-advanced-sub" { "Network graph visualisation" }
                                }
                                a href="/integrations/mermaid" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "Mermaid" }
                                    span class="mui-gallery__nav-advanced-sub" { "Text-to-diagram renderer" }
                                }
                            }
                            div class="mui-gallery__nav-advanced-group" {
                                span class="mui-gallery__nav-advanced-group-label" { "Canvas" }
                                a href="/integrations/excalidraw" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "Excalidraw" }
                                    span class="mui-gallery__nav-advanced-sub" { "Sketchy whiteboard canvas" }
                                }
                                a href="/integrations/threejs" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "Three.js" }
                                    span class="mui-gallery__nav-advanced-sub" { "WebGL 3D scene" }
                                }
                            }
                            div class="mui-gallery__nav-advanced-group" {
                                span class="mui-gallery__nav-advanced-group-label" { "Data" }
                                a href="/integrations/ag-grid" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "AG Grid" }
                                    span class="mui-gallery__nav-advanced-sub" { "Enterprise data grid" }
                                }
                                a href="/integrations/echarts" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "Apache ECharts" }
                                    span class="mui-gallery__nav-advanced-sub" { "Charting library" }
                                }
                            }
                            div class="mui-gallery__nav-advanced-group" {
                                span class="mui-gallery__nav-advanced-group-label" { "Maps & Scheduling" }
                                a href="/integrations/leaflet" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "Leaflet" }
                                    span class="mui-gallery__nav-advanced-sub" { "Interactive maps" }
                                }
                                a href="/integrations/fullcalendar" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "FullCalendar" }
                                    span class="mui-gallery__nav-advanced-sub" { "Scheduling, drag-drop events" }
                                }
                            }
                            div class="mui-gallery__nav-advanced-group" {
                                span class="mui-gallery__nav-advanced-group-label" { "Media" }
                                a href="/integrations/wavesurfer" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "Wavesurfer" }
                                    span class="mui-gallery__nav-advanced-sub" { "Audio waveform player" }
                                }
                                a href="/integrations/pdfjs" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "PDF.js" }
                                    span class="mui-gallery__nav-advanced-sub" { "Inline PDF viewer" }
                                }
                            }
                            div class="mui-gallery__nav-advanced-group" {
                                span class="mui-gallery__nav-advanced-group-label" { "Terminal" }
                                a href="/integrations/xterm" role="menuitem" {
                                    span class="mui-gallery__nav-advanced-label" { "xterm.js" }
                                    span class="mui-gallery__nav-advanced-sub" { "Terminal emulator" }
                                }
                            }
                        }
                    }
                    a href="https://docs.rs/maud-ui" target="_blank" rel="noopener" class="mui-btn mui-btn--ghost mui-btn--sm" style="text-decoration:none;" {
                        "Docs"
                    }
                    a href="https://github.com/hgeldenhuys/maud-ui" target="_blank" rel="noopener" class="mui-btn mui-btn--ghost mui-btn--sm" style="text-decoration:none;" {
                        "GitHub"
                    }
                }
                div class="mui-showcase__tools" {
                    button type="button"
                           class="mui-btn mui-btn--outline mui-btn--sm mui-showcase__tool-btn"
                           data-mui="theme-toggle" id="theme-toggle"
                           title="Toggle theme" aria-label="Toggle theme" {
                        "\u{25D0}"
                    }
                    button type="button"
                           class="mui-btn mui-btn--outline mui-btn--sm mui-showcase__tool-btn"
                           data-mui="dir-toggle" id="dir-toggle"
                           title="Toggle reading direction" aria-label="Toggle reading direction" {
                        "\u{21C4}"
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
                        // Tier header → `/#slug` so it always navigates to the
                        // index gallery first and then scrolls to the tier
                        // section, no matter which page the user is on.
                        a class="mui-gallery__nav-tier" href=(format!("/#{}", tier.slug)) {
                            (tier.title)
                        }
                        div class="mui-gallery__nav-items" {
                            @for comp in tier.components {
                                @if component_content(comp).is_some() {
                                    // Component link → `/{slug}` (absolute)
                                    // so it navigates to the component's own
                                    // page from any context. The in-page
                                    // IntersectionObserver on `/` still
                                    // tracks scroll-based active state via
                                    // the #{slug} anchor next to each name.
                                    a class="mui-gallery__nav-item" href=(format!("/{}", comp)) data-slug=(comp) {
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
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
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
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

/// `/integrations/monaco-editor` — embed Monaco (VS Code's editor)
/// inside maud-ui chrome. Demonstrates that the primitive + block CSS
/// cooperates cleanly with heavyweight third-party widgets.
///
/// Monaco is loaded from the jsdelivr CDN; no npm dep needed on the
/// consumer side. The editor syncs to `<html data-theme>` so the
/// light/dark toggle in the header flips the Monaco theme too.
pub fn integrations_monaco_page() -> Markup {
    let sample_rust = r##"// maud-ui login handler — returns the login block as HTML.
use axum::{extract::Form, response::Redirect, Router, routing::post};
use maud_ui::blocks::auth::login;
use maud::{html, Markup};

#[derive(serde::Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

async fn submit(Form(f): Form<LoginForm>) -> Result<Redirect, Markup> {
    match authenticate(&f.email, &f.password).await {
        Ok(user) => Ok(Redirect::to(&format!("/u/{}", user.id))),
        Err(err) => Err(login::render(login::Props {
            action: "/login".into(),
            email_value: f.email,
            error: Some(err.to_string()),
            ..Default::default()
        })),
    }
}

pub fn routes() -> Router {
    Router::new().route("/login", post(submit))
}
"##;

    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Monaco editor \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" }
                            span { " / " }
                            span { "Integrations" }
                            span { " / " }
                            span { "Monaco editor" }
                        }

                        section class="mui-gallery__component" id="integration-monaco" {
                            h3 class="mui-gallery__component-name" { "Monaco editor \u{2014} Integration" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "The editor behind VS Code, embedded inside a maud-ui shell. File header, toolbar, language dropdown, and status bar are all plain maud-ui primitives wrapping the Monaco instance. The editor theme auto-syncs with "
                                code style="font-family:var(--mui-font-mono);font-size:0.875rem;" { "<html data-theme>" }
                                " so toggling the gallery theme flips Monaco too."
                            }

                            // The integration frame
                            div class="mui-integration mui-integration--monaco" {
                                // File header row
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(
                                                r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/></svg>"##.to_string()))
                                        }
                                        span id="mui-monaco-filename" { "src/routes/auth.rs" }
                                        span class="mui-integration__dirty" id="mui-monaco-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        select id="mui-monaco-lang" class="mui-integration__select" aria-label="Language" {
                                            option value="rust" selected { "Rust" }
                                            option value="typescript" { "TypeScript" }
                                            option value="javascript" { "JavaScript" }
                                            option value="html" { "HTML" }
                                            option value="css" { "CSS" }
                                            option value="json" { "JSON" }
                                            option value="markdown" { "Markdown" }
                                            option value="sql" { "SQL" }
                                            option value="python" { "Python" }
                                            option value="go" { "Go" }
                                        }
                                        button type="button" id="mui-monaco-format"
                                               class="mui-btn mui-btn--outline mui-btn--sm"
                                               aria-label="Format document" { "Format" }
                                        button type="button" id="mui-monaco-run"
                                               class="mui-btn mui-btn--primary mui-btn--sm" { "Run" }
                                    }
                                }

                                // The editor mount point
                                div class="mui-integration__editor" id="mui-monaco-editor" {
                                    // Loading placeholder shown until Monaco finishes loading
                                    div class="mui-integration__loading" id="mui-monaco-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading Monaco from CDN\u{2026}" }
                                    }
                                }

                                // Status bar at the bottom
                                div class="mui-integration__statusbar" {
                                    span id="mui-monaco-status-lang" { "Rust" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-monaco-status-pos" { "Ln 1, Col 1" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-monaco-status-lines" { "0 lines" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-monaco-status-theme" {
                                        "maud-ui-dark"
                                    }
                                }
                            }

                            // Output panel (shows "Run" clicks)
                            div class="mui-integration__output" {
                                div class="mui-integration__output-header" {
                                    span { "Output" }
                                    button type="button" id="mui-monaco-clear"
                                           class="mui-btn mui-btn--ghost mui-btn--sm" { "Clear" }
                                }
                                pre class="mui-integration__output-body" id="mui-monaco-output" {
                                    "// Click " span style="color:var(--mui-accent-text);" { "Run" } " to see output here."
                                }
                            }

                            (code_example("Usage — drop Monaco into any maud-ui page", r##"// In your <head>:
//   <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/monaco-editor@0.50.0/min/vs/editor/editor.main.css">
//   <script src="https://cdn.jsdelivr.net/npm/monaco-editor@0.50.0/min/vs/loader.min.js"></script>
//
// In your body (where you want the editor):
html! {
    div class="mui-integration mui-integration--monaco" {
        div class="mui-integration__header" {
            div class="mui-integration__filepath" { "src/main.rs" }
            div class="mui-integration__toolbar" {
                (button::render(button::Props {
                    label: "Format".into(),
                    variant: button::Variant::Outline,
                    size: button::Size::Sm,
                    ..Default::default()
                }))
            }
        }
        div class="mui-integration__editor" id="editor" {}
    }
}

// Then bootstrap Monaco (matches --mui-* theme tokens automatically):
// require.config({ paths: { vs: 'https://cdn.jsdelivr.net/.../min/vs' } });
// require(['vs/editor/editor.main'], () => {
//   monaco.editor.defineTheme('maud-ui-dark', { base: 'vs-dark', ... });
//   monaco.editor.create(document.getElementById('editor'), {
//     value: '// your code here',
//     language: 'rust',
//     theme: 'maud-ui-dark',
//     fontFamily: 'var(--mui-font-mono)',
//     minimap: { enabled: false },
//     automaticLayout: true,
//   });
// });
"##))
                        }

                        div class="mui-gallery__back" {
                            a href="/" class="mui-btn mui-btn--outline mui-btn--sm" {
                                "\u{2190} Back to Gallery"
                            }
                        }
                    }
                }

                // Sample code injected as a JS constant so the page loads
                // without Monaco and the bootstrap can hydrate with this
                // content once Monaco's AMD modules resolve.
                script {
                    (maud::PreEscaped(format!(
                        "window.__MUI_MONACO_SAMPLE__ = {};",
                        serde_json_lite_escape(sample_rust),
                    )))
                }

                // Monaco CDN loader + bootstrap
                link rel="stylesheet" data-name="vs/editor/editor.main"
                     href="https://cdn.jsdelivr.net/npm/monaco-editor@0.50.0/min/vs/editor/editor.main.css";
                script src="https://cdn.jsdelivr.net/npm/monaco-editor@0.50.0/min/vs/loader.min.js" {}
                script { (maud::PreEscaped(monaco_bootstrap())) }

                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

/// Inline CSS for the integration layout — shell around the editor,
/// header/toolbar row, status bar, output panel. Kept inline (not in
/// the bundle) since it's demo-only, not part of the public `mui-*`
/// API. Shared by Monaco, xyflow, and Excalidraw integration pages.
fn integration_shell_css() -> &'static str {
    r#"
.mui-integration {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-lg);
    overflow: hidden;
    background: var(--mui-bg-card);
}

.mui-integration__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid var(--mui-border);
    background: var(--mui-bg);
    min-height: 2.75rem;
    flex-wrap: wrap;
}

.mui-integration__filepath {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--mui-text);
    font-family: var(--mui-font-mono);
    font-size: 0.8125rem;
}

.mui-integration__filepath-icon {
    color: var(--mui-text-muted);
    display: inline-flex;
}

.mui-integration__dirty {
    color: var(--mui-text-subtle);
    font-size: 0.75rem;
    transition: color var(--mui-transition);
}

.mui-integration__dirty[data-dirty="true"] {
    color: var(--mui-accent-text);
}

.mui-integration__toolbar {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
}

.mui-integration__select {
    height: 2rem;
    padding: 0 2rem 0 0.75rem;
    font-size: 0.8125rem;
    background: var(--mui-bg-card);
    color: var(--mui-text);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-md);
    appearance: none;
    -webkit-appearance: none;
    background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 24 24' fill='none' stroke='%23a1a1aa' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'><polyline points='6 9 12 15 18 9'/></svg>");
    background-repeat: no-repeat;
    background-position: right 0.5rem center;
    background-size: 1rem;
    cursor: pointer;
    font-family: inherit;
}

.mui-integration__select:focus-visible {
    outline: 2px solid var(--mui-border-focus);
    outline-offset: 1px;
}

.mui-integration__editor {
    position: relative;
    height: 24rem;
    min-height: 20rem;
    background: var(--mui-bg);
}

@media (min-width: 1024px) {
    .mui-integration__editor {
        height: 28rem;
    }
}

.mui-integration__loading {
    position: absolute;
    inset: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    color: var(--mui-text-muted);
    font-size: 0.875rem;
    z-index: 1;
}

/* Fade out the loading overlay once Monaco mounts. */
.mui-integration__editor[data-ready="true"] .mui-integration__loading {
    opacity: 0;
    pointer-events: none;
    transition: opacity 200ms;
}

.mui-integration__statusbar {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.375rem 0.75rem;
    border-top: 1px solid var(--mui-border);
    background: var(--mui-bg);
    font-family: var(--mui-font-mono);
    font-size: 0.75rem;
    color: var(--mui-text-muted);
}

.mui-integration__statusbar-sep {
    color: var(--mui-text-subtle);
    font-size: 0.625rem;
}

.mui-integration__statusbar-spacer {
    flex: 1;
}

.mui-integration__statusbar-theme {
    color: var(--mui-accent-text);
    font-weight: 500;
}

/* Output panel */
.mui-integration__output {
    margin-top: 1rem;
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-md);
    overflow: hidden;
    background: var(--mui-bg-card);
}

.mui-integration__output-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    border-bottom: 1px solid var(--mui-border);
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-weight: 600;
    color: var(--mui-text-muted);
    background: var(--mui-bg);
}

.mui-integration__output-body {
    margin: 0;
    padding: 0.875rem 0.75rem;
    font-family: var(--mui-font-mono);
    font-size: 0.8125rem;
    color: var(--mui-text);
    white-space: pre-wrap;
    max-height: 12rem;
    overflow-y: auto;
    line-height: 1.5;
}
"#
}

/// Inline JS that boots Monaco via AMD loader, registers a maud-ui
/// theme, wires the toolbar/status-bar widgets, and reacts to
/// `data-theme` flips on the <html> element.
fn monaco_bootstrap() -> &'static str {
    r##"
(function () {
  'use strict';
  // Wait for the AMD loader (served separately by <script src=...>).
  if (typeof require !== 'function' || !require.config) {
    console.warn('[maud-ui] Monaco loader not present — skipping editor init.');
    return;
  }

  require.config({ paths: { vs: 'https://cdn.jsdelivr.net/npm/monaco-editor@0.50.0/min/vs' } });

  require(['vs/editor/editor.main'], function () {
    // Resolve token colors from the live --mui-* variables so the editor
    // matches whatever custom theme the consumer is running.
    function readToken(name) {
      return getComputedStyle(document.documentElement).getPropertyValue(name).trim();
    }

    function defineThemes() {
      monaco.editor.defineTheme('maud-ui-dark', {
        base: 'vs-dark',
        inherit: true,
        rules: [
          { token: '', foreground: readToken('--mui-text').replace('#','') || 'fafafa' },
          { token: 'comment', foreground: '71717a', fontStyle: 'italic' },
          { token: 'keyword', foreground: '60a5fa' },
          { token: 'string', foreground: '4ade80' },
          { token: 'number', foreground: 'f472b6' },
          { token: 'type', foreground: '818cf8' },
        ],
        colors: {
          'editor.background': readToken('--mui-bg') || '#0a0a0b',
          'editor.foreground': readToken('--mui-text') || '#fafafa',
          'editor.lineHighlightBackground': readToken('--mui-bg-card') || '#111113',
          'editorLineNumber.foreground': readToken('--mui-text-subtle') || '#8e8e93',
          'editorLineNumber.activeForeground': readToken('--mui-text-muted') || '#a1a1aa',
          'editorGutter.background': readToken('--mui-bg') || '#0a0a0b',
          'editor.selectionBackground': '#264f78',
          'editor.inactiveSelectionBackground': '#3a3d41',
          'editorCursor.foreground': readToken('--mui-accent') || '#2563eb',
          'editorIndentGuide.background': readToken('--mui-border') || '#27272a',
          'editorIndentGuide.activeBackground': readToken('--mui-border-hover') || '#3f3f46',
          'editorWhitespace.foreground': '#3f3f46',
        },
      });

      monaco.editor.defineTheme('maud-ui-light', {
        base: 'vs',
        inherit: true,
        rules: [
          { token: 'comment', foreground: '71717a', fontStyle: 'italic' },
          { token: 'keyword', foreground: '1d4ed8' },
          { token: 'string', foreground: '15803d' },
          { token: 'type', foreground: '6d28d9' },
        ],
        colors: {
          'editor.background': '#ffffff',
          'editor.foreground': '#09090b',
          'editorLineNumber.foreground': '#a1a1aa',
          'editorLineNumber.activeForeground': '#71717a',
          'editorCursor.foreground': '#2563eb',
          'editorIndentGuide.background': '#e4e4e7',
        },
      });
    }

    defineThemes();

    function pickTheme() {
      return (document.documentElement.getAttribute('data-theme') || 'dark') === 'light'
        ? 'maud-ui-light'
        : 'maud-ui-dark';
    }

    var host = document.getElementById('mui-monaco-editor');
    if (!host) return;

    var editor = monaco.editor.create(host, {
      value: (window.__MUI_MONACO_SAMPLE__ || '// empty\n'),
      language: 'rust',
      theme: pickTheme(),
      fontSize: 13,
      fontFamily: getComputedStyle(document.documentElement).getPropertyValue('--mui-font-mono').trim() || 'ui-monospace, monospace',
      fontLigatures: true,
      minimap: { enabled: false },
      scrollBeyondLastLine: false,
      automaticLayout: true,
      roundedSelection: false,
      padding: { top: 12, bottom: 12 },
      renderLineHighlight: 'all',
      smoothScrolling: true,
      cursorBlinking: 'smooth',
    });

    host.setAttribute('data-ready', 'true');

    // ── Toolbar wiring ────────────────────────────────────────
    var lang = document.getElementById('mui-monaco-lang');
    var fileEl = document.getElementById('mui-monaco-filename');
    var statusLang = document.getElementById('mui-monaco-status-lang');
    var statusPos = document.getElementById('mui-monaco-status-pos');
    var statusLines = document.getElementById('mui-monaco-status-lines');
    var statusTheme = document.getElementById('mui-monaco-status-theme');
    var dirty = document.getElementById('mui-monaco-dirty');
    var output = document.getElementById('mui-monaco-output');

    var LANG_TO_EXT = {
      rust: 'rs', typescript: 'ts', javascript: 'js', html: 'html',
      css: 'css', json: 'json', markdown: 'md', sql: 'sql',
      python: 'py', go: 'go',
    };
    var LANG_DISPLAY = {
      rust: 'Rust', typescript: 'TypeScript', javascript: 'JavaScript',
      html: 'HTML', css: 'CSS', json: 'JSON', markdown: 'Markdown',
      sql: 'SQL', python: 'Python', go: 'Go',
    };

    lang.addEventListener('change', function () {
      monaco.editor.setModelLanguage(editor.getModel(), lang.value);
      statusLang.textContent = LANG_DISPLAY[lang.value] || lang.value;
      fileEl.textContent = 'src/scratch.' + (LANG_TO_EXT[lang.value] || 'txt');
    });

    document.getElementById('mui-monaco-format').addEventListener('click', function () {
      editor.getAction('editor.action.formatDocument').run();
    });

    document.getElementById('mui-monaco-run').addEventListener('click', function () {
      var code = editor.getValue();
      var lines = code.split('\n').length;
      var now = new Date().toLocaleTimeString();
      var langName = LANG_DISPLAY[lang.value] || lang.value;
      output.textContent =
        '[' + now + '] would run ' + langName + ' — ' +
        lines + ' lines, ' + code.length + ' chars.\n' +
        '  (this demo doesn\'t actually execute — your app wires Run to its own endpoint.)';
    });

    document.getElementById('mui-monaco-clear').addEventListener('click', function () {
      output.textContent = '// cleared.';
    });

    // ── Status bar updates ───────────────────────────────────
    editor.onDidChangeCursorPosition(function (e) {
      statusPos.textContent = 'Ln ' + e.position.lineNumber + ', Col ' + e.position.column;
    });

    function updateLines() {
      statusLines.textContent = editor.getModel().getLineCount() + ' lines';
    }
    updateLines();

    editor.onDidChangeModelContent(function () {
      updateLines();
      dirty.setAttribute('data-dirty', 'true');
    });

    // ── Theme sync with data-theme toggle ────────────────────
    var themeObserver = new MutationObserver(function () {
      var next = pickTheme();
      // Redefine themes so they pick up fresh CSS variables after a
      // toggle (tokens change on :root[data-theme]).
      defineThemes();
      monaco.editor.setTheme(next);
      statusTheme.textContent = next;
    });
    themeObserver.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['data-theme'],
    });
  });
})();
"##
}

/// `/integrations/xyflow` — embed xyflow (React Flow) node editor
/// inside a maud-ui shell. Uses native ESM + importmap to pull React
/// and `@xyflow/react` from esm.sh at runtime, no build step.
pub fn integrations_xyflow_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("xyflow \u{2014} maud-ui integrations"))
                // Shell (header/editor/statusbar/output shared with Monaco)
                // must load BEFORE the per-integration overrides, otherwise
                // `.mui-integration__editor` has no height and the graph
                // collapses to 0px.
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(xyflow_css())) }
                // xyflow ships its CSS separately; pull it from esm.sh so
                // edges, handles, and controls render correctly.
                link rel="stylesheet"
                     href="https://esm.sh/@xyflow/react@12.3.6/dist/style.css";
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" }
                            span { " / " }
                            span { "Integrations" }
                            span { " / " }
                            span { "xyflow" }
                        }

                        section class="mui-gallery__component" id="integration-xyflow" {
                            h3 class="mui-gallery__component-name" { "xyflow \u{2014} Node editor" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "A React Flow / xyflow graph editor embedded in a maud-ui shell. "
                                "Nodes are draggable, edges are connectable, the minimap and controls "
                                "are wired, and the React Flow colour mode flips automatically with "
                                "the gallery's "
                                code style="font-family:var(--mui-font-mono);font-size:0.875rem;" { "data-theme" }
                                " attribute. Loaded from "
                                code style="font-family:var(--mui-font-mono);font-size:0.875rem;" { "esm.sh" }
                                " at runtime \u{2014} no bundler."
                            }

                            div class="mui-integration mui-integration--xyflow" {
                                // File header row (mirrors the Monaco demo shell)
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(
                                                r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="6" cy="6" r="3"/><circle cx="18" cy="6" r="3"/><circle cx="12" cy="18" r="3"/><path d="M6 9v3a3 3 0 0 0 3 3h6a3 3 0 0 0 3-3V9"/><path d="M12 12v3"/></svg>"##.to_string()))
                                        }
                                        span id="mui-xyflow-title" { "flows/api-pipeline.json" }
                                        span class="mui-integration__dirty" id="mui-xyflow-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        select id="mui-xyflow-layout" class="mui-integration__select" aria-label="Layout" {
                                            option value="horizontal" selected { "Horizontal" }
                                            option value="vertical" { "Vertical" }
                                        }
                                        button type="button" id="mui-xyflow-add"
                                               class="mui-btn mui-btn--outline mui-btn--sm" { "+ Node" }
                                        button type="button" id="mui-xyflow-fit"
                                               class="mui-btn mui-btn--outline mui-btn--sm" { "Fit" }
                                        button type="button" id="mui-xyflow-reset"
                                               class="mui-btn mui-btn--ghost mui-btn--sm" { "Reset" }
                                        button type="button" id="mui-xyflow-export"
                                               class="mui-btn mui-btn--primary mui-btn--sm" { "Export JSON" }
                                    }
                                }

                                // The flow editor mount point — React Flow needs
                                // a positioned parent with explicit dimensions,
                                // which `.mui-integration__editor` already provides.
                                div class="mui-integration__editor" id="mui-xyflow-root" {
                                    div class="mui-integration__loading" id="mui-xyflow-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading xyflow + React from esm.sh\u{2026}" }
                                    }
                                }

                                div class="mui-integration__statusbar" {
                                    span id="mui-xyflow-status-nodes" { "0 nodes" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-xyflow-status-edges" { "0 edges" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-xyflow-status-selected" { "No selection" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-xyflow-status-theme" {
                                        "dark"
                                    }
                                }
                            }

                            // Export panel — shows JSON dump of current graph
                            div class="mui-integration__output" {
                                div class="mui-integration__output-header" {
                                    span { "Graph JSON" }
                                    button type="button" id="mui-xyflow-clear-output"
                                           class="mui-btn mui-btn--ghost mui-btn--sm" { "Clear" }
                                }
                                pre class="mui-integration__output-body" id="mui-xyflow-output" {
                                    "// Click " span style="color:var(--mui-accent-text);" { "Export JSON" } " to dump the current nodes and edges."
                                }
                            }

                            (code_example("Usage \u{2014} mount xyflow inside a maud-ui page", r##"// In your <head>:
//   <link rel="stylesheet" href="https://esm.sh/@xyflow/react@12.3.6/dist/style.css">
//   <script type="importmap">
//     { "imports": {
//         "react":           "https://esm.sh/react@18.3.1",
//         "react-dom/client":"https://esm.sh/react-dom@18.3.1/client",
//         "@xyflow/react":   "https://esm.sh/@xyflow/react@12.3.6?deps=react@18.3.1"
//     }}
//   </script>
//
// Your maud template renders an empty mount point:
html! {
    div class="mui-integration mui-integration--xyflow" {
        div class="mui-integration__header" { /* toolbar */ }
        div class="mui-integration__editor" id="flow-root" {}
    }
}

// Then a <script type="module"> mounts ReactFlow into the div:
//   import { createRoot } from 'react-dom/client';
//   import { ReactFlow, Background, Controls, MiniMap } from '@xyflow/react';
//   const root = createRoot(document.getElementById('flow-root'));
//   root.render(React.createElement(ReactFlow, {
//     defaultNodes: [/* ... */],
//     defaultEdges: [/* ... */],
//     fitView: true,
//     colorMode: document.documentElement.dataset.theme || 'dark',
//   }, React.createElement(Background), React.createElement(Controls)));
"##))
                        }

                        div class="mui-gallery__back" {
                            a href="/" class="mui-btn mui-btn--outline mui-btn--sm" {
                                "\u{2190} Back to Gallery"
                            }
                        }
                    }
                }

                // Native ESM importmap — maps bare specifiers to esm.sh URLs.
                // `?deps=react@18.3.1,react-dom@18.3.1` pins xyflow's React
                // to the exact copy we provide, avoiding a dual-React crash
                // (same root cause as WoW #104 in the Kapable workspace).
                script type="importmap" {
                    (maud::PreEscaped(r##"{
  "imports": {
    "react":              "https://esm.sh/react@18.3.1",
    "react-dom":          "https://esm.sh/react-dom@18.3.1",
    "react-dom/client":   "https://esm.sh/react-dom@18.3.1/client",
    "react/jsx-runtime":  "https://esm.sh/react@18.3.1/jsx-runtime",
    "@xyflow/react":      "https://esm.sh/@xyflow/react@12.3.6?deps=react@18.3.1,react-dom@18.3.1"
  }
}"##))
                }
                script type="module" { (maud::PreEscaped(xyflow_bootstrap())) }

                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

/// Inline CSS for the xyflow integration. Scopes overrides to
/// `.mui-integration--xyflow` so xyflow's own stylesheet still drives
/// handles / edges / minimap internals, while the host shell gets
/// theme-aware colours.
fn xyflow_css() -> &'static str {
    r#"
.mui-integration--xyflow .react-flow__renderer,
.mui-integration--xyflow .react-flow__viewport {
    background: var(--mui-bg);
}

/* Tame xyflow's default node chrome to match maud-ui tokens.
 * We target the *default* node only — custom node types will keep
 * their own styling. */
.mui-integration--xyflow .react-flow__node-default,
.mui-integration--xyflow .react-flow__node-input,
.mui-integration--xyflow .react-flow__node-output {
    background: var(--mui-bg-card);
    color: var(--mui-text);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-md);
    font-family: var(--mui-font-sans);
    font-size: 0.8125rem;
    padding: 0.5rem 0.875rem;
    box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
    transition: border-color var(--mui-transition),
                box-shadow var(--mui-transition);
}

.mui-integration--xyflow .react-flow__node-default:hover,
.mui-integration--xyflow .react-flow__node-input:hover,
.mui-integration--xyflow .react-flow__node-output:hover {
    border-color: var(--mui-border-hover);
}

.mui-integration--xyflow .react-flow__node.selected > * {
    border-color: var(--mui-accent);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--mui-accent) 30%, transparent);
}

.mui-integration--xyflow .react-flow__handle {
    background: var(--mui-accent);
    border: 2px solid var(--mui-bg);
    width: 10px;
    height: 10px;
}

.mui-integration--xyflow .react-flow__edge-path {
    stroke: var(--mui-border-hover);
    stroke-width: 1.5;
}

.mui-integration--xyflow .react-flow__edge.selected .react-flow__edge-path,
.mui-integration--xyflow .react-flow__edge:hover .react-flow__edge-path {
    stroke: var(--mui-accent);
}

.mui-integration--xyflow .react-flow__minimap {
    background: var(--mui-bg-card);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-md);
}

.mui-integration--xyflow .react-flow__controls {
    background: var(--mui-bg-card);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-md);
    overflow: hidden;
    box-shadow: none;
}

.mui-integration--xyflow .react-flow__controls-button {
    background: var(--mui-bg-card);
    border-bottom: 1px solid var(--mui-border);
    color: var(--mui-text);
    width: 24px;
    height: 24px;
}
.mui-integration--xyflow .react-flow__controls-button:last-child { border-bottom: 0; }
.mui-integration--xyflow .react-flow__controls-button:hover {
    background: var(--mui-bg);
}
.mui-integration--xyflow .react-flow__controls-button svg { fill: currentColor; }

.mui-integration--xyflow .react-flow__attribution {
    background: transparent;
    color: var(--mui-text-subtle);
    font-size: 0.625rem;
    opacity: 0.6;
}
.mui-integration--xyflow .react-flow__attribution a { color: var(--mui-text-subtle); }

/* Background pattern colour via CSS var so dot/grid blends into the theme */
.mui-integration--xyflow .react-flow__background {
    background-color: var(--mui-bg);
}
"#
}

/// Inline ES module that pulls React + xyflow from esm.sh (via the
/// importmap above), seeds a sample pipeline graph, and wires the
/// toolbar / status bar. Kept in one string so the static exporter
/// ships it verbatim.
fn xyflow_bootstrap() -> &'static str {
    r##"
import * as React from 'react';
import { createRoot } from 'react-dom/client';
import {
  ReactFlow,
  Background,
  Controls,
  MiniMap,
  addEdge,
  applyNodeChanges,
  applyEdgeChanges,
  MarkerType,
} from '@xyflow/react';

const h = React.createElement;

// ── Sample graph — a plausible API request pipeline ──────────────
const INITIAL_NODES = [
  { id: '1', type: 'input',  position: { x: 0,   y: 80  }, data: { label: 'HTTP Request' } },
  { id: '2',                 position: { x: 180, y: 80  }, data: { label: 'Auth middleware' } },
  { id: '3',                 position: { x: 360, y: 0   }, data: { label: 'Validate body' } },
  { id: '4',                 position: { x: 360, y: 160 }, data: { label: 'Rate limit' } },
  { id: '5',                 position: { x: 540, y: 80  }, data: { label: 'Handler' } },
  { id: '6',                 position: { x: 720, y: 0   }, data: { label: 'SQL query' } },
  { id: '7',                 position: { x: 720, y: 160 }, data: { label: 'Cache write' } },
  { id: '8', type: 'output', position: { x: 900, y: 80  }, data: { label: 'Response' } },
];

const INITIAL_EDGES = [
  { id: 'e1-2', source: '1', target: '2', animated: true,  markerEnd: { type: MarkerType.ArrowClosed } },
  { id: 'e2-3', source: '2', target: '3', markerEnd: { type: MarkerType.ArrowClosed } },
  { id: 'e2-4', source: '2', target: '4', markerEnd: { type: MarkerType.ArrowClosed } },
  { id: 'e3-5', source: '3', target: '5', markerEnd: { type: MarkerType.ArrowClosed } },
  { id: 'e4-5', source: '4', target: '5', markerEnd: { type: MarkerType.ArrowClosed } },
  { id: 'e5-6', source: '5', target: '6', markerEnd: { type: MarkerType.ArrowClosed } },
  { id: 'e5-7', source: '5', target: '7', markerEnd: { type: MarkerType.ArrowClosed } },
  { id: 'e6-8', source: '6', target: '8', animated: true, markerEnd: { type: MarkerType.ArrowClosed } },
  { id: 'e7-8', source: '7', target: '8', animated: true, markerEnd: { type: MarkerType.ArrowClosed } },
];

function pickColorMode() {
  return document.documentElement.getAttribute('data-theme') === 'light' ? 'light' : 'dark';
}

function Flow({ onGraphChange, apiRef }) {
  const [nodes, setNodes] = React.useState(INITIAL_NODES);
  const [edges, setEdges] = React.useState(INITIAL_EDGES);
  const [colorMode, setColorMode] = React.useState(pickColorMode());
  const [selectedId, setSelectedId] = React.useState(null);

  const onNodesChange = React.useCallback(
    (changes) => setNodes((ns) => applyNodeChanges(changes, ns)),
    [],
  );
  const onEdgesChange = React.useCallback(
    (changes) => setEdges((es) => applyEdgeChanges(changes, es)),
    [],
  );
  const onConnect = React.useCallback(
    (params) => setEdges((es) => addEdge({ ...params, markerEnd: { type: MarkerType.ArrowClosed } }, es)),
    [],
  );

  // Publish graph state & imperative API back up to the bootstrap
  // script so the toolbar can drive it without re-rendering React.
  React.useEffect(() => {
    onGraphChange?.(nodes, edges, selectedId);
  }, [nodes, edges, selectedId, onGraphChange]);

  React.useEffect(() => {
    apiRef.current = {
      addNode: (label) => {
        const id = String(Date.now());
        const offsetX = 200 + Math.random() * 500;
        const offsetY = 40 + Math.random() * 200;
        setNodes((ns) => [...ns, { id, position: { x: offsetX, y: offsetY }, data: { label } }]);
      },
      reset: () => {
        setNodes(INITIAL_NODES);
        setEdges(INITIAL_EDGES);
      },
      layout: (direction) => {
        // Simple deterministic layout — no dagre dependency, just
        // bucket nodes by column order and distribute them vertically
        // (or horizontally, flipped).
        setNodes((ns) => {
          const sorted = [...ns].sort((a, b) => Number(a.id) - Number(b.id));
          return sorted.map((n, i) => ({
            ...n,
            position: direction === 'vertical'
              ? { x: 120 + (i % 3) * 220, y: 40 + Math.floor(i / 3) * 140 }
              : { x: 40 + i * 150,         y: 80 + (i % 2) * 120 },
          }));
        });
      },
    };
  }, [apiRef]);

  // Sync React colorMode with the gallery's data-theme attribute.
  React.useEffect(() => {
    const obs = new MutationObserver(() => setColorMode(pickColorMode()));
    obs.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
    return () => obs.disconnect();
  }, []);

  // Surface current theme to the status bar.
  React.useEffect(() => {
    const badge = document.getElementById('mui-xyflow-status-theme');
    if (badge) badge.textContent = colorMode;
  }, [colorMode]);

  return h(
    ReactFlow,
    {
      nodes,
      edges,
      onNodesChange,
      onEdgesChange,
      onConnect,
      onSelectionChange: (sel) => {
        const first = sel.nodes?.[0] || sel.edges?.[0];
        setSelectedId(first ? first.id : null);
      },
      fitView: true,
      colorMode,
      proOptions: { hideAttribution: false },
    },
    h(Background, { gap: 16, size: 1, color: 'var(--mui-border)' }),
    h(Controls, { showInteractive: false }),
    h(MiniMap, { pannable: true, zoomable: true, maskColor: 'rgba(0,0,0,0.4)' }),
  );
}

// ── Mount ─────────────────────────────────────────────────────────
const host = document.getElementById('mui-xyflow-root');
if (host) {
  // Hide the loading overlay as soon as we have a first paint.
  requestAnimationFrame(() => host.setAttribute('data-ready', 'true'));

  const apiRef = { current: null };
  let latestNodes = INITIAL_NODES, latestEdges = INITIAL_EDGES;
  const statusNodes = document.getElementById('mui-xyflow-status-nodes');
  const statusEdges = document.getElementById('mui-xyflow-status-edges');
  const statusSel   = document.getElementById('mui-xyflow-status-selected');
  const dirty       = document.getElementById('mui-xyflow-dirty');
  const output      = document.getElementById('mui-xyflow-output');

  function onGraphChange(nodes, edges, selectedId) {
    latestNodes = nodes;
    latestEdges = edges;
    if (statusNodes) statusNodes.textContent = nodes.length + ' node' + (nodes.length === 1 ? '' : 's');
    if (statusEdges) statusEdges.textContent = edges.length + ' edge' + (edges.length === 1 ? '' : 's');
    if (statusSel) {
      if (selectedId) {
        const match = nodes.find((n) => n.id === selectedId);
        statusSel.textContent = match ? 'Selected: ' + (match.data.label || match.id) : 'Edge: ' + selectedId;
      } else {
        statusSel.textContent = 'No selection';
      }
    }
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  }

  const root = createRoot(host);
  root.render(h(Flow, { onGraphChange, apiRef }));

  // ── Toolbar wiring ────────────────────────────────────────────
  const addBtn = document.getElementById('mui-xyflow-add');
  if (addBtn) {
    let n = 0;
    addBtn.addEventListener('click', () => apiRef.current?.addNode('New step ' + (++n)));
  }

  const resetBtn = document.getElementById('mui-xyflow-reset');
  if (resetBtn) {
    resetBtn.addEventListener('click', () => {
      apiRef.current?.reset();
      if (dirty) dirty.setAttribute('data-dirty', 'false');
    });
  }

  const fitBtn = document.getElementById('mui-xyflow-fit');
  if (fitBtn) {
    fitBtn.addEventListener('click', () => {
      // `Controls` exposes Fit — easiest path is a synthetic click on
      // xyflow's built-in fit-view control button.
      const fitControl = host.querySelector('.react-flow__controls-fitview');
      if (fitControl) fitControl.click();
    });
  }

  const layoutSel = document.getElementById('mui-xyflow-layout');
  if (layoutSel) {
    layoutSel.addEventListener('change', () => apiRef.current?.layout(layoutSel.value));
  }

  const exportBtn = document.getElementById('mui-xyflow-export');
  if (exportBtn && output) {
    exportBtn.addEventListener('click', () => {
      const payload = {
        nodes: latestNodes.map((n) => ({ id: n.id, position: n.position, data: n.data, type: n.type })),
        edges: latestEdges.map((e) => ({ id: e.id, source: e.source, target: e.target, animated: !!e.animated })),
      };
      output.textContent = JSON.stringify(payload, null, 2);
    });
  }

  const clearOut = document.getElementById('mui-xyflow-clear-output');
  if (clearOut && output) {
    clearOut.addEventListener('click', () => { output.textContent = '// cleared.'; });
  }
}
"##
}

/// `/integrations/excalidraw` — embed the Excalidraw canvas inside a
/// maud-ui shell. Same ESM-importmap strategy as the xyflow page.
pub fn integrations_excalidraw_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Excalidraw \u{2014} maud-ui integrations"))
                // Shell CSS first, per-integration overrides second.
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(excalidraw_css())) }
                // Excalidraw ships its own stylesheet — pull it from the
                // same esm.sh CDN we use for the JS.
                link rel="stylesheet"
                     href="https://esm.sh/@excalidraw/excalidraw@0.17.6/index.css";
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" }
                            span { " / " }
                            span { "Integrations" }
                            span { " / " }
                            span { "Excalidraw" }
                        }

                        section class="mui-gallery__component" id="integration-excalidraw" {
                            h3 class="mui-gallery__component-name" { "Excalidraw \u{2014} Whiteboard" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "An Excalidraw canvas embedded inside a maud-ui shell. The maud-ui "
                                "toolbar wraps Excalidraw's imperative API \u{2014} the "
                                code style="font-family:var(--mui-font-mono);font-size:0.875rem;" { "excalidrawAPI" }
                                " ref drives export, reset, and shape-insertion. Canvas theme follows "
                                "the gallery's "
                                code style="font-family:var(--mui-font-mono);font-size:0.875rem;" { "data-theme" }
                                " attribute."
                            }

                            div class="mui-integration mui-integration--excalidraw" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(
                                                r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 19l7-7 3 3-7 7-3-3z"/><path d="M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z"/><path d="M2 2l7.586 7.586"/><circle cx="11" cy="11" r="2"/></svg>"##.to_string()))
                                        }
                                        span id="mui-excal-title" { "sketches/architecture.excalidraw" }
                                        span class="mui-integration__dirty" id="mui-excal-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        button type="button" id="mui-excal-rect"
                                               class="mui-btn mui-btn--outline mui-btn--sm" { "+ Rect" }
                                        button type="button" id="mui-excal-ellipse"
                                               class="mui-btn mui-btn--outline mui-btn--sm" { "+ Ellipse" }
                                        button type="button" id="mui-excal-text"
                                               class="mui-btn mui-btn--outline mui-btn--sm" { "+ Text" }
                                        button type="button" id="mui-excal-reset"
                                               class="mui-btn mui-btn--ghost mui-btn--sm" { "Reset" }
                                        button type="button" id="mui-excal-png"
                                               class="mui-btn mui-btn--outline mui-btn--sm" { "PNG" }
                                        button type="button" id="mui-excal-svg"
                                               class="mui-btn mui-btn--primary mui-btn--sm" { "SVG" }
                                    }
                                }

                                div class="mui-integration__editor mui-integration__editor--excal" id="mui-excal-root" {
                                    div class="mui-integration__loading" id="mui-excal-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading Excalidraw from esm.sh\u{2026}" }
                                    }
                                }

                                div class="mui-integration__statusbar" {
                                    span id="mui-excal-status-elements" { "0 elements" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-excal-status-selected" { "No selection" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-excal-status-tool" { "Tool: select" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-excal-status-theme" {
                                        "dark"
                                    }
                                }
                            }

                            div class="mui-integration__output" {
                                div class="mui-integration__output-header" {
                                    span { "Last export" }
                                    button type="button" id="mui-excal-clear-output"
                                           class="mui-btn mui-btn--ghost mui-btn--sm" { "Clear" }
                                }
                                pre class="mui-integration__output-body" id="mui-excal-output" {
                                    "// Click " span style="color:var(--mui-accent-text);" { "PNG" } " or " span style="color:var(--mui-accent-text);" { "SVG" } " to see export metadata here."
                                }
                            }

                            (code_example("Usage \u{2014} mount Excalidraw inside a maud-ui page", r##"// In your <head>:
//   <link rel="stylesheet" href="https://esm.sh/@excalidraw/excalidraw@0.17.6/index.css">
//   <script type="importmap">
//     { "imports": {
//         "react":                 "https://esm.sh/react@18.3.1",
//         "react-dom/client":      "https://esm.sh/react-dom@18.3.1/client",
//         "@excalidraw/excalidraw":"https://esm.sh/@excalidraw/excalidraw@0.17.6?deps=react@18.3.1"
//     }}
//   </script>
//
// Your maud template renders an empty mount point:
html! {
    div class="mui-integration mui-integration--excalidraw" {
        div class="mui-integration__header" { /* toolbar */ }
        div class="mui-integration__editor" id="excal-root" {}
    }
}
// Then a <script type="module"> mounts Excalidraw:
//   import { Excalidraw, exportToSvg } from '@excalidraw/excalidraw';
//   const root = createRoot(document.getElementById('excal-root'));
//   root.render(React.createElement(Excalidraw, {
//     theme: document.documentElement.dataset.theme || 'dark',
//     excalidrawAPI: (api) => window.__excal = api,
//   }));
"##))
                        }

                        div class="mui-gallery__back" {
                            a href="/" class="mui-btn mui-btn--outline mui-btn--sm" {
                                "\u{2190} Back to Gallery"
                            }
                        }
                    }
                }

                // ESM importmap (same pattern as xyflow — pinned React
                // passed through `?deps=` to avoid duplicate React copies).
                script type="importmap" {
                    (maud::PreEscaped(r##"{
  "imports": {
    "react":                  "https://esm.sh/react@18.3.1",
    "react-dom":              "https://esm.sh/react-dom@18.3.1",
    "react-dom/client":       "https://esm.sh/react-dom@18.3.1/client",
    "react/jsx-runtime":      "https://esm.sh/react@18.3.1/jsx-runtime",
    "@excalidraw/excalidraw": "https://esm.sh/@excalidraw/excalidraw@0.17.6?deps=react@18.3.1,react-dom@18.3.1"
  }
}"##))
                }
                script type="module" { (maud::PreEscaped(excalidraw_bootstrap())) }

                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

/// Inline CSS for the Excalidraw integration. Excalidraw owns its
/// inside chrome — we only need to make the *host* div give it the
/// right box (Excalidraw mounts into `position: absolute; inset: 0`).
fn excalidraw_css() -> &'static str {
    r#"
.mui-integration--excalidraw .mui-integration__editor--excal {
    /* Excalidraw demands a parent with explicit height *and* position. */
    height: 32rem;
    min-height: 26rem;
    position: relative;
    overflow: hidden;
}

@media (min-width: 1024px) {
    .mui-integration--excalidraw .mui-integration__editor--excal {
        height: 36rem;
    }
}

/* Excalidraw v0.17 renders a root div with class `excalidraw` — size
 * it to fill our host. */
.mui-integration--excalidraw .excalidraw {
    height: 100% !important;
    width: 100% !important;
}

/* Round the corners of the canvas so it blends with the maud-ui
 * shell instead of fighting it. */
.mui-integration--excalidraw .excalidraw .App-menu__left,
.mui-integration--excalidraw .excalidraw .App-toolbar {
    border-radius: var(--mui-radius-md);
}

/* Excalidraw's welcome / hint overlay can overflow on small hosts;
 * hide it to keep the demo tidy. */
.mui-integration--excalidraw .welcome-screen-center,
.mui-integration--excalidraw .HintViewer {
    display: none;
}
"#
}

/// Inline ES module that pulls Excalidraw from esm.sh, captures its
/// imperative API, and wires the toolbar / status bar.
fn excalidraw_bootstrap() -> &'static str {
    r##"
import * as React from 'react';
import { createRoot } from 'react-dom/client';
// Excalidraw v0.17 ships as CJS; esm.sh wraps it so named exports can
// land on either the top-level namespace OR on `.default`. Resolve at
// runtime to cover both shapes — same trick we use for Shiki on
// esm.sh in other parts of the Kapable stack.
import * as ExcalidrawNs from '@excalidraw/excalidraw';

const ExcalidrawLib = ExcalidrawNs.Excalidraw ? ExcalidrawNs : (ExcalidrawNs.default || {});
const Excalidraw    = ExcalidrawLib.Excalidraw;
const exportToBlob  = ExcalidrawLib.exportToBlob;
const exportToSvg   = ExcalidrawLib.exportToSvg;

if (!Excalidraw) {
  console.error('[maud-ui] Could not resolve Excalidraw export from esm.sh — tried namespace and .default.',
                'Keys on ns:', Object.keys(ExcalidrawNs || {}),
                'Keys on default:', Object.keys(ExcalidrawNs.default || {}));
}

const h = React.createElement;

function pickTheme() {
  return document.documentElement.getAttribute('data-theme') === 'light' ? 'light' : 'dark';
}

// Seed scene — a few shapes so the canvas isn't empty on first load.
const INITIAL_ELEMENTS = [
  { type: 'rectangle', version: 1, versionNonce: 1, isDeleted: false,
    id: 'rect-1', fillStyle: 'solid', strokeWidth: 1, strokeStyle: 'solid',
    roughness: 1, opacity: 100, angle: 0, x: 120, y: 80, width: 220, height: 120,
    strokeColor: '#2563eb', backgroundColor: 'transparent', seed: 1, groupIds: [],
    frameId: null, roundness: { type: 3 }, boundElements: null, updated: 1,
    link: null, locked: false },
  { type: 'text', version: 1, versionNonce: 2, isDeleted: false,
    id: 'txt-1', fillStyle: 'solid', strokeWidth: 1, strokeStyle: 'solid',
    roughness: 1, opacity: 100, angle: 0, x: 150, y: 120, width: 160, height: 25,
    strokeColor: '#2563eb', backgroundColor: 'transparent', seed: 2, groupIds: [],
    frameId: null, roundness: null, boundElements: null, updated: 1, link: null, locked: false,
    fontSize: 20, fontFamily: 1, text: 'maud-ui + Excalidraw',
    baseline: 18, textAlign: 'left', verticalAlign: 'top',
    containerId: null, originalText: 'maud-ui + Excalidraw', lineHeight: 1.25 },
  { type: 'arrow', version: 1, versionNonce: 3, isDeleted: false,
    id: 'arr-1', fillStyle: 'hachure', strokeWidth: 2, strokeStyle: 'solid',
    roughness: 1, opacity: 100, angle: 0, x: 360, y: 140, width: 120, height: 0,
    strokeColor: '#6366f1', backgroundColor: 'transparent', seed: 3, groupIds: [],
    frameId: null, roundness: { type: 2 }, boundElements: null, updated: 1, link: null, locked: false,
    points: [[0, 0], [120, 0]], lastCommittedPoint: null, startBinding: null, endBinding: null,
    startArrowhead: null, endArrowhead: 'arrow' },
  { type: 'ellipse', version: 1, versionNonce: 4, isDeleted: false,
    id: 'ell-1', fillStyle: 'solid', strokeWidth: 1, strokeStyle: 'solid',
    roughness: 1, opacity: 100, angle: 0, x: 500, y: 80, width: 180, height: 120,
    strokeColor: '#db2777', backgroundColor: 'transparent', seed: 4, groupIds: [],
    frameId: null, roundness: null, boundElements: null, updated: 1, link: null, locked: false },
];

function App({ apiRef, onChange }) {
  const [theme, setTheme] = React.useState(pickTheme());

  // Theme sync with gallery data-theme attribute.
  React.useEffect(() => {
    const obs = new MutationObserver(() => setTheme(pickTheme()));
    obs.observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
    return () => obs.disconnect();
  }, []);

  React.useEffect(() => {
    const badge = document.getElementById('mui-excal-status-theme');
    if (badge) badge.textContent = theme;
  }, [theme]);

  return h(Excalidraw, {
    theme,
    initialData: {
      elements: INITIAL_ELEMENTS,
      appState: { viewBackgroundColor: 'transparent', currentItemStrokeColor: '#2563eb' },
      scrollToContent: true,
    },
    excalidrawAPI: (api) => { apiRef.current = api; },
    onChange: (elements, appState) => onChange?.(elements, appState),
    UIOptions: {
      canvasActions: { saveToActiveFile: false, loadScene: false, export: false },
    },
  });
}

// ── Mount ─────────────────────────────────────────────────────────
const host = document.getElementById('mui-excal-root');
if (host) {
  requestAnimationFrame(() => host.setAttribute('data-ready', 'true'));

  const apiRef = { current: null };
  const statusElements = document.getElementById('mui-excal-status-elements');
  const statusSel      = document.getElementById('mui-excal-status-selected');
  const statusTool     = document.getElementById('mui-excal-status-tool');
  const dirty          = document.getElementById('mui-excal-dirty');
  const output         = document.getElementById('mui-excal-output');

  let lastElements = INITIAL_ELEMENTS;

  function onChange(elements, appState) {
    lastElements = elements;
    const visible = elements.filter((el) => !el.isDeleted);
    if (statusElements) {
      statusElements.textContent = visible.length + ' element' + (visible.length === 1 ? '' : 's');
    }
    if (statusSel) {
      const selIds = Object.keys(appState.selectedElementIds || {});
      statusSel.textContent = selIds.length === 0
        ? 'No selection'
        : selIds.length === 1 ? '1 selected' : selIds.length + ' selected';
    }
    if (statusTool) {
      statusTool.textContent = 'Tool: ' + (appState.activeTool?.type || 'select');
    }
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  }

  const root = createRoot(host);
  root.render(h(App, { apiRef, onChange }));

  // ── Toolbar wiring ────────────────────────────────────────────
  function makeShape(kind, x, y) {
    const id = kind + '-' + Date.now();
    if (kind === 'text') {
      return {
        type: 'text', version: 1, versionNonce: Math.random() * 1e9 | 0, isDeleted: false,
        id, fillStyle: 'solid', strokeWidth: 1, strokeStyle: 'solid',
        roughness: 1, opacity: 100, angle: 0, x, y, width: 180, height: 28,
        strokeColor: '#2563eb', backgroundColor: 'transparent',
        seed: Math.random() * 1e9 | 0, groupIds: [], frameId: null,
        roundness: null, boundElements: null, updated: 1, link: null, locked: false,
        fontSize: 22, fontFamily: 1, text: 'Click to edit',
        baseline: 20, textAlign: 'left', verticalAlign: 'top',
        containerId: null, originalText: 'Click to edit', lineHeight: 1.25,
      };
    }
    const base = {
      version: 1, versionNonce: Math.random() * 1e9 | 0, isDeleted: false,
      id, fillStyle: 'solid', strokeWidth: 1, strokeStyle: 'solid',
      roughness: 1, opacity: 100, angle: 0, x, y, width: 160, height: 100,
      strokeColor: kind === 'rectangle' ? '#2563eb' : '#db2777',
      backgroundColor: 'transparent',
      seed: Math.random() * 1e9 | 0, groupIds: [], frameId: null,
      roundness: kind === 'rectangle' ? { type: 3 } : null,
      boundElements: null, updated: 1, link: null, locked: false,
    };
    return { ...base, type: kind };
  }

  function addShape(kind) {
    const api = apiRef.current;
    if (!api) return;
    const existing = api.getSceneElements();
    const ox = 80 + Math.random() * 400;
    const oy = 60 + Math.random() * 180;
    api.updateScene({ elements: [...existing, makeShape(kind, ox, oy)] });
  }

  document.getElementById('mui-excal-rect')?.addEventListener('click', () => addShape('rectangle'));
  document.getElementById('mui-excal-ellipse')?.addEventListener('click', () => addShape('ellipse'));
  document.getElementById('mui-excal-text')?.addEventListener('click', () => addShape('text'));

  document.getElementById('mui-excal-reset')?.addEventListener('click', () => {
    const api = apiRef.current;
    if (!api) return;
    api.updateScene({ elements: INITIAL_ELEMENTS });
    if (dirty) dirty.setAttribute('data-dirty', 'false');
  });

  async function writeExport(label, blobOrNode) {
    if (!output) return;
    if (label === 'SVG') {
      const svgText = new XMLSerializer().serializeToString(blobOrNode);
      const size = new Blob([svgText]).size;
      output.textContent =
        '[SVG] ' + size + ' bytes, ' + lastElements.filter((e) => !e.isDeleted).length + ' elements\n\n' +
        svgText.slice(0, 600) + (svgText.length > 600 ? '\n\n\u2026 (truncated)' : '');
    } else {
      output.textContent =
        '[' + label + '] ' + blobOrNode.size + ' bytes, type=' + blobOrNode.type + '\n' +
        '  (blob ready to download or POST \u2014 this demo doesn\'t save to disk.)';
    }
  }

  document.getElementById('mui-excal-png')?.addEventListener('click', async () => {
    const api = apiRef.current;
    if (!api) return;
    try {
      const blob = await exportToBlob({
        elements: api.getSceneElements(),
        appState: { ...api.getAppState(), exportBackground: true, viewBackgroundColor: '#ffffff' },
        files: api.getFiles(),
        mimeType: 'image/png',
      });
      writeExport('PNG', blob);
    } catch (err) {
      if (output) output.textContent = '[PNG export failed] ' + err.message;
    }
  });

  document.getElementById('mui-excal-svg')?.addEventListener('click', async () => {
    const api = apiRef.current;
    if (!api) return;
    try {
      const svg = await exportToSvg({
        elements: api.getSceneElements(),
        appState: { ...api.getAppState(), exportBackground: false },
        files: api.getFiles(),
      });
      writeExport('SVG', svg);
    } catch (err) {
      if (output) output.textContent = '[SVG export failed] ' + err.message;
    }
  });

  document.getElementById('mui-excal-clear-output')?.addEventListener('click', () => {
    if (output) output.textContent = '// cleared.';
  });
}
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  xterm.js — terminal emulator integration
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_xterm_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("xterm.js \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(xterm_css())) }
                link rel="stylesheet" href="https://esm.sh/@xterm/xterm@5.5.0/css/xterm.css";
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "xterm.js" }
                        }
                        section class="mui-gallery__component" id="integration-xterm" {
                            h3 class="mui-gallery__component-name" { "xterm.js \u{2014} Terminal emulator" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "A full-fidelity terminal inside a maud-ui shell. Type commands, "
                                "resize with the "
                                code style="font-family:var(--mui-font-mono);font-size:0.875rem;" { "FitAddon" }
                                ", and watch the background track the gallery's "
                                code style="font-family:var(--mui-font-mono);font-size:0.875rem;" { "data-theme" }
                                ". Pair with a WebSocket for a live shell or agent transcript viewer."
                            }
                            div class="mui-integration mui-integration--xterm" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 17 10 11 4 5"/><line x1="12" y1="19" x2="20" y2="19"/></svg>"##.to_string()))
                                        }
                                        span id="mui-xterm-title" { "~/maud-ui-demo" }
                                        span class="mui-integration__dirty" id="mui-xterm-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        button type="button" id="mui-xterm-clear" class="mui-btn mui-btn--outline mui-btn--sm" { "Clear" }
                                        button type="button" id="mui-xterm-demo" class="mui-btn mui-btn--outline mui-btn--sm" { "Run demo" }
                                        button type="button" id="mui-xterm-fit" class="mui-btn mui-btn--primary mui-btn--sm" { "Fit" }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-xterm-root" {
                                    div class="mui-integration__loading" id="mui-xterm-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading xterm.js from esm.sh\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-xterm-status-size" { "\u{2014} cols \u{00d7} \u{2014} rows" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-xterm-status-cursor" { "Ln 1, Col 1" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-xterm-status-lines" { "0 lines written" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-xterm-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script type="importmap" {
                    (maud::PreEscaped(r##"{
  "imports": {
    "@xterm/xterm":         "https://esm.sh/@xterm/xterm@5.5.0",
    "@xterm/addon-fit":     "https://esm.sh/@xterm/addon-fit@0.10.0",
    "@xterm/addon-web-links":"https://esm.sh/@xterm/addon-web-links@0.11.0"
  }
}"##))
                }
                script type="module" { (maud::PreEscaped(xterm_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn xterm_css() -> &'static str {
    r#"
.mui-integration--xterm .mui-integration__editor {
    padding: 0.5rem;
    background: var(--mui-bg);
}
.mui-integration--xterm .xterm,
.mui-integration--xterm .xterm-viewport,
.mui-integration--xterm .xterm-screen {
    width: 100% !important;
    height: 100% !important;
}
.mui-integration--xterm .xterm-viewport { background: var(--mui-bg) !important; }
"#
}

fn xterm_bootstrap() -> &'static str {
    r##"
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';

function readVar(name, fallback) {
  return getComputedStyle(document.documentElement).getPropertyValue(name).trim() || fallback;
}
function buildTheme() {
  return {
    background:  readVar('--mui-bg',          '#0a0a0b'),
    foreground:  readVar('--mui-text',        '#fafafa'),
    cursor:      readVar('--mui-accent',      '#2563eb'),
    cursorAccent:readVar('--mui-bg',          '#0a0a0b'),
    selectionBackground: '#264f78',
    black:   '#1e1e1e',  red: '#f87171',   green:  '#4ade80',  yellow: '#facc15',
    blue:    '#60a5fa',  magenta:'#f472b6', cyan:   '#67e8f9',  white:  '#e5e5e5',
    brightBlack: '#525252', brightRed:'#fca5a5', brightGreen:'#86efac',
    brightYellow:'#fde68a',  brightBlue:'#93c5fd', brightMagenta:'#f9a8d4',
    brightCyan:'#a5f3fc', brightWhite:'#ffffff',
  };
}

const host = document.getElementById('mui-xterm-root');
if (host) {
  const term = new Terminal({
    theme: buildTheme(),
    fontFamily: readVar('--mui-font-mono', 'ui-monospace, monospace'),
    fontSize: 13,
    cursorBlink: true,
    convertEol: true,
    scrollback: 1000,
    allowTransparency: true,
  });
  const fit = new FitAddon();
  term.loadAddon(fit);
  term.loadAddon(new WebLinksAddon());

  // Drop the loading overlay, then mount.
  host.replaceChildren();
  term.open(host);
  requestAnimationFrame(() => fit.fit());

  const GREEN = '\x1b[32m', CYAN = '\x1b[36m', GRAY = '\x1b[90m', RESET = '\x1b[0m', BOLD = '\x1b[1m';
  term.writeln(BOLD + 'maud-ui xterm.js demo' + RESET);
  term.writeln(GRAY + '\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500\u2500' + RESET);
  term.writeln('Type ' + CYAN + 'help' + RESET + ' for commands, or press Enter.');
  term.write('\r\n' + GREEN + '$ ' + RESET);

  let buf = '', lines = 0;
  const statusSize   = document.getElementById('mui-xterm-status-size');
  const statusCursor = document.getElementById('mui-xterm-status-cursor');
  const statusLines  = document.getElementById('mui-xterm-status-lines');
  const statusTheme  = document.getElementById('mui-xterm-status-theme');
  const dirty        = document.getElementById('mui-xterm-dirty');

  function updateSize() {
    if (statusSize) statusSize.textContent = term.cols + ' cols \u00d7 ' + term.rows + ' rows';
  }
  updateSize();
  term.onResize(updateSize);
  term.onCursorMove(() => {
    if (statusCursor) {
      const pos = term.buffer.active;
      statusCursor.textContent = 'Ln ' + (pos.cursorY + 1) + ', Col ' + (pos.cursorX + 1);
    }
  });

  function runCommand(cmd) {
    lines++;
    if (statusLines) statusLines.textContent = lines + ' line' + (lines === 1 ? '' : 's') + ' written';
    if (dirty) dirty.setAttribute('data-dirty', 'true');
    switch ((cmd || '').trim()) {
      case '':  break;
      case 'help':
        term.writeln('  ' + CYAN + 'help' + RESET + '       show this message');
        term.writeln('  ' + CYAN + 'ls' + RESET + '         list gallery sections');
        term.writeln('  ' + CYAN + 'whoami' + RESET + '     print caller');
        term.writeln('  ' + CYAN + 'date' + RESET + '       print local date');
        term.writeln('  ' + CYAN + 'clear' + RESET + '      clear scrollback');
        term.writeln('  ' + CYAN + 'theme' + RESET + '      show theme tokens');
        break;
      case 'ls':     term.writeln('primitives/  blocks/  integrations/  README.md'); break;
      case 'whoami': term.writeln('maud-ui demo user'); break;
      case 'date':   term.writeln(new Date().toString()); break;
      case 'clear':  term.clear(); break;
      case 'theme':
        term.writeln('--mui-bg:     ' + readVar('--mui-bg'));
        term.writeln('--mui-text:   ' + readVar('--mui-text'));
        term.writeln('--mui-accent: ' + readVar('--mui-accent'));
        break;
      default:
        term.writeln(GRAY + cmd + ': command not found (try ' + CYAN + 'help' + RESET + GRAY + ')' + RESET);
    }
  }

  term.onData((data) => {
    for (const ch of data) {
      if (ch === '\r') {
        term.write('\r\n');
        runCommand(buf);
        buf = '';
        term.write(GREEN + '$ ' + RESET);
      } else if (ch === '\x7f') {
        if (buf.length > 0) { buf = buf.slice(0, -1); term.write('\b \b'); }
      } else if (ch >= ' ' && ch < '\x7f') {
        buf += ch;
        term.write(ch);
      }
    }
  });

  document.getElementById('mui-xterm-clear')?.addEventListener('click', () => term.clear());
  document.getElementById('mui-xterm-demo')?.addEventListener('click', () => {
    term.writeln('');
    term.writeln(BOLD + CYAN + 'Running demo pipeline...' + RESET);
    const steps = [
      [GRAY + '[00:00]' + RESET, 'fetch http request'],
      [GRAY + '[00:01]' + RESET, GREEN + 'ok' + RESET + ' auth middleware'],
      [GRAY + '[00:02]' + RESET, GREEN + 'ok' + RESET + ' validate body'],
      [GRAY + '[00:03]' + RESET, GREEN + 'ok' + RESET + ' handler'],
      [GRAY + '[00:04]' + RESET, GREEN + 'ok' + RESET + ' sql query (24 rows)'],
      [GRAY + '[00:05]' + RESET, GREEN + 'ok' + RESET + ' response 200 OK'],
    ];
    steps.forEach((s, i) => setTimeout(() => term.writeln('  ' + s[0] + ' ' + s[1]), i * 220));
    setTimeout(() => term.write('\r\n' + GREEN + '$ ' + RESET), steps.length * 220 + 60);
  });
  document.getElementById('mui-xterm-fit')?.addEventListener('click', () => fit.fit());

  new MutationObserver(() => {
    term.options.theme = buildTheme();
    if (statusTheme) statusTheme.textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });

  window.addEventListener('resize', () => fit.fit());
}
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  FullCalendar — scheduling integration
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_fullcalendar_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("FullCalendar \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(fullcalendar_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "FullCalendar" }
                        }
                        section class="mui-gallery__component" id="integration-fullcalendar" {
                            h3 class="mui-gallery__component-name" { "FullCalendar \u{2014} Scheduling" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "A FullCalendar 6 calendar inside a maud-ui shell \u{2014} month / week / day "
                                "views, drag-to-move events, click-to-create. The maud-ui toolbar proxies "
                                "FullCalendar's imperative API so navigation and view switches flow through "
                                "the native shell."
                            }
                            div class="mui-integration mui-integration--fullcalendar" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="4" width="18" height="18" rx="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg>"##.to_string()))
                                        }
                                        span id="mui-fc-title" { "schedule/q2-2026.ics" }
                                        span class="mui-integration__dirty" id="mui-fc-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        button type="button" id="mui-fc-prev"  class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2039}" }
                                        button type="button" id="mui-fc-today" class="mui-btn mui-btn--outline mui-btn--sm" { "Today" }
                                        button type="button" id="mui-fc-next"  class="mui-btn mui-btn--outline mui-btn--sm" { "\u{203a}" }
                                        select id="mui-fc-view" class="mui-integration__select" aria-label="View" {
                                            option value="dayGridMonth" selected { "Month" }
                                            option value="timeGridWeek" { "Week" }
                                            option value="timeGridDay" { "Day" }
                                            option value="listWeek" { "List" }
                                        }
                                        button type="button" id="mui-fc-add" class="mui-btn mui-btn--primary mui-btn--sm" { "+ Event" }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-fc-root" {
                                    div class="mui-integration__loading" id="mui-fc-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading FullCalendar from CDN\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-fc-status-range" { "\u{2014}" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-fc-status-count" { "0 events" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-fc-status-view" { "dayGridMonth" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-fc-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script src="https://cdn.jsdelivr.net/npm/fullcalendar@6.1.15/index.global.min.js" {}
                script { (maud::PreEscaped(fullcalendar_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn fullcalendar_css() -> &'static str {
    r#"
.mui-integration--fullcalendar .mui-integration__editor {
    height: 32rem;
    padding: 0.75rem;
    overflow: auto;
}
@media (min-width: 1024px) {
    .mui-integration--fullcalendar .mui-integration__editor { height: 40rem; }
}
.mui-integration--fullcalendar .fc {
    --fc-border-color: var(--mui-border);
    --fc-page-bg-color: var(--mui-bg-card);
    --fc-neutral-bg-color: var(--mui-bg);
    --fc-today-bg-color: color-mix(in srgb, var(--mui-accent) 12%, transparent);
    --fc-event-bg-color: var(--mui-accent);
    --fc-event-border-color: var(--mui-accent);
    --fc-event-text-color: #fff;
    --fc-list-event-hover-bg-color: var(--mui-bg);
    color: var(--mui-text);
    font-family: var(--mui-font-sans);
}
.mui-integration--fullcalendar .fc-toolbar-title { color: var(--mui-text); font-size: 1rem !important; }
.mui-integration--fullcalendar .fc-col-header-cell-cushion,
.mui-integration--fullcalendar .fc-daygrid-day-number,
.mui-integration--fullcalendar .fc-timegrid-slot-label-cushion {
    color: var(--mui-text-muted);
    text-decoration: none;
}
.mui-integration--fullcalendar .fc-day-today .fc-daygrid-day-number { color: var(--mui-accent-text); font-weight: 600; }
"#
}

fn fullcalendar_bootstrap() -> &'static str {
    r##"
(function () {
  if (typeof FullCalendar === 'undefined') {
    console.error('[maud-ui] FullCalendar global missing \u2014 CDN load failed');
    return;
  }
  const host = document.getElementById('mui-fc-root');
  if (!host) return;
  host.replaceChildren();

  const today = new Date();
  const iso = (d) => d.toISOString().slice(0, 10);
  const add = (base, days) => { const n = new Date(base); n.setDate(n.getDate() + days); return n; };
  const events = [
    { title: 'maud-ui 0.2 release',   start: iso(add(today, 1)), allDay: true, color: '#2563eb' },
    { title: 'Konductor sprint demo', start: iso(add(today, 3)) + 'T10:00:00', end: iso(add(today, 3)) + 'T11:30:00' },
    { title: 'Drain session',         start: iso(add(today, 5)) + 'T14:00:00', end: iso(add(today, 5)) + 'T18:00:00' },
    { title: 'Retro',                 start: iso(add(today, 7)) + 'T09:00:00', end: iso(add(today, 7)) + 'T09:45:00' },
    { title: 'Platform integrity',    start: iso(add(today, -2)), allDay: true, color: '#db2777' },
    { title: 'Office hours',          start: iso(add(today, 10)) + 'T15:00:00', end: iso(add(today, 10)) + 'T16:00:00' },
  ];

  const calendar = new FullCalendar.Calendar(host, {
    initialView: 'dayGridMonth',
    headerToolbar: false,
    events,
    editable: true,
    selectable: true,
    nowIndicator: true,
    dateClick: (info) => {
      const title = window.prompt('Event title?', 'New event');
      if (title) { calendar.addEvent({ title, start: info.dateStr, allDay: info.allDay }); updateStatus(); }
    },
    eventChange: () => { updateStatus(); document.getElementById('mui-fc-dirty')?.setAttribute('data-dirty', 'true'); },
    datesSet: updateStatus,
  });
  calendar.render();

  const statusRange = document.getElementById('mui-fc-status-range');
  const statusCount = document.getElementById('mui-fc-status-count');
  const statusView  = document.getElementById('mui-fc-status-view');
  const statusTheme = document.getElementById('mui-fc-status-theme');

  function fmt(d) { return d.toLocaleDateString(undefined, { month: 'short', day: 'numeric', year: 'numeric' }); }
  function updateStatus() {
    const v = calendar.view;
    if (statusRange) statusRange.textContent = fmt(v.currentStart) + ' \u2013 ' + fmt(new Date(v.currentEnd.getTime() - 1));
    if (statusView)  statusView.textContent  = v.type;
    if (statusCount) {
      const n = calendar.getEvents().length;
      statusCount.textContent = n + ' event' + (n === 1 ? '' : 's');
    }
  }

  document.getElementById('mui-fc-prev')?.addEventListener('click',  () => calendar.prev());
  document.getElementById('mui-fc-next')?.addEventListener('click',  () => calendar.next());
  document.getElementById('mui-fc-today')?.addEventListener('click', () => calendar.today());
  document.getElementById('mui-fc-view')?.addEventListener('change', (e) => calendar.changeView(e.target.value));
  document.getElementById('mui-fc-add')?.addEventListener('click', () => {
    const title = window.prompt('Event title?', 'New event');
    if (!title) return;
    const now = new Date(); now.setMinutes(0, 0, 0); now.setHours(now.getHours() + 1);
    const end = new Date(now); end.setHours(end.getHours() + 1);
    calendar.addEvent({ title, start: now, end });
    updateStatus();
    document.getElementById('mui-fc-dirty')?.setAttribute('data-dirty', 'true');
  });

  new MutationObserver(() => {
    if (statusTheme) statusTheme.textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
})();
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  Leaflet — interactive map integration
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_leaflet_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Leaflet \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(leaflet_css())) }
                link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css";
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "Leaflet" }
                        }
                        section class="mui-gallery__component" id="integration-leaflet" {
                            h3 class="mui-gallery__component-name" { "Leaflet \u{2014} Interactive map" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "A Leaflet map with an OpenStreetMap tile layer and a set of markers, "
                                "all inside a maud-ui shell. The toolbar exposes Leaflet's imperative "
                                "API \u{2014} add markers, switch tile providers, fit bounds \u{2014} without "
                                "exposing Leaflet's own controls."
                            }
                            div class="mui-integration mui-integration--leaflet" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="10" r="3"/><path d="M12 2a8 8 0 0 1 8 8c0 4.5-8 12-8 12s-8-7.5-8-12a8 8 0 0 1 8-8z"/></svg>"##.to_string()))
                                        }
                                        span id="mui-leaflet-title" { "maps/san-francisco.geojson" }
                                        span class="mui-integration__dirty" id="mui-leaflet-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        select id="mui-leaflet-tiles" class="mui-integration__select" aria-label="Tile provider" {
                                            option value="osm" selected { "OpenStreetMap" }
                                            option value="dark"          { "Carto Dark" }
                                            option value="light"         { "Carto Light" }
                                        }
                                        button type="button" id="mui-leaflet-add"   class="mui-btn mui-btn--outline mui-btn--sm" { "+ Marker" }
                                        button type="button" id="mui-leaflet-clear" class="mui-btn mui-btn--ghost mui-btn--sm" { "Clear" }
                                        button type="button" id="mui-leaflet-fit"   class="mui-btn mui-btn--primary mui-btn--sm" { "Fit" }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-leaflet-root" {
                                    div class="mui-integration__loading" id="mui-leaflet-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading Leaflet + OSM tiles\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-leaflet-status-center" { "Lat \u{2014}, Lng \u{2014}" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-leaflet-status-zoom"   { "Zoom \u{2014}" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-leaflet-status-markers"{ "0 markers" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-leaflet-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" {}
                script { (maud::PreEscaped(leaflet_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn leaflet_css() -> &'static str {
    r#"
.mui-integration--leaflet .mui-integration__editor {
    height: 28rem;
    padding: 0;
}
@media (min-width: 1024px) {
    .mui-integration--leaflet .mui-integration__editor { height: 34rem; }
}
.mui-integration--leaflet .leaflet-container { height: 100% !important; width: 100% !important; background: var(--mui-bg); }
.mui-integration--leaflet .leaflet-control-zoom { display: none; }
.mui-integration--leaflet .leaflet-control-attribution {
    background: var(--mui-bg-card);
    color: var(--mui-text-subtle);
    font-size: 0.625rem;
}
.mui-integration--leaflet .leaflet-control-attribution a { color: var(--mui-text-muted); }
"#
}

fn leaflet_bootstrap() -> &'static str {
    r##"
(function () {
  if (typeof L === 'undefined') {
    console.error('[maud-ui] Leaflet global L missing \u2014 CDN load failed');
    return;
  }
  const host = document.getElementById('mui-leaflet-root');
  if (!host) return;
  host.replaceChildren();
  const mapEl = document.createElement('div');
  mapEl.id = 'mui-leaflet-map';
  mapEl.style.width = '100%';
  mapEl.style.height = '100%';
  host.appendChild(mapEl);

  const map = L.map(mapEl, { zoomControl: false, attributionControl: true }).setView([37.7749, -122.4194], 12);

  const tiles = {
    osm:   L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', { maxZoom: 19, attribution: '\u00a9 OpenStreetMap' }),
    dark:  L.tileLayer('https://{s}.basemaps.cartocdn.com/dark_all/{z}/{x}/{y}{r}.png', { maxZoom: 19, attribution: '\u00a9 OpenStreetMap \u00a9 CARTO' }),
    light: L.tileLayer('https://{s}.basemaps.cartocdn.com/light_all/{z}/{x}/{y}{r}.png', { maxZoom: 19, attribution: '\u00a9 OpenStreetMap \u00a9 CARTO' }),
  };
  tiles.osm.addTo(map);
  let activeKey = 'osm';

  const seed = [
    { lat: 37.8199, lng: -122.4783, title: 'Golden Gate Bridge' },
    { lat: 37.8024, lng: -122.4058, title: 'Coit Tower' },
    { lat: 37.8087, lng: -122.4098, title: 'Pier 39' },
    { lat: 37.7694, lng: -122.4862, title: 'Golden Gate Park' },
  ];
  const markers = L.layerGroup().addTo(map);
  for (const m of seed) {
    L.marker([m.lat, m.lng]).addTo(markers).bindPopup(document.createTextNode(m.title));
  }

  const statusCenter  = document.getElementById('mui-leaflet-status-center');
  const statusZoom    = document.getElementById('mui-leaflet-status-zoom');
  const statusMarkers = document.getElementById('mui-leaflet-status-markers');
  const statusTheme   = document.getElementById('mui-leaflet-status-theme');
  const dirty         = document.getElementById('mui-leaflet-dirty');

  function updateStatus() {
    const c = map.getCenter();
    if (statusCenter)  statusCenter.textContent  = 'Lat ' + c.lat.toFixed(4) + ', Lng ' + c.lng.toFixed(4);
    if (statusZoom)    statusZoom.textContent    = 'Zoom ' + map.getZoom();
    if (statusMarkers) statusMarkers.textContent = markers.getLayers().length + ' markers';
  }
  updateStatus();
  map.on('moveend zoomend layeradd layerremove', updateStatus);

  document.getElementById('mui-leaflet-tiles')?.addEventListener('change', (e) => {
    tiles[activeKey].remove();
    activeKey = e.target.value;
    tiles[activeKey].addTo(map);
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  });
  document.getElementById('mui-leaflet-add')?.addEventListener('click', () => {
    const c = map.getCenter();
    const jitter = () => (Math.random() - 0.5) * 0.02;
    const pt = [c.lat + jitter(), c.lng + jitter()];
    const label = 'Pinned ' + pt[0].toFixed(4) + ', ' + pt[1].toFixed(4);
    L.marker(pt).addTo(markers).bindPopup(document.createTextNode(label));
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  });
  document.getElementById('mui-leaflet-clear')?.addEventListener('click', () => {
    markers.clearLayers();
    updateStatus();
  });
  document.getElementById('mui-leaflet-fit')?.addEventListener('click', () => {
    const layers = markers.getLayers();
    if (layers.length === 0) { map.setView([37.7749, -122.4194], 12); return; }
    const group = L.featureGroup(layers);
    map.fitBounds(group.getBounds().pad(0.2));
  });

  new MutationObserver(() => {
    if (statusTheme) statusTheme.textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });

  setTimeout(() => map.invalidateSize(), 100);
})();
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  TipTap — rich text editor integration
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_tiptap_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("TipTap \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(tiptap_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "TipTap" }
                        }
                        section class="mui-gallery__component" id="integration-tiptap" {
                            h3 class="mui-gallery__component-name" { "TipTap \u{2014} Rich text editor" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "A TipTap (ProseMirror) editor with a maud-ui toolbar. Bold, italic, "
                                "headings, lists, blockquote, code block, undo / redo \u{2014} all driven "
                                "through TipTap's chainable command API. Active-format state on each "
                                "button reflects the cursor position."
                            }
                            div class="mui-integration mui-integration--tiptap" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polyline points="4 7 4 4 20 4 20 7"/><line x1="9" y1="20" x2="15" y2="20"/><line x1="12" y1="4" x2="12" y2="20"/></svg>"##.to_string()))
                                        }
                                        span id="mui-tt-title" { "docs/draft.md" }
                                        span class="mui-integration__dirty" id="mui-tt-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar mui-tt__toolbar" {
                                        button type="button" data-tt-cmd="bold"       class="mui-btn mui-btn--outline mui-btn--sm" title="Bold" { "B" }
                                        button type="button" data-tt-cmd="italic"     class="mui-btn mui-btn--outline mui-btn--sm" title="Italic" { em { "I" } }
                                        button type="button" data-tt-cmd="strike"     class="mui-btn mui-btn--outline mui-btn--sm" title="Strikethrough" { s { "S" } }
                                        button type="button" data-tt-cmd="h1"         class="mui-btn mui-btn--outline mui-btn--sm" title="Heading 1" { "H1" }
                                        button type="button" data-tt-cmd="h2"         class="mui-btn mui-btn--outline mui-btn--sm" title="Heading 2" { "H2" }
                                        button type="button" data-tt-cmd="h3"         class="mui-btn mui-btn--outline mui-btn--sm" title="Heading 3" { "H3" }
                                        button type="button" data-tt-cmd="bulletList" class="mui-btn mui-btn--outline mui-btn--sm" title="Bullet list" { "\u{2022}" }
                                        button type="button" data-tt-cmd="orderedList" class="mui-btn mui-btn--outline mui-btn--sm" title="Ordered list" { "1." }
                                        button type="button" data-tt-cmd="blockquote" class="mui-btn mui-btn--outline mui-btn--sm" title="Blockquote" { "\u{201c}\u{201d}" }
                                        button type="button" data-tt-cmd="codeBlock"  class="mui-btn mui-btn--outline mui-btn--sm" title="Code block" { "{ }" }
                                        button type="button" data-tt-cmd="undo"       class="mui-btn mui-btn--ghost mui-btn--sm" title="Undo" { "\u{21b6}" }
                                        button type="button" data-tt-cmd="redo"       class="mui-btn mui-btn--ghost mui-btn--sm" title="Redo" { "\u{21b7}" }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-tt-root" {
                                    div class="mui-integration__loading" id="mui-tt-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading TipTap + ProseMirror from esm.sh\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-tt-status-words" { "0 words" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-tt-status-chars" { "0 characters" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-tt-status-active" { "paragraph" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-tt-status-theme" { "dark" }
                                }
                            }
                            div class="mui-integration__output" {
                                div class="mui-integration__output-header" {
                                    span { "HTML output" }
                                    button type="button" id="mui-tt-copy" class="mui-btn mui-btn--ghost mui-btn--sm" { "Copy" }
                                }
                                pre class="mui-integration__output-body" id="mui-tt-output" { "<!-- live HTML will render here as you type -->" }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script type="importmap" {
                    (maud::PreEscaped(r##"{
  "imports": {
    "@tiptap/core":         "https://esm.sh/@tiptap/core@2.10.3",
    "@tiptap/starter-kit":  "https://esm.sh/@tiptap/starter-kit@2.10.3"
  }
}"##))
                }
                script type="module" { (maud::PreEscaped(tiptap_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn tiptap_css() -> &'static str {
    r#"
.mui-integration--tiptap .mui-integration__editor {
    height: 24rem;
    overflow-y: auto;
    padding: 1rem 1.25rem;
    font-family: var(--mui-font-sans);
    font-size: 0.9375rem;
    line-height: 1.65;
    color: var(--mui-text);
}
@media (min-width: 1024px) {
    .mui-integration--tiptap .mui-integration__editor { height: 28rem; }
}
.mui-integration--tiptap .ProseMirror { outline: none; min-height: 100%; }
.mui-integration--tiptap .ProseMirror > *:first-child { margin-top: 0; }
.mui-integration--tiptap h1 { font-size: 1.5rem;   margin: 1.25rem 0 0.5rem; font-weight: 700; }
.mui-integration--tiptap h2 { font-size: 1.25rem;  margin: 1.1rem  0 0.5rem; font-weight: 700; }
.mui-integration--tiptap h3 { font-size: 1.0625rem;margin: 1rem    0 0.5rem; font-weight: 600; }
.mui-integration--tiptap ul, .mui-integration--tiptap ol { padding-left: 1.5rem; margin: 0.5rem 0; }
.mui-integration--tiptap blockquote {
    border-left: 3px solid var(--mui-border-hover);
    padding-left: 0.875rem;
    color: var(--mui-text-muted);
    margin: 0.75rem 0;
}
.mui-integration--tiptap pre {
    background: var(--mui-bg);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-md);
    padding: 0.75rem 0.875rem;
    font-family: var(--mui-font-mono);
    font-size: 0.8125rem;
    overflow-x: auto;
}
.mui-integration--tiptap code {
    font-family: var(--mui-font-mono);
    font-size: 0.875em;
    padding: 0.05rem 0.25rem;
    background: var(--mui-bg);
    border-radius: var(--mui-radius-sm);
}
.mui-tt__toolbar { flex-wrap: wrap; row-gap: 0.25rem; }
.mui-tt__toolbar .mui-btn.is-active {
    background: color-mix(in srgb, var(--mui-accent) 18%, transparent);
    border-color: var(--mui-accent);
    color: var(--mui-accent-text);
}
"#
}

fn tiptap_bootstrap() -> &'static str {
    r##"
import { Editor } from '@tiptap/core';
import StarterKit from '@tiptap/starter-kit';

const host = document.getElementById('mui-tt-root');
if (host) {
  host.replaceChildren();

  const initial = `
<h1>maud-ui \u00d7 TipTap</h1>
<p>This is a <strong>rich text editor</strong> inside a maud-ui shell. Type, format, and watch the
status bar track your progress. All toolbar buttons proxy TipTap's chainable command API.</p>
<h3>Try it</h3>
<ul>
  <li>Press <code>Cmd/Ctrl&nbsp;+&nbsp;B</code> to bold</li>
  <li>Start a line with <code>##</code> for a heading</li>
  <li>Start a line with <code>*</code> for a bullet list</li>
</ul>
<blockquote>TipTap is a headless wrapper around ProseMirror. maud-ui provides the chrome.</blockquote>
<pre><code>editor.chain().focus().toggleBold().run();</code></pre>
  `;

  const editor = new Editor({
    element: host,
    extensions: [StarterKit],
    content: initial,
    autofocus: false,
  });

  const dirty       = document.getElementById('mui-tt-dirty');
  const statusWords = document.getElementById('mui-tt-status-words');
  const statusChars = document.getElementById('mui-tt-status-chars');
  const statusActive= document.getElementById('mui-tt-status-active');
  const statusTheme = document.getElementById('mui-tt-status-theme');
  const output      = document.getElementById('mui-tt-output');
  const buttons     = Array.from(document.querySelectorAll('.mui-tt__toolbar [data-tt-cmd]'));

  function currentBlock() {
    const s = editor.state.selection.$from;
    const node = s.node(s.depth);
    const t = node.type.name;
    if (t === 'heading') return 'heading ' + (node.attrs.level || 1);
    if (t === 'codeBlock') return 'code block';
    if (t === 'bulletList' || t === 'orderedList') return t;
    if (t === 'blockquote') return 'blockquote';
    return t;
  }

  function refresh() {
    const text = editor.state.doc.textContent;
    const words = text.trim() ? text.trim().split(/\s+/).length : 0;
    if (statusWords) statusWords.textContent = words + ' word' + (words === 1 ? '' : 's');
    if (statusChars) statusChars.textContent = text.length + ' character' + (text.length === 1 ? '' : 's');
    if (statusActive) statusActive.textContent = currentBlock();
    if (output) output.textContent = editor.getHTML();
    for (const b of buttons) {
      const cmd = b.getAttribute('data-tt-cmd');
      const active =
        cmd === 'h1'   ? editor.isActive('heading', { level: 1 }) :
        cmd === 'h2'   ? editor.isActive('heading', { level: 2 }) :
        cmd === 'h3'   ? editor.isActive('heading', { level: 3 }) :
        editor.isActive(cmd);
      b.classList.toggle('is-active', !!active);
    }
  }

  editor.on('update', () => { if (dirty) dirty.setAttribute('data-dirty', 'true'); refresh(); });
  editor.on('selectionUpdate', refresh);
  refresh();

  for (const b of buttons) {
    b.addEventListener('click', () => {
      const cmd = b.getAttribute('data-tt-cmd');
      const chain = editor.chain().focus();
      switch (cmd) {
        case 'bold':        chain.toggleBold().run(); break;
        case 'italic':      chain.toggleItalic().run(); break;
        case 'strike':      chain.toggleStrike().run(); break;
        case 'h1':          chain.toggleHeading({ level: 1 }).run(); break;
        case 'h2':          chain.toggleHeading({ level: 2 }).run(); break;
        case 'h3':          chain.toggleHeading({ level: 3 }).run(); break;
        case 'bulletList':  chain.toggleBulletList().run(); break;
        case 'orderedList': chain.toggleOrderedList().run(); break;
        case 'blockquote':  chain.toggleBlockquote().run(); break;
        case 'codeBlock':   chain.toggleCodeBlock().run(); break;
        case 'undo':        chain.undo().run(); break;
        case 'redo':        chain.redo().run(); break;
      }
    });
  }

  document.getElementById('mui-tt-copy')?.addEventListener('click', async () => {
    try { await navigator.clipboard.writeText(editor.getHTML()); } catch {}
  });

  new MutationObserver(() => {
    if (statusTheme) statusTheme.textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
}
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  Three.js — WebGL 3D scene
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_threejs_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Three.js \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(threejs_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "Three.js" }
                        }
                        section class="mui-gallery__component" id="integration-threejs" {
                            h3 class="mui-gallery__component-name" { "Three.js \u{2014} WebGL 3D" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "A Three.js scene \u{2014} camera, lights, meshes, orbit controls, grid, "
                                "axes helper \u{2014} mounted into a maud-ui shell. The toolbar swaps the "
                                "displayed geometry and toggles wireframe, and the status bar tracks FPS, "
                                "triangle count, and the active shape."
                            }
                            div class="mui-integration mui-integration--threejs" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><polygon points="12 2 2 19 22 19 12 2"/><line x1="12" y1="2" x2="12" y2="19"/></svg>"##.to_string()))
                                        }
                                        span id="mui-three-title" { "scenes/demo.glb" }
                                        span class="mui-integration__dirty" id="mui-three-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        select id="mui-three-shape" class="mui-integration__select" aria-label="Shape" {
                                            option value="torusKnot" selected { "Torus knot" }
                                            option value="icosahedron" { "Icosahedron" }
                                            option value="box"          { "Box" }
                                            option value="sphere"       { "Sphere" }
                                            option value="cone"         { "Cone" }
                                        }
                                        button type="button" id="mui-three-wire"  class="mui-btn mui-btn--outline mui-btn--sm" { "Wireframe" }
                                        button type="button" id="mui-three-spin"  class="mui-btn mui-btn--outline mui-btn--sm" { "Spin" }
                                        button type="button" id="mui-three-reset" class="mui-btn mui-btn--primary mui-btn--sm" { "Reset camera" }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-three-root" {
                                    div class="mui-integration__loading" id="mui-three-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading Three.js from esm.sh\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-three-status-fps" { "\u{2014} fps" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-three-status-tris"{ "\u{2014} triangles" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-three-status-shape"{ "torusKnot" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-three-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script type="importmap" {
                    (maud::PreEscaped(r##"{
  "imports": {
    "three":                            "https://esm.sh/three@0.170.0",
    "three/addons/controls/OrbitControls.js": "https://esm.sh/three@0.170.0/examples/jsm/controls/OrbitControls.js"
  }
}"##))
                }
                script type="module" { (maud::PreEscaped(threejs_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn threejs_css() -> &'static str {
    r#"
.mui-integration--threejs .mui-integration__editor {
    height: 32rem;
    padding: 0;
    background: var(--mui-bg);
}
@media (min-width: 1024px) {
    .mui-integration--threejs .mui-integration__editor { height: 38rem; }
}
.mui-integration--threejs canvas { display: block; width: 100% !important; height: 100% !important; }
"#
}

fn threejs_bootstrap() -> &'static str {
    r##"
import * as THREE from 'three';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';

const host = document.getElementById('mui-three-root');
if (host) {
  host.replaceChildren();

  function bgColor() {
    const c = getComputedStyle(document.documentElement).getPropertyValue('--mui-bg').trim();
    try { return new THREE.Color(c || '#0a0a0b'); } catch { return new THREE.Color('#0a0a0b'); }
  }

  const scene = new THREE.Scene();
  scene.background = bgColor();

  const camera = new THREE.PerspectiveCamera(45, 1, 0.1, 500);
  camera.position.set(4, 3, 6);

  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
  host.appendChild(renderer.domElement);

  function resize() {
    const r = host.getBoundingClientRect();
    renderer.setSize(r.width, r.height, false);
    camera.aspect = r.width / Math.max(r.height, 1);
    camera.updateProjectionMatrix();
  }
  resize();
  new ResizeObserver(resize).observe(host);

  scene.add(new THREE.AmbientLight(0xffffff, 0.5));
  const point = new THREE.PointLight(0xffffff, 1.2, 50);
  point.position.set(5, 5, 5);
  scene.add(point);
  const rim = new THREE.DirectionalLight(0x60a5fa, 0.7);
  rim.position.set(-4, 2, -4);
  scene.add(rim);

  scene.add(new THREE.GridHelper(20, 20, 0x3f3f46, 0x27272a));
  scene.add(new THREE.AxesHelper(1.5));

  const material = new THREE.MeshPhongMaterial({ color: 0x2563eb, shininess: 90, wireframe: false });
  let mesh = makeMesh('torusKnot');
  scene.add(mesh);

  function makeMesh(kind) {
    let g;
    switch (kind) {
      case 'icosahedron': g = new THREE.IcosahedronGeometry(1.2, 0); break;
      case 'box':         g = new THREE.BoxGeometry(1.6, 1.6, 1.6); break;
      case 'sphere':      g = new THREE.SphereGeometry(1.2, 32, 32); break;
      case 'cone':        g = new THREE.ConeGeometry(1.2, 2, 32); break;
      case 'torusKnot':
      default:            g = new THREE.TorusKnotGeometry(0.9, 0.28, 120, 16);
    }
    const m = new THREE.Mesh(g, material);
    m.position.y = 1;
    return m;
  }

  const controls = new OrbitControls(camera, renderer.domElement);
  controls.enableDamping = true;

  let spinning = true;
  let frames = 0, lastFps = performance.now();
  const statusFps   = document.getElementById('mui-three-status-fps');
  const statusTris  = document.getElementById('mui-three-status-tris');
  const statusShape = document.getElementById('mui-three-status-shape');
  const statusTheme = document.getElementById('mui-three-status-theme');
  const dirty       = document.getElementById('mui-three-dirty');

  function updateTris() {
    const tris = (mesh.geometry.index ? mesh.geometry.index.count : mesh.geometry.attributes.position.count) / 3;
    if (statusTris) statusTris.textContent = Math.round(tris).toLocaleString() + ' triangles';
  }
  updateTris();

  function animate(now) {
    if (spinning) {
      mesh.rotation.x += 0.006;
      mesh.rotation.y += 0.011;
    }
    controls.update();
    renderer.render(scene, camera);
    frames++;
    if (now - lastFps > 500) {
      const fps = (frames * 1000) / (now - lastFps);
      if (statusFps) statusFps.textContent = fps.toFixed(0) + ' fps';
      frames = 0; lastFps = now;
    }
    requestAnimationFrame(animate);
  }
  requestAnimationFrame(animate);

  document.getElementById('mui-three-shape')?.addEventListener('change', (e) => {
    scene.remove(mesh);
    mesh.geometry.dispose();
    mesh = makeMesh(e.target.value);
    scene.add(mesh);
    if (statusShape) statusShape.textContent = e.target.value;
    updateTris();
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  });
  document.getElementById('mui-three-wire')?.addEventListener('click', () => {
    material.wireframe = !material.wireframe;
    material.needsUpdate = true;
  });
  document.getElementById('mui-three-spin')?.addEventListener('click', () => { spinning = !spinning; });
  document.getElementById('mui-three-reset')?.addEventListener('click', () => {
    camera.position.set(4, 3, 6);
    controls.target.set(0, 1, 0);
    controls.update();
  });

  new MutationObserver(() => {
    scene.background = bgColor();
    if (statusTheme) statusTheme.textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
}
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  AG Grid Community — enterprise-grade data grid
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_aggrid_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("AG Grid \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(aggrid_css())) }
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/ag-grid-community@32.3.3/styles/ag-grid.css";
                link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/ag-grid-community@32.3.3/styles/ag-theme-quartz.css";
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "AG Grid" }
                        }
                        section class="mui-gallery__component" id="integration-aggrid" {
                            h3 class="mui-gallery__component-name" { "AG Grid \u{2014} Enterprise data grid" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "AG Grid Community inside a maud-ui shell. 50 seed rows, sortable / "
                                "filterable columns, row selection, CSV export \u{2014} the data-heavy "
                                "story. Theme flips between quartz and quartz-dark with the gallery's "
                                "data-theme."
                            }
                            div class="mui-integration mui-integration--aggrid" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><rect x="3" y="3" width="18" height="18" rx="2"/><line x1="3" y1="9" x2="21" y2="9"/><line x1="3" y1="15" x2="21" y2="15"/><line x1="9" y1="3" x2="9" y2="21"/><line x1="15" y1="3" x2="15" y2="21"/></svg>"##.to_string()))
                                        }
                                        span id="mui-ag-title" { "data/users.csv" }
                                        span class="mui-integration__dirty" id="mui-ag-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        button type="button" id="mui-ag-add"    class="mui-btn mui-btn--outline mui-btn--sm" { "+ Row" }
                                        button type="button" id="mui-ag-delete" class="mui-btn mui-btn--outline mui-btn--sm" { "Delete selected" }
                                        button type="button" id="mui-ag-clear"  class="mui-btn mui-btn--ghost mui-btn--sm" { "Clear filters" }
                                        button type="button" id="mui-ag-csv"    class="mui-btn mui-btn--primary mui-btn--sm" { "Export CSV" }
                                    }
                                }
                                div class="mui-integration__editor ag-theme-quartz-dark" id="mui-ag-root" {
                                    div class="mui-integration__loading" id="mui-ag-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading AG Grid from CDN\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-ag-status-rows"  { "0 rows" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-ag-status-sel"   { "0 selected" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-ag-status-filter"{ "no filters" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-ag-status-theme" { "quartz-dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script src="https://cdn.jsdelivr.net/npm/ag-grid-community@32.3.3/dist/ag-grid-community.min.js" {}
                script { (maud::PreEscaped(aggrid_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn aggrid_css() -> &'static str {
    r#"
.mui-integration--aggrid .mui-integration__editor {
    height: 30rem;
    padding: 0;
}
@media (min-width: 1024px) {
    .mui-integration--aggrid .mui-integration__editor { height: 36rem; }
}
.mui-integration--aggrid .ag-theme-quartz,
.mui-integration--aggrid .ag-theme-quartz-dark {
    --ag-font-family: var(--mui-font-sans);
    --ag-font-size: 13px;
    height: 100%;
    width: 100%;
    display: flex;
    flex-direction: column;
}
/* AG Grid v32 ships an .ag-root-wrapper that doesn't assume a percent
 * height from its container in every theme flavour — force it to fill
 * our shell so the virtualised rows have room to render. Without this
 * the wrapper collapses to ~2px and the grid appears empty despite
 * rows being present in the DOM. */
.mui-integration--aggrid .ag-root-wrapper {
    height: 100% !important;
    flex: 1 1 auto;
}
"#
}

fn aggrid_bootstrap() -> &'static str {
    r##"
(function () {
  if (typeof agGrid === 'undefined') {
    console.error('[maud-ui] agGrid global missing \u2014 CDN load failed');
    return;
  }
  const host = document.getElementById('mui-ag-root');
  if (!host) return;
  host.replaceChildren();

  const FIRST = ['Alex','Blair','Charlie','Dana','Eli','Fran','Gil','Harper','Indigo','Jules','Kai','Lee','Morgan','Nico','Ola','Parker','Quinn','Riley','Sam','Taylor'];
  const LAST  = ['Adams','Brooks','Clark','Davis','Evans','Fisher','Gray','Harris','Irwin','Jones','Kim','Lopez','Miller','Nguyen','Ortiz','Patel','Quinn','Ramos','Smith','Taylor'];
  const ROLES = ['Engineer','Designer','PM','Ops','Researcher','Analyst'];
  const STATUS= ['active','invited','dormant'];

  function seed(n) {
    const rows = [];
    for (let i = 1; i <= n; i++) {
      const first = FIRST[i % FIRST.length];
      const last  = LAST[(i * 7) % LAST.length];
      rows.push({
        id: i,
        name: first + ' ' + last,
        email: (first + '.' + last).toLowerCase() + '@maud-ui.dev',
        role:  ROLES[i % ROLES.length],
        salary: 60000 + ((i * 1273) % 80000),
        joined: new Date(2023, (i * 3) % 12, (i * 5) % 27 + 1).toISOString().slice(0, 10),
        status: STATUS[i % STATUS.length],
      });
    }
    return rows;
  }

  const columnDefs = [
    { field: 'id',     headerName: '#',       maxWidth: 80,  pinned: 'left', checkboxSelection: true, headerCheckboxSelection: true },
    { field: 'name',   headerName: 'Name',    flex: 1.2, editable: true },
    { field: 'email',  headerName: 'Email',   flex: 1.6 },
    { field: 'role',   headerName: 'Role',    flex: 0.9, editable: true },
    { field: 'salary', headerName: 'Salary',  flex: 0.9, type: 'numericColumn',
      valueFormatter: (p) => '$' + (p.value || 0).toLocaleString() },
    { field: 'joined', headerName: 'Joined',  flex: 0.8 },
    { field: 'status', headerName: 'Status',  flex: 0.7,
      cellStyle: (p) => ({
        color: p.value === 'active' ? '#4ade80' : p.value === 'dormant' ? '#f87171' : '#facc15',
        fontWeight: 600,
      }) },
  ];

  let rowData = seed(50);
  const statusRows   = document.getElementById('mui-ag-status-rows');
  const statusSel    = document.getElementById('mui-ag-status-sel');
  const statusFilter = document.getElementById('mui-ag-status-filter');
  const statusTheme  = document.getElementById('mui-ag-status-theme');
  const dirty        = document.getElementById('mui-ag-dirty');

  const gridOptions = {
    columnDefs,
    rowData,
    defaultColDef: { sortable: true, filter: true, resizable: true, minWidth: 90 },
    rowSelection: 'multiple',
    animateRows: true,
    onGridReady: (p) => {
      p.api.sizeColumnsToFit();
      updateStatus(p.api);
    },
    onFirstDataRendered: (p) => updateStatus(p.api),
    onSelectionChanged: (p) => updateStatus(p.api),
    onFilterChanged:    (p) => updateStatus(p.api),
    onCellValueChanged: () => { if (dirty) dirty.setAttribute('data-dirty', 'true'); },
  };

  const api = agGrid.createGrid(host, gridOptions);

  function updateStatus(gApi) {
    let rows = 0;
    gApi.forEachNodeAfterFilter(() => rows++);
    if (statusRows) statusRows.textContent = rows + ' row' + (rows === 1 ? '' : 's');
    if (statusSel)  statusSel.textContent  = gApi.getSelectedNodes().length + ' selected';
    if (statusFilter) {
      const fm = gApi.getFilterModel() || {};
      const n = Object.keys(fm).length;
      statusFilter.textContent = n === 0 ? 'no filters' : n + ' filter' + (n === 1 ? '' : 's') + ' active';
    }
  }

  function applyTheme() {
    const dark = (document.documentElement.getAttribute('data-theme') || 'dark') !== 'light';
    host.classList.toggle('ag-theme-quartz-dark', dark);
    host.classList.toggle('ag-theme-quartz', !dark);
    if (statusTheme) statusTheme.textContent = dark ? 'quartz-dark' : 'quartz';
  }
  applyTheme();
  new MutationObserver(applyTheme).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });

  document.getElementById('mui-ag-add')?.addEventListener('click', () => {
    const n = rowData.length + 1;
    const extra = seed(1)[0];
    extra.id = n;
    rowData.push(extra);
    api.applyTransaction({ add: [extra] });
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  });
  document.getElementById('mui-ag-delete')?.addEventListener('click', () => {
    const sel = api.getSelectedNodes().map((n) => n.data);
    if (sel.length === 0) return;
    api.applyTransaction({ remove: sel });
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  });
  document.getElementById('mui-ag-clear')?.addEventListener('click', () => {
    api.setFilterModel(null);
  });
  document.getElementById('mui-ag-csv')?.addEventListener('click', () => {
    api.exportDataAsCsv?.({ fileName: 'maud-ui-users.csv' });
  });
})();
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  Mermaid — text-to-diagram renderer
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_mermaid_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Mermaid \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(mermaid_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "Mermaid" }
                        }
                        section class="mui-gallery__component" id="integration-mermaid" {
                            h3 class="mui-gallery__component-name" { "Mermaid \u{2014} Text to diagram" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "Mermaid's text-to-diagram renderer in a maud-ui split pane \u{2014} source on "
                                "the left, rendered SVG on the right. Tabs switch between flowchart, "
                                "sequence, class, and Gantt diagrams."
                            }
                            div class="mui-integration mui-integration--mermaid" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12 3v18"/><path d="M3 12h18"/><circle cx="12" cy="12" r="9"/></svg>"##.to_string()))
                                        }
                                        span id="mui-mer-title" { "diagrams/flow.mmd" }
                                        span class="mui-integration__dirty" id="mui-mer-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        button type="button" data-mer-kind="flowchart" class="mui-btn mui-btn--outline mui-btn--sm is-active" { "Flowchart" }
                                        button type="button" data-mer-kind="sequence"  class="mui-btn mui-btn--outline mui-btn--sm" { "Sequence" }
                                        button type="button" data-mer-kind="class"     class="mui-btn mui-btn--outline mui-btn--sm" { "Class" }
                                        button type="button" data-mer-kind="gantt"     class="mui-btn mui-btn--outline mui-btn--sm" { "Gantt" }
                                        button type="button" id="mui-mer-render"       class="mui-btn mui-btn--primary mui-btn--sm" { "Render" }
                                    }
                                }
                                div class="mui-integration__editor mui-mer__split" id="mui-mer-root" {
                                    textarea id="mui-mer-src" spellcheck="false" {}
                                    div id="mui-mer-output" class="mui-mer__output" {
                                        div class="mui-integration__loading" id="mui-mer-loading" {
                                            span class="mui-spin" aria-hidden="true" {
                                                (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                            }
                                            span { "Loading Mermaid from esm.sh\u{2026}" }
                                        }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-mer-status-kind"  { "flowchart" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-mer-status-lines" { "0 source lines" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-mer-status-nodes" { "\u{2014}" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-mer-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script type="importmap" {
                    (maud::PreEscaped(r##"{"imports":{"mermaid":"https://esm.sh/mermaid@11.4.1"}}"##))
                }
                script type="module" { (maud::PreEscaped(mermaid_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn mermaid_css() -> &'static str {
    r#"
.mui-integration--mermaid .mui-integration__editor {
    height: 30rem;
    padding: 0;
    display: grid;
    grid-template-columns: 1fr 1fr;
}
@media (min-width: 1024px) {
    .mui-integration--mermaid .mui-integration__editor { height: 34rem; }
}
.mui-integration--mermaid textarea {
    background: var(--mui-bg);
    color: var(--mui-text);
    border: 0;
    border-right: 1px solid var(--mui-border);
    font-family: var(--mui-font-mono);
    font-size: 0.8125rem;
    line-height: 1.55;
    padding: 0.875rem 1rem;
    resize: none;
    outline: none;
}
.mui-mer__output {
    background: var(--mui-bg-card);
    overflow: auto;
    padding: 1rem;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
}
.mui-mer__output svg { max-width: 100%; height: auto; }
.mui-integration--mermaid [data-mer-kind].is-active {
    background: color-mix(in srgb, var(--mui-accent) 18%, transparent);
    border-color: var(--mui-accent);
    color: var(--mui-accent-text);
}
"#
}

fn mermaid_bootstrap() -> &'static str {
    r##"
import mermaid from 'mermaid';

const SAMPLES = {
  flowchart: `flowchart LR
    A[HTTP Request] --> B[Auth]
    B --> C[Validate]
    B --> D[Rate limit]
    C --> E[Handler]
    D --> E
    E --> F[(Database)]
    E --> G[[Cache]]
    F --> H[Response]
    G --> H`,
  sequence: `sequenceDiagram
    participant U as User
    participant C as Console
    participant A as API
    participant D as Database
    U->>C: Navigate to /users
    C->>A: GET /v1/users
    A->>D: SELECT * FROM users
    D-->>A: rows
    A-->>C: 200 OK
    C-->>U: render list`,
  class: `classDiagram
    class User {
      +Uuid id
      +String email
      +Role role
      +login()
      +logout()
    }
    class Session {
      +Uuid id
      +Uuid user_id
      +DateTime expires_at
    }
    class ApiKey {
      +String prefix
      +Bytes hash
    }
    User "1" --> "many" Session
    User "1" --> "many" ApiKey`,
  gantt: `gantt
    title maud-ui roadmap
    dateFormat  YYYY-MM-DD
    section Gallery
    Components       :done,   a1, 2026-01-01, 60d
    Blocks           :done,   a2, 2026-02-15, 30d
    Integrations v1  :done,   a3, 2026-04-01, 14d
    Integrations v2  :active, a4, 2026-04-18, 10d
    section Polish
    Theming pass     :        b1, 2026-04-28, 14d
    i18n             :        b2, 2026-05-05, 21d`,
};

let kind = 'flowchart';

const src    = document.getElementById('mui-mer-src');
const output = document.getElementById('mui-mer-output');
const tabBtns = Array.from(document.querySelectorAll('[data-mer-kind]'));
const statusKind  = document.getElementById('mui-mer-status-kind');
const statusLines = document.getElementById('mui-mer-status-lines');
const statusNodes = document.getElementById('mui-mer-status-nodes');
const statusTheme = document.getElementById('mui-mer-status-theme');
const dirty       = document.getElementById('mui-mer-dirty');

function pickTheme() {
  return (document.documentElement.getAttribute('data-theme') || 'dark') === 'light' ? 'default' : 'dark';
}

mermaid.initialize({ startOnLoad: false, theme: pickTheme(), securityLevel: 'loose', fontFamily: 'var(--mui-font-sans)' });

src.value = SAMPLES[kind];

async function render() {
  const code = src.value;
  output.replaceChildren();
  try {
    const { svg } = await mermaid.render('mui-mer-svg-' + Date.now(), code);
    const holder = document.createElement('div');
    holder.innerHTML = svg; // mermaid's own output is trusted
    while (holder.firstChild) output.appendChild(holder.firstChild);
    if (statusNodes) {
      const svgEl = output.querySelector('svg');
      const nodes = svgEl ? svgEl.querySelectorAll('g.node, .actor, .classGroup, .taskText').length : 0;
      statusNodes.textContent = nodes + ' node' + (nodes === 1 ? '' : 's');
    }
  } catch (err) {
    const pre = document.createElement('pre');
    pre.style.color = '#f87171';
    pre.style.fontFamily = 'var(--mui-font-mono)';
    pre.style.fontSize = '0.8125rem';
    pre.style.whiteSpace = 'pre-wrap';
    pre.textContent = String(err.message || err);
    output.appendChild(pre);
    if (statusNodes) statusNodes.textContent = 'render error';
  }
  const lines = code.split('\n').length;
  if (statusLines) statusLines.textContent = lines + ' source line' + (lines === 1 ? '' : 's');
}

function setKind(k) {
  kind = k;
  src.value = SAMPLES[k];
  for (const b of tabBtns) b.classList.toggle('is-active', b.getAttribute('data-mer-kind') === k);
  if (statusKind) statusKind.textContent = k;
  render();
}

for (const b of tabBtns) b.addEventListener('click', () => setKind(b.getAttribute('data-mer-kind')));
document.getElementById('mui-mer-render')?.addEventListener('click', render);
src.addEventListener('input', () => { if (dirty) dirty.setAttribute('data-dirty', 'true'); });

new MutationObserver(() => {
  mermaid.initialize({ startOnLoad: false, theme: pickTheme(), securityLevel: 'loose', fontFamily: 'var(--mui-font-sans)' });
  if (statusTheme) statusTheme.textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  render();
}).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });

render();
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  Apache ECharts — charting library
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_echarts_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("ECharts \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(echarts_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "Apache ECharts" }
                        }
                        section class="mui-gallery__component" id="integration-echarts" {
                            h3 class="mui-gallery__component-name" { "Apache ECharts \u{2014} Charting library" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "Apache ECharts inside a maud-ui shell. Switch chart type, randomise "
                                "data, and download as PNG without leaving the host chrome. ECharts "
                                "picks up theme from the gallery's data-theme."
                            }
                            div class="mui-integration mui-integration--echarts" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="3" y1="20" x2="21" y2="20"/><line x1="3" y1="20" x2="3" y2="4"/><rect x="6" y="11" width="3" height="9"/><rect x="11" y="6" width="3" height="14"/><rect x="16" y="14" width="3" height="6"/></svg>"##.to_string()))
                                        }
                                        span id="mui-ec-title" { "analytics/q2-kpis.json" }
                                        span class="mui-integration__dirty" id="mui-ec-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        select id="mui-ec-kind" class="mui-integration__select" aria-label="Chart type" {
                                            option value="line" selected { "Line + Bar" }
                                            option value="bar"            { "Bar" }
                                            option value="pie"            { "Pie" }
                                            option value="radar"          { "Radar" }
                                            option value="scatter"        { "Scatter" }
                                        }
                                        button type="button" id="mui-ec-shuffle" class="mui-btn mui-btn--outline mui-btn--sm" { "Randomise" }
                                        button type="button" id="mui-ec-png"     class="mui-btn mui-btn--primary mui-btn--sm" { "PNG" }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-ec-root" {
                                    div class="mui-integration__loading" id="mui-ec-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading ECharts from esm.sh\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-ec-status-series" { "0 series" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-ec-status-points" { "0 data points" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-ec-status-type"   { "line" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-ec-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script type="importmap" {
                    (maud::PreEscaped(r##"{"imports":{"echarts":"https://esm.sh/echarts@5.5.1"}}"##))
                }
                script type="module" { (maud::PreEscaped(echarts_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn echarts_css() -> &'static str {
    r#"
.mui-integration--echarts .mui-integration__editor {
    height: 28rem;
    padding: 0;
    background: var(--mui-bg-card);
}
@media (min-width: 1024px) {
    .mui-integration--echarts .mui-integration__editor { height: 34rem; }
}
"#
}

fn echarts_bootstrap() -> &'static str {
    r##"
import * as echarts from 'echarts';

const host = document.getElementById('mui-ec-root');
if (host) {
  host.replaceChildren();
  const chart = echarts.init(host, null, { renderer: 'svg' });

  let kind = 'line';
  const MONTHS = ['Jan','Feb','Mar','Apr','May','Jun','Jul','Aug','Sep','Oct','Nov','Dec'];
  let series1 = seed(12, 50, 50);
  let series2 = seed(12, 30, 60);

  function seed(n, base, spread) {
    const out = [];
    for (let i = 0; i < n; i++) out.push(Math.round(base + Math.random() * spread));
    return out;
  }

  function build() {
    const common = { tooltip: { trigger: 'axis' }, legend: { textStyle: { color: getVar('--mui-text') } }, backgroundColor: 'transparent' };
    switch (kind) {
      case 'bar':
        return { ...common, xAxis: { type: 'category', data: MONTHS, axisLabel: { color: getVar('--mui-text-muted') } },
          yAxis: { type: 'value', axisLabel: { color: getVar('--mui-text-muted') } },
          series: [{ name: 'Signups', type: 'bar', data: series1, itemStyle: { color: '#2563eb' } }] };
      case 'pie':
        return { ...common,
          series: [{ name: 'Traffic', type: 'pie', radius: ['35%','70%'], label: { color: getVar('--mui-text') },
            data: MONTHS.slice(0, 6).map((m, i) => ({ name: m, value: series1[i] })) }] };
      case 'radar':
        return { ...common,
          radar: { indicator: MONTHS.slice(0, 6).map((m) => ({ name: m, max: 150 })),
            axisName: { color: getVar('--mui-text-muted') } },
          series: [{ type: 'radar',
            data: [
              { value: series1.slice(0, 6), name: 'Plan',   lineStyle: { color: '#2563eb' }, itemStyle: { color: '#2563eb' } },
              { value: series2.slice(0, 6), name: 'Actual', lineStyle: { color: '#db2777' }, itemStyle: { color: '#db2777' } },
            ] }] };
      case 'scatter':
        return { ...common, xAxis: { axisLabel: { color: getVar('--mui-text-muted') } },
          yAxis: { axisLabel: { color: getVar('--mui-text-muted') } },
          series: [{ type: 'scatter', symbolSize: 14,
            data: series1.map((v, i) => [i, v]), itemStyle: { color: '#2563eb' } }] };
      case 'line':
      default:
        return { ...common, xAxis: { type: 'category', data: MONTHS, axisLabel: { color: getVar('--mui-text-muted') } },
          yAxis: { type: 'value', axisLabel: { color: getVar('--mui-text-muted') } },
          series: [
            { name: 'Plan',   type: 'bar',  data: series1, itemStyle: { color: '#2563eb' } },
            { name: 'Actual', type: 'line', data: series2, smooth: true, lineStyle: { color: '#db2777', width: 2 }, itemStyle: { color: '#db2777' }, areaStyle: { color: 'rgba(219,39,119,0.2)' } },
          ] };
    }
  }

  function getVar(n) { return getComputedStyle(document.documentElement).getPropertyValue(n).trim() || '#fafafa'; }

  function apply() {
    const opt = build();
    chart.clear();
    chart.setOption(opt, true);
    document.getElementById('mui-ec-status-series').textContent = opt.series.length + ' series';
    const pts = opt.series.reduce((a, s) => a + (Array.isArray(s.data) ? s.data.length : 0), 0);
    document.getElementById('mui-ec-status-points').textContent = pts + ' data points';
    document.getElementById('mui-ec-status-type').textContent = kind;
  }
  apply();

  document.getElementById('mui-ec-kind')?.addEventListener('change', (e) => { kind = e.target.value; apply(); });
  document.getElementById('mui-ec-shuffle')?.addEventListener('click', () => {
    series1 = seed(12, 50, 50); series2 = seed(12, 30, 60); apply();
    document.getElementById('mui-ec-dirty')?.setAttribute('data-dirty', 'true');
  });
  document.getElementById('mui-ec-png')?.addEventListener('click', () => {
    const url = chart.getDataURL({ type: 'png', pixelRatio: 2, backgroundColor: getVar('--mui-bg') });
    const a = document.createElement('a');
    a.href = url;
    a.download = 'maud-ui-echarts.png';
    a.click();
  });

  window.addEventListener('resize', () => chart.resize());
  new MutationObserver(() => {
    apply();
    document.getElementById('mui-ec-status-theme').textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
}
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  Wavesurfer.js — audio waveform player
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_wavesurfer_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Wavesurfer \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(wavesurfer_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "Wavesurfer" }
                        }
                        section class="mui-gallery__component" id="integration-wavesurfer" {
                            h3 class="mui-gallery__component-name" { "Wavesurfer.js \u{2014} Audio waveform" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "Wavesurfer.js inside a maud-ui shell, playing a waveform of an "
                                "in-browser-synthesised sample \u{2014} no external audio, no CORS. Click "
                                "the waveform to scrub. The maud-ui toolbar drives the imperative "
                                "API for play / pause / zoom / speed."
                            }
                            div class="mui-integration mui-integration--wavesurfer" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><line x1="4" y1="12" x2="4" y2="12"/><line x1="8" y1="8"  x2="8" y2="16"/><line x1="12" y1="4" x2="12" y2="20"/><line x1="16" y1="7" x2="16" y2="17"/><line x1="20" y1="10" x2="20" y2="14"/></svg>"##.to_string()))
                                        }
                                        span id="mui-ws-title" { "audio/sample-440hz.wav" }
                                        span class="mui-integration__dirty" id="mui-ws-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        button type="button" id="mui-ws-play"     class="mui-btn mui-btn--primary mui-btn--sm" { "Play" }
                                        button type="button" id="mui-ws-stop"     class="mui-btn mui-btn--outline mui-btn--sm" { "Stop" }
                                        button type="button" id="mui-ws-zoom-in"  class="mui-btn mui-btn--outline mui-btn--sm" { "Zoom +" }
                                        button type="button" id="mui-ws-zoom-out" class="mui-btn mui-btn--outline mui-btn--sm" { "Zoom -" }
                                        select id="mui-ws-speed" class="mui-integration__select" aria-label="Playback speed" {
                                            option value="0.5"         { "0.5\u{00d7}" }
                                            option value="1" selected  { "1\u{00d7}" }
                                            option value="1.5"         { "1.5\u{00d7}" }
                                            option value="2"           { "2\u{00d7}" }
                                        }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-ws-root" {
                                    div class="mui-integration__loading" id="mui-ws-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Synthesising audio + loading wavesurfer.js\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-ws-status-time"  { "00:00 / 00:00" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-ws-status-speed" { "1\u{00d7}" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-ws-status-zoom"  { "px/s \u{2014}" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-ws-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script type="importmap" {
                    (maud::PreEscaped(r##"{"imports":{"wavesurfer.js":"https://esm.sh/wavesurfer.js@7.8.13"}}"##))
                }
                script type="module" { (maud::PreEscaped(wavesurfer_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn wavesurfer_css() -> &'static str {
    r#"
.mui-integration--wavesurfer .mui-integration__editor {
    height: 14rem;
    padding: 1.5rem 1.5rem;
    background: var(--mui-bg);
    display: flex;
    align-items: center;
    justify-content: stretch;
}
#mui-ws-surface { width: 100%; }
"#
}

fn wavesurfer_bootstrap() -> &'static str {
    r##"
import WaveSurfer from 'wavesurfer.js';

const host = document.getElementById('mui-ws-root');
if (host) {
  // ── Build an AudioBuffer in-browser via OfflineAudioContext, then hand
  //    it to wavesurfer via the WebAudio backend. This sidesteps MediaElement
  //    decoding of a data blob (which was failing silently on v7) and gives
  //    us real playable audio without any CORS dependency.
  //
  //    20 seconds at 100 px/s default = 2000 px waveform — comfortably
  //    wider than the container, so zoom + / - visibly scrolls / shrinks
  //    the rendering from the first click. A short clip + default 50 px/s
  //    would stretch-to-fit the container and make zoom look broken until
  //    the 6th click.
  const SR = 44100, DUR = 20;

  function synth() {
    const ctx = new (window.OfflineAudioContext || window.webkitOfflineAudioContext)(1, SR * DUR, SR);
    const data = ctx.createBuffer(1, SR * DUR, SR);
    const ch = data.getChannelData(0);
    // A gentle pitch sweep (220 Hz → 880 Hz), with a slow vibrato and a
    // periodic amplitude pulse — produces a visually rich waveform that
    // reads well at any zoom level.
    for (let i = 0; i < ch.length; i++) {
      const t = i / SR;
      const p = t / DUR;                                // 0..1 progress
      const f = 220 + (880 - 220) * p;                  // pitch sweep
      const env = 0.5 * (1 + Math.sin(2 * Math.PI * 0.5 * t));  // 0.5 Hz pulse
      const vib = 1 + 0.004 * Math.sin(2 * Math.PI * 5 * t);    // 5 Hz vibrato
      ch[i] = env * (
        0.55 * Math.sin(2 * Math.PI * f * t * vib) +
        0.20 * Math.sin(2 * Math.PI * f * 1.5 * t) +
        0.04 * (Math.random() * 2 - 1)
      );
    }
    // Short fade-in + fade-out so we don't click.
    const fade = SR * 0.05;
    for (let i = 0; i < fade; i++) {
      ch[i] *= i / fade;
      ch[ch.length - 1 - i] *= i / fade;
    }
    return data;
  }

  // Pre-compute peaks for wavesurfer's waveform rendering (faster than
  // waiting for decode).
  function computePeaks(buffer, n) {
    const ch = buffer.getChannelData(0);
    const step = Math.floor(ch.length / n);
    const peaks = new Float32Array(n);
    for (let i = 0; i < n; i++) {
      let max = 0;
      const s = i * step, e = Math.min(s + step, ch.length);
      for (let j = s; j < e; j++) { const v = Math.abs(ch[j]); if (v > max) max = v; }
      peaks[i] = max;
    }
    return Array.from(peaks);
  }

  host.replaceChildren();
  const surface = document.createElement('div');
  surface.id = 'mui-ws-surface';
  host.appendChild(surface);

  function readVar(n, fb) { return getComputedStyle(document.documentElement).getPropertyValue(n).trim() || fb; }

  const audioBuffer = synth();
  const peaks = computePeaks(audioBuffer, 800);

  const ws = WaveSurfer.create({
    container: surface,
    height: 120,
    waveColor:     readVar('--mui-border-hover', '#3f3f46'),
    progressColor: readVar('--mui-accent',       '#2563eb'),
    cursorColor:   readVar('--mui-accent-text',  '#93c5fd'),
    barWidth: 2,
    barGap: 2,
    barRadius: 2,
    normalize: true,
    peaks: [peaks],
    duration: DUR,
  });

  // Set up a manual Web Audio playback pipeline on top of wavesurfer's
  // visual cursor. wavesurfer drives the UI, we drive the sound.
  let audioCtx = null;
  let currentSrc = null;
  let playStartCtxTime = 0;
  let playStartBufTime = 0;
  let isPlaying = false;
  let rate = 1;
  let currentTime = 0;

  function ensureCtx() {
    if (!audioCtx) audioCtx = new (window.AudioContext || window.webkitAudioContext)();
    if (audioCtx.state === 'suspended') audioCtx.resume();
    return audioCtx;
  }

  function stopSrc() {
    if (currentSrc) {
      try { currentSrc.stop(); } catch {}
      try { currentSrc.disconnect(); } catch {}
      currentSrc = null;
    }
  }

  function playFrom(t) {
    stopSrc();
    const ctx = ensureCtx();
    currentSrc = ctx.createBufferSource();
    currentSrc.buffer = audioBuffer;
    currentSrc.playbackRate.value = rate;
    currentSrc.connect(ctx.destination);
    currentSrc.onended = () => {
      if (currentSrc) {
        isPlaying = false;
        currentTime = 0;
        if (playBtn) playBtn.textContent = 'Play';
        ws.seekTo(0);
        updateTime();
      }
    };
    playStartCtxTime = ctx.currentTime;
    playStartBufTime = Math.max(0, Math.min(DUR, t));
    currentSrc.start(0, playStartBufTime);
    isPlaying = true;
  }

  function pause() {
    if (!isPlaying) return;
    currentTime = currentBufTime();
    stopSrc();
    isPlaying = false;
  }

  function currentBufTime() {
    if (!isPlaying || !audioCtx) return currentTime;
    return playStartBufTime + (audioCtx.currentTime - playStartCtxTime) * rate;
  }

  const playBtn      = document.getElementById('mui-ws-play');
  const stopBtn      = document.getElementById('mui-ws-stop');
  const zoomInBtn    = document.getElementById('mui-ws-zoom-in');
  const zoomOutBtn   = document.getElementById('mui-ws-zoom-out');
  const speedSel     = document.getElementById('mui-ws-speed');
  const statusTime   = document.getElementById('mui-ws-status-time');
  const statusSpeed  = document.getElementById('mui-ws-status-speed');
  const statusZoom   = document.getElementById('mui-ws-status-zoom');
  const statusTheme  = document.getElementById('mui-ws-status-theme');
  const dirty        = document.getElementById('mui-ws-dirty');

  // Default 100 px/s × 20s = 2000 px (already wider than container so
  // zoom buttons have an immediate visible effect). Step by 40 so the
  // change registers visually on every click.
  let zoom = 100;
  function fmt(sec) {
    const m = Math.floor(sec / 60);
    const s = Math.floor(sec % 60);
    return (m < 10 ? '0' : '') + m + ':' + (s < 10 ? '0' : '') + s;
  }
  function updateTime() {
    const cur = Math.min(DUR, Math.max(0, currentBufTime()));
    if (statusTime) statusTime.textContent = fmt(cur) + ' / ' + fmt(DUR);
    if (isPlaying) ws.seekTo(cur / DUR);
  }
  function updateZoom() { if (statusZoom) statusZoom.textContent = 'px/s ' + zoom; }
  updateZoom();

  // Drive the wavesurfer cursor from our audio clock.
  function tick() {
    if (isPlaying) {
      const cur = currentBufTime();
      if (cur >= DUR) {
        // `onended` handles the rest.
      } else {
        updateTime();
      }
    }
    requestAnimationFrame(tick);
  }
  requestAnimationFrame(tick);

  ws.on('ready', () => { ws.zoom(zoom); updateTime(); });
  ws.on('interaction', (t) => {
    // User clicked the waveform — t is a fraction 0..1.
    currentTime = t * DUR;
    if (isPlaying) playFrom(currentTime);
    updateTime();
  });

  playBtn?.addEventListener('click', () => {
    if (isPlaying) {
      pause();
      if (playBtn) playBtn.textContent = 'Play';
    } else {
      playFrom(currentTime);
      if (playBtn) playBtn.textContent = 'Pause';
    }
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  });
  stopBtn?.addEventListener('click', () => {
    stopSrc();
    isPlaying = false;
    currentTime = 0;
    ws.seekTo(0);
    updateTime();
    if (playBtn) playBtn.textContent = 'Play';
  });
  zoomInBtn?.addEventListener('click',  () => { zoom = Math.min(400, zoom + 40); ws.zoom(zoom); updateZoom(); });
  zoomOutBtn?.addEventListener('click', () => { zoom = Math.max(30,  zoom - 40); ws.zoom(zoom); updateZoom(); });
  speedSel?.addEventListener('change', (e) => {
    rate = parseFloat(e.target.value);
    if (statusSpeed) statusSpeed.textContent = rate + '\u00d7';
    if (isPlaying) playFrom(currentBufTime());
  });

  new MutationObserver(() => {
    ws.setOptions({
      waveColor:     readVar('--mui-border-hover', '#3f3f46'),
      progressColor: readVar('--mui-accent',       '#2563eb'),
      cursorColor:   readVar('--mui-accent-text',  '#93c5fd'),
    });
    if (statusTheme) statusTheme.textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
}
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  PDF.js — inline PDF viewer
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_pdfjs_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("PDF.js \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(pdfjs_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "PDF.js" }
                        }
                        section class="mui-gallery__component" id="integration-pdfjs" {
                            h3 class="mui-gallery__component-name" { "PDF.js \u{2014} Inline PDF viewer" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "Mozilla's PDF.js renders a sample document directly onto a canvas inside "
                                "a maud-ui shell. The maud-ui toolbar drives the imperative API \u{2014} "
                                "page navigation, zoom, fit-to-width. The seed PDF is a small in-browser "
                                "generated file so the demo has zero external dependencies."
                            }
                            div class="mui-integration mui-integration--pdfjs" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><path d="M14 2v6h6"/><line x1="9" y1="13" x2="15" y2="13"/><line x1="9" y1="17" x2="13" y2="17"/></svg>"##.to_string()))
                                        }
                                        span id="mui-pdf-title" { "docs/maud-ui-intro.pdf" }
                                        span class="mui-integration__dirty" id="mui-pdf-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        button type="button" id="mui-pdf-prev"     class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2039} Prev" }
                                        button type="button" id="mui-pdf-next"     class="mui-btn mui-btn--outline mui-btn--sm" { "Next \u{203a}" }
                                        button type="button" id="mui-pdf-zoom-out" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2212}" }
                                        button type="button" id="mui-pdf-zoom-in"  class="mui-btn mui-btn--outline mui-btn--sm" { "+" }
                                        button type="button" id="mui-pdf-fit"      class="mui-btn mui-btn--primary mui-btn--sm" { "Fit width" }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-pdf-root" {
                                    div class="mui-integration__loading" id="mui-pdf-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading PDF.js + worker\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-pdf-status-page"  { "Page \u{2014} of \u{2014}" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-pdf-status-zoom"  { "\u{2014}%" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-pdf-status-bytes" { "\u{2014} bytes" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-pdf-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script type="importmap" {
                    (maud::PreEscaped(r##"{
  "imports": {
    "pdfjs-dist":           "https://esm.sh/pdfjs-dist@4.9.155",
    "pdfjs-dist/worker":    "https://esm.sh/pdfjs-dist@4.9.155/build/pdf.worker.mjs"
  }
}"##))
                }
                script type="module" { (maud::PreEscaped(pdfjs_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn pdfjs_css() -> &'static str {
    r#"
.mui-integration--pdfjs .mui-integration__editor {
    height: 34rem;
    padding: 1rem;
    overflow: auto;
    background: var(--mui-bg);
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
}
@media (min-width: 1024px) {
    .mui-integration--pdfjs .mui-integration__editor { height: 42rem; }
}
.mui-integration--pdfjs canvas {
    box-shadow: 0 6px 16px rgba(0, 0, 0, 0.35);
    border-radius: var(--mui-radius-sm);
    background: #fff;
}
"#
}

fn pdfjs_bootstrap() -> &'static str {
    r##"
import * as pdfjsLib from 'pdfjs-dist';

// pdf.js requires a worker URL. esm.sh serves it at the same version.
pdfjsLib.GlobalWorkerOptions.workerSrc = 'https://esm.sh/pdfjs-dist@4.9.155/build/pdf.worker.mjs';

// Minimal hand-rolled multi-page PDF. Three pages, plain text content.
// PDF spec references kept terse since this is purely a seed doc.
function buildPdfBytes() {
  const pages = [
    'maud-ui \u00d7 PDF.js',
    'This is page 2 of the inline demo.',
    'And page 3 \u2014 rendered entirely from PDF.js.',
  ];
  const enc = new TextEncoder();
  const parts = [];
  const offsets = [];
  function emit(s) {
    offsets.push(parts.reduce((a, p) => a + p.length, 0));
    parts.push(enc.encode(s));
  }
  emit('%PDF-1.4\n');
  // 1: Catalog
  emit('1 0 obj\n<< /Type /Catalog /Pages 2 0 R >>\nendobj\n');
  // 2: Pages
  const pageRefs = pages.map((_, i) => (3 + i * 2) + ' 0 R').join(' ');
  emit('2 0 obj\n<< /Type /Pages /Kids [' + pageRefs + '] /Count ' + pages.length + ' >>\nendobj\n');
  // 3..: each page + its content stream
  for (let i = 0; i < pages.length; i++) {
    const pageIdx = 3 + i * 2;
    const streamIdx = pageIdx + 1;
    emit(pageIdx + ' 0 obj\n<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Contents '
         + streamIdx + ' 0 R /Resources << /Font << /F1 << /Type /Font /Subtype /Type1 /BaseFont /Helvetica >> >> >> >>\nendobj\n');
    const body = 'BT /F1 26 Tf 60 720 Td (' + pages[i].replace(/([()\\])/g, '\\$1') + ') Tj ET\n'
               + 'BT /F1 12 Tf 60 680 Td (Page ' + (i + 1) + ' of ' + pages.length + ') Tj ET\n'
               + 'BT /F1 12 Tf 60 660 Td (maud-ui \u2014 a gallery of components built for maud + htmx) Tj ET\n';
    emit(streamIdx + ' 0 obj\n<< /Length ' + body.length + ' >>\nstream\n' + body + 'endstream\nendobj\n');
  }
  const xrefOffset = parts.reduce((a, p) => a + p.length, 0);
  const objCount = 2 + pages.length * 2;
  let xref = 'xref\n0 ' + (objCount + 1) + '\n0000000000 65535 f \n';
  for (let i = 0; i < offsets.length; i++) {
    xref += (offsets[i].toString().padStart(10, '0')) + ' 00000 n \n';
  }
  xref += 'trailer\n<< /Size ' + (objCount + 1) + ' /Root 1 0 R >>\nstartxref\n' + xrefOffset + '\n%%EOF\n';
  parts.push(enc.encode(xref));
  const total = parts.reduce((a, p) => a + p.length, 0);
  const out = new Uint8Array(total);
  let off = 0;
  for (const p of parts) { out.set(p, off); off += p.length; }
  return out;
}

const host = document.getElementById('mui-pdf-root');
if (host) {
  const bytes = buildPdfBytes();
  document.getElementById('mui-pdf-status-bytes').textContent = bytes.length.toLocaleString() + ' bytes';

  (async () => {
    const doc = await pdfjsLib.getDocument({ data: bytes }).promise;
    host.replaceChildren();
    const scroll = document.createElement('div');
    scroll.style.cssText = 'display:flex;flex-direction:column;align-items:center;gap:1rem;width:100%;';
    host.appendChild(scroll);

    let scale = 1.1;
    let current = 1;
    const total = doc.numPages;
    const canvases = [];
    const pages = [];

    for (let i = 1; i <= total; i++) {
      const page = await doc.getPage(i);
      pages.push(page);
      const canvas = document.createElement('canvas');
      canvases.push(canvas);
      scroll.appendChild(canvas);
    }

    async function render() {
      for (let i = 0; i < pages.length; i++) {
        const vp = pages[i].getViewport({ scale });
        const c = canvases[i];
        c.width = vp.width; c.height = vp.height;
        await pages[i].render({ canvasContext: c.getContext('2d'), viewport: vp }).promise;
      }
      document.getElementById('mui-pdf-status-page').textContent = 'Page ' + current + ' of ' + total;
      document.getElementById('mui-pdf-status-zoom').textContent = Math.round(scale * 100) + '%';
    }
    await render();

    function scrollTo(i) {
      current = Math.max(1, Math.min(total, i));
      canvases[current - 1].scrollIntoView({ behavior: 'smooth', block: 'start' });
      document.getElementById('mui-pdf-status-page').textContent = 'Page ' + current + ' of ' + total;
    }

    document.getElementById('mui-pdf-prev')?.addEventListener('click', () => scrollTo(current - 1));
    document.getElementById('mui-pdf-next')?.addEventListener('click', () => scrollTo(current + 1));
    document.getElementById('mui-pdf-zoom-in')?.addEventListener('click',  async () => { scale = Math.min(3, scale + 0.15); await render(); });
    document.getElementById('mui-pdf-zoom-out')?.addEventListener('click', async () => { scale = Math.max(0.4, scale - 0.15); await render(); });
    document.getElementById('mui-pdf-fit')?.addEventListener('click', async () => {
      const targetWidth = host.clientWidth - 32;
      const vp = pages[0].getViewport({ scale: 1 });
      scale = targetWidth / vp.width;
      await render();
    });

    new MutationObserver(() => {
      document.getElementById('mui-pdf-status-theme').textContent = document.documentElement.getAttribute('data-theme') || 'dark';
    }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
  })().catch((err) => {
    console.error('[maud-ui] PDF.js failed:', err);
    host.replaceChildren();
    const pre = document.createElement('pre');
    pre.style.cssText = 'color:#f87171;padding:1rem;font-family:var(--mui-font-mono);font-size:0.8125rem;';
    pre.textContent = '[PDF.js error] ' + String(err.message || err);
    host.appendChild(pre);
  });
}
"##
}

// ═══════════════════════════════════════════════════════════════════════
//  Cytoscape.js — network graph visualisation
// ═══════════════════════════════════════════════════════════════════════

pub fn integrations_cytoscape_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("Cytoscape \u{2014} maud-ui integrations"))
                style { (maud::PreEscaped(integration_shell_css())) }
                style { (maud::PreEscaped(cytoscape_css())) }
            }
            body {
                (page_header())
                div class="mui-gallery" {
                    (sidebar_nav())
                    main class="mui-gallery__main" {
                        nav class="mui-gallery__breadcrumb" {
                            a href="/" { "Gallery" } span { " / " } span { "Integrations" } span { " / " } span { "Cytoscape" }
                        }
                        section class="mui-gallery__component" id="integration-cytoscape" {
                            h3 class="mui-gallery__component-name" { "Cytoscape.js \u{2014} Network graph" }
                            p style="font-size:0.9375rem;color:var(--mui-text-muted);max-width:48rem;margin:0 0 1.5rem;line-height:1.55;" {
                                "Cytoscape.js is the sibling-by-different-mission of xyflow \u{2014} where "
                                "xyflow is an "
                                em { "editor" }
                                ", Cytoscape is a "
                                em { "visualisation / analytics" }
                                " library. A network of Kapable's service dependencies mounts into a "
                                "maud-ui shell with a choice of layouts."
                            }
                            div class="mui-integration mui-integration--cytoscape" {
                                div class="mui-integration__header" {
                                    div class="mui-integration__filepath" {
                                        span class="mui-integration__filepath-icon" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="5" r="2"/><circle cx="5" cy="19" r="2"/><circle cx="19" cy="19" r="2"/><circle cx="12" cy="12" r="2"/><line x1="12" y1="7" x2="12" y2="10"/><line x1="10" y1="14" x2="6" y2="17"/><line x1="14" y1="14" x2="18" y2="17"/></svg>"##.to_string()))
                                        }
                                        span id="mui-cy-title" { "graphs/service-mesh.json" }
                                        span class="mui-integration__dirty" id="mui-cy-dirty" aria-hidden="true" { "\u{25cf}" }
                                    }
                                    div class="mui-integration__toolbar" {
                                        select id="mui-cy-layout" class="mui-integration__select" aria-label="Layout" {
                                            option value="cose" selected  { "Force (cose)" }
                                            option value="concentric"      { "Concentric" }
                                            option value="breadthfirst"    { "Tree" }
                                            option value="grid"            { "Grid" }
                                            option value="circle"          { "Circle" }
                                        }
                                        button type="button" id="mui-cy-add"   class="mui-btn mui-btn--outline mui-btn--sm" { "+ Node" }
                                        button type="button" id="mui-cy-fit"   class="mui-btn mui-btn--outline mui-btn--sm" { "Fit" }
                                        button type="button" id="mui-cy-reset" class="mui-btn mui-btn--primary mui-btn--sm" { "Reset" }
                                    }
                                }
                                div class="mui-integration__editor" id="mui-cy-root" {
                                    div class="mui-integration__loading" id="mui-cy-loading" {
                                        span class="mui-spin" aria-hidden="true" {
                                            (maud::PreEscaped(r##"<svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 12a9 9 0 1 1-6.219-8.56"/></svg>"##.to_string()))
                                        }
                                        span { "Loading Cytoscape.js from esm.sh\u{2026}" }
                                    }
                                }
                                div class="mui-integration__statusbar" {
                                    span id="mui-cy-status-nodes" { "0 nodes" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-cy-status-edges" { "0 edges" }
                                    span class="mui-integration__statusbar-sep" aria-hidden="true" { "\u{2022}" }
                                    span id="mui-cy-status-sel"   { "No selection" }
                                    span class="mui-integration__statusbar-spacer" {}
                                    span class="mui-integration__statusbar-theme" id="mui-cy-status-theme" { "dark" }
                                }
                            }
                        }
                        div class="mui-gallery__back" { a href="/" class="mui-btn mui-btn--outline mui-btn--sm" { "\u{2190} Back to Gallery" } }
                    }
                }
                script type="importmap" {
                    (maud::PreEscaped(r##"{"imports":{"cytoscape":"https://esm.sh/cytoscape@3.30.4"}}"##))
                }
                script type="module" { (maud::PreEscaped(cytoscape_bootstrap())) }
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}

fn cytoscape_css() -> &'static str {
    r#"
.mui-integration--cytoscape .mui-integration__editor {
    height: 30rem;
    padding: 0;
    background: var(--mui-bg);
}
@media (min-width: 1024px) {
    .mui-integration--cytoscape .mui-integration__editor { height: 36rem; }
}
"#
}

fn cytoscape_bootstrap() -> &'static str {
    r##"
import cytoscape from 'cytoscape';

const host = document.getElementById('mui-cy-root');
if (host) {
  host.replaceChildren();

  function v(n, fb) { return getComputedStyle(document.documentElement).getPropertyValue(n).trim() || fb; }

  const SEED = {
    nodes: [
      { data: { id: 'api',     label: 'kapable-api' } },
      { data: { id: 'proxy',   label: 'kapable-proxy' } },
      { data: { id: 'forge',   label: 'forge' } },
      { data: { id: 'worker',  label: 'worker' } },
      { data: { id: 'tunnel',  label: 'kapable-tunnel' } },
      { data: { id: 'db',      label: 'postgres' } },
      { data: { id: 'console', label: 'console' } },
      { data: { id: 'admin',   label: 'admin' } },
      { data: { id: 'dev',     label: 'developer' } },
      { data: { id: 'caddy',   label: 'caddy' } },
    ],
    edges: [
      { data: { source: 'caddy',   target: 'proxy' } },
      { data: { source: 'proxy',   target: 'api' } },
      { data: { source: 'proxy',   target: 'console' } },
      { data: { source: 'proxy',   target: 'admin' } },
      { data: { source: 'proxy',   target: 'dev' } },
      { data: { source: 'api',     target: 'db' } },
      { data: { source: 'api',     target: 'forge' } },
      { data: { source: 'forge',   target: 'worker' } },
      { data: { source: 'worker',  target: 'db' } },
      { data: { source: 'api',     target: 'tunnel' } },
      { data: { source: 'console', target: 'api' } },
      { data: { source: 'admin',   target: 'api' } },
    ],
  };

  const cy = cytoscape({
    container: host,
    elements: JSON.parse(JSON.stringify(SEED)),
    style: [
      { selector: 'node',
        style: {
          'background-color': v('--mui-bg-card', '#111113'),
          'border-color':     v('--mui-border',   '#27272a'),
          'border-width':     1,
          'color':            v('--mui-text',     '#fafafa'),
          'label':            'data(label)',
          'font-family':      v('--mui-font-sans', 'system-ui'),
          'font-size':        12,
          'text-valign':      'bottom',
          'text-margin-y':    6,
          'width':            36,
          'height':           36,
        } },
      { selector: 'node:selected',
        style: {
          'border-color':  v('--mui-accent', '#2563eb'),
          'border-width':  2,
        } },
      { selector: 'edge',
        style: {
          'width':            1.5,
          'line-color':       v('--mui-border-hover', '#3f3f46'),
          'target-arrow-color': v('--mui-border-hover', '#3f3f46'),
          'target-arrow-shape':'triangle',
          'curve-style':      'bezier',
        } },
      { selector: 'edge:selected',
        style: { 'line-color': v('--mui-accent', '#2563eb'), 'target-arrow-color': v('--mui-accent', '#2563eb') } },
    ],
    layout: { name: 'cose', animate: true, padding: 30 },
    minZoom: 0.2, maxZoom: 3,
  });

  const statusNodes = document.getElementById('mui-cy-status-nodes');
  const statusEdges = document.getElementById('mui-cy-status-edges');
  const statusSel   = document.getElementById('mui-cy-status-sel');
  const statusTheme = document.getElementById('mui-cy-status-theme');
  const dirty       = document.getElementById('mui-cy-dirty');

  function updateCounts() {
    if (statusNodes) statusNodes.textContent = cy.nodes().length + ' nodes';
    if (statusEdges) statusEdges.textContent = cy.edges().length + ' edges';
  }
  updateCounts();
  cy.on('select unselect', () => {
    const sel = cy.$(':selected');
    if (statusSel) statusSel.textContent = sel.length === 0 ? 'No selection' : (sel[0].data('label') || sel[0].id()) + (sel.length > 1 ? ' (+' + (sel.length - 1) + ')' : '');
  });
  cy.on('add remove', updateCounts);

  document.getElementById('mui-cy-layout')?.addEventListener('change', (e) => {
    cy.layout({ name: e.target.value, animate: true, padding: 30 }).run();
  });
  document.getElementById('mui-cy-add')?.addEventListener('click', () => {
    const id = 'n' + Date.now();
    cy.add({ data: { id, label: 'svc-' + (cy.nodes().length + 1) } });
    const src = cy.nodes()[Math.floor(Math.random() * (cy.nodes().length - 1))].id();
    cy.add({ data: { id: 'e' + id, source: src, target: id } });
    cy.layout({ name: 'cose', animate: true, padding: 30 }).run();
    if (dirty) dirty.setAttribute('data-dirty', 'true');
  });
  document.getElementById('mui-cy-fit')?.addEventListener('click', () => cy.fit(null, 40));
  document.getElementById('mui-cy-reset')?.addEventListener('click', () => {
    cy.elements().remove();
    cy.add(JSON.parse(JSON.stringify(SEED)));
    cy.layout({ name: 'cose', animate: true, padding: 30 }).run();
    updateCounts();
  });

  new MutationObserver(() => {
    cy.style()
      .selector('node').style({
        'background-color': v('--mui-bg-card', '#111113'),
        'border-color':     v('--mui-border',   '#27272a'),
        'color':            v('--mui-text',     '#fafafa'),
      })
      .selector('edge').style({
        'line-color':         v('--mui-border-hover', '#3f3f46'),
        'target-arrow-color': v('--mui-border-hover', '#3f3f46'),
      })
      .update();
    if (statusTheme) statusTheme.textContent = document.documentElement.getAttribute('data-theme') || 'dark';
  }).observe(document.documentElement, { attributes: true, attributeFilter: ['data-theme'] });
}
"##
}

/// Minimal JSON-string escaper for injecting sample code as a JS literal
/// (avoids pulling in serde as a runtime dependency just for this).
fn serde_json_lite_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 16);
    out.push('"');
    for c in s.chars() {
        match c {
            '"'  => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x08' => out.push_str("\\b"),
            '\x0c' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
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
                script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
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
                        script src=(format!("/js/maud-ui.js?v={}", JS_VER)) defer {}
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

/* ── Sticky compact page header ─────────────────────────────────────
 * Overrides the dist maud-ui.css baseline (which shipped 2rem padding
 * + a two-row "brand above nav" layout). The new header is a single
 * row with brand · nav · tools, pinned to the top with a blurred
 * backdrop. --mui-header-h is published as a custom property so the
 * sidebar and section anchors can offset accurately. */
:root { --mui-header-h: 3.25rem; }

.mui-showcase__header {
    position: sticky;
    top: 0;
    z-index: 50;
    padding: 0.5rem 1.25rem !important;
    background: color-mix(in srgb, var(--mui-bg) 82%, transparent);
    -webkit-backdrop-filter: saturate(150%) blur(14px);
    backdrop-filter: saturate(150%) blur(14px);
}
.mui-showcase__header h1 {
    /* the old dist rule styled an <h1> inside the header; we no
     * longer render one, but keep this reset in case something else
     * hits it. */
    font-size: 1rem;
    margin: 0;
}

.mui-showcase__header-inner {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    flex-wrap: wrap;
    min-height: 2.25rem;
}

.mui-showcase__brand {
    display: inline-flex;
    align-items: baseline;
    gap: 0.625rem;
    color: inherit;
    text-decoration: none;
    padding-inline: 0.25rem;
    flex-shrink: 0;
}

/* Search input — takes flexible middle space, bounded so it doesn't
 * crowd out the nav on wider viewports. Hotkey: "/" to focus. */
.mui-showcase__search {
    position: relative;
    display: inline-flex;
    align-items: center;
    flex: 1 1 14rem;
    max-width: 26rem;
    min-width: 9rem;
}
.mui-showcase__search-icon {
    position: absolute;
    left: 0.5rem;
    display: inline-flex;
    color: var(--mui-text-subtle);
    pointer-events: none;
}
.mui-showcase__search-input {
    width: 100%;
    height: 2rem;
    padding: 0 2rem 0 1.875rem;
    font-size: 0.8125rem;
    font-family: inherit;
    color: var(--mui-text);
    background: var(--mui-bg-card);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-md);
    outline: none;
    transition: border-color var(--mui-transition),
                box-shadow var(--mui-transition);
}
.mui-showcase__search-input::placeholder { color: var(--mui-text-subtle); }
.mui-showcase__search-input:focus-visible {
    border-color: var(--mui-accent);
    box-shadow: 0 0 0 2px color-mix(in srgb, var(--mui-accent) 25%, transparent);
}
.mui-showcase__search-input::-webkit-search-cancel-button { -webkit-appearance: none; }

.mui-showcase__search-hint {
    position: absolute;
    right: 0.5rem;
    font-family: var(--mui-font-mono);
    font-size: 0.6875rem;
    line-height: 1;
    padding: 0.125rem 0.375rem;
    background: var(--mui-bg);
    color: var(--mui-text-muted);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-sm);
    pointer-events: none;
}
.mui-showcase__search--focused .mui-showcase__search-hint,
.mui-showcase__search--dirty  .mui-showcase__search-hint { opacity: 0; }

/* When filtering, grey out group headers whose items are all hidden,
 * and hide the empty group entirely. */
.mui-gallery__nav-item[hidden] { display: none; }
.mui-gallery__nav-group[data-mui-search-empty="1"] { display: none; }

/* Search-match highlight in the sidebar text */
.mui-gallery__nav-item mark {
    background: color-mix(in srgb, var(--mui-accent) 30%, transparent);
    color: var(--mui-text);
    border-radius: 2px;
    padding: 0 1px;
}

/* No-results state inside the sidebar */
.mui-gallery__nav-empty {
    padding: 0.75rem 1rem;
    font-size: 0.75rem;
    color: var(--mui-text-muted);
    display: none;
}
.mui-gallery__nav[data-mui-search-empty="1"] .mui-gallery__nav-empty { display: block; }

@media (max-width: 760px) {
    /* Drop the hint + shrink a touch on narrow screens */
    .mui-showcase__search-hint { display: none; }
    .mui-showcase__search { min-width: 7rem; }
}
.mui-showcase__brand-name {
    font-size: 1rem;
    font-weight: 700;
    letter-spacing: -0.01em;
    color: var(--mui-text);
}
.mui-showcase__brand-count {
    font-size: 0.75rem;
    color: var(--mui-text-muted);
}

.mui-showcase__nav {
    display: inline-flex;
    gap: 0.125rem;
    align-items: center;
    flex-wrap: wrap;
    /* Reset the dist CSS default that paints <nav> elements with a
     * card background — inside the header we want the nav to sit
     * transparent on the header's own backdrop. */
    background: transparent;
    padding: 0;
    border: 0;
    box-shadow: none;
}

.mui-showcase__tools {
    display: inline-flex;
    gap: 0.25rem;
    align-items: center;
    margin-left: 0.25rem;
    padding-left: 0.5rem;
    border-left: 1px solid var(--mui-border);
}

.mui-showcase__tool-btn {
    min-width: 2rem;
    padding-inline: 0.5rem;
    font-size: 1rem;
    line-height: 1;
}

/* The sidebar stuck to top:0 hid behind the new header; pin it
 * below the sticky chrome instead. */
.mui-gallery__sidebar {
    top: var(--mui-header-h) !important;
    height: calc(100vh - var(--mui-header-h)) !important;
}

/* When the user clicks a sidebar / in-page jump link, keep the
 * target heading below the sticky header instead of jumping it off
 * the top of the viewport. */
.mui-gallery__component,
.mui-gallery__breadcrumb,
[id] {
    scroll-margin-top: calc(var(--mui-header-h) + 0.75rem);
}

/* Narrow screens: let brand + count stack smaller + hide subtitle */
@media (max-width: 640px) {
    .mui-showcase__brand-count { display: none; }
    :root { --mui-header-h: 3rem; }
}

/* ── "Advanced" dropdown in the page header ─────────────────────────
 * <details>/<summary>-based, so it works without JS. The menu is
 * positioned absolutely below the summary; `list-style:none` strips
 * the default disclosure triangle and we supply our own caret that
 * rotates on [open]. */
.mui-gallery__nav-advanced {
    position: relative;
}

.mui-gallery__nav-advanced-summary {
    list-style: none;
    display: inline-flex;
    align-items: center;
    gap: 0.375rem;
    cursor: pointer;
    user-select: none;
}
.mui-gallery__nav-advanced-summary::-webkit-details-marker { display: none; }
.mui-gallery__nav-advanced-summary::marker { content: ''; }

.mui-gallery__nav-advanced-caret {
    display: inline-block;
    font-size: 0.75rem;
    line-height: 1;
    transition: transform 150ms ease;
    color: var(--mui-text-muted);
}
.mui-gallery__nav-advanced[open] .mui-gallery__nav-advanced-caret {
    transform: rotate(180deg);
}

.mui-gallery__nav-advanced-menu {
    position: absolute;
    top: calc(100% + 0.375rem);
    right: 0;
    z-index: 60;
    min-width: 18rem;
    max-height: calc(100vh - 8rem);
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    padding: 0.375rem;
    background: var(--mui-bg-card);
    border: 1px solid var(--mui-border);
    border-radius: var(--mui-radius-md);
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.35),
                0 2px 6px rgba(0, 0, 0, 0.2);
}

.mui-gallery__nav-advanced-group {
    display: flex;
    flex-direction: column;
    gap: 0.0625rem;
    padding: 0.25rem 0 0.375rem;
    border-bottom: 1px solid var(--mui-border);
}
.mui-gallery__nav-advanced-group:last-child { border-bottom: 0; }

.mui-gallery__nav-advanced-group-label {
    display: block;
    padding: 0.375rem 0.75rem 0.25rem;
    font-size: 0.6875rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--mui-text-subtle);
}

.mui-gallery__nav-advanced-menu a {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    padding: 0.4375rem 0.75rem;
    border-radius: var(--mui-radius-sm);
    color: var(--mui-text);
    text-decoration: none;
    transition: background-color var(--mui-transition),
                color var(--mui-transition);
}
.mui-gallery__nav-advanced-menu a:hover,
.mui-gallery__nav-advanced-menu a:focus-visible {
    background: var(--mui-bg);
    outline: none;
}

.mui-gallery__nav-advanced-label {
    font-size: 0.8125rem;
    font-weight: 600;
}
.mui-gallery__nav-advanced-sub {
    font-size: 0.75rem;
    color: var(--mui-text-muted);
    font-weight: 400;
}
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
                        // Nav items link to /{slug} for cross-page nav,
                        // so we match by data-slug instead of href.
                        if (item.getAttribute('data-slug') === id) {
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

    // ── Sidebar search ────────────────────────────────────────────
    // Filters .mui-gallery__nav-item entries against the input value.
    // Groups whose items are all hidden collapse. "/" focuses the
    // search box (skipped if the user is already typing into another
    // input). "Esc" clears and restores the full list.
    var search = document.getElementById('mui-search');
    if (search && navItems.length > 0) {
        var wrap = search.closest('.mui-showcase__search');
        var nav = document.querySelector('.mui-gallery__nav');
        var groups = Array.prototype.slice.call(document.querySelectorAll('.mui-gallery__nav-group'));

        // Cache each nav item's display text + its tier header so we
        // can re-run the filter quickly without hitting DOM on each
        // keypress.
        var entries = [];
        for (var i = 0; i < navItems.length; i++) {
            var it = navItems[i];
            entries.push({
                el: it,
                label: it.textContent || '',
                slug: it.getAttribute('data-slug') || '',
                original: it.innerHTML,
            });
        }

        // Ensure "no results" message exists inside the nav container.
        var empty = nav ? nav.querySelector('.mui-gallery__nav-empty') : null;
        if (!empty && nav) {
            empty = document.createElement('div');
            empty.className = 'mui-gallery__nav-empty';
            empty.textContent = 'No matches. Press Esc to clear.';
            nav.appendChild(empty);
        }

        function escapeRegex(s) { return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'); }
        function escapeHtml(s) {
            return s.replace(/&/g, '&amp;')
                    .replace(/</g, '&lt;')
                    .replace(/>/g, '&gt;')
                    .replace(/"/g, '&quot;');
        }

        function runFilter() {
            var q = search.value.trim().toLowerCase();
            var dirty = q.length > 0;
            wrap.classList.toggle('mui-showcase__search--dirty', dirty);

            var visibleCount = 0;
            var pattern = dirty ? new RegExp('(' + escapeRegex(q) + ')', 'ig') : null;

            for (var i = 0; i < entries.length; i++) {
                var e = entries[i];
                var label = e.label.toLowerCase();
                var slug  = e.slug.toLowerCase();
                var match = !dirty || label.indexOf(q) !== -1 || slug.indexOf(q) !== -1;
                if (match) {
                    e.el.hidden = false;
                    if (dirty) {
                        e.el.innerHTML = escapeHtml(e.label).replace(pattern, '<mark>$1</mark>');
                    } else {
                        e.el.innerHTML = e.original;
                    }
                    visibleCount++;
                } else {
                    e.el.hidden = true;
                }
            }

            // Collapse empty groups. A group is "empty" when all its
            // child nav items are hidden.
            for (var g = 0; g < groups.length; g++) {
                var group = groups[g];
                var kids = group.querySelectorAll('.mui-gallery__nav-item');
                var any = false;
                for (var k = 0; k < kids.length; k++) {
                    if (!kids[k].hidden) { any = true; break; }
                }
                group.setAttribute('data-mui-search-empty', any ? '0' : '1');
            }
            if (nav) nav.setAttribute('data-mui-search-empty', visibleCount === 0 && dirty ? '1' : '0');
        }

        search.addEventListener('input', runFilter);
        search.addEventListener('focus', function () { wrap.classList.add('mui-showcase__search--focused'); });
        search.addEventListener('blur',  function () { wrap.classList.remove('mui-showcase__search--focused'); });
        search.addEventListener('keydown', function (e) {
            if (e.key === 'Escape') {
                search.value = '';
                runFilter();
                search.blur();
            }
        });

        // Global "/" hotkey — focus unless the user is typing into
        // another input/textarea/contenteditable.
        document.addEventListener('keydown', function (e) {
            if (e.key !== '/') return;
            var t = e.target;
            if (!t) return;
            var tag = (t.tagName || '').toLowerCase();
            var typing = tag === 'input' || tag === 'textarea' || tag === 'select' || t.isContentEditable;
            if (typing) return;
            e.preventDefault();
            search.focus();
            search.select();
        });
    }
})();
"#
}
