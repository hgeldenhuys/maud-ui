# Toggle

Two-state button component that tracks pressed state (on/off).

## Description

Toggle renders a button that can be pressed (active) or unpressed (inactive), similar to formatting buttons in text editors (Bold, Italic). The component emits `aria-pressed` for accessibility and supports multiple size and variant options.

## Import

```rust
use maud_ui::primitives::toggle;
```

## Example

```rust
use maud_ui::primitives::toggle::{Props, Size, Variant, render};

// Active toggle button
let bold = render(Props {
    label: "B".to_string(),
    pressed: true,
    id: "btn-bold".to_string(),
    ..Default::default()
});

// Disabled toggle
let italic = render(Props {
    label: "I".to_string(),
    pressed: false,
    disabled: true,
    id: "btn-italic".to_string(),
    ..Default::default()
});
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `label` | `String` | `"Toggle"` | Button text or icon |
| `pressed` | `bool` | `false` | Whether the button is in pressed state |
| `disabled` | `bool` | `false` | Whether the button is disabled |
| `id` | `String` | `"toggle"` | Unique identifier |
| `size` | `Size` | `Md` | Button size (Sm, Md, Lg) |
| `variant` | `Variant` | `Default` | Visual style (Default or Outline) |

## Size Enum

```rust
pub enum Size {
    #[default]
    Md,    // Medium (default)
    Sm,    // Small
    Lg,    // Large (new in this session)
}
```

Controls button dimensions and padding.

## Variant Enum

```rust
pub enum Variant {
    #[default]
    Default,   // Solid background
    Outline,   // Border-only style
}
```

## Variants

### Default (Solid)
Solid background styling (high contrast).

```rust
render(Props {
    label: "B".to_string(),
    pressed: true,
    variant: Variant::Default,
    ..Default::default()
})
```

### Outline
Border-only styling for a subtler appearance.

```rust
render(Props {
    label: "B".to_string(),
    pressed: true,
    variant: Variant::Outline,
    ..Default::default()
})
```

### Small Size
Compact button for dense toolbars.

```rust
render(Props {
    label: "B".to_string(),
    size: Size::Sm,
    ..Default::default()
})
```

### Medium Size (Default)
Standard button size.

```rust
render(Props {
    label: "B".to_string(),
    size: Size::Md,
    ..Default::default()
})
```

### Large Size
Prominent button for primary actions or accessibility.

```rust
render(Props {
    label: "B".to_string(),
    size: Size::Lg,
    ..Default::default()
})
```

### Pressed State
Button visually indicates its pressed (active) state.

```rust
render(Props {
    label: "B".to_string(),
    pressed: true,
    ..Default::default()
})
```

### Unpressed State
Button shows unpressed (inactive) state.

```rust
render(Props {
    label: "I".to_string(),
    pressed: false,
    ..Default::default()
})
```

### Disabled
Prevents user interaction and shows disabled styling.

```rust
render(Props {
    label: "U".to_string(),
    disabled: true,
    ..Default::default()
})
```

## Text Editor Toolbar Pattern

```rust
use maud_ui::primitives::toggle;

html! {
    div.toolbar {
        (toggle::render(toggle::Props {
            label: "B".into(),
            pressed: true,
            id: "btn-bold".into(),
            ..Default::default()
        }))
        (toggle::render(toggle::Props {
            label: "I".into(),
            pressed: false,
            id: "btn-italic".into(),
            ..Default::default()
        }))
        (toggle::render(toggle::Props {
            label: "U".into(),
            pressed: false,
            id: "btn-underline".into(),
            ..Default::default()
        }))
    }
}
```

## Accessibility

- **Aria-Pressed:** Emits `aria-pressed="true"` when pressed, `aria-pressed="false"` when unpressed.
- **Toggle Semantics:** Assistive tech announces the button as a toggle and reports its current state.
- **Disabled State:** When disabled, the button has `aria-disabled="true"` or the `disabled` attribute.
- **Keyboard Support:** Buttons respond to Space and Enter keys (native behavior).

## Related

- [Button](/docs/components/button.md) — Standard button component
- [Toggle Group](/docs/components/toggle-group.md) — Multiple toggles in a group (radio or checkbox behavior)
- [Switch](/docs/components/switch.md) — On/off switch (different affordance)

## Shadcn Reference

Maud-ui Toggle aligns with shadcn's `<Toggle>` component, supporting Size (Sm, Md, Lg) and Variant (Default, Outline) options.
