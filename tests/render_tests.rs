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
    stack,
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

/// Stack — the layout primitive. Asserted on the rendered HTML rather than on
/// "it compiles", because every one of these is a contract something outside
/// Rust depends on: the class names are the CSS seam, the element name is the
/// accessibility-tree seam, and the absence of default modifiers is what keeps
/// generated markup readable.
mod stack_layout {
    use maud::html;
    use maud_ui::primitives::stack::{self, Align, Direction, Justify, Props, Space, Tag};
    use maud_ui::tokens::spacing;

    #[test]
    fn default_props_emit_only_the_base_class() {
        let out = stack::render(Props::default()).into_string();
        assert!(
            out.contains(r#"<div class="mui-stack">"#),
            "default stack should carry no modifier classes: {out}"
        );
        assert!(!out.contains("id="), "no id when None: {out}");
        assert!(!out.contains("aria-label"), "no aria-label when None: {out}");
    }

    #[test]
    fn children_pass_through_and_nest() {
        let out = stack::render(Props {
            children: stack::horizontal(html! { span { "leaf" } }),
            ..Default::default()
        })
        .into_string();
        assert!(out.contains("<span>leaf</span>"), "children lost: {out}");
        assert_eq!(
            out,
            concat!(
                r#"<div class="mui-stack">"#,
                r#"<div class="mui-stack mui-stack--horizontal mui-stack--align-center">"#,
                "<span>leaf</span>",
                "</div></div>"
            ),
            "nesting or class order changed"
        );
    }

    #[test]
    fn every_non_default_modifier_reaches_the_class_attribute() {
        let out = stack::render(Props {
            direction: Direction::Horizontal,
            gap: Space::Xl,
            padding: Space::Sm,
            align: Align::Center,
            justify: Justify::Between,
            wrap: true,
            ..Default::default()
        })
        .into_string();
        for expected in [
            "mui-stack",
            "mui-stack--horizontal",
            "mui-stack--gap-xl",
            "mui-stack--p-sm",
            "mui-stack--align-center",
            "mui-stack--justify-between",
            "mui-stack--wrap",
        ] {
            assert!(out.contains(expected), "missing {expected} in: {out}");
        }
    }

    #[test]
    fn default_variants_emit_no_class_but_gap_none_does() {
        // Md is the gap default, so it is carried by `.mui-stack` itself.
        assert_eq!(Space::Md.gap_class(), "");
        // ...but "no gap at all" is NOT the default and must be explicit.
        assert_eq!(Space::None.gap_class(), "mui-stack--gap-none");
        // Padding runs the other way: None is the default, Md is explicit.
        assert_eq!(Space::None.padding_class(), "");
        assert_eq!(Space::Md.padding_class(), "mui-stack--p-md");
        assert_eq!(Direction::Vertical.as_class(), "");
        assert_eq!(Align::Stretch.as_class(), "");
        assert_eq!(Justify::Start.as_class(), "");
    }

    #[test]
    fn every_tag_renders_its_own_element() {
        for (tag, element) in [
            (Tag::Div, "div"),
            (Tag::Article, "article"),
            (Tag::Aside, "aside"),
            (Tag::Nav, "nav"),
            (Tag::Header, "header"),
            (Tag::Footer, "footer"),
            (Tag::Main, "main"),
        ] {
            let out = stack::render(Props {
                tag,
                ..Default::default()
            })
            .into_string();
            assert!(
                out.starts_with(&format!("<{element} ")) && out.ends_with(&format!("</{element}>")),
                "Tag::{tag:?} should render <{element}>: {out}"
            );
            assert_eq!(tag.as_element(), element);
        }
    }

    #[test]
    fn id_and_aria_label_are_emitted_when_present() {
        let out = stack::render(Props {
            tag: Tag::Nav,
            id: Some("primary-nav".into()),
            aria_label: Some("Primary".into()),
            ..Default::default()
        })
        .into_string();
        assert!(out.contains(r#"id="primary-nav""#), "id missing: {out}");
        assert!(out.contains(r#"aria-label="Primary""#), "aria-label missing: {out}");
    }

    /// An unnamed `<section>` is stripped of its `region` role, so the grouping
    /// silently does not exist for screen-reader users. Debug builds refuse it.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "requires aria_label")]
    fn section_without_aria_label_panics_in_debug() {
        let _ = stack::render(Props {
            tag: Tag::Section,
            ..Default::default()
        });
    }

    #[test]
    fn named_section_is_accepted() {
        let out = stack::render(Props {
            tag: Tag::Section,
            aria_label: Some("Release notes".into()),
            ..Default::default()
        })
        .into_string();
        assert!(out.starts_with("<section "), "got: {out}");
    }

    /// `Space::as_length` is documented as mirroring `tokens::spacing`. If the
    /// two ever drift, the docs and the CSS custom properties disagree with the
    /// Rust constants and nothing else would catch it.
    #[test]
    fn space_scale_mirrors_the_spacing_tokens() {
        assert_eq!(Space::Xs.as_length(), spacing::XS);
        assert_eq!(Space::Sm.as_length(), spacing::SM);
        assert_eq!(Space::Md.as_length(), spacing::MD);
        assert_eq!(Space::Lg.as_length(), spacing::LG);
        assert_eq!(Space::Xl.as_length(), spacing::XL);
        assert_eq!(Space::Xxl.as_length(), spacing::XXL);
        assert_eq!(Space::None.as_length(), "0");
    }

    #[test]
    fn helpers_match_their_documented_props() {
        let helper = stack::horizontal(html! {}).into_string();
        let explicit = stack::render(Props {
            direction: Direction::Horizontal,
            align: Align::Center,
            ..Default::default()
        })
        .into_string();
        assert_eq!(helper, explicit);

        let helper = stack::vertical(html! {}).into_string();
        let explicit = stack::render(Props::default()).into_string();
        assert_eq!(helper, explicit);
    }
}
