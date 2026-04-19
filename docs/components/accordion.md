# Accordion

Collapsible vertical disclosure widget that can hold multiple expandable sections with optional single-open constraint.

## Import

```rust
use maud_ui::primitives::accordion::{self, Item, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::accordion;

html! {
    (accordion::render(accordion::Props {
        items: vec![
            accordion::Item {
                id: "item-1".to_string(),
                trigger: "What is this?".to_string(),
                content: html! { p { "This is the answer." } },
                open: false,
            },
        ],
        multiple: false,
        aria_label: Some("FAQ".to_string()),
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| items | Vec<Item> | `[]` | Array of accordion items, each with id, trigger label, content markup, and open state. |
| multiple | bool | `false` | If true, multiple items can be open simultaneously; if false, only one item at a time. |
| aria_label | Option<String> | `None` | Accessible label for the accordion wrapper (emitted as `aria-label`); provide when there is no surrounding heading. |

## Item Struct

| Field | Type | Description |
|-------|------|-------------|
| id | String | Unique identifier for aria-controls and content linking. |
| trigger | String | Label text displayed in the trigger button. |
| content | Markup | Markup content displayed when the item is expanded. |
| open | bool | Initial open state (default false). |

## Accessibility

- Emits `role="region"` on content with `aria-labelledby` pointing to trigger.
- Trigger button exposes `aria-expanded` (true/false) and `aria-controls`.
- Chevron icon marked `aria-hidden="true"` to avoid double announcement.
- `aria-label` on root required when accordion has no surrounding heading for screen reader context.

## Related

Button, Collapsible, Dialog.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/accordion
