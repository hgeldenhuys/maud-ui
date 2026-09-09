# Related entity card

Small context beside a record: a guest, room, account or asset with two to four facts and one action. Auto uses a compact inline row when effective facts are empty; cards with facts stack naturally on phones.

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
| variant | Variant | Auto | Auto selects Inline without facts and Card with facts; Card/Inline force either layout; Compact keeps supplied facts in a dense card. |
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
The plain title uses the requested heading level. `title_markup` is wrapped in a div so a supplied heading is never nested in another heading; choose its level in the caller. Facts use a definition list. The sole action follows the facts in reading order, with a full-width target in Card and a content-width action in Inline; coarse pointers keep a height of at least 44px. Long values wrap in both themes. No JavaScript is required.

## Related
[Facts list](../components/facts_list.md), [record timeline](record-timeline.md), [choice cards](../components/choice_card.md).

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.

## Compact facts (0.11.0)
`Variant::Compact` keeps admitted facts and the sole action together in a denser card. Explicit Inline also retains supplied facts. Add Compact to exhaustive Variant matches. Render guest contact and room facts here once; omit duplicate overview rows in the consuming application.

`State::Absent` (0.11.0) means no input/rule was declared and emits nothing. Keep any caller-owned section heading inside the same conditional. Use Error only for a declared operation that failed; never show an unconfigured-rule message to the user.
