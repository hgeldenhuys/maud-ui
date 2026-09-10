use maud::html;
use maud_ui::{
    blocks::{
        action::Link,
        feedback::{confirm, notice},
        search::results,
        state::State,
        wizard::header,
        worklist::empty,
    },
    primitives::{field, form, native_select},
};

#[test]
fn required_select_renders_constraints_and_only_one_selected_option() {
    let output = native_select::render(native_select::NativeSelectProps {
        name: "method".into(),
        id: "method".into(),
        required: true,
        invalid: true,
        aria_describedby: Some("method-error".into()),
        placeholder: Some("Choose method".into()),
        selected: Some("Cash".into()),
        options: vec![native_select::NativeOption {
            value: "Cash".into(),
            label: "Cash".into(),
            disabled: false,
        }],
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("required aria-invalid=\"true\" aria-describedby=\"method-error\""));
    assert_eq!(output.matches(" selected").count(), 1);
}

#[test]
fn feedback_is_opt_in_and_keeps_native_post_semantics() {
    let ordinary = form::render(form::Props::default()).into_string();
    assert!(ordinary.contains("method=\"post\""));
    assert!(!ordinary.contains("form-feedback"));
    let enhanced = form::render(form::Props {
        feedback: true,
        action: Some("/save".into()),
        pending_label: "Saving guest…".into(),
        ..Default::default()
    })
    .into_string();
    assert!(enhanced.contains("data-mui=\"form-feedback\""));
    assert!(enhanced.contains("data-pending-label=\"Saving guest…\""));
    assert!(!enhanced.contains("novalidate"));
}
#[test]
fn server_field_errors_have_stable_links_and_escaped_text() {
    let output = field::render(field::Props {
        id: "email".into(),
        error: Some("Already used <again>".into()),
        description: Some("Contact email".into()),
        children: html! { input id="email"; },
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("data-field-control=\"email\""));
    assert!(output.contains("id=\"email-err\""));
    assert!(output.contains("id=\"email-desc\""));
    assert!(output.contains("Already used &lt;again&gt;"));
}
#[test]
fn notice_always_leads_with_a_human_line_and_collapses_diagnostics() {
    let output = notice::render(notice::Props {
        message: " ".into(),
        tone: notice::Tone::Error,
        details: Some("<script>failure</script>".into()),
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("role=\"alert\""));
    assert!(
        output.find("Your changes could not be saved.").unwrap() < output.find("<details").unwrap()
    );
    assert!(output.contains("&lt;script&gt;failure&lt;/script&gt;"));
    assert!(!output.contains("data-duration"));
}
#[test]
fn confirm_has_cancel_focus_and_one_real_post_with_escaped_values() {
    let output = confirm::render(confirm::Props {
        id: "confirm".into(),
        action: "/check-in".into(),
        hidden_fields: vec![("csrf".into(), "<token>".into())],
        danger: true,
        ..Default::default()
    })
    .into_string();
    assert_eq!(output.matches("<form ").count(), 1);
    assert!(output.contains("method=\"post\""));
    assert!(output.contains("data-mui-close autofocus"));
    assert!(output.contains("name=\"csrf\" value=\"&lt;token&gt;\""));
    assert!(output.contains("mui-btn--danger"));
    assert!(!output.contains("onclick") && !output.contains("window.confirm"));
}
#[test]
fn search_reports_authoritative_counts_and_does_not_leak_loading_data() {
    let mut props = results::Props {
        query: "Leila".into(),
        total: Some(9),
        items: vec![results::Item {
            title: "Leila".into(),
            description: Some("Leila".into()),
            href: "/guest".into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    let output = results::render(props.clone()).into_string();
    assert!(output.contains("9 results for"));
    assert!(output.contains("Showing 1 of 9."));
    assert!(!output.contains("mui-search-results__description"));
    props.state = State::loading();
    let output = results::render(props).into_string();
    assert!(!output.contains("Leila"));
    assert!(output.contains("aria-busy=\"true\""));
}
#[test]
fn empty_search_preserves_query_and_has_a_clear_action() {
    let output = results::render(results::Props {
        query: "Unknown".into(),
        clear_action: Some(Link {
            label: "Clear search".into(),
            href: "/search".into(),
        }),
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("0 results for “Unknown”"));
    assert!(output.contains("Clear search"));
    assert!(!output.contains("<ul"));
}
#[test]
fn wizard_has_one_current_step_and_explicit_terminal_state() {
    let mut props = header::Props {
        steps: vec!["Choose".into(), "Verify".into(), "Check in".into()],
        current: 99,
        ..Default::default()
    };
    let output = header::render(props.clone()).into_string();
    assert_eq!(output.matches("aria-current=\"step\"").count(), 1);
    assert!(output.contains("Step 3 of 3"));
    props.complete = true;
    let output = header::render(props).into_string();
    assert!(!output.contains("aria-current"));
    assert!(output.contains("All 3 steps complete"));
    let empty = header::render(header::Props::default()).into_string();
    assert!(!empty.contains("<ol"));
}
#[test]
fn first_record_does_not_invent_a_create_permission() {
    assert!(!empty::render(empty::Props::default())
        .into_string()
        .contains("<a"));
    let output = empty::render(empty::Props {
        create: Some(Link {
            label: "Create your first note".into(),
            href: "/notes/new".into(),
        }),
        ..Default::default()
    })
    .into_string();
    assert!(output.contains("href=\"/notes/new\""));
    assert!(output.contains("Create your first note"));
}
