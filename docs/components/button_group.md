# ButtonGroup

Groups multiple buttons with shared borders and optional interactive toggle behavior (exclusive or multiple selection).

## Import

```rust
use maud_ui::primitives::button_group::{self, Orientation, Mode, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::button_group;

html! {
    (button_group::render(button_group::Props {
        children: html! {
            button type="button" class="mui-btn mui-btn--outline" { "Left" }
            button type="button" class="mui-btn mui-btn--outline" { "Center" }
            button type="button" class="mui-btn mui-btn--outline" { "Right" }
        },
        orientation: button_group::Orientation::Horizontal,
        size: None,
        mode: Some(button_group::Mode::Exclusive),
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `children` | `Markup` | `html! {}` | Button elements and optional separators/text cells |
| `orientation` | `Orientation` | `Horizontal` | Horizontal or Vertical layout direction |
| `size` | `Option<String>` | `None` | Optional CSS size class (e.g., `"mui-btn--sm"`) |
| `mode` | `Option<Mode>` | `None` | `None` = static group; `Some(Exclusive)` = radio-like; `Some(Multiple)` = independent toggles |

## Variants / Enums

### Orientation
- `Horizontal`: buttons in a row (default)
- `Vertical`: buttons in a column

### Mode
- `Exclusive`: exactly one button pressed at a time (radio-button behavior)
- `Multiple`: each button toggles independently

## Helper Functions

| Function | Signature | Purpose |
|----------|-----------|---------|
| `separator` | `fn(Orientation) -> Markup` | Renders a visual rule between buttons |
| `separator_for_group` | `fn(Orientation) -> Markup` | Convenience: renders separator opposite to group orientation |
| `text` | `fn(Markup) -> Markup` | Renders non-interactive text cell (e.g., page indicator) |

## Accessibility

Emits `role="group"` on the container. For vertical groups, includes `aria-orientation="vertical"`. In interactive modes (`Exclusive`, `Multiple`), emits `data-mui="button-group"` and `data-mode="exclusive|multiple"` for JS to wire `aria-pressed` state changes. Emits custom `mui:button-group-change` event on activation with `{ value, pressed, target }` in detail.

## Related

Button, Dialog (for trigger dialogs inside button groups)

## Shadcn reference

https://ui.shadcn.com/docs/components/base/button-group
