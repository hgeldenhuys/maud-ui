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
| `variant` | `Variant` | `Variant::Default` | `Default` is the keycap (2px bottom edge + shadow). `Code` is the inline code chip: a 24px bordered pill in the mono face that sits inline in a text line — selectable, no keycap chrome. |

## Rendering

- Each key renders as a `<kbd>` element.
- Keys are separated by `+` (span.mui-kbd-separator) in the keycap variant.
- The keycap group wraps in `span.mui-kbd-container` for flexbox layout (a span, so chips can live inside paragraphs without the browser closing the `<p>`).
- The `Code` variant renders the chip(s) bare — inline content, no wrapper — so a chip inside a sentence does not perturb the line's height.

## Accessibility

- Uses semantic `<kbd>` element for keyboard input notation.
- Screen readers will announce as "Ctrl plus S" etc. (based on platform-specific SR rendering).
- No ARIA labels needed; semantic markup is sufficient.

## Related

- [Tooltip](tooltip.md) — can house keyboard shortcuts.
- [HoverCard](hover-card.md) — rich help card showing multiple shortcuts.

## Shadcn Reference

Minimal kbd component for keyboard notation. Typically embedded in help dialogs or command palette documentation.
