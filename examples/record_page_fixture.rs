//! Deterministic generic-record fixtures. No browser is launched by this example.
use maud::{html, Markup, DOCTYPE};
use maud_ui::{
    blocks::{
        action::{Action, Heading},
        record::{facts, header, page, related_list},
        shell::{app_footer, app_header, brand_mark, page_header, sidebar},
    },
    primitives::{badge, breadcrumb::BreadcrumbItem, card, stack},
};

pub fn guest(density: &str, long: bool) -> Markup {
    let name = if long {
        "Sofia Priyanka Patel-Nair"
    } else {
        "Sofia Patel"
    };
    document(
        density,
        html! {
            (sidebar::render(sidebar::Props {
                id: "generic-record".into(), brand: html! { "Lodge Operations" },
                app_header: app_header::render(app_header::Props { brand_mark: brand_mark::preset("lodge"), ..Default::default() }),
                app_footer: app_footer::render(app_footer::Props { line: Some("Garden House · Independent hospitality".into()), ..Default::default() }),
                nav_groups: vec![sidebar::NavGroup { label: Some("Reservations".into()), items: vec![
                    sidebar::NavItem { label: "Booking".into(), href: "/blocks/record-timeline".into(), ..Default::default() },
                    sidebar::NavItem { label: "Guest".into(), href: "/blocks/record-page".into(), ..Default::default() },
                ] }], active_path: "/blocks/record-page".into(),
                page_header: Some(page_header::Props {
                    breadcrumbs: vec![
                        BreadcrumbItem { label: "Lodge Operations".into(), href: Some("/blocks/shell-sidebar".into()) },
                        BreadcrumbItem { label: "  ".into(), href: Some("#empty".into()) },
                        BreadcrumbItem { label: if long { "Guest directory · Current and returning guests".into() } else { "Guests".into() }, href: Some("/blocks/worklist-header".into()) },
                        BreadcrumbItem { label: format!("Guest: {name}"), href: None },
                    ],
                    search: Some(page_header::Search { action: "/".into(), ..Default::default() }),
                    actions: html! { a class="mui-btn mui-btn--outline" href="/blocks/record-page" { "Journeys" } },
                    switchers: html! {
                        label { span class="mui-sr-only" { "Role" } select class="mui-native-select" { option { "Receptionist" } } }
                        label { span class="mui-sr-only" { "Language" } select class="mui-native-select" { option { "English" } } }
                    }, ..Default::default()
                }),
                children: page::render(page::Props {
                    header: header::Props { title: name.into(), kind: Some("Guest".into()), reference: Some("UI-G5".into()), heading: Heading::H1,
                        primary_action: Some(Action::link("Edit guest", "/blocks/record-facts")),
                        secondary_actions: vec![Action::link("Graph view", "/blocks/record-related-list")], ..Default::default() },
                    children: html! {
                        (facts::render(facts::Props { groups: vec![
                            facts::Group { title: "Identity".into(), facts: vec![facts::Fact::mono("Guest ID", "UI-G5"), facts::Fact::new("Full name", name),
                                facts::Fact::new("Email", if long { "sofia.priyanka.patel.nair@example.com" } else { "sofia.patel@example.com" }), facts::Fact::new("Nationality", "South African")] },
                            facts::Group { title: "Sensitive details".into(), facts: vec![facts::Fact::masked("Phone", "••••••0005"), facts::Fact::masked("ID number", "••••••0005")] },
                            facts::Group { title: "Loyalty".into(), facts: vec![facts::Fact::mono("Loyalty points", "0")] },
                            facts::Group { title: "Preferences".into(), facts: vec![facts::Fact::new("Language", "English"), facts::Fact::new("Contact by", "Email")] },
                        ], ..Default::default() }))
                        (related_list::render(related_list::Props { title: "Bookings".into(), items: vec![related_list::Item {
                            reference: "#UI-B05".into(), title: if long { "#UI-B05 · Garden suite with terrace and late arrival".into() } else { "#UI-B05 #UI-B05 · Garden suite".into() },
                            href: Some("/blocks/record-timeline".into()), status: Some(badge::Props { label: "Confirmed".into(), variant: badge::Variant::Success, ..Default::default() }),
                            date: Some(related_list::Date { label: "8 Sep".into(), datetime: Some("2026-09-08".into()) }),
                        }], ..Default::default() }))
                        (related_list::render(related_list::Props { title: "Notes".into(), empty_message: "No notes yet.".into(), ..Default::default() }))
                    }, ..Default::default()
                }), ..Default::default()
            }))
        },
    )
}

fn document(density: &str, children: Markup) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="light" data-brand="lodge" data-density=(density) style="--mui-brand-radius:.75rem" {
            head { meta charset="utf-8"; meta name="viewport" content="width=device-width, initial-scale=1"; title { "Generic record · " (density) } link rel="stylesheet" href="/css/maud-ui.css"; }
            body { (children) script src="/js/maud-ui.js" defer {} }
        }
    }
}

pub fn cards() -> Markup {
    document(
        "comfortable",
        html! {
            main class="mui-block--shell__content mui-stack" {
                h1 class="mui-h2" { "Independent records" }
                (stack::vertical(html! {
                    @for title in ["Garden suite", "Courtyard room", "Terrace room"] {
                        (card::render(card::Props { title: Some(title.into()), children: html! { p { "An independent room with its own details." } }, ..Default::default() }))
                    }
                }))
            }
        },
    )
}

pub fn minimal() -> Markup {
    document(
        "comfortable",
        html! {
            main class="mui-block--shell__content mui-stack" {
                (page::render(page::Props {
                    header: header::Props { title: "English".into(), kind: Some("Language".into()), reference: Some("EN".into()), heading: Heading::H1, ..Default::default() },
                    children: html! {
                        (facts::render(facts::Props { groups: vec![facts::Group { title: "Language".into(), facts: vec![facts::Fact::new("Display name", "English")] }], ..Default::default() }))
                        (related_list::render(related_list::Props { title: "Guests".into(), empty_message: "No guests yet.".into(), ..Default::default() }))
                    }, ..Default::default()
                }))
            }
        },
    )
}

fn main() {
    let check = std::env::args().any(|arg| arg == "--check");
    let mut pages = vec![
        ("record-page-three-cards.html".to_string(), cards()),
        ("record-page-minimal.html".to_string(), minimal()),
    ];
    for density in ["compact", "comfortable", "spacious"] {
        for long in [false, true] {
            pages.push((
                format!(
                    "record-page-{density}-{}.html",
                    if long { "long" } else { "guest" }
                ),
                guest(density, long),
            ));
        }
    }
    for (name, page) in pages {
        let file = std::path::Path::new("docs/fixtures").join(name);
        let expected = page.into_string() + "\n";
        if check {
            assert_eq!(
                std::fs::read_to_string(&file).unwrap(),
                expected,
                "Stale fixture: {}",
                file.display()
            );
        } else {
            std::fs::write(file, expected).unwrap();
        }
    }
    println!("{} 8 generic-record pages: three densities, long labels, three sibling cards and a single-fact record.", if check { "Verified" } else { "Generated" });
}
