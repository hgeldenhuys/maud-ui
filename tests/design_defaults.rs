use maud::html;
use maud_ui::blocks::{
    record::header as record,
    shell::{app_footer, app_header, page_header, sidebar},
};
use maud_ui::primitives::{badge, breadcrumb::BreadcrumbItem};

#[test]
fn record_markup_slots_override_plain_fallbacks_without_nested_headings() {
    let output = record::render(record::Props {
        title: "Plain fallback".into(),
        title_markup: Some(html! { h1 data-field="guest" { "Bound title" } }),
        status: Some(badge::Props {
            label: "Plain status".into(),
            ..Default::default()
        }),
        status_markup: Some(html! { output { "Bound status" } }),
        ..Default::default()
    })
    .into_string();
    assert!(output.contains(
        "<div class=\"mui-record-header__title\"><h1 data-field=\"guest\">Bound title</h1></div>"
    ));
    assert!(!output.contains("Plain fallback") && !output.contains("Plain status"));
    assert!(output.contains("<output>Bound status</output>"));
    let empty = record::render(record::Props {
        title: "Hidden".into(),
        title_markup: Some(html! {}),
        ..Default::default()
    })
    .into_string();
    assert!(!empty.contains("Hidden") && !empty.contains("<h2"));
    let plain = record::render(record::Props {
        title: "<script>plain</script>".into(),
        ..Default::default()
    })
    .into_string();
    assert!(plain.contains("&lt;script&gt;plain&lt;/script&gt;"));
}

#[test]
fn optional_chrome_emits_nothing_until_a_slot_has_content() {
    assert!(app_header::render(Default::default()).0.is_empty());
    assert!(app_footer::render(Default::default()).0.is_empty());
    assert!(app_header::render(app_header::Props {
        brand: Some(html! {}),
        actions: Some(html! {}),
        ..Default::default()
    })
    .0
    .is_empty());
    assert!(app_footer::render(app_footer::Props {
        line: Some(String::new()),
        ..Default::default()
    })
    .0
    .is_empty());
    assert!(app_header::render(app_header::Props {
        children: html! { span { "Custom" } },
        ..Default::default()
    })
    .0
    .contains("Custom"));
    assert!(app_footer::render(app_footer::Props {
        children: html! { span { "Custom" } },
        ..Default::default()
    })
    .0
    .contains("Custom"));
}

#[test]
fn shell_composes_chrome_and_keeps_nested_current_destination_in_more() {
    let output = sidebar::render(sidebar::Props {
        id: "nested".into(),
        embedded: true,
        app_header: app_header::render(app_header::Props {
            brand: Some(html! { "Masthead" }),
            ..Default::default()
        }),
        app_footer: app_footer::render(app_footer::Props {
            line: Some("Footer".into()),
            ..Default::default()
        }),
        nav_groups: vec![sidebar::NavGroup {
            label: Some("Work".into()),
            items: (0..4)
                .map(|i| sidebar::NavItem {
                    label: format!("Area {i}"),
                    href: format!("/{i}"),
                    children: if i == 3 {
                        vec![sidebar::NavItem {
                            label: "Nested area".into(),
                            href: "/nested".into(),
                            ..Default::default()
                        }]
                    } else {
                        vec![]
                    },
                    ..Default::default()
                })
                .collect(),
        }],
        active_path: "/nested".into(),
        mobile_navigation: sidebar::MobileNavigation::Tabs,
        children: html! { h2 { "Content" } },
        ..Default::default()
    })
    .0;
    assert!(output.find("Masthead").unwrap() < output.find("<aside").unwrap());
    assert!(output.find("Content").unwrap() < output.find("Footer").unwrap());
    assert!(!output.contains("<main") && output.contains("<section"));
    assert_eq!(output.matches("aria-current=\"page\"").count(), 2);
    assert!(output.contains("title=\"Nested area\" aria-label=\"Nested area\""));
    assert!(output.contains("data-nav-key=\"nested-navigation:0:Work\""));
    assert!(output.contains("mui-bottom-tab-bar--inline"));
    let ordinary = sidebar::render(Default::default()).0;
    assert_eq!(ordinary.matches("<main").count(), 1);
}

#[test]
fn page_header_search_has_a_native_fallback_and_a_markup_override() {
    let props = page_header::Props {
        breadcrumbs: vec![BreadcrumbItem {
            label: "Guests".into(),
            href: None,
        }],
        search: Some(page_header::Search {
            action: "/search".into(),
            name: "guest".into(),
            value: "A&B".into(),
            ..Default::default()
        }),
        ..Default::default()
    };
    let output = page_header::render(props.clone()).0;
    assert!(
        output.contains("method=\"get\" action=\"/search\"")
            && output.contains("name=\"guest\" value=\"A&amp;B\"")
    );
    assert!(output.contains("aria-current=\"page\"") && output.contains("type=\"submit\""));
    let custom = page_header::render(page_header::Props {
        search_markup: Some(html! { button { "Command search" } }),
        ..props
    })
    .0;
    assert!(custom.contains("Command search") && !custom.contains("method=\"get\""));
}

#[test]
fn landing_is_one_live_shell_with_registry_driven_component_links() {
    let page = maud_ui::showcase::landing_page().0;
    assert!(page.contains("data-theme=\"light\""));
    assert_eq!(page.matches("data-mui=\"workspace-demo\"").count(), 1);
    assert_eq!(page.matches("<main").count(), 1);
    for class in [
        "mui-page-header",
        "mui-worklist-header",
        "mui-status-chip-group",
        "mui-table",
        "mui-record-header",
        "mui-task-grid",
    ] {
        assert!(page.contains(class), "{class}");
    }
    assert!(page.contains("data-demo-create") && page.contains("data-demo-view"));
    let hero = page
        .split("class=\"lp__hero\"")
        .nth(1)
        .unwrap()
        .split("class=\"lp__claims\"")
        .next()
        .unwrap();
    assert!(!hero.contains("<img") && !hero.contains("linear-gradient"));
}

#[test]
fn data_table_alignment_is_explicit_and_shared_by_headers_plain_and_rich_cells() {
    use maud_ui::primitives::data_table::{self, Align, CellMarkup, Column};
    let head = data_table::column_header_aligned("Amount", true, Align::Right).0;
    assert!(head.contains("scope=\"col\" data-align=\"right\" tabindex=\"0\""));
    assert!(data_table::column_header("Guest", false)
        .0
        .contains("data-align=\"left\""));
    for rich in [false, true] {
        let page = data_table::render(data_table::Props {
            columns: vec![Column {
                key: "amount".into(),
                label: "Amount".into(),
                align: Align::Right,
                ..Default::default()
            }],
            rows: vec![vec!["24.00".into()]],
            rich_rows: if rich {
                vec![vec![CellMarkup::markup(
                    html! { strong { "24.00" } },
                    false,
                )]]
            } else {
                vec![]
            },
            ..Default::default()
        })
        .0;
        assert_eq!(page.matches("data-align=\"right\"").count(), 2);
        if rich {
            assert!(page.contains("<strong>24.00</strong>"));
        }
    }
}
