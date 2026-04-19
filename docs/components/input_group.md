# InputGroup

Input with configurable prefix/suffix and full-width addons (buttons, text, hints) as a visual unit.

## Import

```rust
use maud_ui::primitives::input_group::{self, InputGroupProps, Align};
use maud_ui::primitives::input;
```

## Example

```rust
use maud::html;
use maud_ui::primitives::input_group;

html! {
    div.mui-input-group {
        (input_group::text(html! { "https://" }))
        (input_group::input_el(input::Props {
            placeholder: "example.com".into(),
            ..Default::default()
        }))
        (input_group::button("Go", button::Size::Sm, button::Variant::Primary))
    }
}
```

## Props (InputGroupProps)

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `prefix` | `Option<Markup>` | `None` | Content before the input. |
| `suffix` | `Option<Markup>` | `None` | Content after the input. |
| `children` | `Markup` | `html! {}` | Main row content. |

## Align

```rust
pub enum Align {
    InlineStart,   // Leading (left in LTR), order: -1
    InlineEnd,     // Trailing (right in LTR), order: 99
    BlockStart,    // Above row (full-width), order: -2
    BlockEnd,      // Below row (full-width), order: 100
}
```

## Helpers

- **`addon(children, align)`** — Render markup as an addon at the specified alignment.
- **`button(label, size, variant)`** — Sized button in InlineEnd addon (default). Common for password reveal, search submit, copy.
- **`text(children)`** — Muted supplementary text in InlineStart addon (default). Common for currency prefix, `@`, `https://`.
- **`input_el(props)`** — Render Input control with `data-slot="input-group-control"` for shadcn-compatible selectors.
- **`textarea(props)`** — Render Textarea control with same slot marker.

## Accessibility

- Addons use semantic `<div>` and `<span>` wrappers; buttons render as true `<button>` elements.
- Text addons lack interactive semantics; place them outside form submission flow.
- Input retains all label/error linkage when wrapped in a group.

## Related

- [Input](input.md) — single text input.
- [Button](button.md) — standalone button.
- [Textarea](textarea.md) — multi-line textarea for groups.

## Shadcn Reference

Mirrors shadcn Base UI's InputGroup with Align enum (InlineStart/InlineEnd/BlockStart/BlockEnd) and helper composition (addon, text, button, input_el, textarea).
