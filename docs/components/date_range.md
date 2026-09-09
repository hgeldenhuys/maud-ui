# Date range

Two native dates presented as one range, with a calendar-night count. Both fields submit without JavaScript. The server-rendered count reflects the initial values; enhancement keeps it current while editing.

## Import
```rust
use maud_ui::primitives::date_range::{self, DateInput, Props};
```

## Example
```rust
use maud_ui::primitives::date_range::{self, DateInput, Props};
date_range::render(Props {
    id: "booking-dates".into(), legend: "Stay dates".into(), required: true,
    start: DateInput { name: "arrival".into(), label: "Arrival".into(), value: "2026-09-08".into(), min: Some("2026-09-01".into()), ..Default::default() },
    end: DateInput { name: "departure".into(), label: "Departure".into(), value: "2026-09-11".into(), max: Some("2027-09-01".into()), ..Default::default() },
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| id | String | mui-date-range | Unique fieldset ID; prefixes both input IDs and the output ID. |
| legend | String | Stay dates | Accessible group name. |
| start | DateInput | name start; label Arrival; empty value | Arrival/start field. |
| end | DateInput | name end; label Departure; empty value | Departure/end field. |
| required | bool | false | Native required attribute on both fields. |
| disabled | bool | false | Disables the fieldset and both fields. |
| read_only | bool | false | Native readonly on both fields; values remain submitted. |
| form | Option<String> | None | Explicit form owner ID for both fields. |
| night_label / nights_label | String | night / nights | Localized singular/plural count suffix. |
| incomplete_label | String | Choose both dates | Summary when either value is empty. |
| invalid_label | String | Departure must be after arrival, within the allowed dates. | Summary and enhanced interval validation message. |
| no_script_hint | String | Duration reflects the initial dates. Submit the form to update it. | Visible only without scripting. |

`DateInput { name: String, label: String, value: String, min: Option<String>, max: Option<String> }` defaults to empty strings and None bounds. All date values and bounds use `YYYY-MM-DD`; display is localized by the browser. Native bounds are per field; malformed bounds are ignored. Supply distinct names and one unique component ID per range.

## Variants / Enums
No variants. Incomplete, invalid and valid states are explicit. A valid interval has a strictly later departure and both values within their supplied bounds. A same-day interval is invalid and never displayed as a zero-night stay.

## Helper Functions
`nights_between(start: &str, end: &str) -> Option<u32>` counts Gregorian calendar nights for years 0001–9999. Returns None for malformed, impossible, same-day or reversed dates. It uses ordinal dates, so daylight-saving transitions cannot alter the count. This helper checks the interval only; bounds remain a separate validation concern.

## Accessibility and progressive enhancement
Native `input type="date"` controls provide keyboard editing and the platform date picker. Labels, fieldset/legend and a shared `aria-describedby` name the fields and associate the output. The count is a polite live region; invalid intervals set `aria-invalid`. Enhancement listens to input/change and updates after native form reset, including external form owners and newly inserted HTMX content. It adds custom validity to the departure for invalid intervals and clears it on correction. Readonly/disabled fields are not blocked by enhanced validation.

With JavaScript disabled, native required/min/max still apply; the initial night count stays static and a no-script hint explains it. Submit to the application to refresh the count. The server must always validate interval order, bounds and availability. No custom calendar traps focus or replaces native editing. At a container width of 30rem, the two fields stack; both light and dark themes retain the same behavior.

## Related
[Choice cards](choice_card.md), [date picker](date_picker.md), [form](form.md), [action row](../blocks/action-row.md).

## Shadcn reference
Application-specific pair of native date inputs; no direct upstream equivalent.
