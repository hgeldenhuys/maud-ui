use maud::html;
use maud_ui::{
    blocks::{
        self,
        record::related_card,
        shell::{app_header, brand_mark, page_header, sidebar},
        state::State,
        task,
    },
    brand,
    primitives::{breadcrumb::BreadcrumbItem, facts_list::Fact},
    showcase,
};

#[test]
fn undeclared_inputs_have_no_wrapper_heading_or_error() {
    for slug in blocks::state::OPERATIONAL_BLOCKS {
        assert!(blocks::state::preview(slug).is_some());
    }
    assert!(blocks::attention_banner::render(Default::default())
        .0
        .is_empty());
    assert!(
        blocks::attention_banner::render(blocks::attention_banner::Props {
            state: State::Absent,
            title: "Should not be visible".into(),
            subline: Some("No rule configured".into()),
            ..Default::default()
        })
        .0
        .is_empty()
    );
    assert!(blocks::record::money::render(blocks::record::money::Props {
        state: State::Absent,
        ..Default::default()
    })
    .0
    .is_empty());
    assert!(task::grid::render(Default::default()).0.is_empty());
}
#[test]
fn single_task_retains_full_copy_and_count() {
    let description =
        "Check all supplied identity documents before completing the application review";
    let markup = task::grid::render(task::grid::Props {
        tasks: vec![task::grid::Task {
            title: "Review".into(),
            description: description.into(),
            action: blocks::action::Action::link("Open", "/queue"),
        }],
        ..Default::default()
    })
    .0;
    assert!(markup.contains("data-count=\"1\""));
    assert!(markup.contains(&format!("title=\"{description}\"")));
    assert!(!markup.contains('…'));
}
#[test]
fn compact_related_context_preserves_admitted_facts_and_action_contract() {
    let markup = related_card::render(related_card::Props {
        variant: related_card::Variant::Compact,
        title: "Guest".into(),
        facts: vec![
            Fact::new("Email", "guest@example.com"),
            Fact::new("Phone", "+1 514 555 0120"),
        ],
        action_markup: Some(html! { a href="/guest" data-action="read-guest" { "View" } }),
        ..Default::default()
    })
    .0;
    for part in [
        "mui-related-card--compact",
        "guest@example.com",
        "+1 514 555 0120",
        "data-action=\"read-guest\"",
        "<dl",
    ] {
        assert!(markup.contains(part));
    }
}
#[test]
fn native_search_disclosure_keeps_get_values_without_script() {
    let markup = page_header::render(page_header::Props {
        breadcrumbs: vec![BreadcrumbItem {
            label: "Workspace".into(),
            href: None,
        }],
        search: Some(page_header::Search {
            action: "/search".into(),
            value: "Amira & Co".into(),
            ..Default::default()
        }),
        ..Default::default()
    })
    .0;
    assert!(markup.contains("<details class=\"mui-page-header__search-disclosure\""));
    assert!(markup.contains("<summary"));
    for part in [
        "method=\"get\" action=\"/search\"",
        "name=\"q\" value=\"Amira &amp; Co\"",
        "type=\"submit\"",
    ] {
        assert!(markup.contains(part));
    }
}
#[test]
fn brand_contract_is_minimal_and_logo_slots_survive_both_headers() {
    assert_eq!(brand::TOKENS.len(), 9);
    assert_eq!(brand::BRANDS.len(), 3);
    for b in brand::BRANDS {
        assert_eq!(b.values.len(), brand::TOKENS.len());
    }
    let props = brand_mark::Props {
        wordmark: "Northline".into(),
        href: Some("/accounts".into()),
        logo: Some(html! { img src="/logo.svg" alt=""; }),
        tagline: Some(html! { span data-field="tagline" { "Business banking" } }),
    };
    for markup in [
        app_header::render(app_header::Props {
            brand_mark: Some(props.clone()),
            ..Default::default()
        }),
        sidebar::render(sidebar::Props {
            brand_mark: Some(props),
            ..Default::default()
        }),
    ] {
        for part in [
            "mui-brand-mark",
            "Northline",
            "src=\"/logo.svg\"",
            "data-field=\"tagline\"",
            "href=\"/accounts\"",
        ] {
            assert!(markup.0.contains(part));
        }
    }
    assert!(brand_mark::render(Default::default()).0.is_empty());
}
#[test]
fn banking_is_a_third_native_workspace_and_compares_exact_amounts() {
    let page = showcase::landing_page().0;
    for id in ["lp-bank-tab", "lp-bank-panel"] {
        assert_eq!(page.matches(&format!("id=\"{id}\"")).count(), 1);
    }
    for part in [
        "data-default-density=\"compact\"",
        "Northline",
        "$204,730.45",
        "$199,930.45",
        "$4,800.00",
        "−$9,950.01",
        "−$9,950.10",
        "KYC queue",
        "mui-related-card--compact",
        "data-bank-review",
    ] {
        assert!(page.contains(part), "{part}");
    }
    assert!(!page.contains("Exceptions unavailable"));
}
#[test]
fn theme_brand_editor_and_brand_mark_api_are_registered() {
    let page = showcase::theme_customizer_page().0;
    assert_eq!(page.matches("data-brand-token=\"").count(), 9);
    for name in ["Lodge", "Bank", "Clinic", "Download brand.css"] {
        assert!(page.contains(name));
    }
    let api = showcase::block_page_by_name("shell-brand-mark").0;
    assert!(api.contains("data-generated=\"rust-props\""));
    assert!(api.contains("api-shell-brand-mark-accessibility"));
}
