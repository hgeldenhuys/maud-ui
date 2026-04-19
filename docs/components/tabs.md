# Tabs

Tabbed interface for organizing content into separate panels with optional orientation and activation mode control.

## Description

Tabs provides a multi-section container where users select between labeled panels. Each tab can be individually disabled. The component supports horizontal and vertical layouts, segmented and underline visual variants, and both automatic (keyboard arrow keys) and manual (requires explicit click/Enter) activation modes.

## Import

```rust
use maud_ui::primitives::tabs;
```

## Example

```rust
use maud::{html, Markup};
use maud_ui::primitives::tabs::{Props, Tab, Orientation, Variant, ActivationMode, render};

let tabs = vec![
    Tab {
        id: "account".to_string(),
        label: "Account".to_string(),
        content: html! { p { "Account settings here" } },
        disabled: false,
    },
    Tab {
        id: "password".to_string(),
        label: "Password".to_string(),
        content: html! { p { "Change password here" } },
        disabled: false,
    },
];

let tabs_ui = render(Props {
    tabs,
    default_active: 0,
    aria_label: "Settings".to_string(),
    ..Default::default()
});
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `tabs` | `Vec<Tab>` | `vec![]` | List of tab definitions |
| `default_active` | `usize` | `0` | Index of initially active tab |
| `aria_label` | `String` | `"Tabs"` | Accessible label for the tab list |
| `orientation` | `Orientation` | `Horizontal` | Layout direction (Horizontal or Vertical) |
| `variant` | `Variant` | `Default` | Visual style (Default/segmented or Line/underline) |
| `activation_mode` | `ActivationMode` | `Automatic` | How tabs activate (Automatic on arrow keys or Manual on click/Enter) |

## Tab

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `""` | Unique identifier for the tab (used in aria-controls, etc.) |
| `label` | `String` | `""` | Display text on the tab trigger button |
| `content` | `Markup` | `html! {}` | Panel content (displayed when tab is active) |
| `disabled` | `bool` | `false` | Whether the tab is disabled and unselectable |

## Enums

### Orientation

```rust
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}
```

Controls the layout direction of the tab list. Vertical orientation changes `aria-orientation="vertical"` and applies different CSS layout.

### Variant

```rust
pub enum Variant {
    #[default]
    Default,    // Segmented tabs with background and border
    Line,       // Underline style (less prominent)
}
```

Controls the visual appearance of the tab triggers.

### ActivationMode

```rust
pub enum ActivationMode {
    #[default]
    Automatic,  // Arrow keys activate tabs
    Manual,     // Only click or Enter activates tabs
}
```

Controls how users switch between tabs. Automatic mode is responsive to keyboard navigation; Manual requires explicit user action.

## Variants

### Default (Segmented)
Horizontal tabs with segmented background and border styling.

```rust
render(Props {
    tabs,
    orientation: Orientation::Horizontal,
    variant: Variant::Default,
    ..Default::default()
})
```

### Line (Underline)
Underline style for a minimalist appearance.

```rust
render(Props {
    tabs,
    variant: Variant::Line,
    ..Default::default()
})
```

### Vertical
Tab triggers arranged vertically on the left, panels on the right.

```rust
render(Props {
    tabs,
    orientation: Orientation::Vertical,
    ..Default::default()
})
```

### Disabled Tabs
Individual tabs can be disabled to prevent selection.

```rust
let tabs = vec![
    Tab { disabled: false, ..Default::default() },
    Tab { disabled: true, ..Default::default() },  // Unselectable
];

render(Props { tabs, ..Default::default() })
```

### Manual Activation Mode
Tabs only respond to explicit user interaction (click or Enter key).

```rust
render(Props {
    tabs,
    activation_mode: ActivationMode::Manual,
    ..Default::default()
})
```

## Accessibility

- **ARIA Labels:** Each tab list has an `aria-label`. Individual tabs have `aria-selected`, `aria-controls`, and `aria-disabled` attributes.
- **Keyboard Navigation:** In Automatic mode, arrow keys navigate and activate tabs. In Manual mode, arrow keys navigate but don't activate (requires Enter or click).
- **Roving Tabindex:** Only the active (or first) tab is in the DOM tab order; others have `tabindex="-1"`.
- **Panel Association:** Tab panels are linked via `aria-labelledby` to their triggers.

## Related

- [Button](/docs/components/button.md) — Similar trigger patterns
- [Card](/docs/components/card.md) — Often paired with tabs for content organization
- [Dialog](/docs/components/dialog.md) — Alternative modal pattern for multi-step content

## Shadcn Reference

Maud-ui Tabs aligns with shadcn's Tabs component, supporting both Default and Line variants plus orientation options.
