use maud::{html, PreEscaped, DOCTYPE};
use maud_ui::{assets, blocks::shell::{page_header, sidebar}, primitives::breadcrumb::BreadcrumbItem};

fn main() {
    let tabs = std::env::args().any(|arg| arg == "--tabs");
    let groups = [("Operations", "Bookings", sidebar::ItemKind::Workflow),
        ("Reference", "Rooms", sidebar::ItemKind::Reference),
        ("Reports", "Occupancy", sidebar::ItemKind::Report)]
        .into_iter().map(|(group,label,kind)| sidebar::NavGroup {
            label: Some(group.into()), items: vec![sidebar::NavItem {
                label: label.into(), href: format!("#{label}"), icon: Some(sidebar::item_icon(kind)),
                ..Default::default()
            }],
        }).collect();
    let shell = sidebar::render(sidebar::Props {
        id: "nav-fixture".into(), nav_groups: groups, active_path: "#Bookings".into(),
        mobile_navigation: if tabs { sidebar::MobileNavigation::Tabs } else { sidebar::MobileNavigation::Drawer },
        brand: html! { span.mui-block--shell__brand-name { "Lodge Operations" } },
        page_header: Some(page_header::Props {
            title: Some("Bookings".into()),
            breadcrumbs: vec![BreadcrumbItem { label: "Lodge Operations".into(), href: Some("#home".into()) }, BreadcrumbItem { label: "Bookings".into(), href: None }],
            search: Some(page_header::Search { action: "/search".into(), ..Default::default() }),
            switchers: html! { label { span.mui-sr-only { "Role" } select { option { "Manager" } } } button.mui-btn type="button" { "Theme" } },
            ..Default::default()
        }),
        children: html! { h1 { "Bookings" } p { "A complete native navigation fixture." } },
        ..Default::default()
    });
    println!("{}", html! { (DOCTYPE) html lang="en" { head {
        meta charset="utf-8"; meta name="viewport" content="width=device-width, initial-scale=1";
        title { "Navigation fixture" } style { (PreEscaped(assets::CSS_MIN)) }
    } body { (shell) script { (PreEscaped(assets::JS)) } } } }.into_string());
}
