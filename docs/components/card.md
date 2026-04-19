# Card

A container primitive with optional header (title, description, top-right action), body, and footer sections. Flexible layout for displaying grouped content.

## Import

```rust
use maud_ui::primitives::card::{self, Size, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::card;

html! {
    (card::render(card::Props {
        title: Some("Settings".into()),
        description: Some("Manage your preferences.".into()),
        children: html! { p { "Your content here" } },
        footer: None,
        size: card::Size::Default,
        action: None,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `title` | `Option<String>` | `None` | Optional card title displayed in header |
| `description` | `Option<String>` | `None` | Optional subtitle displayed below title in header |
| `children` | `Markup` | `html! {}` | Main content markup (card body) |
| `footer` | `Option<Markup>` | `None` | Optional footer markup (bottom section) |
| `size` | `Size` | `Default` | Card padding size variant |
| `action` | `Option<Markup>` | `None` | Optional top-right header slot (e.g., button for secondary action) |

## Variants / Enums

### Size
- `Default`: standard padding (1.5rem)
- `Sm`: compact padding (0.75rem)

## Helper Functions

| Function | Signature | Purpose |
|----------|-----------|---------|
| `action` | `fn(Markup) -> Markup` | Renders the top-right header action slot |
| `content` | `fn(Markup) -> Markup` | Renders the card body wrapper |
| `footer` | `fn(Markup) -> Markup` | Renders the card footer wrapper |

## Accessibility

Semantic structure with implicit landmarks (header, main body, footer). Use ARIA labels in the action slot if it contains interactive elements without visible labels.

## Related

Button (for footer CTAs), Switch (for settings cards), Dialog (for modal card variants)

## Shadcn reference

https://ui.shadcn.com/docs/components/base/card
