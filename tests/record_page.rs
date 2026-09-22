use maud::html;
use maud_ui::{
    blocks::{
        action::Action,
        record::{facts, header, page, related_list},
        shell::page_header,
    },
    primitives::{
        breadcrumb::{self, BreadcrumbItem},
        stack,
    },
};

#[allow(dead_code)]
#[path = "../examples/record_page_fixture.rs"]
mod fixture;

#[test]
fn facts_share_one_surface_and_single_facts_are_rows() {
    let output = facts::render(facts::Props {
        groups: vec![
            facts::Group {
                title: "Identity".into(),
                facts: vec![
                    facts::Fact::new("Name", "Sofia"),
                    facts::Fact::new("Country", "South Africa"),
                ],
            },
            facts::Group {
                title: "Loyalty".into(),
                facts: vec![facts::Fact::mono("Points", "0")],
            },
            facts::Group {
                title: "Empty group".into(),
                ..Default::default()
            },
        ],
        ..Default::default()
    })
    .0;
    assert_eq!(output.matches("<section ").count(), 1);
    assert_eq!(output.matches("<dl ").count(), 2);
    assert_eq!(output.matches("<dt ").count(), 3);
    assert_eq!(output.matches("data-single=\"true\"").count(), 1);
    assert!(!output.contains("Empty group") && !output.contains("mui-record-facts--inline"));
    let single = facts::render(facts::Props {
        groups: vec![facts::Group {
            title: "Loyalty".into(),
            facts: vec![facts::Fact::mono("Points", "0")],
        }],
        ..Default::default()
    })
    .0;
    assert!(single.contains("mui-record-facts--inline") && single.contains(">0<"));
    assert!(facts::render(Default::default()).0.is_empty());
}

#[test]
fn facts_preserve_masking_hints_and_canonical_value_ownership() {
    let output = facts::render(facts::Props { groups: vec![facts::Group { title: "Contact <details>".into(), facts: vec![
        facts::Fact::masked("Phone", "••••0005"),
        facts::Fact { value_markup: Some(html! { a href="mailto:test@example.com" data-field="email" { "Bound email" } }), ..facts::Fact::new("Email", "MUST NOT APPEAR") },
        facts::Fact { value_markup: Some(html! {}), ..facts::Fact::new("Hidden field", "MUST NOT APPEAR") },
        facts::Fact::new("Escaped", "<script>alert(1)</script>"),
    ] }], masked_hint: "masqué".into(), ..Default::default() }).0;
    assert!(output.contains("Contact &lt;details&gt;"));
    assert!(output.contains("data-mono=\"true\">••••0005"));
    assert!(output.contains(">masqué</span>"));
    assert!(output.contains("data-field=\"email\""));
    assert!(output.contains("&lt;script&gt;alert(1)&lt;/script&gt;"));
    assert!(!output.contains("Hidden field") && !output.contains("MUST NOT APPEAR"));
}

#[test]
fn related_rows_deduplicate_only_exact_reference_prefixes() {
    for title in [
        "UI-B05",
        "#UI-B05 #UI-B05",
        "UI-B05 · Garden suite",
        "#UI-B05: Garden suite",
    ] {
        let output = related_list::render(related_list::Props {
            title: "Bookings".into(),
            items: vec![related_list::Item {
                reference: "UI-B05".into(),
                title: title.into(),
                href: Some("/booking".into()),
                ..Default::default()
            }],
            ..Default::default()
        })
        .0;
        assert_eq!(output.matches(">UI-B05</span>").count(), 1);
        assert!(!output.contains("#UI-B05"));
        assert_eq!(output.matches("<a ").count(), 1);
        assert!(
            !output.contains("COMPOSITION")
                && !output.contains("nested")
                && !output.contains("via guest")
        );
    }
    let distinct = related_list::render(related_list::Props {
        items: vec![related_list::Item {
            reference: "UI-B05".into(),
            title: "UI-B050".into(),
            ..Default::default()
        }],
        ..Default::default()
    })
    .0;
    assert!(distinct.contains(">UI-B050</span>"));
    assert!(!distinct.contains("<a "));
}

#[test]
fn related_empty_and_optional_values_do_not_create_empty_rows_or_dead_links() {
    let output = related_list::render(related_list::Props {
        title: "Notes".into(),
        empty_message: "No notes yet.".into(),
        items: vec![Default::default()],
        ..Default::default()
    })
    .0;
    assert_eq!(output.matches(">No notes yet.</p>").count(), 1);
    assert!(!output.contains("<ul") && !output.contains("mui-card"));
    let row = related_list::render(related_list::Props {
        items: vec![related_list::Item {
            title: "Guest <5>".into(),
            href: Some(" ".into()),
            date: Some(related_list::Date {
                label: " ".into(),
                ..Default::default()
            }),
            ..Default::default()
        }],
        ..Default::default()
    })
    .0;
    assert!(row.contains("Guest &lt;5&gt;"));
    assert!(!row.contains("<a ") && !row.contains("<time "));
}

