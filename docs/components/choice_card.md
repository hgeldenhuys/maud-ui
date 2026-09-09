# Choice card

Native radio cards for choosing one room, plan or delivery option. The entire card labels its radio. No JavaScript is required for keyboard selection or form submission.

## Import
```rust
use maud_ui::primitives::choice_card::{self, Choice, Props};
```

## Example
```rust
use maud_ui::primitives::choice_card::{self, Choice, Props};
choice_card::render(Props {
    id: "booking-room".into(), name: "room_id".into(), legend: "Choose a room".into(),
    selected: Some("4".into()), required: true,
    choices: vec![
        Choice { value: "4".into(), title: "Juniper 04".into(), facts: vec!["Queen bed · Garden terrace".into()], price: Some("CAD $240 / night".into()), ..Default::default() },
        Choice { value: "12".into(), title: "Cedar 12".into(), disabled_reason: Some("Booked for these dates".into()), ..Default::default() },
    ],
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| id | String | mui-choice-card | Unique fieldset ID; prefixes radio and reason IDs. |
| name | String | choice | Shared native radio field name; must be unique to this group within its form. |
| legend | String | Choose an option | Visible, accessible group name. |
| choices | Vec<Choice> | empty | Ordered caller-supplied choices. |
| selected | Option<String> | None | Value to check; first matching enabled choice wins. Unknown/disabled selection checks none. |
| required | bool | false | Native required radio group. |
| disabled | bool | false | Disables the whole fieldset and its inputs. |
| form | Option<String> | None | Form owner ID when rendered outside the form. |

`Choice` fields: `value: String` and `title: String` default empty; `facts: Vec<String>` defaults empty; `price: Option<String>` and `disabled_reason: Option<String>` default None. Price includes its currency and unit, supplied by the caller. A disabled reason both disables the radio and displays its explanation; use a meaningful nonempty reason. Values, labels, facts and prices are escaped. Supply stable, unique values; duplicate values never produce multiple checked radios in server markup.

## Variants / Enums
None. Native checked, required and disabled states govern the cards. The caller supplies availability; choosing a card does not reserve inventory or calculate a quote.

## Helper Functions
None beyond `render(Props)` and `showcase()`.

## Accessibility
Fieldset/legend name the group; visible native radio controls provide semantics and state even without CSS. Tab enters the group, arrow keys change the checked radio and skip disabled options, Space selects the focused option. The native browser owns keyboard behavior. The disabled explanation is associated with `aria-describedby` and remains readable in document flow. Full-card labels provide large targets; focus outlines and checked borders supplement color, including forced-colors mode. Cards reflow into one column at a container width of 30rem. Both themes use the same structure.

Give each rendered picker a distinct id and each independent group a distinct name or form owner. Revalidate the submitted value and availability on the server.

## Related
[Radio group](radio_group.md), [date range](date_range.md), [action row](../blocks/action-row.md).

## Shadcn reference
Application-specific native-radio composition; no direct upstream equivalent.
