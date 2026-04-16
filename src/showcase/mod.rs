//! Live component gallery. Serves `showcase_page()` as the `showcase` axum example.
//! Each component also has its own route via `component_page_by_name()`.

use maud::{html, Markup, DOCTYPE};

use crate::primitives;

/// All component slug names, used for nav generation and route dispatch.
pub const COMPONENT_NAMES: &[&str] = &[
    "button",
    "badge",
    "avatar",
    "separator",
    "progress",
    "meter",
    "input",
    "textarea",
    "checkbox",
    "radio",
    "field",
    "fieldset",
    "number_field",
    "switch",
    "toggle",
    "toggle_group",
    "tabs",
    "collapsible",
    "accordion",
    "toast",
    "tooltip",
    "dialog",
    "alert_dialog",
    "popover",
    "select",
    "menu",
    "context_menu",
    "slider",
    "scroll_area",
];

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
        "button" => primitives::button::showcase(),
        "badge" => primitives::badge::showcase(),
        "avatar" => primitives::avatar::showcase(),
        "separator" => primitives::separator::showcase(),
        "progress" => primitives::progress::showcase(),
        "meter" => primitives::meter::showcase(),
        "input" => primitives::input::showcase(),
        "textarea" => primitives::textarea::showcase(),
        "checkbox" => primitives::checkbox::showcase(),
        "radio" => primitives::radio::showcase(),
        "field" => primitives::field::showcase(),
        "fieldset" => primitives::fieldset::showcase(),
        "number_field" => primitives::number_field::showcase(),
        "switch" => primitives::switch::showcase(),
        "toggle" => primitives::toggle::showcase(),
        "toggle_group" => primitives::toggle_group::showcase(),
        "tabs" => primitives::tabs::showcase(),
        "collapsible" => primitives::collapsible::showcase(),
        "accordion" => primitives::accordion::showcase(),
        "toast" => primitives::toast::showcase(),
        "tooltip" => primitives::tooltip::showcase(),
        "dialog" => primitives::dialog::showcase(),
        "alert_dialog" => primitives::alert_dialog::showcase(),
        "popover" => primitives::popover::showcase(),
        "select" => primitives::select::showcase(),
        "menu" => primitives::menu::showcase(),
        "context_menu" => primitives::context_menu::showcase(),
        "slider" => primitives::slider::showcase(),
        "scroll_area" => primitives::scroll_area::showcase(),
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
        link rel="stylesheet" href="/dist/maud-ui.css";
    }
}

/// Shared header with theme toggle.
fn page_header() -> Markup {
    html! {
        header.mui-showcase__header {
            div style="display:flex;justify-content:space-between;align-items:center;" {
                div {
                    h1 { a href="/" style="color:inherit;text-decoration:none;" { "maud-ui" } }
                    p.mui-text-muted { "Headless accessible components for maud + htmx" }
                }
                button type="button" class="mui-btn mui-btn--outline mui-btn--md" data-mui="theme-toggle" {
                    "Toggle theme"
                }
            }
        }
    }
}

/// Navigation grid linking to each component's individual page.
fn nav_grid() -> Markup {
    html! {
        nav.mui-showcase__nav {
            h2 { "Components" }
            div.mui-showcase__nav-grid {
                @for name in COMPONENT_NAMES {
                    a.mui-showcase__nav-link href=(format!("/{}", name)) {
                        (display_name(name))
                    }
                }
            }
        }
    }
}

/// Main gallery page at `/`. Shows nav grid plus all 29 components inline.
pub fn showcase_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                (page_head("maud-ui \u{00b7} Component Gallery"))
            }
            body {
                (page_header())
                main.mui-showcase {
                    (nav_grid())
                    @for name in COMPONENT_NAMES {
                        @if let Some(content) = component_content(name) {
                            section id=(name) {
                                h2 { (display_name(name)) }
                                (content)
                            }
                        }
                    }
                }
                script src="/dist/maud-ui.js" defer {}
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
                main.mui-showcase {
                    nav.mui-showcase__breadcrumb {
                        a href="/" { "Gallery" }
                        span { " / " }
                        span { (display_name(name)) }
                    }
                    section id=(name) {
                        h2 { (display_name(name)) }
                        (content)
                    }
                    div.mui-showcase__back {
                        a href="/" class="mui-btn mui-btn--outline mui-btn--sm" {
                            "\u{2190} Back to Gallery"
                        }
                    }
                }
                script src="/dist/maud-ui.js" defer {}
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
                        main.mui-showcase {
                            section {
                                h2 { "Component not found" }
                                p { "No component named \"" (name) "\" exists." }
                                a href="/" class="mui-btn mui-btn--outline mui-btn--sm" {
                                    "\u{2190} Back to Gallery"
                                }
                            }
                        }
                        script src="/dist/maud-ui.js" defer {}
                    }
                }
            }
        }
    }
}
