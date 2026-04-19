# Textarea

Multi-line text input field with resize control, validation states, and screen-reader support.

## Description

Textarea renders an HTML `<textarea>` element with configurable row height, resize behavior, and form validation states. The component distinguishes between visual invalid state (red border) and semantic ARIA invalid state (for form frameworks that validate independently).

## Import

```rust
use maud_ui::primitives::textarea;
```

## Example

```rust
use maud_ui::primitives::textarea::{Props, Resize, render};

let feedback = render(Props {
    name: "feedback".into(),
    id: "feedback-field".into(),
    placeholder: "What's on your mind?".into(),
    rows: 5,
    ..Default::default()
});

let invalid_field = render(Props {
    name: "bio".into(),
    aria_invalid: true,
    ..Default::default()
});
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `name` | `String` | `""` | Form field name |
| `placeholder` | `String` | `""` | Placeholder text |
| `value` | `String` | `""` | Initial text content |
| `rows` | `u32` | `3` | Visible row height |
| `id` | `String` | `""` | Unique identifier |
| `disabled` | `bool` | `false` | Whether the field is disabled |
| `required` | `bool` | `false` | Whether the field is required (adds HTML `required` attribute) |
| `invalid` | `bool` | `false` | Visual invalid state (red border styling) |
| `aria_invalid` | `bool` | `false` | Semantic invalid state (emits `aria-invalid="true"`) |
| `readonly` | `bool` | `false` | Whether the field is read-only |
| `resize` | `Resize` | `Vertical` | Resize behavior |

## Resize Enum

```rust
pub enum Resize {
    None,                  // No resizing allowed
    Vertical,              // Resize vertically only
    Horizontal,            // Resize horizontally only
    Both,                  // Resize in both directions
}
```

Controls CSS `resize` behavior for the textarea.

## Variants

### Standard Textarea
Default multi-line input with vertical resize.

```rust
render(Props {
    name: "feedback".into(),
    id: "feedback-field".into(),
    placeholder: "Tell us what you think".into(),
    rows: 4,
    ..Default::default()
})
```

### Disabled
Prevents user input and shows disabled styling.

```rust
render(Props {
    name: "archived".into(),
    disabled: true,
    value: "This account has been archived".into(),
    ..Default::default()
})
```

### Invalid (Visual)
Shows red border and invalid styling for validation feedback.

```rust
render(Props {
    name: "bio".into(),
    invalid: true,
    ..Default::default()
})
```

### Aria Invalid (Semantic)
Sets `aria-invalid="true"` for screen readers without triggering visual styling. Used when form frameworks validate independently.

```rust
render(Props {
    name: "release-notes".into(),
    aria_invalid: true,
    ..Default::default()
})
```

### Required
Indicates a required field (adds HTML `required` attribute).

```rust
render(Props {
    name: "description".into(),
    required: true,
    placeholder: "Describe the issue".into(),
    ..Default::default()
})
```

### Read-Only
Displays text content but prevents editing.

```rust
render(Props {
    name: "admin-notes".into(),
    readonly: true,
    value: "Admin-only field. Flagged 2026-04-10.".into(),
    ..Default::default()
})
```

### No Resize
Prevent user resizing (fixed dimensions).

```rust
render(Props {
    name: "signature".into(),
    rows: 5,
    resize: Resize::None,
    ..Default::default()
})
```

### Custom Row Height
Adjust visible row count based on expected content length.

```rust
render(Props {
    name: "long-form".into(),
    rows: 10,
    placeholder: "Write your full essay here".into(),
    ..Default::default()
})
```

## Accessibility

- **Aria-Invalid:** Set `aria_invalid: true` when form frameworks perform validation. Screen readers announce the field as invalid.
- **Aria-Invalid vs Invalid:** The `invalid` field triggers red border styling; `aria_invalid` emits the semantic attribute. Either can be set independently, but both set the ARIA attribute.
- **Label Association:** Pair with a `<label>` using the textarea's `id` and the label's `for` attribute.
- **Required:** Use the `required` field to indicate mandatory input (conveyed to assistive tech and form validation APIs).
- **Placeholder:** Not a substitute for labels. Use labels for semantic clarity.

## Related

- [Input](/docs/components/input.md) — Single-line text field
- [Label](/docs/components/label.md) — Pair with textareas for accessibility
- [Form](/docs/components/form.md) — Form field patterns and validation

## Shadcn Reference

Maud-ui Textarea maps to shadcn's `<Textarea>` component with equivalent props and variants.
