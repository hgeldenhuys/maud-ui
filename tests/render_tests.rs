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
    code_block,
    collapsible,
    combobox,
    command,
    context_menu,
    data_table,
    date_picker,
    dialog,
    diff,
    direction,
    drawer,
    empty_state,
    field,
    fieldset,
    form,
    grid,
    hover_card,
    input,
    input_group,
    input_otp,
    item,
    kbd,
    label,
    menu,
    menubar,
    message,
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
    sheet,
    sidebar,
    skeleton,
    slider,
    sonner,
    spinner,
    stack,
    streaming_cursor,
    swatch,
    switch,
    table,
    tabs,
    textarea,
    toast,
    toggle,
    toggle_group,
    tool_call,
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

/// Grid — the two-dimensional container. Same reasoning as `stack_layout`:
/// the class attribute IS the CSS contract, and the props that silently do
/// nothing (min_column on a fixed count) have to actually stay silent.
mod grid_layout {
    use maud::html;
    use maud_ui::primitives::grid::{self, Align, Columns, MinColumn, Props, Space};

    #[test]
    fn default_props_emit_only_the_base_class() {
        let out = grid::render(Props::default()).into_string();
        assert_eq!(out, r#"<div class="mui-grid"></div>"#, "got: {out}");
    }

    #[test]
    fn every_non_default_modifier_reaches_the_class_attribute() {
        let out = grid::render(Props {
            columns: Columns::Four,
            gap: Space::Xl,
            padding: Space::Sm,
            align: Align::Center,
            ..Default::default()
        })
        .into_string();
        for expected in [
            "mui-grid",
            "mui-grid--cols-4",
            "mui-grid--gap-xl",
            "mui-grid--p-sm",
            "mui-grid--align-center",
            // Fixed counts collapse on narrow viewports by default.
            "mui-grid--collapse",
        ] {
            assert!(out.contains(expected), "missing {expected} in: {out}");
        }
    }

    /// `min_column` only means anything for auto-fit tracks. Emitting it on a
    /// fixed count would put a class in the HTML that changes nothing — a lie
    /// to anyone reading the markup to work out what the layout does.
    #[test]
    fn min_column_is_suppressed_on_fixed_counts() {
        let auto_fit = grid::render(Props {
            min_column: MinColumn::Xl,
            ..Default::default()
        })
        .into_string();
        assert!(auto_fit.contains("mui-grid--min-xl"), "got: {auto_fit}");

        let fixed = grid::render(Props {
            columns: Columns::Three,
            min_column: MinColumn::Xl,
            ..Default::default()
        })
        .into_string();
        assert!(!fixed.contains("mui-grid--min"), "inert class emitted: {fixed}");
    }

    /// Auto-fit already reflows, so the collapse modifier would be dead weight.
    #[test]
    fn collapse_is_suppressed_on_auto_fit_and_when_opted_out() {
        let auto_fit = grid::render(Props::default()).into_string();
        assert!(!auto_fit.contains("mui-grid--collapse"), "got: {auto_fit}");

        let opted_out = grid::render(Props {
            columns: Columns::Six,
            collapse_narrow: false,
            ..Default::default()
        })
        .into_string();
        assert!(!opted_out.contains("mui-grid--collapse"), "got: {opted_out}");
    }

    #[test]
    fn column_classes_and_counts_agree() {
        for (columns, class, count) in [
            (Columns::One, "mui-grid--cols-1", Some(1u8)),
            (Columns::Two, "mui-grid--cols-2", Some(2)),
            (Columns::Three, "mui-grid--cols-3", Some(3)),
            (Columns::Four, "mui-grid--cols-4", Some(4)),
            (Columns::Five, "mui-grid--cols-5", Some(5)),
            (Columns::Six, "mui-grid--cols-6", Some(6)),
            (Columns::Twelve, "mui-grid--cols-12", Some(12)),
        ] {
            assert_eq!(columns.as_class(), class);
            assert_eq!(columns.count(), count);
        }
        assert_eq!(Columns::AutoFit.as_class(), "");
        assert_eq!(Columns::AutoFit.count(), None);
    }

    /// grid and stack must keep sharing ONE scale — a second copy would drift.
    #[test]
    fn grid_and_stack_share_the_same_space_and_align_types() {
        let from_stack: Space = maud_ui::primitives::stack::Space::Lg;
        let from_grid: Space = Space::Lg;
        assert_eq!(from_stack, from_grid);

        let from_stack: Align = maud_ui::primitives::stack::Align::Center;
        assert_eq!(from_stack, Align::Center);
    }

    #[test]
    fn children_pass_through_and_id_is_emitted() {
        let out = grid::render(Props {
            id: Some("tiles".into()),
            children: html! { span { "a" } span { "b" } },
            ..Default::default()
        })
        .into_string();
        assert!(out.contains(r#"id="tiles""#), "got: {out}");
        assert!(out.contains("<span>a</span><span>b</span>"), "got: {out}");
    }

    #[test]
    fn helpers_match_their_documented_props() {
        assert_eq!(
            grid::auto(html! {}).into_string(),
            grid::render(Props::default()).into_string()
        );
        assert_eq!(
            grid::columns(Columns::Three, html! {}).into_string(),
            grid::render(Props {
                columns: Columns::Three,
                ..Default::default()
            })
            .into_string()
        );
    }
}

/// Form — the submission contract. Every assertion here is about an attribute
/// a BROWSER acts on, so "it compiles" is worth nothing: a missing enctype
/// silently uploads filenames instead of files, and a defaulted method decides
/// whether a password ends up in the URL.
mod form_contract {
    use maud::html;
    use maud_ui::primitives::form::{self, Enctype, Method, Props};

    /// The one deliberate divergence from HTML. If this ever flips back to the
    /// platform default, a form whose author forgot `method` starts putting
    /// every field into the URL, browser history, and the Referer header.
    #[test]
    fn method_defaults_to_post_not_html_default_get() {
        let out = form::render(Props::default()).into_string();
        assert!(out.contains(r#"method="post""#), "got: {out}");
        assert_eq!(Method::Post, Method::default());
    }

    #[test]
    fn default_form_omits_every_optional_attribute() {
        let out = form::render(Props::default()).into_string();
        assert_eq!(
            out,
            r#"<form class="mui-form" method="post"></form>"#,
            "a default form should carry method and nothing else: {out}"
        );
    }

    /// urlencoded is the HTML default, so emitting it would be noise; multipart
    /// is NOT optional for file uploads and must always reach the markup.
    #[test]
    fn enctype_is_omitted_when_default_and_emitted_otherwise() {
        assert_eq!(Enctype::UrlEncoded.as_attr(), None);
        let default = form::render(Props::default()).into_string();
        assert!(!default.contains("enctype"), "got: {default}");

        let multipart = form::render(Props {
            enctype: Enctype::Multipart,
            ..Default::default()
        })
        .into_string();
        assert!(
            multipart.contains(r#"enctype="multipart/form-data""#),
            "got: {multipart}"
        );
    }

    #[test]
    fn every_method_maps_to_its_attribute() {
        for (method, attr) in [
            (Method::Get, "get"),
            (Method::Post, "post"),
            (Method::Dialog, "dialog"),
        ] {
            assert_eq!(method.as_attr(), attr);
            let out = form::render(Props {
                method,
                ..Default::default()
            })
            .into_string();
            assert!(out.contains(&format!(r#"method="{attr}""#)), "got: {out}");
        }
    }

    /// `autocomplete` is inverted: the default (true) is the browser's own
    /// behaviour and emits nothing; only opting OUT produces an attribute.
    #[test]
    fn autocomplete_only_emits_when_disabled() {
        let on = form::render(Props::default()).into_string();
        assert!(!on.contains("autocomplete"), "got: {on}");

        let off = form::render(Props {
            autocomplete: false,
            ..Default::default()
        })
        .into_string();
        assert!(off.contains(r#"autocomplete="off""#), "got: {off}");
    }

    #[test]
    fn novalidate_is_a_boolean_attribute() {
        let off = form::render(Props::default()).into_string();
        assert!(!off.contains("novalidate"), "got: {off}");

        let on = form::render(Props {
            novalidate: true,
            ..Default::default()
        })
        .into_string();
        // Boolean attributes render bare, not `novalidate="true"`.
        assert!(on.contains("novalidate"), "got: {on}");
        assert!(!on.contains("novalidate="), "got: {on}");
    }

    #[test]
    fn action_and_labels_are_emitted_when_present() {
        let out = form::render(Props {
            action: Some("/login".into()),
            id: Some("login-form".into()),
            aria_label: Some("Sign in".into()),
            children: html! { input name="email"; },
            ..Default::default()
        })
        .into_string();
        assert!(out.contains(r#"action="/login""#), "got: {out}");
        assert!(out.contains(r#"id="login-form""#), "got: {out}");
        assert!(out.contains(r#"aria-label="Sign in""#), "got: {out}");
        assert!(out.contains(r#"<input name="email">"#), "got: {out}");
    }

    #[test]
    fn stacked_is_a_post_form_wrapping_a_stack() {
        let out = form::stacked("/save", html! { span { "field" } }).into_string();
        assert!(out.starts_with(r#"<form class="mui-form" action="/save" method="post">"#), "got: {out}");
        assert!(out.contains(r#"<div class="mui-stack mui-stack--gap-lg">"#), "got: {out}");
        assert!(out.contains("<span>field</span>"), "got: {out}");
    }
}

/// The conversation tier — five primitives that shipped finished but
/// registered nowhere until 0.4.0. Each assertion below is a defect found
/// while writing their docs, so each one is a regression guard, not a
/// restatement of the implementation.
mod conversation_0_4_0 {
    use maud_ui::primitives::{code_block, diff, streaming_cursor};

    /// A diff conveys add-vs-remove by row tint and a `+`/`-` sigil that is
    /// `aria-hidden`. Without a text channel a screen reader gets the line
    /// content and no way to tell an addition from a deletion — the one thing
    /// a diff exists to say. Same contract `item::status_dot` already keeps.
    #[test]
    fn diff_rows_announce_add_and_remove_to_assistive_tech() {
        let out = diff::render(diff::Props {
            lines: vec![
                diff::Line {
                    kind: diff::LineKind::Add,
                    old_line_no: None,
                    new_line_no: Some(1),
                    text: "let x = 1;".into(),
                },
                diff::Line {
                    kind: diff::LineKind::Remove,
                    old_line_no: Some(1),
                    new_line_no: None,
                    text: "let x = 0;".into(),
                },
                diff::Line {
                    kind: diff::LineKind::Context,
                    old_line_no: Some(2),
                    new_line_no: Some(2),
                    text: "println!();".into(),
                },
            ],
            ..Default::default()
        })
        .into_string();

        assert!(
            out.contains(r#"<span class="mui-visually-hidden">Added: </span>let x = 1;"#),
            "added line must carry a text state, not colour alone: {out}"
        );
        assert!(
            out.contains(r#"<span class="mui-visually-hidden">Removed: </span>let x = 0;"#),
            "removed line must carry a text state, not colour alone: {out}"
        );
        // Context lines are the baseline — prefixing every unchanged line would
        // bury the two states that matter.
        assert!(
            !out.contains("Unchanged"),
            "context lines must not be announced: {out}"
        );
        assert_eq!(
            out.matches("mui-visually-hidden").count(),
            2,
            "exactly the add and the remove row: {out}"
        );
    }

    /// The announcement has to sit INSIDE the role="cell" span: content in a
    /// role="row" but outside a cell is not reliably announced.
    #[test]
    fn diff_announcement_is_inside_the_cell() {
        let out = diff::render(diff::Props {
            lines: vec![diff::Line {
                kind: diff::LineKind::Add,
                old_line_no: None,
                new_line_no: Some(1),
                text: "x".into(),
            }],
            ..Default::default()
        })
        .into_string();
        assert!(
            out.contains(
                r#"<span class="mui-diff__text" role="cell"><span class="mui-visually-hidden">Added: </span>x</span>"#
            ),
            "got: {out}"
        );
    }

    /// `show_copy` was documented as "default true" while a derived `Default`
    /// made it `false`, so every `..Default::default()` silently dropped the
    /// copy button.
    #[test]
    fn code_block_show_copy_defaults_to_true_as_documented() {
        assert!(code_block::Props::default().show_copy);
        let out = code_block::render(code_block::Props {
            code: "let x = 1;".into(),
            ..Default::default()
        })
        .into_string();
        assert!(out.contains("copy"), "copy affordance missing: {out}");
    }

    /// All three streaming indicators animate infinitely. Reduced-motion users
    /// must get a static indicator that is still VISIBLE — not a removed one.
    #[test]
    fn streaming_indicators_have_a_reduced_motion_fallback() {
        let css = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/css/components/streaming_cursor.css"
        ))
        .unwrap();
        let (_, reduced) = css
            .split_once("@media (prefers-reduced-motion: reduce)")
            .expect("no prefers-reduced-motion block — three infinite animations run unconditionally");
        for selector in [
            ".mui-streaming__cursor",
            ".mui-streaming__dots span",
            ".mui-streaming__pulse",
        ] {
            assert!(
                reduced.contains(selector),
                "{selector} still animates under reduced motion"
            );
        }
        assert!(
            reduced.contains("animation: none"),
            "fallback must stop the animation, leaving the indicator visible"
        );
        // display:none / opacity:0 would remove the indicator instead of
        // stilling it — the user would lose the signal entirely.
        let block_end = reduced.find('\n').map(|_| reduced).unwrap_or(reduced);
        assert!(
            !block_end.contains("display: none") && !block_end.contains("opacity: 0"),
            "reduced motion must still the indicator, not hide it"
        );
    }

    /// Every conversation primitive is reachable through the public dispatch —
    /// the thing that was broken for all five until 0.4.0.
    #[test]
    fn all_five_conversation_primitives_render_a_real_page() {
        for slug in [
            "message",
            "streaming_cursor",
            "code_block",
            "diff",
            "tool_call",
        ] {
            let page = maud_ui::showcase::component_page_by_name(slug).into_string();
            assert!(
                !page.contains("Component not found"),
                "{slug} falls through to the 404 page"
            );
        }
    }

    #[test]
    fn streaming_cursor_marks_the_caret_decorative() {
        let out = streaming_cursor::render(streaming_cursor::Props::default()).into_string();
        assert!(
            out.contains(r#"aria-hidden="true""#),
            "the caret is decoration; screen readers must not announce it: {out}"
        );
    }
}
