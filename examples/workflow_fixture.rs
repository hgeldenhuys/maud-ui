//! Real-browser fixtures. Fake persistence is supplied only by workflow-browser.mjs.
use maud::{html, Markup, PreEscaped, DOCTYPE};
use maud_ui::{
    blocks::{
        action::{Heading, Link},
        feedback::{confirm, notice},
        search::results,
        shell::{page_header, sidebar},
        wizard::header,
        worklist::empty,
    },
    primitives::{breadcrumb::BreadcrumbItem, field, form, native_select, radio_group},
};

fn guest_field(id: &str, label: &str, kind: &str, description: Option<&str>) -> Markup {
    field::render(field::Props {
        id: id.into(),
        label: label.into(),
        required: true,
        description: description.map(Into::into),
        children: html! { input class="mui-input" id=(id) name=(id) type=(kind) required aria-describedby=[if id == "email" { Some("privacy") } else { None }]; },
        ..Default::default()
    })
}
fn contents() -> Markup {
    html! {
        div class="mui-workflow-fixture mui-stack" {
            h1 { "Arrival desk" }
            section id="guest-section" class="mui-stack" style="max-width:24rem;" {
                h2 { "Create a guest" }
                (form::render(form::Props { id: Some("guest-form".into()), action: Some("/fixtures/save".into()), aria_label: Some("Create a guest".into()), feedback: true, pending_label: "Saving guest…".into(), children: html! {
                    div class="mui-stack" {
                        (guest_field("name", "Full name", "text", None))
                        (guest_field("email", "Email", "email", Some("Use the guest’s contact email.")))
                        p id="privacy" class="mui-sr-only" { "Used only for this guest’s stay." }
                        (field::render(field::Props { id: "method".into(), label: "Payment method".into(), required: true, children: html! {
                            (native_select::render(native_select::NativeSelectProps { id: "method".into(), name: "method".into(), required: true, placeholder: Some("Choose a method".into()), options: vec![native_select::NativeOption { value: "Cash".into(), label: "Cash".into(), disabled: false }, native_select::NativeOption { value: "BankTransfer".into(), label: "Bank transfer".into(), disabled: false }], ..Default::default() }))
                        }, ..Default::default() }))
                        (radio_group::render(radio_group::Props { name: "room".into(), label: "Preferred room".into(), selected: None, required: true, orientation: radio_group::Orientation::Horizontal, options: vec![radio_group::RadioOption { value: "garden".into(), label: "Garden suite".into(), description: None }, radio_group::RadioOption { value: "terrace".into(), label: "Terrace room".into(), description: None }], ..Default::default() }))
                        (notice::region("guest-result"))
                        div class="mui-confirm__actions" {
                            button id="save-guest" type="submit" class="mui-btn mui-btn--primary" name="intent" value="create" aria-disabled="false" { span { "Save guest" } }
                            button id="cancel-edit" type="reset" class="mui-btn mui-btn--outline" { "Cancel" }
                            button id="locked-submit" type="submit" hidden disabled { "Unavailable action" }
                        }
                    }
                }, ..Default::default() }))
            }
            section class="mui-stack" {
                h2 { "Review a check-in" }
                (confirm::trigger("check-in", "Review check-in"))
                (confirm::render(confirm::Props { id: "check-in".into(), title: "Check in Leila Morgan?".into(), message: "Garden suite · 10–11 September. Mark this booking as checked in.".into(), confirm_label: "Check in guest".into(), pending_label: "Checking in…".into(), cancel_label: "Keep booking".into(), action: "/fixtures/check-in".into(), hidden_fields: vec![("booking".into(), "B-410".into())], ..Default::default() }))
                (notice::region("check-in-result"))
            }
            section class="mui-stack" id="wizard" {
                (header::render(header::Props { context: Some("Leila Morgan · Garden suite · 10–11 September".into()), steps: vec!["Select booking".into(), "Verify guest".into(), "Check in".into()], current: 2, heading: Heading::H2, ..Default::default() }))
                h3 id="step-title" tabindex="-1" { "Verify Leila’s details" }
                p { "Confirm the guest’s name and arrival details before check-in." }
                button id="next-step" class="mui-btn mui-btn--primary" type="button" { "Guest verified" }
            }
            (results::render(results::Props { query: "Leila".into(), items: vec![results::Item { kind: "Guest".into(), title: "Leila Alexandra Morgan-Smith".into(), description: Some("leila.alexandra.morgan.smith@example.com".into()), reference: Some("G-204".into()), href: "#guest-section".into() }], ..Default::default() }))
            (results::render(results::Props { id: "no-results".into(), query: "Unknown guest".into(), clear_action: Some(Link { label: "Clear search".into(), href: "#search-results".into() }), ..Default::default() }))
            (empty::render(empty::Props { title: "No notes yet".into(), description: Some("Keep arrival requests and handover details together.".into()), create: Some(Link { label: "Create your first note".into(), href: "#new-note".into() }), ..Default::default() }))
            section id="new-note" tabindex="-1" { h2 { "New note" } label for="note" { "Note" } textarea id="note" class="mui-textarea" {} }
        }
    }
}
fn document(brand: &str, density: &str) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="light" data-brand=(brand) data-density=(density) {
            head { meta charset="utf-8"; meta name="viewport" content="width=device-width, initial-scale=1"; title { "Workflow fixtures" } link rel="stylesheet" href="/css/maud-ui.css"; }
            body {
                (sidebar::render(sidebar::Props { id: "workflow-shell".into(), brand: html! { "Lodge Operations" }, nav_groups: vec![sidebar::NavGroup { label: Some("Reservations".into()), items: vec![sidebar::NavItem { label: "Guests".into(), href: "#guest-section".into(), ..Default::default() }, sidebar::NavItem { label: "Notes".into(), href: "#new-note".into(), ..Default::default() }] }], page_header: Some(page_header::Props { breadcrumbs: vec![BreadcrumbItem { label: "Arrival desk".into(), href: None }], ..Default::default() }), children: contents(), ..Default::default() }))
                script src="/js/maud-ui.js" {}
                script { (PreEscaped(include_str!("workflow-fixture.js"))) }
            }
        }
    }
}
fn main() {
    let check = std::env::args().any(|arg| arg == "--check");
    for brand in ["lodge", "neutral"] {
        for density in ["compact", "comfortable", "spacious"] {
            let path = format!("docs/fixtures/workflow-{brand}-{density}.html");
            let output = document(brand, density).into_string() + "\n";
            if check {
                assert_eq!(
                    std::fs::read_to_string(&path).unwrap(),
                    output,
                    "Stale {path}"
                );
            } else {
                std::fs::write(path, output).unwrap();
            }
        }
    }
    println!(
        "{} six workflow fixtures.",
        if check { "Verified" } else { "Generated" }
    );
}
