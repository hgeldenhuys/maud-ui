# Action row

One density choice for Save, Cancel and optional overflow. Use it within a form or alongside a record. Primary comes first; secondary actions follow; native details/summary contains the rest.

## Import and example
```rust
use maud::html;
use maud_ui::blocks::{action::Action, action_row::{self, Density, Props}};
html! {
    form method="post" action="/bookings/14" {
        input type="hidden" name="csrf" value="server-issued-token";
        // Render the application's fields here.
        (action_row::render(Props {
            primary: Some(Action::submit("Save changes")),
            secondary: vec![Action::link("Cancel", "/bookings/14")],
            density: Density::Compact,
            ..Default::default()
        }))
    }
};
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| primary | Option<action::Action> | None | One primary action. |
| secondary / overflow | Vec<action::Action> | empty | Ordered secondary actions and native disclosure contents. |
| primary_markup / secondary_markup / overflow_markup | Option<Markup> | None | Override the corresponding typed actions, including an explicitly empty fragment. |
| density | Density | Compact | Row, Compact or Comfortable. |
| aria_label | String | Actions | Accessible group name. |
| overflow_label | String | More actions | Native summary text. |

All effective slots empty renders nothing; empty overflow renders no summary. Canonical fragments keep their original form, CSRF, confirmation, pending/result and data attributes. Give their controls the `mui-btn` class to share density. Density rules override child size classes inside this row; keep business behavior in the caller.

## Density and shared Action contract
`Density::Row`: 32px minimum, 12px text and zero block padding. `Compact`: 32px minimum with 14px text. `Comfortable`: 36px minimum. Every density has a 44px coarse-pointer floor, wrapping labels and visible keyboard focus. `Density::as_str()` returns row/compact/comfortable. These values use the existing theme tokens.

`Action { label: String, target: Target }` supports:

- `Action::link(label, href)` → a native anchor.
- `Action::submit(label)` → a button submitting its surrounding form, without creating a nested form.
- `Target::Submit { form: Option<String>, name: Option<String>, value: Option<String> }` → an explicit form owner and/or submitter name/value. All optional values default None in the helper.
- `Target::Post { action: String, hidden_fields: Vec<(String, String)> }` → a separate POST form. Supply CSRF and other server-owned values. Use outside an existing form; use Submit or a canonical submit fragment inside one.

Adding Submit extends the shared public Target enum in 0.10.0. Consumers with exhaustive matches must add a Submit arm. Existing Link/Post output and button Size::Row/Sm APIs remain valid. The shipped bundle now preserves standalone row buttons' zero block padding.

## Accessibility
Native buttons, anchors and details need no JavaScript. Tab traverses controls, Enter activates the focused action, Space toggles the summary. The overflow has disclosure semantics, not an ARIA menu. Native form validation and submission stay intact. Both themes share the same geometry; long actions wrap and coarse pointers retain the larger target.

## Related
[Form](../components/form.md), [record money](record-money.md), [button](../components/button.md).
