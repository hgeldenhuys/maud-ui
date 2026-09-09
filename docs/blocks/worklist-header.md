# Worklist header

A reusable block with a title, localized count sentence, GET search and one primary action. At phone widths it stacks in reading order.

## Import and example
```rust
use maud_ui::blocks::{action::Action, worklist::header::{self, Props, Search}};
header::render(Props {
    title: "Reservations".into(),
    count_sentence: "48 reservations · 12 arriving today".into(),
    search: Some(Search { action: "/reservations".into(), ..Default::default() }),
    primary_action: Some(Action::link("New reservation", "/reservations/new")),
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| title | String | empty | Visible heading. |
| count_sentence | String | empty | Caller formats counts/plurals/localization; omitted when empty. |
| search | Option<Search> | None | Inline native GET search. |
| primary_action | Option<Action> | None | Exactly one link or POST form. |
| heading | action::Heading | H2 | H1, H2 or H3 for the surrounding page. |

Search fields: `action: String` (empty/current route), `name: String` (q), `value: String` (empty), `label: String` (Search records), `placeholder: String` (Search by name or reference), `hidden_fields: Vec<(String,String)>` (empty). Hidden fields retain filters; remove pagination offsets. The search has a label and submit button and works without JS.

## Actions
`action::Action { label: String, target: Target }` accepts `Target::Link(String)` or `Target::Post { action: String, hidden_fields: Vec<(String,String)> }`. `Action::link(label, href)` is the navigation shortcut. POST forms accept caller-owned CSRF hidden fields. Never nest this block inside another form.

## Composition and accessibility
Place `status_chip_group` and a table after this block. Counts describe the server's actual result scope. Use `Heading::H1` for a page title, H2 for a section. All strings and hidden values are escaped by Maud. Search URLs and actions are supplied by the application.

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.

`State::Absent` (0.11.0) means no input/rule was declared and emits nothing. Keep any caller-owned section heading inside the same conditional. Use Error only for a declared operation that failed; never show an unconfigured-rule message to the user.
