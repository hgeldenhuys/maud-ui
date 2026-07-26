//! Snapshot tests for component HTML output. Expanded in Wave 4.
//!
//! For now, assert that every component's `showcase()` function compiles and
//! returns non-empty markup. This catches API-level regressions during parallel
//! wave integration.

macro_rules! assert_showcase_renders {
    ($($module:ident),* $(,)?) => {
        $(
            #[test]
            #[allow(non_snake_case)]
            fn $module() {
                let markup = maud_ui::primitives::$module::showcase();
                let rendered = markup.into_string();
                assert!(!rendered.is_empty(), "showcase() returned empty markup");
            }
        )*
    };
}

assert_showcase_renders!(
    accordion,
    alert,
    alert_dialog,
    aspect_ratio,
    avatar,
    badge,
    breadcrumb,
    button,
    button_group,
    calendar,
    card,
    carousel,
    chart,
    checkbox,
    collapsible,
    combobox,
    command,
    context_menu,
    data_table,
    date_picker,
    dialog,
    drawer,
    empty_state,
    field,
    fieldset,
    hover_card,
    input,
    input_group,
    input_otp,
    kbd,
    label,
    menu,
    menubar,
    meter,
    native_select,
    navigation_menu,
    number_field,
    pagination,
    popover,
    progress,
    radio,
    radio_group,
    resizable,
    scroll_area,
    select,
    separator,
    skeleton,
    slider,
    spinner,
    switch,
    table,
    tabs,
    textarea,
    toast,
    toggle,
    toggle_group,
    tooltip,
    typography,
);

/// 0.3.0 growth items — asserted on the rendered HTML, because each one exists
/// to be addressable or announceable by something outside Rust (an SSE patcher,
/// a screen reader, a media query). "It compiles" says nothing about that.
mod growth_0_3_0 {
    use maud::html;
    use maud_ui::blocks::dashboard::stats;
    use maud_ui::primitives::{badge, collapsible, item, table, typography};

    #[test]
    fn badge_mono_adds_modifier() {
        let out = badge::render(badge::Props {
            label: "b6e38d1e".into(),
            mono: true,
            ..Default::default()
        })
        .into_string();
        assert!(out.contains("mui-badge--mono"), "got: {out}");
    }

    #[test]
    fn badge_hue_variants_render_distinct_classes() {
        for (variant, expected) in [
            (badge::Variant::Info, "mui-badge--info"),
            (badge::Variant::Accent, "mui-badge--accent"),
            (badge::Variant::Violet, "mui-badge--violet"),
            (badge::Variant::Rose, "mui-badge--rose"),
        ] {
            let out = badge::render(badge::Props {
                label: "x".into(),
                variant,
                ..Default::default()
            })
            .into_string();
            assert!(out.contains(expected), "expected {expected} in: {out}");
        }
    }

