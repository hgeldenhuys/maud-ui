//! Deterministic pages for comparing shell geometry, with and without JavaScript.
use maud::{html, Markup, DOCTYPE};
use maud_ui::{
    blocks::{
        attention_banner,
        shell::{app_footer, app_header, brand_mark, page_header, sidebar},
    },
    primitives::{breadcrumb::BreadcrumbItem, table},
};

pub fn page(density: &str, long: bool, scoped: bool) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="light" data-brand="lodge" data-density=[(!scoped).then_some(density)] style="--mui-brand-radius:.75rem" data-fixture-length=(if long { "long" } else { "short" }) {
            head {
                meta charset="utf-8"; meta name="viewport" content="width=device-width, initial-scale=1";
                title { "Shell frame · " (density) }
                link rel="stylesheet" href="/css/maud-ui.css";
            }
            body {
                (sidebar::render(sidebar::Props {
                    id: "frame-fixture".into(),
                    brand_mark: Some(brand_mark::Props { wordmark: "Front desk".into(), ..Default::default() }),
                    app_header: app_header::render(app_header::Props { brand_mark: brand_mark::preset("lodge"), actions: Some(html! { button type="button" class="mui-btn mui-btn--outline" { "Account" } }), ..Default::default() }),
                    app_footer: app_footer::render(app_footer::Props { line: Some("Garden House · Independent hospitality".into()), ..Default::default() }),
                    nav_groups: vec![
                        sidebar::NavGroup { label: Some("Reservations".into()), items: vec![
                            sidebar::NavItem { label: "Overview".into(), href: "#overview".into(), ..Default::default() },
                            sidebar::NavItem { label: "Stays".into(), href: "#stays".into(), children: vec![sidebar::NavItem { label: "Booking".into(), href: "#booking".into(), ..Default::default() }], ..Default::default() },
                        ] },
                        sidebar::NavGroup { label: Some("Accommodation".into()), items: (1..=30).map(|n| sidebar::NavItem { label: format!("Room {n}"), href: format!("#room-{n}"), ..Default::default() }).collect() },
                    ],
                    active_path: "#booking".into(),
                    page_header: Some(page_header::Props {
                        breadcrumbs: vec![BreadcrumbItem { label: "Garden House".into(), href: Some("#home".into()) }, BreadcrumbItem { label: "Bookings".into(), href: None }],
                        search: Some(page_header::Search { action: "/search".into(), ..Default::default() }),
                        switchers: html! { label { span class="mui-sr-only" { "Role" } select class="mui-native-select" { option { "Receptionist" } } } },
                        ..Default::default()
                    }),
                    children: html! {
                        div class="mui-page-stack" data-density=[scoped.then_some(density)] data-fixture-content {
                            (attention_banner::render(attention_banner::Props { title: "Maya arrives at 11:30".into(), subline: Some("Room 12 · Two nights".into()), ..Default::default() }))
                            section class="mui-card" id="overview" aria-label="Today" {
                                div class="mui-card__body" { h1 class="mui-h3" { "Today" } p class="mui-caption" { "2 arriving · 1 departing · of 10 visible" } }
                            }
                            nav aria-label="Fixture density" {
                                @for mode in ["compact", "comfortable", "spacious"] {
                                    a class="mui-btn mui-btn--outline" href=(format!("/fixtures/shell-frame-{mode}-{}.html", if long { "long" } else { "short" })) { (mode) }
                                }
                            }
                            label { "Find a guest" input class="mui-input" type="search" name="guest"; }
                            (table::render(table::Props {
                                caption: Some("Bookings · Frame verification fixture".into()), headers: vec!["Guest".into(), "Room".into()],
                                rows: (1..=if long { 70 } else { 2 }).map(|n| vec![format!("Guest {n}"), format!("Room {n}")]).collect(),
                                ..Default::default()
                            }))
                        }
                    },
                    ..Default::default()
                }))
                script src="/js/maud-ui.js" defer {}
            }
        }
    }
}

fn main() {
    let check = std::env::args().any(|arg| arg == "--check");
    let directory = std::path::Path::new("docs/fixtures");
    if !check {
        std::fs::create_dir_all(directory).unwrap();
    }
    for density in ["compact", "comfortable", "spacious"] {
        for (long, scoped, suffix) in [
            (false, false, "short"),
            (true, false, "long"),
            (false, true, "scoped"),
        ] {
            let file = directory.join(format!("shell-frame-{density}-{suffix}.html"));
            let expected = page(density, long, scoped).into_string() + "\n";
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
    }
    println!(
        "{} 9 shell frame pages: three densities, short/long content, container density.",
        if check { "Verified" } else { "Generated" }
    );
}
