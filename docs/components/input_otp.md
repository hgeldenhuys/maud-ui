# InputOTP

One-time password input with grouped, individually focusable slots supporting digits, alphanumeric, and custom regex patterns.

## Import

```rust
use maud_ui::primitives::input_otp::{self, Props, OtpPattern};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::input_otp;

html! {
    (input_otp::render(input_otp::Props {
        length: 6,
        group_size: 3,
        name: "verification-code".to_string(),
        id: "otp".to_string(),
        pattern: input_otp::OtpPattern::Digits,
        aria_invalid: false,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `length` | `usize` | `6` | Total number of input slots. |
| `group_size` | `usize` | `3` | Slots per group before visual separator (0 = no separator). |
| `name` | `String` | `"otp"` | HTML name attribute for the hidden value input. |
| `id` | `String` | `"otp"` | HTML id prefix for each slot (id-0, id-1, etc.). |
| `disabled` | `bool` | `false` | Disables all slots. |
| `pattern` | `OtpPattern` | `Digits` | Accepted character set per slot. |
| `aria_invalid` | `bool` | `false` | Sets `aria-invalid="true"` on each slot for error announcement. |

## OtpPattern

```rust
pub enum OtpPattern {
    Digits,                    // [0-9], inputmode="numeric"
    DigitsAndChars,           // [0-9A-Za-z], inputmode="text"
    Custom(String),           // Custom regex, inputmode="text"
}
```

## Slots

Each slot renders as a single-character `<input>` with:

- `maxlength="1"` — prevents multi-char entry.
- `pattern=".."` — HTML5 validation.
- `autocomplete="one-time-code"` — browser password manager integration.
- `aria-label="Digit N"` — ordinal labeling.
- `aria-invalid=[Some("true")]` — when `aria_invalid: true`.

A hidden input with `name="otp"` collects the final value.

## Grouping

Visual separator (`<div class="mui-input-otp__separator">`) inserted before every `group_size`-th slot. Set `group_size=0` to disable grouping.

## Accessibility

- Container has `role="group"` + `aria-label="One-time password"`.
- Each slot labeled with `aria-label="Digit N"` for screen readers.
- Separator has `role="separator"`.
- `aria_invalid: true` announces validation failure without native form submission.

## Related

- [Input](input.md) — single text input.
- [Field](field.md) — wrapper with label/description/error.

## Shadcn Reference

Mirrors shadcn's InputOTP with OtpPattern enum (Digits, DigitsAndChars, Custom) and slot-based rendering.
