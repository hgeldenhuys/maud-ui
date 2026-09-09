//! Shared, explicitly fictional workspace used by the landing and shell preview.
use crate::{
    blocks::{
        action::{Action, Heading},
        record, task, worklist,
    },
    primitives::{badge, dialog, status_chip_group as chips, table},
};
use maud::{html, Markup};

pub(super) fn content(id: &str) -> Markup {
    let record_id = format!("{id}-record");
    let new_id = format!("{id}-new");
    let form_id = format!("{id}-new-form");
    html! {
        div class="mui-workspace-example mui-stack" data-mui="workspace-demo" {
            (worklist::header::render(worklist::header::Props {
                title: "Reservations".into(), count_sentence: "4 reservations · Tuesday, 8 September".into(),
                search: Some(worklist::header::Search { action: format!("#{id}"), label: "Find a guest".into(), placeholder: "Find a guest…".into(), ..Default::default() }),
                primary_action: Some(Action::link("New reservation", format!("#{new_id}"))), ..Default::default()
            }))
            (chips::render(chips::Props {
                items: [("All",4,chips::Tone::Neutral),("Arriving",2,chips::Tone::Info),("Checked in",1,chips::Tone::Success),("Needs review",1,chips::Tone::Warning)]
                    .into_iter().enumerate().map(|(i,(label,count,tone))| chips::Chip { label: label.into(), count: Some(count), tone, href: format!("#{id}-filter-{i}") }).collect(),
                ..Default::default()
            }))
            div class="mui-workspace-example__table" id=(format!("{id}-list")) {
                @for i in 0..4 { span class="mui-workspace-example__anchor" id=(format!("{id}-filter-{i}")) {} }
                (table::render(table::Props {
                    headers: ["Guest", "Room", "Status", "Nights", ""].into_iter().map(str::to_string).collect(),
                    right_align_cols: vec![3], hide_cols_sm: vec![1,3], hoverable: true,
                    caption: Some("Reservations in this example".into()),
                    rich_rows: [
                        ("Amira Khan", "Garden suite", "Arriving", badge::Variant::Info, "4", "RS-2048"),
                        ("Theo Martin", "Courtyard room", "Arriving", badge::Variant::Info, "2", "RS-2049"),
                        ("Lina Chen", "Terrace suite", "Checked in", badge::Variant::Success, "3", "RS-2046"),
                        ("Jonas Nielsen", "Garden room", "Needs review", badge::Variant::Warning, "5", "RS-2050"),
                    ].into_iter().map(|(guest,room,status,tone,nights,reference)| vec![
                        table::CellMarkup::markup(html! { span class="mui-workspace-example__guest" data-guest=(guest) data-reference=(reference) data-status=(status) data-room=(room) { (guest) } span class="mui-workspace-example__reference" { (reference) } }, false),
                        table::CellMarkup::text(room), table::CellMarkup::markup(badge::render(badge::Props { label: status.into(), variant: tone, ..Default::default() }), false),
                        table::CellMarkup::right(nights), table::CellMarkup::markup(html! { a class="mui-btn mui-btn--ghost mui-btn--row" href=(format!("#{record_id}")) data-demo-view aria-label=(format!("View {guest}'s reservation")) { "View" } }, false),
                    ]).collect(), ..Default::default()
                }))
                p class="mui-workspace-example__empty" hidden { "No guests match. Try another name or choose All." }
            }
            section class="mui-workspace-example__record" id=(&record_id) tabindex="-1" aria-label="Selected reservation" {
                (record::header::render(record::header::Props {
                    title: "Amira Khan".into(), title_markup: Some(html! { h3 data-demo-title { "Amira Khan" } }),
                    subtitle: Some("RS-2048 · Garden suite · 8–12 September".into()),
                    status_markup: Some(html! { span data-demo-status { (badge::render(badge::Props { label: "Arriving".into(), variant: badge::Variant::Info, ..Default::default() })) } }),
                    heading: Heading::H3, secondary_actions: vec![Action::link("Explore the record header", "/blocks/record-header")], ..Default::default()
                }))
                p class="mui-workspace-example__note" { "A little context, right where the next decision happens." }
            }
            (task::grid::render(task::grid::Props {
                tasks: vec![
                    task::grid::Task { title: "Plan a stay".into(), description: "Add a reservation to this example.".into(), action: Action::link("New reservation", format!("#{new_id}")) },
                    task::grid::Task { title: "Prepare arrivals".into(), description: "See who's arriving and their room.".into(), action: Action::link("Show arrivals", format!("#{id}-filter-1")) },
                    task::grid::Task { title: "Build your workspace".into(), description: "Use this shell in your own app.".into(), action: Action::link("View the recipe", "/blocks/shell-sidebar") },
                ], ..Default::default()
            }))
            p class="mui-workspace-example__feedback" role="status" aria-live="polite" { "Example data · Changes stay on this page." }
            (dialog::render(dialog::Props {
                id: new_id, title: "New reservation".into(), description: Some("Try the form. This adds a guest to the example on this page.".into()),
                children: html! {
                    form id=(&form_id) class="mui-workspace-example__form" method="dialog" data-demo-create {
                        label { "Guest name" input class="mui-input" name="guest" required maxlength="80" autocomplete="off" placeholder="Guest name"; }
                        label { "Room" select class="mui-native-select" name="room" { option { "Garden suite" } option { "Courtyard room" } option { "Terrace suite" } } }
                        label { "Nights" input class="mui-input" type="number" name="nights" value="2" min="1" max="30" required; }
                    }
                },
                footer: Some(html! { button class="mui-btn mui-btn--primary" type="submit" form=(form_id) { "Add to example" } }), ..Default::default()
            }))
        }
    }
}
