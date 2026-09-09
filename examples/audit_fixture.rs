//! Standalone HTML used by the rendered geometry audit; no network assets.
use maud::{html, PreEscaped, DOCTYPE};
use maud_ui::{blocks::shell::page_header, primitives::{breadcrumb::BreadcrumbItem, progress}};
fn main() {
    println!("{}", html! { (DOCTYPE) html lang="en" { head {
        meta charset="utf-8"; meta name="viewport" content="width=device-width, initial-scale=1";
        title { "Audit fixture" }
        style { (PreEscaped(maud_ui::assets::CSS_MIN)) }
    } body {
        (page_header::render(page_header::Props {
            breadcrumbs: vec![BreadcrumbItem { label: "Workspace".into(), href: Some("#main".into()) }, BreadcrumbItem { label: "Reservations".into(), href: None }],
            search: Some(page_header::Search { action: "/search".into(), ..Default::default() }),
            ..Default::default()
        }))
        main #main { h1 { "Reservations" }
            (progress::render(progress::Props { indeterminate: true, label: "Loading reservations".into(), ..Default::default() }))
        }
        script { (PreEscaped(maud_ui::assets::JS)) }
    } } }.into_string());
}
