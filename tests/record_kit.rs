//! Native form/data contracts for the record kit; visual layout is a separate review.
use maud::html;
use maud_ui::{
    blocks::{
        action::{Action, Heading, Target},
        action_row, attention_banner,
        record::{money, related_card, stay_timeline, timeline},
        worklist::grouped,
    },
    primitives::{badge, choice_card, date_range, facts_list::Fact},
};

#[test]
fn timeline_keeps_order_explicit_states_and_alias() {
    let milestones = [
        timeline::State::Done,
        timeline::State::Current,
        timeline::State::Scheduled,
    ]
    .into_iter()
    .enumerate()
    .map(|(i, state)| timeline::Milestone {
        label: format!("Step {i} <safe>"),
        date: format!("Date {i}"),
        datetime: Some("2026-09-08".into()),
        subline: Some("Caller & context".into()),
        state,
        ..Default::default()
    })
    .collect();
    let props = timeline::Props {
        milestones,
        heading: Heading::H3,
        ..Default::default()
    };
    let output = stay_timeline::render(props.clone()).into_string();
    assert_eq!(output, timeline::render(props).into_string());
    assert_eq!(output.matches("aria-current=\"step\"").count(), 1);
    assert_eq!(output.matches("<li ").count(), 3);
    for state in ["done", "current", "scheduled"] {
        assert!(output.contains(&format!("data-state=\"{state}\"")));
    }
    assert!(output.contains("datetime=\"2026-09-08\""));
    assert!(output.contains("Step 0 &lt;safe&gt;"));
    assert!(output.contains("Caller &amp; context"));
    assert!(output.find("Step 0").unwrap() < output.find("Step 2").unwrap());
    assert!(output.contains("<h3 "));
    assert!(timeline::render(Default::default())
        .into_string()
        .is_empty());
}
#[test]
fn timeline_accepts_five_scheduled_steps_and_localized_labels() {
    let output = timeline::render(timeline::Props {
        milestones: (0..5)
            .map(|_| timeline::Milestone {
                label: "Planned".into(),
                state_label: Some("Prévu".into()),
                ..Default::default()
            })
            .collect(),
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches(">Prévu<").count(), 5);
    assert!(!output.contains("aria-current"));
}
#[test]
#[should_panic(expected = "at most one current milestone")]
fn timeline_rejects_ambiguous_current_steps() {
    timeline::render(timeline::Props {
        milestones: vec![
            timeline::Milestone {
                state: timeline::State::Current,
                ..Default::default()
            };
            2
        ],
        ..Default::default()
    });
}
#[test]
fn unavailable_money_is_not_zero_or_a_warning() {
    let output = money::render(Default::default()).into_string();
    assert_eq!(output.matches(">Unavailable<").count(), 3);
    assert!(!output.contains("$0"));
    assert!(!output.contains("data-emphasis"));
    assert!(!output.contains("mui-record-money__actions"));
}
#[test]
fn money_preserves_canonical_fields_and_actions_without_calculation() {
    let output = money::render(money::Props {
        currency: "CAD <currency>".into(),
        total: money::Figure::new("Total", "$840 & tax"),
        paid: money::Figure {
            label: "Paid".into(),
            value: Some(html! { span data-field="masked-payment" { "Restricted" } }),
        },
        due: money::Figure::new("Due", "$560"),
        breakdown: Some("3 × $240 + $120 fees".into()),
        actions: html! { a href="/pay" data-action="finance-review" { "Review" } },
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("CAD &lt;currency&gt;"));
    assert!(output.contains("$840 &amp; tax"));
    assert!(output.contains("data-field=\"masked-payment\""));
    assert!(output.contains("data-action=\"finance-review\""));
    assert_eq!(output.matches("data-emphasis=\"warning\"").count(), 1);
    assert!(!output.contains("$280"));
}
#[test]
fn money_known_zero_can_be_quiet_and_empty_fragment_stays_empty() {
    let output = money::render(money::Props {
        total: money::Figure {
            label: "Total".into(),
            value: Some(html! {}),
        },
        due: money::Figure::new("Due", "$0"),
        emphasize_due: false,
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("$0"));
    assert_eq!(output.matches(">Unavailable<").count(), 1);
    assert!(!output.contains("data-emphasis"));
}
#[test]
fn related_card_has_one_action_and_escaped_definition_list() {
    let output = related_card::render(related_card::Props {
        title: "Room <4>".into(),
        facts: vec![Fact::new("Bed", "Queen & cot"), Fact::mono("Floor", "01")],
        action: Some(Action::link("View", "/room?x=1&y=2")),
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("Room &lt;4&gt;"));
    assert!(output.contains("Queen &amp; cot"));
    assert_eq!(output.matches("<dt ").count(), 2);
    assert_eq!(output.matches("<a ").count(), 1);
}
#[test]
fn related_card_slots_preserve_ownership_and_empty_overrides() {
    let output = related_card::render(related_card::Props {
        title: "fallback title".into(),
        title_markup: Some(html! { h3 data-field="room" { "Canonical" } }),
        facts: vec![Fact::new("fallback fact", "value")],
        facts_markup: Some(html! {}),
        action: Some(Action::link("fallback action", "/fallback")),
        action_markup: Some(html! { a data-action="room" href="/room" { "Room" } }),
        ..Default::default()
    })
    .into_string();
    assert!(!output.contains("fallback"));
    assert!(!output.contains("<h2"));
    assert_eq!(output.matches("<h3").count(), 1);
    assert_eq!(output.matches("<a ").count(), 1);
}
fn row(identity: &str) -> grouped::Row {
    grouped::Row {
        identity: identity.into(),
        facts: [
            Fact::new("Arrival", "Unknown"),
            Fact::new("Payment", "Unavailable"),
        ],
        status: badge::Props {
            label: "Review".into(),
            ..Default::default()
        },
        action: Some(Action::link("View", "/record")),
        ..Default::default()
    }
}
#[test]
fn worklist_keeps_caller_order_scoped_labels_and_single_row_dom() {
    let output = grouped::render(grouped::Props {
        groups: vec![
            grouped::Group {
                label: "Later · 20 total".into(),
                rows: vec![row("Maya"), row("Oliver")],
                ..Default::default()
            },
            grouped::Group {
                label: "Empty group".into(),
                ..Default::default()
            },
            grouped::Group {
                label: "Time unknown".into(),
                rows: vec![row("Amina")],
                ..Default::default()
            },
        ],
        footer: Some("Page 1 of admitted results".into()),
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("<li ").count(), 3);
    assert_eq!(output.matches("<dt ").count(), 6);
    assert_eq!(output.matches(">Maya<").count(), 1);
    assert!(output.contains("20 total"));
    assert!(!output.contains("Empty group"));
    assert!(output.find("Oliver").unwrap() < output.find("Amina").unwrap());
    assert!(output.contains("Page 1 of admitted results"));
    assert_eq!(output.matches("mui-btn--row").count(), 3);
}
#[test]
fn worklist_empty_and_canonical_overrides() {
    let empty = grouped::render(grouped::Props {
        empty_message: "No arrivals <today>".into(),
        footer: Some("Updated at 10:00".into()),
        ..Default::default()
    })
    .into_string();
    assert!(empty.contains("No arrivals &lt;today&gt;"));
    assert!(!empty.contains("<ul"));
    assert!(empty.contains("Updated at 10:00"));
    let output = grouped::render(grouped::Props {
        groups: vec![grouped::Group {
            rows: vec![grouped::Row {
                identity_markup: Some(
                    html! { a data-entity="guest" href="/guest" { "Canonical identity" } },
                ),
                status_markup: Some(html! {}),
                action_markup: Some(
                    html! { button type="button" data-action="review" { "Canonical action" } },
                ),
                ..row("Fallback")
            }],
            ..Default::default()
        }],
        ..Default::default()
    })
    .into_string();
    assert!(!output.contains("Fallback"));
    assert!(!output.contains("mui-badge"));
    assert!(output.contains("data-entity=\"guest\""));
    assert!(output.contains("data-action=\"review\""));
    assert_eq!(output.matches("<button ").count(), 1);
}
fn room(value: &str) -> choice_card::Choice {
    choice_card::Choice {
        value: value.into(),
        title: format!("Room {value}"),
        ..Default::default()
    }
}
#[test]
fn choice_cards_use_native_radio_group_contract_and_disabled_reason() {
    let output = choice_card::render(choice_card::Props {
        id: "rooms".into(),
        name: "room_id".into(),
        form: Some("booking".into()),
        required: true,
        selected: Some("4".into()),
        choices: vec![
            room("4"),
            choice_card::Choice {
                disabled_reason: Some("Unavailable <today>".into()),
                ..room("12")
            },
        ],
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("type=\"radio\"").count(), 2);
    assert_eq!(output.matches("name=\"room_id\"").count(), 2);
    assert_eq!(output.matches("form=\"booking\"").count(), 2);
    assert_eq!(output.matches(" required").count(), 2);
    assert_eq!(output.matches(" checked").count(), 1);
    assert_eq!(output.matches(" disabled").count(), 1);
    assert!(output.contains("aria-describedby=\"rooms-1-reason\""));
    assert!(output.contains("id=\"rooms-1-reason\">Unavailable &lt;today&gt;"));
    assert!(!output.contains("tabindex"));
    assert!(!output.contains("role=\"radio\""));
    assert!(!output.contains("data-mui="));
}
#[test]
fn choice_cards_do_not_select_disabled_or_unknown_values_or_multiple_duplicates() {
    for selected in ["disabled", "unknown"] {
        let output = choice_card::render(choice_card::Props {
            selected: Some(selected.into()),
            choices: vec![
                room("enabled"),
                choice_card::Choice {
                    disabled_reason: Some("Booked".into()),
                    ..room("disabled")
                },
            ],
            ..Default::default()
        })
        .into_string();
        assert!(!output.contains(" checked"));
    }
    let output = choice_card::render(choice_card::Props {
        selected: Some("4".into()),
        disabled: true,
        choices: vec![room("4"), room("4")],
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches(" checked").count(), 1);
    assert_eq!(output.matches(" disabled").count(), 3);
}
#[test]
fn calendar_nights_handle_leap_centuries_years_and_dst_without_timezones() {
    for (start, end, nights) in [
        ("2026-09-08", "2026-09-11", 3),
        ("2024-02-28", "2024-03-01", 2),
        ("1900-02-28", "1900-03-01", 1),
        ("2000-02-28", "2000-03-01", 2),
        ("2026-03-07", "2026-03-09", 2),
        ("2026-10-31", "2026-11-02", 2),
        ("0099-12-31", "0100-01-01", 1),
        ("0001-01-01", "9999-12-31", 3_652_058),
    ] {
        assert_eq!(
            date_range::nights_between(start, end),
            Some(nights),
            "{start} to {end}"
        );
    }
}
#[test]
fn calendar_nights_reject_invalid_same_day_and_reversed_ranges() {
    for date in [
        "",
        "0000-01-01",
        "2026-02-29",
        "1900-02-29",
        "2026-04-31",
        "2026-13-01",
        "2026-00-01",
        "2026-01-00",
        "2026-9-08",
        "２０２６-09-08",
        "2026-09-08T12:00:00Z",
        "202x-09-08",
    ] {
        assert_eq!(
            date_range::nights_between(date, "9999-12-31"),
            None,
            "{date}"
        );
    }
    assert_eq!(date_range::nights_between("2026-09-08", "2026-09-08"), None);
    assert_eq!(date_range::nights_between("2026-09-09", "2026-09-08"), None);
}
fn range() -> date_range::Props {
    date_range::Props {
        id: "dates".into(),
        start: date_range::DateInput {
            name: "arrival".into(),
            label: "Arrival".into(),
            value: "2026-09-08".into(),
            ..Default::default()
        },
        end: date_range::DateInput {
            name: "departure".into(),
            label: "Departure".into(),
            value: "2026-09-11".into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
#[test]
fn date_range_initial_count_and_native_constraints_survive_without_js() {
    let mut props = range();
    props.required = true;
    props.read_only = true;
    props.form = Some("booking".into());
    props.start.min = Some("2026-09-01".into());
    props.end.max = Some("2026-09-30".into());
    let output = date_range::render(props).into_string();
    assert_eq!(output.matches("type=\"date\"").count(), 2);
    assert_eq!(output.matches(" required").count(), 2);
    assert_eq!(output.matches(" readonly").count(), 2);
    assert_eq!(output.matches("form=\"booking\"").count(), 2);
    assert_eq!(
        output
            .matches("aria-describedby=\"dates-duration\"")
            .count(),
        2
    );
    assert!(output.contains(">3 nights</output>"));
    assert!(output.contains("<noscript>"));
    assert!(output.contains("min=\"2026-09-01\""));
    assert!(output.contains("max=\"2026-09-30\""));
    assert!(!output.contains("aria-invalid"));
    assert!(output.contains("for=\"dates-start dates-end\""));
}
#[test]
fn date_range_valid_incomplete_invalid_bounds_and_localized_duration() {
    let mut props = range();
    props.end.value = "2026-09-09".into();
    props.night_label = "nuit".into();
    assert!(date_range::render(props.clone())
        .into_string()
        .contains(">1 nuit</output>"));
    props.end.value.clear();
    assert!(date_range::render(props.clone())
        .into_string()
        .contains(">Choose both dates</output>"));
    props = range();
    props.start.min = Some("2026-09-09".into());
    let output = date_range::render(props.clone()).into_string();
    assert_eq!(output.matches("aria-invalid=\"true\"").count(), 2);
    assert!(!output.contains(">3 nights</output>"));
    props.start.min = Some("2026-09-08".into());
    props.end.max = Some("2026-09-11".into());
    props.disabled = true;
    let output = date_range::render(props).into_string();
    assert_eq!(output.matches(" disabled").count(), 3);
    assert!(output.contains(">3 nights</output>"));
}
#[test]
fn action_row_empty_overrides_and_densities() {
    assert!(action_row::render(Default::default())
        .into_string()
        .is_empty());
    assert!(action_row::render(action_row::Props {
        primary: Some(Action::submit("Save")),
        primary_markup: Some(html! {}),
        ..Default::default()
    })
    .into_string()
    .is_empty());
    for density in [
        action_row::Density::Row,
        action_row::Density::Compact,
        action_row::Density::Comfortable,
    ] {
        let output = action_row::render(action_row::Props {
            density,
            primary: Some(Action::submit("Save")),
            secondary: vec![Action::link("Cancel", "/cancel")],
            overflow: vec![Action::link("Invoice", "/invoice")],
            ..Default::default()
        })
        .into_string();
        assert!(output.contains(&format!("data-density=\"{}\"", density.as_str())));
        assert!(output.contains("<details "));
        assert!(output.contains("<summary "));
        assert!(!output.contains("role=\"menu\""));
        assert!(output.find(">Save<").unwrap() < output.find(">Cancel<").unwrap());
        assert!(!output.contains("<form"));
    }
}
#[test]
fn submit_contract_preserves_form_owner_name_value_and_post_csrf() {
    let submit = Action {
        label: "Save".into(),
        target: Target::Submit {
            form: Some("booking".into()),
            name: Some("intent".into()),
            value: Some("save & close".into()),
        },
    };
    let output = action_row::render(action_row::Props {
        primary: Some(submit),
        ..Default::default()
    })
    .into_string();
    assert!(output
        .contains("type=\"submit\" form=\"booking\" name=\"intent\" value=\"save &amp; close\""));
    assert!(!output.contains("<form"));
    let post = Action {
        label: "Review".into(),
        target: Target::Post {
            action: "/payment".into(),
            hidden_fields: vec![("csrf".into(), "token&value".into())],
        },
    };
    let output = action_row::render(action_row::Props {
        primary: Some(post),
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("<form ").count(), 1);
    assert!(output.contains("method=\"post\" action=\"/payment\""));
    assert!(output.contains("name=\"csrf\" value=\"token&amp;value\""));
    let output = action_row::render(action_row::Props {
        primary_markup: Some(
            html! { button type="submit" data-action="save" data-result="region" { "Save" } },
        ),
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("data-action=\"save\" data-result=\"region\""));
}
#[test]
fn banner_tones_one_action_and_safe_no_script_dismissal() {
    for tone in [
        attention_banner::Tone::Warning,
        attention_banner::Tone::Danger,
        attention_banner::Tone::Info,
    ] {
        let output = attention_banner::render(attention_banner::Props {
            title: "Arrival <review>".into(),
            tone,
            action: Some(Action::link("Review", "/record")),
            dismiss: Some(attention_banner::Dismiss {
                focus_target: Some("next".into()),
                ..Default::default()
            }),
            ..Default::default()
        })
        .into_string();
        assert!(output.contains(&format!("data-tone=\"{}\"", tone.as_str())));
        assert!(output.contains("Arrival &lt;review&gt;"));
        assert_eq!(output.matches("<a ").count(), 1);
        assert!(output.contains("aria-label=\"Dismiss notice\" data-dismiss-focus=\"next\" hidden"));
        assert!(!output.contains("role=\"alert\""));
    }
    let output = attention_banner::render(attention_banner::Props {
        title: "Required".into(),
        action: Some(Action::link("Fallback", "/fallback")),
        action_markup: Some(html! {}),
        ..Default::default()
    })
    .into_string();
    assert!(!output.contains("<button"));
    assert!(!output.contains("Fallback"));
}
#[test]
fn every_new_preview_contains_both_themes_and_api_docs() {
    for (slug, preview) in [
        ("record-timeline", timeline::preview()),
        ("record-money", money::preview()),
        ("record-related-card", related_card::preview()),
        ("worklist-grouped", grouped::preview()),
        ("attention-banner", attention_banner::preview()),
        ("action-row", action_row::preview()),
    ] {
        let output = preview.into_string();
        assert!(output.contains("data-theme=\"light\""), "{slug}");
        assert!(output.contains("data-theme=\"dark\""), "{slug}");
        let docs = maud_ui::showcase::docs::render_block_docs(slug)
            .unwrap()
            .into_string();
        assert!(docs.contains("Props"));
        assert!(docs.contains("Accessibility") || docs.contains("accessibility"));
    }
    for preview in [choice_card::showcase(), date_range::showcase()] {
        let output = preview.into_string();
        assert!(output.contains("data-theme=\"light\""));
        assert!(output.contains("data-theme=\"dark\""));
    }
}

#[test]
fn grouped_facts_preserve_canonical_markers_or_explicit_empty_override() {
    for markup in [
        html! { dl class="mui-facts" { div class="mui-facts__row" { dt { "Payment" } dd data-field="balance" data-classification="masked" { "Restricted" } } div class="mui-facts__row" { dt { "Arrival" } dd data-field="eta" { "Unknown" } } } },
        html! {},
    ] {
        let output = grouped::render(grouped::Props {
            groups: vec![grouped::Group {
                rows: vec![grouped::Row {
                    facts_markup: Some(markup.clone()),
                    ..row("Guest")
                }],
                ..Default::default()
            }],
            ..Default::default()
        })
        .into_string();
        assert!(!output.contains("Unavailable"));
        assert!(output.contains(&markup.into_string()));
    }
}
#[test]
fn related_api_links_reach_gallery_routes_instead_of_markdown_404s() {
    let block = maud_ui::showcase::docs::render_block_docs("action-row")
        .unwrap()
        .into_string();
    assert!(block.contains("href=\"/form\""));
    assert!(block.contains("href=\"/blocks/record-money\""));
    assert!(!block.contains(".md\""));
    let primitive = maud_ui::showcase::docs::render_component_docs("date_range")
        .unwrap()
        .into_string();
    assert!(primitive.contains("href=\"/choice_card\""));
    assert!(primitive.contains("href=\"/blocks/action-row\""));
    assert!(!primitive.contains(".md\""));
    let existing = maud_ui::showcase::docs::render_component_docs("form")
        .unwrap()
        .into_string();
    assert!(existing.contains("href=\"/blocks/action-row\""));
}
