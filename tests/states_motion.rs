//! Props-driven state contracts and the final gallery structure.
use maud::html;
use maud_ui::{
    blocks::{
        self,
        action::Action,
        record,
        state::{self, State},
    },
    primitives::{empty_state, facts_list::Fact},
    showcase,
};

macro_rules! operational_states {
    ($($name:ident: $module:path),* $(,)?) => { $(
        #[test]
        fn $name() {
            use $module as block;
            for state in [State::loading(), State::empty("No related items"), State::error("Request failed"), State::disabled("Editing unavailable")] {
                let expected = state.as_str();
                let output = block::render(block::Props { state: state.clone(), ..Default::default() }).into_string();
                assert!(output.contains(&format!("data-state=\"{expected}\"")));
                match state {
                    State::Loading { .. } => { assert!(output.contains("mui-skeleton")); assert!(output.contains("aria-busy=\"true\"")); },
                    State::Empty { .. } => { assert!(output.contains("No related items")); assert!(!output.contains("role=\"alert\"")); },
                    State::Error { .. } => { assert!(output.contains("Request failed")); assert!(output.contains("role=\"alert\"")); },
                    State::Disabled { .. } => { assert!(output.contains("inert=\"\" aria-disabled=\"true\"")); assert!(output.contains("Editing unavailable")); },
                    State::Ready => unreachable!(),
                }
            }
        }
    )* };
}
operational_states!(
    worklist_header: blocks::worklist::header, grouped: blocks::worklist::grouped,
    record_header: blocks::record::header, timeline: blocks::record::timeline,
    money: blocks::record::money, related: blocks::record::related_card,
    tasks: blocks::task::grid, banner: blocks::attention_banner, actions: blocks::action_row,
    sidebar: blocks::shell::sidebar, page_header: blocks::shell::page_header,
    app_header: blocks::shell::app_header, app_footer: blocks::shell::app_footer,
);
#[test]
fn nonready_state_never_renders_supplied_data_or_actions() {
    for state in [
        State::loading(),
        State::empty("No data"),
        State::error("Unavailable"),
    ] {
        let output = record::related_card::render(record::related_card::Props {
            state,
            title: "MUST NOT APPEAR".into(),
            action_markup: Some(html! { button data-action="hidden-write" { "Unsafe fallback" } }),
            ..Default::default()
        })
        .into_string();
        assert!(!output.contains("MUST NOT APPEAR"));
        assert!(!output.contains("hidden-write"));
    }
}
#[test]
fn disabled_retains_canonical_fragments_in_an_inert_subtree() {
    let output = record::related_card::render(record::related_card::Props {
        state: State::disabled("View only"), title_markup: Some(html! { h3 data-field="identity" { "Guest" } }),
        action_markup: Some(html! { form action="/write" method="post" { input type="hidden" name="csrf" value="token"; button type="submit" data-action="save" { "Save" } } }), ..Default::default()
    }).into_string();
    assert!(output.contains("inert=\"\" aria-disabled=\"true\""));
    assert!(output.contains("data-field=\"identity\""));
    assert!(output.contains("data-action=\"save\""));
    assert!(output.contains("name=\"csrf\" value=\"token\""));
}
#[test]
fn empty_and_error_actions_remain_native_and_distinct() {
    let render = |state| {
        blocks::action_row::render(blocks::action_row::Props {
            state,
            ..Default::default()
        })
        .into_string()
    };
    let empty = render(State::Empty {
        message: "No payments".into(),
        action: Some(Action::link("Add payment", "/payments/new")),
    });
    let error = render(State::Error {
        message: "Payments unavailable".into(),
        retry: Some(Action::link("Try again", "/payments")),
    });
    assert!(empty.contains("href=\"/payments/new\""));
    assert!(!empty.contains("role=\"alert\""));
    assert!(error.contains("href=\"/payments\""));
    assert!(error.contains("role=\"alert\""));
}
#[test]
fn related_auto_selects_inline_only_when_effective_facts_are_empty() {
    let output = record::related_card::render(record::related_card::Props {
        title: "Guest".into(),
        action: Some(Action::link("View", "/guest")),
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("mui-related-card--inline"));
    assert_eq!(output.matches("<a ").count(), 1);
    let with_fact = record::related_card::Props {
        facts: vec![Fact::new("Room", "4")],
        ..Default::default()
    };
    assert!(!record::related_card::render(with_fact.clone())
        .into_string()
        .contains("mui-related-card--inline"));
    assert!(record::related_card::render(record::related_card::Props {
        facts_markup: Some(html! {}),
        ..with_fact
    })
    .into_string()
    .contains("mui-related-card--inline"));
    assert!(!record::related_card::render(record::related_card::Props {
        variant: record::related_card::Variant::Card,
        ..Default::default()
    })
    .into_string()
    .contains("mui-related-card--inline"));
}
#[test]
fn inline_empty_state_omits_hero_glyph_heading_and_description() {
    let output = empty_state::render(
        empty_state::Props::new("No payments yet")
            .with_variant(empty_state::Variant::Inline)
            .with_icon("◇")
            .with_description("Must stay compact")
            .with_action(html! { a href="/new" { "Add payment" } }),
    )
    .into_string();
    assert!(output.contains("mui-empty-state--inline"));
    assert!(output.contains("No payments yet"));
    assert!(output.contains("Add payment"));
    for omitted in ["◇", "Must stay compact", "<h3", "mui-empty-state__icon"] {
        assert!(!output.contains(omitted));
    }
}
#[test]
fn breadcrumbs_keep_full_root_and_current_and_identify_only_middle_for_ellipsis() {
    use maud_ui::primitives::breadcrumb::{self, BreadcrumbItem};
    let output = breadcrumb::render(breadcrumb::Props {
        items: vec![
            BreadcrumbItem {
                label: "Garden House".into(),
                href: Some("/".into()),
            },
            BreadcrumbItem {
                label: "Reservations for all locations".into(),
                href: Some("/reservations".into()),
            },
            BreadcrumbItem {
                label: "Amira Khan".into(),
                href: None,
            },
        ],
        ..Default::default()
    })
    .into_string();
    for position in ["root", "middle", "current"] {
        assert_eq!(
            output
                .matches(&format!("data-position=\"{position}\""))
                .count(),
            1
        );
    }
    assert!(output.contains("title=\"Reservations for all locations\""));
    assert!(output.contains("Garden House"));
    assert!(output.contains("Amira Khan"));
    assert!(output.contains("aria-current=\"page\""));
}
#[test]
fn all_operational_examples_show_each_state_once() {
    assert_eq!(state::OPERATIONAL_BLOCKS.len(), 13);
    for slug in state::OPERATIONAL_BLOCKS {
        let output = state::preview(slug).unwrap().into_string();
        for state in ["loading", "empty", "error", "disabled"] {
            assert_eq!(
                output
                    .matches(&format!("class=\"mui-block-state\" data-state=\"{state}\""))
                    .count(),
                1,
                "{slug}: {state}"
            );
        }
    }
}
#[test]
fn every_api_page_has_four_sections_and_generated_props() {
    for (block, slugs) in [
        (false, showcase::COMPONENT_NAMES),
        (true, blocks::BLOCK_NAMES),
    ] {
        for slug in slugs {
            let output = if block {
                showcase::block_page_by_name(slug)
            } else {
                showcase::component_page_by_name(slug)
            }
            .into_string();
            let mut positions = vec![];
            for section in ["anatomy", "props", "examples", "accessibility"] {
                positions.push(
                    output
                        .find(&format!("id=\"api-{slug}-{section}\""))
                        .unwrap_or_else(|| panic!("{slug}: missing {section}")),
                );
            }
            assert!(positions.windows(2).all(|p| p[0] < p[1]));
            assert!(output.contains("data-generated=\"rust-props\""), "{slug}");
            assert!(
                output.contains("<h1 class=\"mui-gallery__component-name\""),
                "{slug}"
            );
        }
    }
}
#[test]
fn generated_defaults_follow_actual_rust_and_do_not_invent_a_default_trait() {
    let related = showcase::component_page_by_name("date_range").into_string();
    assert!(related.contains("Choose both dates"));
    assert!(related.contains("data-label=\"Default\""));
    let action = showcase::block_page_by_name("action-row").into_string();
    assert!(action.contains("&quot;More&quot;"));
    assert!(action.contains("State"));
    let menu = showcase::component_page_by_name("menu").into_string();
    assert!(menu.contains("No struct Default; supply explicitly"));
}
#[test]
fn landing_record_tab_preserves_no_script_navigation_and_uses_the_record_kit() {
    let output = showcase::landing_page().into_string();
    for id in [
        "lp-worklist-tab",
        "lp-record-tab",
        "lp-worklist-panel",
        "lp-record-panel",
    ] {
        assert_eq!(output.matches(&format!("id=\"{id}\"")).count(), 1);
    }
    assert!(output.contains("href=\"#lp-record-panel\""));
    for class in [
        "mui-record-timeline",
        "mui-record-money",
        "mui-related-card--inline",
        "mui-empty-state--inline",
        "mui-attention-banner",
    ] {
        assert!(output.contains(class));
    }
    assert!(output.contains("Manager example"));
    assert!(!output.contains("data-workspace-panel hidden"));
}
