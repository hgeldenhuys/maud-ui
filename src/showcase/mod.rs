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
                            a href="/integrations/monaco-editor" role="menuitem" {
                                span class="mui-gallery__nav-advanced-label" { "Monaco editor" }
                                span class="mui-gallery__nav-advanced-sub" { "VS Code's editor, embedded" }
                            }
                            a href="/integrations/xyflow" role="menuitem" {
                                span class="mui-gallery__nav-advanced-label" { "xyflow" }
                                span class="mui-gallery__nav-advanced-sub" { "React Flow node editor" }
                            }
                            a href="/integrations/excalidraw" role="menuitem" {
                                span class="mui-gallery__nav-advanced-label" { "Excalidraw" }
                                span class="mui-gallery__nav-advanced-sub" { "Sketchy whiteboard canvas" }
                            }
                        }
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
                    span.mui-text-subtle style="font-size:0.8125rem;" { "Dir:" }
                    button type="button" class="mui-btn mui-btn--outline mui-btn--sm" data-mui="dir-toggle" id="dir-toggle" aria-label="Toggle reading direction" {
                        "RTL"
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
    min-width: 16rem;
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

.mui-gallery__nav-advanced-menu a {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    padding: 0.5rem 0.75rem;
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
})();
"#
}
