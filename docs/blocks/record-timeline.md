# Record timeline

Three to five dated milestones for a stay, case or delivery. The caller supplies the order and the distinction between actual events, the current step and scheduled events. The layout changes from horizontal to vertical when its container is 40rem or narrower.

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
An ordered list, visible state text and a check for Done accompany color. The current step alone has `aria-current="step"`. Markers are decorative. No JavaScript or interactive step navigation; actions belong in the record header. At 40rem the same list becomes vertical, retaining reading order in light and dark themes.

## Related
[Record header](record-header.md), [money](record-money.md), [related entity](record-related-card.md).
