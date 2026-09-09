# Grouped worklist

Time or state headings over compact operational rows. At 40rem or narrower each row becomes a card using the same DOM, order, identity and actions.

## Import and example
```rust
use maud_ui::{blocks::{action::Action, worklist::grouped::{self, Group, Props, Row}}, primitives::{badge, facts_list::Fact}};
grouped::render(Props {
    aria_label: "Today's arrivals".into(),
    groups: vec![Group {
        label: "11:30 · 1 arriving".into(),
        rows: vec![Row {
            identity: "Maya Chen".into(), subline: "CL-260908-014 · Juniper 04".into(),
            facts: [Fact::new("Guests", "2 adults"), Fact::new("Payment", "CAD $560 due")],
            status: badge::Props { label: "Room ready".into(), variant: badge::Variant::Success, ..Default::default() },
            action: Some(Action::link("Review check-in", "/bookings/14/check-in")),
            ..Default::default()
        }],
        ..Default::default()
    }],
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| aria_label | String | Grouped worklist | Region name. |
| group_heading | action::Heading | H3 | H1/H2/H3 for each group heading. |
| groups | Vec<Group> | empty | Caller-defined and caller-ordered. Empty groups are omitted. |
| empty_message | String | No items to show. | Shown when no group has rows. |
| footer | Option<String> | None | Optional timezone, freshness or partial-result explanation; also retained when empty. |

`Group` fields: `label: String` defaults empty, `subline: Option<String>` defaults None, `rows: Vec<Row>` defaults empty. Supply the correctly scoped count as part of the label. The block does not count a page of rows and call it a total.

`Row` fields:

| Field | Type | Default | Description |
|---|---|---|---|
| identity / subline | String | empty | Primary identity and supporting record reference. |
| href | Option<String> | None | Optional native identity link. |
| identity_markup | Option<Markup> | None | Overrides identity and href, preserving a canonical entity link. |
| facts | [facts_list::Fact; 2] | two empty facts | Exactly two named facts; format unknown values explicitly. |
| facts_markup | Option<Markup> | None | Overrides the facts region, including empty. Supply two canonical fields in a `mui-facts` definition list to retain the layout. |
| status | badge::Props | badge default | Named state and tone. |
| status_markup | Option<Markup> | None | Overrides badge, including empty. |
| action | Option<action::Action> | None | One compact action. |
| action_markup | Option<Markup> | None | Overrides action, including empty; retain required action attributes. |
| note | Option<String> | None | A supporting note below the row's main facts. |

## Accessibility and data
Semantic sections and lists preserve identity → facts → status → action → note order on desktop and phone. Status meaning is textual. Native links and forms work without JavaScript; no duplicate mobile controls or click-anywhere rows. Strings are escaped, markup slots are trusted caller fragments. Use only admitted data; the component never derives ETA, readiness, balances or business state. Colors follow the active light/dark theme.

## Related
[Worklist header](worklist-header.md), [attention banner](attention-banner.md), [record header](record-header.md).

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.
