# Record header

Title and status, one muted kind/reference line, then the main verb and native More disclosure. Put collection navigation in the shell breadcrumb.

## Import and example
```rust
use maud_ui::blocks::{action::{Action, Heading}, record::header::{self, Props}};
header::render(Props {
    title: "Sofia Patel".into(), kind: Some("Guest".into()), reference: Some("UI-G5".into()), heading: Heading::H1,
    primary_action: Some(Action::link("Edit guest", "/ui/guests/UI-G5/edit")),
    secondary_actions: vec![Action::link("Graph view", "/ui/guests/UI-G5/graph")],
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| title | String | empty | Identity heading. |
| title_markup | Option<Markup> | None | Overrides title. Rendered in a div so a supplied field-emitter heading is never nested in another heading. |
| subtitle | Option<String> | None | Legacy single-line fallback when kind/reference are absent. |
| kind | Option<String> | None | Human-facing kind, joined to reference with one middle dot. |
| reference | Option<String> | None | Reference shown once in the identity line. |
| status | Option<badge::Props> | None | Text and semantic tone. |
| status_markup | Option<Markup> | None | Overrides status, including an explicitly empty fragment. |
| primary_action | Option<action::Action> | None | One named link or native POST action. |
| secondary_actions | Vec<action::Action> | empty | Quiet actions in a native disclosure. |
| back | Option<action::Link> | None | Legacy navigation link, outside actions. Prefer a shell breadcrumb preserving collection filters. |
| heading | action::Heading | H2 | H1/H2/H3 for composition. |

## Actions and accessibility
For a bound field heading, pass `title_markup: Some(html! { h1 { (record.name) } })` and choose its heading level in the caller. Plain `title` is escaped and uses `heading`; the markup slot takes precedence. The same precedence applies to a field-emitted status in `status_markup`.

The [shared Action contract](worklist-header.md) supports navigation and POST with hidden fields. Empty secondary groups are omitted; no client script is needed. Status text supplies meaning beyond color. Long titles and action groups wrap. The back link comes first in reading order, and the record header never creates a page or main landmark of its own.

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.

`State::Absent` (0.11.0) means no input/rule was declared and emits nothing. Keep any caller-owned section heading inside the same conditional. Use Error only for a declared operation that failed; never show an unconfigured-rule message to the user.

## Record-page hierarchy (0.13.0)

The action row follows identity vertically at every viewport. Use the record's main verb as `primary_action`; put developer views such as Graph view in `secondary_actions`, behind More. Never put Back to list in these actions. `kind` and `reference` replace `subtitle` when either has nonblank content; an all-blank identity falls back to a nonblank subtitle. There is one muted paragraph, not a second profile-view description. Use [Record page](record-page.md) to own the gap to subsequent facts and collections. Existing title/status markup overrides and native action/form ownership remain unchanged.