    #[test]
    fn stat_card_value_id_lands_on_the_value_node() {
        let out = stats::render(stats::Props {
            cards: vec![stats::StatCard {
                label: "Commits".into(),
                value: "79".into(),
                value_id: Some("stat-commits".into()),
                ..Default::default()
            }],
            ..Default::default()
        })
        .into_string();
        // The id must sit on the value <p>, not the card — a live patcher
        // replaces the number, not the tile.
        assert!(
            out.contains(r#"class="mui-block--stats__value" id="stat-commits""#),
            "got: {out}"
        );
    }

    #[test]
    fn stat_card_without_value_id_emits_no_id() {
        let out = stats::render(stats::Props {
            cards: vec![stats::StatCard {
                label: "Commits".into(),
                value: "79".into(),
                ..Default::default()
            }],
            ..Default::default()
        })
        .into_string();
        assert!(!out.contains("id="), "unexpected id in: {out}");
    }

    #[test]
    fn status_dot_pairs_colour_with_a_text_label() {
        let out = item::status_dot(item::Tone::Down).into_string();
        assert!(out.contains("mui-item__dot--down"), "got: {out}");
        assert!(out.contains(r#"aria-hidden="true""#), "got: {out}");
        // Colour must not be the only channel carrying the state.
        assert!(out.contains("mui-visually-hidden"), "got: {out}");
        assert!(out.contains("Down"), "got: {out}");
    }

    #[test]
    fn item_tone_tints_the_row() {
        let out = item::render(item::Props {
            tone: Some(item::Tone::Warn),
            ..Default::default()
        })
        .into_string();
        assert!(out.contains("mui-item--tone-warn"), "got: {out}");
    }

    #[test]
    fn item_without_tone_stays_untinted() {
        let out = item::render(item::Props::default()).into_string();
        assert!(!out.contains("mui-item--tone-"), "got: {out}");
    }

    #[test]
    fn table_hide_cols_sm_marks_header_and_body_cells() {
        let out = table::render(table::Props {
            headers: vec!["A".into(), "B".into()],
            rich_rows: vec![vec![
                table::CellMarkup::text("a1"),
                table::CellMarkup::text("b1"),
            ]],
            hide_cols_sm: vec![1],
            ..Default::default()
        })
        .into_string();
        assert_eq!(
            out.matches("data-hide-sm").count(),
            2,
            "expected the flag on exactly the th and td of column 1: {out}"
        );
        // Column 0 must be untouched.
        assert!(out.contains(r#"<th class="mui-table__th">A</th>"#), "got: {out}");
    }

    #[test]
    fn table_wrapper_is_keyboard_reachable() {
        let out = table::render(table::Props {
            headers: vec!["A".into()],
            rows: vec![vec!["a".into()]],
            ..Default::default()
        })
        .into_string();
        // A horizontally scrolling region that cannot take focus cannot be
        // scrolled without a pointer.
        assert!(out.contains(r#"tabindex="0""#), "got: {out}");
    }

    #[test]
    fn collapsible_native_renders_a_details_element() {
        let out = collapsible::render(collapsible::Props {
            trigger_label: "Details".into(),
            content: html! { p { "body" } },
            native: true,
            open: true,
            id: "d1".into(),
        })
        .into_string();
        assert!(out.contains("<details"), "got: {out}");
        assert!(out.contains("<summary"), "got: {out}");
        assert!(out.contains(" open"), "got: {out}");
        // No scripted trigger, and nothing relying on a `hidden` toggle.
        assert!(!out.contains("aria-expanded"), "got: {out}");
    }

    #[test]
    fn collapsible_default_stays_scripted() {
        let out = collapsible::render(collapsible::Props::default()).into_string();
        assert!(out.contains("aria-expanded"), "got: {out}");
        assert!(!out.contains("<details"), "got: {out}");
    }

    #[test]
    fn eyebrow_is_a_real_heading() {
        let out = typography::eyebrow("Service health").into_string();
        assert!(out.starts_with("<h2"), "eyebrow must keep the outline: {out}");
        assert!(out.contains("mui-eyebrow"), "got: {out}");
    }

    #[test]
    fn prose_wraps_server_rendered_html() {
        let out = typography::prose(maud::PreEscaped("<p>b</p>".to_string())).into_string();
        assert!(out.contains(r#"class="mui-prose""#), "got: {out}");
        assert!(out.contains("<p>b</p>"), "blob must not be escaped: {out}");
    }

    #[test]
    fn prose_demotes_blob_headings_below_page_level() {
        let out = typography::prose(maud::PreEscaped(
            "<h1>Top</h1><p>x</p><h2>Sub</h2><h3 id=\"a\">Deep</h3>".to_string(),
        ))
        .into_string();
        // A blob must not be able to inject a page-level heading.
        assert!(!out.contains("<h1"), "h1 leaked: {out}");
        assert!(!out.contains("<h2"), "h2 leaked: {out}");
        assert!(out.contains("<h4>Top</h4>"), "got: {out}");
        assert!(out.contains("<h5>Sub</h5>"), "got: {out}");
        // Attributes survive the rename.
        assert!(out.contains("<h6 id=\"a\">Deep</h6>"), "got: {out}");
    }

    #[test]
    fn prose_clamps_deep_headings_at_h6() {
        let out = typography::prose(maud::PreEscaped("<h5>five</h5><h6>six</h6>".to_string()))
            .into_string();
        assert!(out.contains("<h6>five</h6>"), "got: {out}");
        assert!(out.contains("<h6>six</h6>"), "got: {out}");
    }

    #[test]
    fn prose_leaves_non_heading_tags_alone() {
        // <hr> and <html> start with "h" + a character that must not be read
        // as a level; escaped markup in a code sample must not be rewritten.
        let blob = "<hr><p>a &lt;h1&gt; sample</p><pre><code>&lt;h2&gt;</code></pre>";
        let out = typography::prose(maud::PreEscaped(blob.to_string())).into_string();
        assert!(out.contains("<hr>"), "got: {out}");
        assert!(out.contains("&lt;h1&gt;"), "escaped text rewritten: {out}");
        assert!(out.contains("&lt;h2&gt;"), "escaped text rewritten: {out}");
    }

    #[test]
    fn prose_preserves_multibyte_content() {
        let out = typography::prose(maud::PreEscaped(
            "<h1>Iterationsübersicht — 概要</h1><p>café ☕</p>".to_string(),
        ))
        .into_string();
        assert!(out.contains("<h4>Iterationsübersicht — 概要</h4>"), "got: {out}");
        assert!(out.contains("café ☕"), "got: {out}");
    }

    #[test]
    fn prose_at_base_one_is_a_passthrough() {
        let out = typography::prose_at(maud::PreEscaped("<h1>T</h1>".to_string()), 1)
            .into_string();
        assert!(out.contains("<h1>T</h1>"), "got: {out}");
    }
}
