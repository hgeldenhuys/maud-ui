# Record timeline

Three to five dated milestones for a stay, case or delivery. The caller supplies the order and the distinction between actual events, the current step and scheduled events. The layout is horizontal at viewport widths of at least 48rem, including narrow desktop columns, and vertical below 48rem. The frame sizes to its content.

## Import and example
```rust
use maud_ui::blocks::record::timeline::{self, Milestone, Props, State};
timeline::render(Props {
    title: "The stay".into(),
    milestones: vec![
        Milestone { date: "24 Aug".into(), label: "Booked".into(), state: State::Done, ..Default::default() },
        Milestone { date: "Today · 11:30".into(), label: "Arrival review".into(), subline: Some("Juniper 04 is ready".into()), state: State::Current, ..Default::default() },
        Milestone { date: "11 Sep".into(), label: "Check out".into(), state: State::Scheduled, ..Default::default() },
    ],
    ..Default::default()
});
```
`maud_ui::blocks::record::stay_timeline` is an alias of this module, including all types and functions.

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| title | String | Timeline | Section heading and accessible name. |
| heading | action::Heading | H2 | H1, H2 or H3 for the surrounding hierarchy. |
| milestones | Vec<Milestone> | empty | Caller order; empty renders nothing. Designed for 3–5; smaller or longer sets remain valid. |

`Milestone` fields: `date: String` and `label: String` default empty; `datetime: Option<String>` supplies an optional ISO date/datetime for `<time>` (otherwise the visible date is a span); `subline: Option<String>` supplies supporting copy; `state: State` defaults Scheduled; `state_label: Option<String>` replaces the visible state label for localization. Optional fields default None. Strings are escaped.

## States and helpers
`State::{Done, Current, Scheduled}`; `State::as_str()` supplies the lowercase data attribute and `State::label()` supplies the English text. A timeline may have no current step; passing multiple Current milestones panics to reject ambiguous progress. Dates do not determine states or reorder events.

## Accessibility
An ordered list, visible state text and a check for Done accompany color. The current step alone has `aria-current="step"`. Markers are decorative. No JavaScript or interactive step navigation; actions belong in the record header. Below a 48rem viewport the same list becomes vertical, retaining reading order in light and dark themes.

## Related
[Record header](record-header.md), [money](record-money.md), [related entity](record-related-card.md).

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.

`State::Absent` (0.11.0) means no input/rule was declared and emits nothing. Keep any caller-owned section heading inside the same conditional. Use Error only for a declared operation that failed; never show an unconfigured-rule message to the user.
