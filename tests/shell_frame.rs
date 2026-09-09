use maud::html;
use maud_ui::{blocks::shell::sidebar, primitives, showcase};

#[allow(dead_code)]
#[path = "../examples/frame_fixture.rs"]
mod fixture;

#[test]
fn fixtures_keep_one_navigation_node_in_a_full_height_column() {
    for density in ["compact", "comfortable", "spacious"] {
        for long in [false, true] {
            let page = fixture::page(density, long, false).into_string();
            for class in [
                "mui-block--shell__body",
                "mui-block--shell__sidebar-column",
                "mui-block--shell__sidebar",
                "mui-block--shell__main",
                "mui-block--shell__content",
            ] {
                assert_eq!(
                    page.matches(&format!("class=\"{class}\"")).count(),
                    1,
                    "{class}"
                );
            }
            assert_eq!(page.matches("id=\"frame-fixture-navigation\"").count(), 1);
            assert_eq!(page.matches("data-contains-current=\"true\"").count(), 1);
            assert!(page.contains(&format!("data-density=\"{density}\"")));
            assert_eq!(
                page.matches("class=\"mui-table__row\"").count(),
                if long { 70 } else { 2 }
            );
        }
    }
}

#[test]
fn nested_current_item_marks_only_its_group_and_first_exact_destination() {
    let group = || sidebar::NavGroup {
        label: Some("Reservations".into()),
        items: vec![sidebar::NavItem {
            label: "Stays".into(),
            href: "/stays".into(),
            children: vec![sidebar::NavItem {
                label: "Booking".into(),
                href: "/booking".into(),
                ..Default::default()
            }],
            ..Default::default()
        }],
    };
    let output = sidebar::render(sidebar::Props {
        active_path: "/booking".into(),
        nav_groups: vec![group(), group()],
        ..Default::default()
    })
    .0;
    assert_eq!(output.matches("aria-current=\"page\"").count(), 1);
    assert_eq!(output.matches("data-contains-current=\"true\"").count(), 1);
    assert!(output.contains("--mui-nav-depth:1"));
    let absent = sidebar::render(sidebar::Props {
        active_path: "/missing".into(),
        nav_groups: vec![group()],
        ..Default::default()
    })
    .0;
    assert!(!absent.contains("data-contains-current"));
}

#[test]
fn primitive_groups_and_brand_export_explain_the_same_contract() {
    let current = primitives::sidebar::menu_link("/booking", "Booking", None, true);
    let group =
        primitives::sidebar::collapsible_group("reservations", "Reservations", current, true).0;
    assert!(group.contains("data-contains-current=\"true\""));
    let empty = primitives::sidebar::collapsible_group("none", "None", html! {}, true).0;
    assert!(!empty.contains("data-contains-current"));
    let theme = showcase::theme_customizer_page().0;
    assert_eq!(theme.matches("data-brand-radius-sample=").count(), 3);
    assert_eq!(theme.matches("data-brand-token=").count(), 9);
    assert!(theme.contains("lg = min(brand * 1.5, 12px)"));
}
