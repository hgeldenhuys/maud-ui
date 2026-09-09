# Record money

A currency label, three figures and one line of explanation. Values are already formatted by the caller; this block never subtracts balances, adds taxes, chooses a currency or substitutes zero for unavailable data.

## Import and example
```rust
use maud_ui::blocks::{action::Action, action_row, record::money::{self, Figure, Props}};
money::render(Props {
    currency: "CAD".into(),
    total: Figure::new("Total stay", "$840"),
    paid: Figure::new("Paid", "$280"),
    due: Figure::new("Due at check-in", "$560"),
    breakdown: Some("3 × $240 room nights + $120 taxes & fees".into()),
    actions: action_row::render(action_row::Props {
        primary: Some(Action::link("Review payment", "/bookings/14/payment")),
        ..Default::default()
    }),
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| title | String | Payment | Heading and accessible name. |
| heading | action::Heading | H2 | H1/H2/H3. |
| currency | String | empty | Explicit currency from the supplied money value; empty omits the label. |
| total / paid / due | Figure | labels Total / Paid / Due; value None | Three named values; no calculation. |
| emphasize_due | bool | true | Warning emphasis when the due value exists. Pass false for a settled/refunded balance. |
| unavailable_label | String | Unavailable | Text for a None value; localizable. |
| breakdown | Option<String> | None | Caller-supplied one-line explanation; wraps on small screens. |
| actions | Markup | empty | An action_row or the caller's canonical actions. Empty omits the footer. |

`Figure { label: String, value: Option<Markup> }` defaults to an empty label and None value. `Figure::new(label, value)` escapes a formatted string. For a canonical field fragment, pass `value: Some(field_markup)` directly; preserve its classification, masking and annotations. `Some(html! {})` deliberately remains empty and is not replaced. None is unavailable, while a supplied `$0` is a known zero. Only include fields and actions already admitted for this viewer.

## Accessibility and layout
A definition list gives each figure its label; Due remains explicit in text as well as warning color. The three figures stay together on phones, with smaller type and wrapping for long localized amounts. The same semantic tokens support light and dark themes. No JavaScript is required.

## Related
[Action row](action-row.md), [record timeline](record-timeline.md), [attention banner](attention-banner.md).
