# Badge

Minimal status indicator for tagging, labeling, and highlighting items with optional leading icon.

## Import

```rust
use maud_ui::primitives::badge::{self, Variant, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::badge;

html! {
    (badge::render(badge::Props {
        label: "Live".to_string(),
        variant: badge::Variant::Success,
        href: None,
        leading_icon: None,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| label | String | `""` | Text content displayed in the badge. |
| variant | Variant | `Default` | Visual variant (color scheme): Default, Secondary, Success, Warning, Danger, Outline, Ghost, Link. |
| href | Option<String> | `None` | Optional URL; if Some, badge renders as an `<a>` element; if None, renders as a `<span>`. |
| leading_icon | Option<Markup> | `None` | Optional icon markup rendered before the label with `data-icon="inline-start"`. |
| mono | bool | `false` | Render the label in the monospace face — for shas, ids, counts. |
| chip | bool | `false` | Render as a taller (26px) hollow chip: bordered-transparent, 1px foreground-10% border, 6px radius. The counterpart to the filled pill. |
| trailing_count | Option<String> | `None` | Trailing count set apart in the mono face — the `2` in `mail 2`. |
| kbd | Option<String> | `None` | Trailing kbd hint in a subtle key cap — the `⌘K` in a command chip. |

## Hollow chip

`chip: true` swaps the pill for a taller bordered-transparent chip sized to sit inline with 24–28px controls. It carries an optional `trailing_count` or `kbd`:

```rust
badge::render(badge::Props {
    label: "mail".into(),
    variant: badge::Variant::Outline,
    chip: true,
    trailing_count: Some("2".into()),
    ..Default::default()
});

badge::render(badge::Props {
    label: "Command".into(),
    variant: badge::Variant::Outline,
    chip: true,
    kbd: Some("⌘K".into()),
    ..Default::default()
});
```

## Variants

| Variant | Use Case |
|---------|----------|
| Default | General-purpose status or tag. |
| Secondary | Muted or secondary information. |
| Success | Positive status (approved, live, etc.). |
| Warning | Cautionary status (beta, experimental, etc.). |
| Danger | Error or destructive state. |
| Outline | Outlined badge for subtle emphasis. |
| Ghost | Ghost-style badge with minimal styling. |
| Link | Badge styled as a link for navigation. |

## Accessibility

- When `href` is present, badge is an `<a>` element with standard link semantics.
- When `href` is None, badge is a `<span>` (non-interactive).
- Icon (if present) is inline with the label; no special ARIA needed if icon is decorative.

## Related

Alert, Button, Label, Tag.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/badge
