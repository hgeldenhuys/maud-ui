# Related entity card

Small context beside a record: a guest, room, account or asset with two to four facts and one action. Cards stack naturally on phones.

## Import and example
```rust
use maud_ui::{blocks::{action::Action, record::related_card::{self, Props}}, primitives::facts_list::Fact};
related_card::render(Props {
    title: "Juniper 04".into(), subtitle: Some("Garden room · Ready".into()),
    facts: vec![Fact::new("Room", "Queen bed · Ground floor"), Fact::new("Capacity", "2 adults")],
    action: Some(Action::link("View room", "/rooms/4")),
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| title | String | empty | Escaped entity title. |
| title_markup | Option<Markup> | None | Overrides title, including an explicitly empty fragment. |
| subtitle | Option<String> | None | Supporting identity/state. |
| facts | Vec<facts_list::Fact> | empty | Usually 2–4 facts; caller controls order. |
| facts_markup | Option<Markup> | None | Overrides facts with canonical field fragments. |
| action | Option<action::Action> | None | One navigation, submit or native POST action. |
| action_markup | Option<Markup> | None | Overrides action; supply one canonical action. |
| heading | action::Heading | H2 | Heading used for a plain title. |

`Fact { label: String, value: String, mono: bool }` has empty labels/values and false mono by default; `Fact::new(label, value)` escapes both. `Fact::mono(label, value)` opts into monospace. The markup overrides preserve caller-owned field and action contracts; an empty override suppresses the typed value. No data lookup or readiness inference occurs here.

## Accessibility and layout
The plain title uses the requested heading level. `title_markup` is wrapped in a div so a supplied heading is never nested in another heading; choose its level in the caller. Facts use a definition list. The sole action follows the facts in reading order, with a full-width target and a coarse-pointer height of at least 44px. Long values wrap in both themes. No JavaScript is required.

## Related
[Facts list](../components/facts_list.md), [record timeline](record-timeline.md), [choice cards](../components/choice_card.md).
