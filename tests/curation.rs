use maud::html;
use maud_ui::{
    assets,
    blocks::{
        action::{Action, Heading, Link, Target},
        record,
        shell::sidebar,
        task, worklist,
    },
    primitives::{bottom_tab_bar as bar, button, empty_state, status_chip_group as chips},
};

#[test]
fn counted_filters_keep_native_navigation_one_current_and_escaped_labels() {
    let items = [
        chips::Tone::Neutral,
        chips::Tone::Info,
        chips::Tone::Success,
        chips::Tone::Warning,
        chips::Tone::Danger,
    ]
    .into_iter()
    .map(|tone| chips::Chip {
        label: "<script>guest</script>".into(),
        href: "/records?q=one&status=two".into(),
        count: 0,
        tone,
    })
    .collect();
    let output = chips::render(chips::Props {
        items,
        current: usize::MAX,
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("aria-current=\"page\"").count(), 1);
    assert_eq!(output.matches("<a ").count(), 5);
    assert!(output.contains("href=\"/records?q=one&amp;status=two\""));
    assert!(output.contains("&lt;script&gt;guest&lt;/script&gt;"));
    for tone in ["neutral", "info", "success", "warning", "danger"] {
        assert!(output.contains(&format!("data-tone=\"{tone}\"")));
    }
    assert!(!output.contains("tabindex=\"-1\""));
    assert!(!chips::render(Default::default())
        .into_string()
        .contains("aria-current"));
}

#[test]
fn mobile_navigation_hands_more_to_a_dialog_and_retains_its_fallback() {
    let output = bar::render(bar::Props {
        items: vec![bar::Item {
            label: "Home".into(),
            href: "/".into(),
            icon: None,
        }],
        current_href: Some("/".into()),
        more: Some(bar::More {
            label: "More".into(),
            target_id: "nav-drawer".into(),
            fallback_href: "/navigation".into(),
            current: true,
        }),
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("aria-current=\"page\"").count(), 1);
    assert!(output
        .contains("href=\"/navigation\" aria-controls=\"nav-drawer\" aria-haspopup=\"dialog\""));
    assert!(!output.contains("role=\"tab"));
    assert!(assets::CSS.contains("env(safe-area-inset-bottom"));
}

#[test]
#[should_panic(expected = "at most five")]
fn mobile_navigation_never_silently_discards_a_sixth_destination() {
    bar::render(bar::Props {
        items: (0..6)
            .map(|i| bar::Item {
                label: i.to_string(),
                href: format!("/{i}"),
                icon: None,
            })
            .collect(),
        ..Default::default()
    });
}

#[test]
fn worklist_search_is_a_labelled_get_form_and_preserves_filters() {
    let output = worklist::header::render(worklist::header::Props {
        title: "Reservations".into(),
        count_sentence: "12 reservations · 2 arriving".into(),
        heading: Heading::H1,
        search: Some(worklist::header::Search {
            action: "/reservations".into(),
            value: "\"><script>".into(),
            hidden_fields: vec![("status".into(), "arriving".into())],
            ..Default::default()
        }),
        primary_action: Some(Action::link("New reservation", "/reservations/new")),
    })
    .into_string();
    assert!(output.contains("<h1 "));
    assert!(output.contains("method=\"get\" action=\"/reservations\""));
    assert!(output.contains("name=\"status\" value=\"arriving\""));
    assert!(output.contains("name=\"q\" value=\"&quot;&gt;&lt;script&gt;\""));
    assert!(output.contains("<label ") && output.contains("Search records</span>"));
    assert_eq!(output.matches("mui-btn--primary").count(), 1);
    assert!(output.contains("12 reservations · 2 arriving"));
}

#[test]
fn record_keeps_one_primary_post_and_native_secondary_disclosure() {
    let output = record::header::render(record::header::Props {
        title: "Sofia".into(),
        subtitle: Some("RS-2048".into()),
        primary_action: Some(Action {
            label: "Check in".into(),
            target: Target::Post {
                action: "/records/2048/check-in".into(),
                hidden_fields: vec![("csrf".into(), "\"token<&".into())],
            },
        }),
        secondary_actions: vec![
            Action::link("Edit", "/edit"),
            Action::link("Invoice", "/invoice"),
        ],
        back: Some(Link {
            label: "Reservations".into(),
            href: "/records?status=arriving".into(),
        }),
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("mui-btn--primary").count(), 1);
    assert!(output.contains("method=\"post\" action=\"/records/2048/check-in\""));
    assert!(output.contains("value=\"&quot;token&lt;&amp;\""));
    assert!(output.contains("<details ") && output.contains("<summary>More actions</summary>"));
    assert!(output.find("Reservations</a>").unwrap() < output.find("Sofia</h2>").unwrap());
    let minimal = record::header::render(Default::default()).into_string();
    assert!(!minimal.contains("<details") && !minimal.contains("mui-record-header__actions"));
}

#[test]
fn task_launcher_has_one_named_action_per_task_and_complete_description() {
    let output = task::grid::render(task::grid::Props {
        tasks: vec![task::grid::Task {
            title: "Check in".into(),
            description: "Confirm all guest details before arrival.".into(),
            action: Action::link("Start check-in", "/start"),
        }],
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("<a ").count(), 1);
    assert!(output.contains("<ul ") && output.contains("<li ") && output.contains("<h3 "));
    assert!(output.contains("Confirm all guest details before arrival.</p>"));
}

#[test]
fn shell_group_header_ids_and_more_current_survive_without_javascript() {
    let output = sidebar::render(sidebar::Props {
        id: "desk".into(),
        header: Some(html! { label { "Workspace" select { option { "Front desk" } } } }),
        nav_groups: vec![sidebar::NavGroup {
            label: Some("Operations".into()),
            items: (0..6)
                .map(|i| sidebar::NavItem {
                    label: format!("Destination {i}"),
                    href: format!("/{i}"),
                    icon: None,
                    badge: None,
                })
                .collect(),
        }],
        active_path: "/5".into(),
        mobile_navigation: sidebar::MobileNavigation::Tabs,
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("id=\"desk-navigation\"").count(), 1);
    assert_eq!(output.matches("id=\"desk-drawer\"").count(), 1);
    assert!(output.contains("role=\"group\" aria-label=\"Operations\""));
    assert!(output.contains("Workspace<select>"));
    assert!(output.contains("href=\"#desk-navigation\""));
    assert_eq!(output.matches("aria-current=\"page\"").count(), 2); // one per navigation region
    assert!(output.contains("Destination 5"));
    assert!(!output.contains("data-mui-init") && !output.contains(" hidden"));
    let drawer = sidebar::render(Default::default()).into_string();
    assert!(!drawer.contains("class=\"mui-bottom-tab-bar"));
    assert!(drawer.contains("data-mobile-navigation=\"drawer\""));
}

#[test]
fn empty_filtered_and_failed_have_distinct_copy_and_presentation() {
    let output: Vec<_> = [
        empty_state::Variant::Empty,
        empty_state::Variant::Filtered,
        empty_state::Variant::Failed,
    ]
    .into_iter()
    .map(|variant| empty_state::render(empty_state::Props::for_variant(variant)).into_string())
    .collect();
    assert!(output[0].contains("Nothing here yet") && output[0].contains("data-state=\"empty\""));
    assert!(
        output[1].contains("No matching results") && output[1].contains("data-state=\"filtered\"")
    );
    assert!(
        output[2].contains("Could not load records") && output[2].contains("data-state=\"failed\"")
    );
    assert!(!output[2].contains("No matching") && !output[2].contains("Nothing here"));
}

#[test]
fn row_actions_are_native_disabled_controls_including_icon_label_path() {
    for aria_label in [None, Some("Check in Sofia".into())] {
        let output = button::render(button::Props {
            label: "Check in".into(),
            variant: button::Variant::Outline,
            size: button::Size::Row,
            disabled: true,
            button_type: "submit",
            aria_label,
            ..Default::default()
        })
        .into_string();
        assert!(output.contains("mui-btn--outline mui-btn--row"));
        assert!(output.contains(" disabled aria-disabled=\"true\""));
    }
}

#[test]
fn every_block_has_a_live_route_and_operational_blocks_have_api_docs() {
    for slug in maud_ui::blocks::BLOCK_NAMES {
        let page = maud_ui::showcase::block_page_by_name(slug).into_string();
        assert!(!page.contains("Block not found"), "{slug}");
    }
    for slug in [
        "worklist-header",
        "record-header",
        "task-grid",
        "shell-sidebar",
    ] {
        assert!(maud_ui::showcase::docs::render_block_docs(slug)
            .unwrap()
            .into_string()
            .contains("data-label=\"Field\""));
    }
}

#[test]
fn shipped_docs_match_markdown_sources() {
    use pulldown_cmark::{html::push_html, Options, Parser};
    for directory in ["docs/components", "docs/blocks"] {
        for entry in std::fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|e| e != "md")
                || path.file_stem().is_some_and(|n| n == "README")
            {
                continue;
            }
            let markdown = std::fs::read_to_string(&path).unwrap();
            let mut expected = String::new();
            push_html(
                &mut expected,
                Parser::new_ext(
                    &markdown,
                    Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
                ),
            );
            let rendered = path
                .parent()
                .unwrap()
                .join("rendered")
                .join(path.file_name().unwrap())
                .with_extension("html");
            assert!(
                std::fs::read_to_string(rendered).unwrap() == expected,
                "{} is stale; run cargo run --example build_docs",
                path.display()
            );
        }
    }
}

#[test]
fn complete_assets_include_the_curation_contract_without_import_waterfalls() {
    for css in [assets::CSS, assets::CSS_MIN] {
        for selector in [
            ".mui-status-chip-group",
            ".mui-bottom-tab-bar",
            ".mui-worklist-header",
            ".mui-record-header",
            ".mui-task-grid",
            ".mui-btn--row",
        ] {
            assert!(css.contains(selector), "missing {selector}");
        }
        assert!(!css.contains("@import "));
    }
    for js in [assets::JS, assets::JS_MIN] {
        assert!(
            js.contains("status-chip-group")
                && js.contains("shell-navigation")
                && js.contains("navigation-trigger")
        );
    }
}
