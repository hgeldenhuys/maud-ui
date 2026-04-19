# Collapsible

A minimal expandable section with a trigger button and animated content. Unlike accordion, this is a standalone toggle with no card wrapping.

## Import

```rust
use maud_ui::primitives::collapsible::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::collapsible;

html! {
    (collapsible::render(collapsible::Props {
        trigger_label: "Show details".into(),
        content: html! { p { "Hidden content here" } },
        open: false,
        id: "details".into(),
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `trigger_label` | `String` | `"Toggle"` | Label text displayed in the trigger button |
| `content` | `Markup` | `html! {}` | Markup content displayed when expanded |
| `open` | `bool` | `false` | Initial open/expanded state |
| `id` | `String` | `"collapsible"` | Unique identifier for aria-controls and linking |

## Accessibility

The trigger button has `aria-expanded` reflecting the open state and `aria-controls` pointing to the content region. Content is hidden via the `hidden` attribute when closed. A chevron icon (▾) is included as a visual indicator. Keyboard: Space/Enter toggles, Tab navigates.

## Related

Accordion (for grouped collapsibles), Dialog (for modal collapsibles)

## Shadcn reference

https://ui.shadcn.com/docs/components/base/collapsible
