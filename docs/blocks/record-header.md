# Record header

Identity, status, a back link, one primary action and a secondary disclosure. The primary action remains visible; More actions uses native details/summary.

## Import and example
```rust
use maud_ui::blocks::{action::{Action, Link}, record::header::{self, Props}};
header::render(Props {
    title: "Sofia Davis".into(), subtitle: Some("Reservation RS-2048 · Garden suite".into()),
    primary_action: Some(Action::link("Check in", "/reservations/2048/check-in")),
    secondary_actions: vec![Action::link("Edit reservation", "/reservations/2048/edit")],
    back: Some(Link { label: "Reservations".into(), href: "/reservations".into() }),
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| title | String | empty | Identity heading. |
| title_markup | Option<Markup> | None | Overrides title. Rendered in a div so a supplied field-emitter heading is never nested in another heading. |
| subtitle | Option<String> | None | Reference, supporting identity or dates. |
| status | Option<badge::Props> | None | Text and semantic tone. |
| status_markup | Option<Markup> | None | Overrides status, including an explicitly empty fragment. |
| primary_action | Option<action::Action> | None | One named link or native POST action. |
| secondary_actions | Vec<action::Action> | empty | Quiet actions in a native disclosure. |
| back | Option<action::Link> | None | Link has label: String and href: String. Preserve the collection's filters here. |
| heading | action::Heading | H2 | H1/H2/H3 for composition. |

## Actions and accessibility
For a bound field heading, pass `title_markup: Some(html! { h1 { (record.name) } })` and choose its heading level in the caller. Plain `title` is escaped and uses `heading`; the markup slot takes precedence. The same precedence applies to a field-emitted status in `status_markup`.

The [shared Action contract](worklist-header.md) supports navigation and POST with hidden fields. Empty secondary groups are omitted; no client script is needed. Status text supplies meaning beyond color. Long titles and action groups wrap. The back link comes first in reading order, and the record header never creates a page or main landmark of its own.
