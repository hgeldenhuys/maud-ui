# Segmented control

A boxed switch between a small set of mutually exclusive views. Rendered as real `<a>` links so it works with JavaScript disabled: each stop is a URL (`?view=chat`), and the selected stop is marked `aria-current="page"`. The consumer can wire client-side switching on top to avoid the navigation.

## Import

```rust
use maud_ui::primitives::segmented_control::{self, Props, Stop};
```

## Example

```rust
use maud_ui::primitives::segmented_control::{self, Props, Stop};

segmented_control::render(Props {
    stops: vec![
        Stop::new("Clean", "?view=clean"),
        Stop::new("Chat", "?view=chat"),
        Stop::new("Raw", "?view=raw"),
    ],
    selected: 1,
    aria_label: Some("Transcript verbosity".into()),
})
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| stops | Vec\<Stop\> | `[]` | The stops, left to right. |
| selected | usize | `0` | Index of the active stop. |
| aria_label | Option\<String\> | `None` | Accessible label for the group. |

## Stop

`Stop::new(label, href)` — a visible label and the URL the stop navigates to.

## Accessibility

The group is `role="tablist"`; each stop is `role="tab"` with `aria-selected`, and the active stop also carries `aria-current="page"`. Because the stops are anchors, keyboard and screen-reader users get native link semantics with no JS.

## CSS classes

- `mui-segmented` — the container.
- `mui-segmented__stop` — a stop; `mui-segmented__stop--active` on the selected one.

## Related

Tabs (for full panelled views), Toggle group, Composer (uses this as its verbosity switch).