#[test]
fn header_renders_one_identity_line_then_main_verb_then_more() {
    let output = header::render(header::Props {
        title: "Sofia Patel".into(),
        kind: Some("Guest".into()),
        reference: Some("UI-G5".into()),
        subtitle: Some("MUST NOT APPEAR".into()),
        primary_action: Some(Action::link("Edit guest", "/edit")),
        secondary_actions: vec![Action::link("Graph view", "/graph")],
        ..Default::default()
    })
    .0;
    assert_eq!(output.matches(">Guest · UI-G5</p>").count(), 1);
    assert!(!output.contains("MUST NOT APPEAR") && !output.contains("Back to list"));
    let subtitle = output.find(">Guest · UI-G5</p>").unwrap();
    let primary = output.find(">Edit guest</a>").unwrap();
    let more = output.find("<details ").unwrap();
    let graph = output.find(">Graph view</a>").unwrap();
    assert!(subtitle < primary && primary < more && more < graph);
    assert_eq!(output.matches("mui-btn--primary").count(), 1);
}

#[test]
fn empty_breadcrumbs_are_removed_before_separators_positions_and_header_title() {
    let crumb = |label: &str, href: Option<&str>| BreadcrumbItem {
        label: label.into(),
        href: href.map(str::to_string),
    };
    for blank in ["", " ", "\t\n", "\u{2003}"] {
        let output = breadcrumb::render(breadcrumb::Props {
            items: vec![
                crumb(blank, None),
                crumb("Lodge Operations", Some("/")),
                crumb(blank, Some("/empty")),
                crumb("Guest: Sofia Patel", None),
                crumb(blank, None),
            ],
            // 0.19.0: `None` renders no separator at all; the default trail
            // pins "/" explicitly.
            separator: Some("/"),
            ..Default::default()
        })
        .0;
        assert_eq!(output.matches("mui-breadcrumb__separator").count(), 1);
        assert_eq!(output.matches("data-position=\"current\"").count(), 1);
        assert!(!output.contains("/empty"));
        assert!(output.contains("data-position=\"root\""));
        debug_assert!(!output
            .contains("<span role=\"link\" aria-disabled=\"true\" aria-current=\"page\"></span>"));
    }
    assert!(breadcrumb::render(breadcrumb::Props {
        items: vec![crumb(" ", None)],
        ..Default::default()
    })
    .0
    .is_empty());
    let output = page_header::render(page_header::Props {
        title: Some(" ".into()),
        breadcrumbs: vec![
            crumb(" ", None),
            crumb("Sofia Patel", None),
            crumb("", None),
        ],
        search: Some(Default::default()),
        ..Default::default()
    })
    .0;
    assert!(
        output.contains("data-single-crumb=\"true\"")
            && output.contains("mui-page-header__title\">Sofia Patel")
    );
    assert!(output.contains("placeholder=\"Search\""));
    let field = output
        .split("class=\"mui-page-header__search-field\"")
        .nth(1)
        .unwrap()
        .split("</label>")
        .next()
        .unwrap();
    assert!(field.contains("<input ") && field.contains("<kbd "));
}

#[test]
fn generic_fixture_has_one_facts_surface_four_groups_and_library_stacks() {
    for density in ["compact", "comfortable", "spacious"] {
        let output = fixture::guest(density, false).0;
        assert_eq!(
            output
                .matches("class=\"mui-record-facts mui-stack\"")
                .count(),
            1
        );
        assert_eq!(
            output.matches("class=\"mui-record-facts__group\"").count(),
            4
        );
        assert_eq!(
            output.matches("class=\"mui-record-facts__masked\"").count(),
            2
        );
        assert_eq!(
            output.matches("class=\"mui-related-list__item\"").count(),
            1
        );
        assert_eq!(output.matches("<h1 ").count(), 1);
        assert!(output.contains("mui-record-page mui-stack"));
        assert!(!output.contains("#empty"));
    }
    let cards = fixture::cards().0;
    assert_eq!(cards.matches("class=\"mui-card\"").count(), 3);
    assert!(cards.contains("<div class=\"mui-stack\"><div class=\"mui-card\">"));
    assert_eq!(stack::Props::default().gap, stack::Space::Lg);
    assert_eq!(
        page::Props::default().header.heading,
        maud_ui::blocks::action::Heading::H1
    );
}

#[test]
fn new_block_docs_and_previews_are_registered_in_both_themes() {
    for (slug, markup) in [
        ("record-facts", facts::preview()),
        ("record-related-list", related_list::preview()),
        ("record-page", page::preview()),
    ] {
        assert!(maud_ui::blocks::BLOCK_NAMES.contains(&slug));
        assert!(
            markup.0.contains("data-theme=\"light\"") && markup.0.contains("data-theme=\"dark\"")
        );
        let docs = maud_ui::showcase::docs::render_block_docs(slug).unwrap().0;
        assert!(docs.contains("Props") && docs.to_lowercase().contains("accessibility"));
    }
}
