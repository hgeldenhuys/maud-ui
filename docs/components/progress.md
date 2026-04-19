# Progress

Determinate and indeterminate progress bar indicators for file uploads, form steps, loading states, and real-time completion tracking.

## Import

```rust
use maud_ui::primitives::progress::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::progress;

html! {
    (progress::render(progress::Props {
        value: 65,
        max: 100,
        label: "Uploading file, 65 percent".into(),
        indeterminate: false,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| value | u32 | `0` | Current progress value (0..=max). Ignored if `indeterminate: true`. |
| max | u32 | `100` | Maximum value for percentage calculation. |
| label | String | `""` | ARIA label for the progress bar (required for a11y). |
| indeterminate | bool | `false` | If true, render as animated indeterminate progress (busy indicator). |

## Variants

### Determinate

Concrete progress: bar fills proportionally to `value / max`.

```rust
(progress::render(progress::Props {
    value: 40,
    max: 100,
    label: "Sync status, 40 percent".into(),
    indeterminate: false,
}))
```

### Indeterminate

Animated bar for unknown duration (e.g., "processing...").

```rust
(progress::render(progress::Props {
    indeterminate: true,
    label: "Processing request".into(),
    ..Default::default()
}))
```

## Helpers

### label(text: &str) -> Markup

Renders a `<span class="mui-progress__label">` for semantic label text. Matches shadcn `ProgressLabel`.

```rust
(progress::label("Download progress"))
```

### value(val: u32) -> Markup

Renders a `<span class="mui-progress__value">` displaying `"{val}%"`. Matches shadcn `ProgressValue`.

```rust
(progress::value(75))  // Output: "75%"
```

## Usage Pattern

Combine label, value, and render in a flex layout:

```rust
html! {
    div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:0.5rem" {
        (progress::label("Sync status"))
        (progress::value(40))
    }
    (progress::render(progress::Props {
        value: 40,
        max: 100,
        label: "Sync status, 40 percent".into(),
        indeterminate: false,
    }))
}
```

## Accessibility

- Uses `role="progressbar"` on the outer container.
- Determinate: `aria-valuenow`, `aria-valuemin="0"`, `aria-valuemax="{max}"`, `aria-label="{label}"`.
- Indeterminate: `aria-valuemin="0"`, `aria-valuemax="{max}"`, `aria-label="{label}"` (no `valuenow`).
- Label must be descriptive for screen readers (e.g., "Uploading file, 65 percent").

## Related

Skeleton (loading placeholder), Spinner (busy indicator).

## Shadcn reference

https://ui.shadcn.com/docs/components/base/progress
