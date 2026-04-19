# Kbd

Keyboard shortcut display component for help cards, tooltips, and command palettes.

## Import

```rust
use maud_ui::primitives::kbd::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::kbd;

html! {
    (kbd::render(kbd::Props {
        keys: vec!["⌘".to_string(), "K".to_string()],
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `keys` | `Vec<String>` | `vec![]` | Sequence of key labels (e.g., ["Ctrl", "S"] or ["⌘", "K"]). |

## Rendering

- Each key renders as a `<kbd>` element.
- Keys are separated by `+` (span.mui-kbd-separator).
- Container wraps in `div.mui-kbd-container` for flexbox layout.

## Accessibility

- Uses semantic `<kbd>` element for keyboard input notation.
- Screen readers will announce as "Ctrl plus S" etc. (based on platform-specific SR rendering).
- No ARIA labels needed; semantic markup is sufficient.

## Related

- [Tooltip](tooltip.md) — can house keyboard shortcuts.
- [HoverCard](hover-card.md) — rich help card showing multiple shortcuts.

## Shadcn Reference

Minimal kbd component for keyboard notation. Typically embedded in help dialogs or command palette documentation.
