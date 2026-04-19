# Item

Media + content + actions list row for notifications, search results, file listings, and other row-based layouts.

## Import

```rust
use maud_ui::primitives::item::{self, Props, Variant, Size, MediaVariant};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::item;

html! {
    (item::group(html! {
        (item::render(item::Props {
            variant: item::Variant::Default,
            size: item::Size::Default,
            children: html! {
                (item::media(item::MediaVariant::Icon, html! { "📧" }))
                (item::content(html! {
                    (item::title(html! { "New message" }))
                    (item::description(html! { "From Jane Doe" }))
                }))
                (item::actions(html! {
                    button { "Reply" }
                }))
            },
            ..Default::default()
        }))
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `Option<String>` | `None` | Optional DOM id for wiring skip-links or triggers. |
| `variant` | `Variant` | `Default` | Visual frame style (Default, Outline, Muted). |
| `size` | `Size` | `Default` | Density (Default, Sm, Xs). |
| `children` | `Markup` | `html! {}` | Row content (media, content, actions, header, footer). |

## Variant

```rust
pub enum Variant {
    Default,
    Outline,
    Muted,
}
```

## Size

```rust
pub enum Size {
    Default,
    Sm,
    Xs,
}
```

## MediaVariant

```rust
pub enum MediaVariant {
    Default,
    Icon,
    Image,
}
```

## Helpers

- **`group(children)`** — Wrap items with consistent spacing and `role="list"`.
- **`media(variant, children)`** — Icon/avatar/image slot. Sizing hints via MediaVariant.
- **`content(children)`** — Primary textual stack (usually title + description).
- **`title(children)`** — Item heading (h3).
- **`description(children)`** — Muted supporting text (p).
- **`actions(children)`** — Trailing action slot (buttons, timestamps, badges).
- **`header(children)`** — Optional banner above main row (e.g., timestamp, category).
- **`footer(children)`** — Optional strip below main row (e.g., source, author).
- **`separator()`** — Visual `<hr>` divider between grouped items.

## Accessibility

- Item group has `role="list"`.
- Each item implicitly treated as `role="listitem"`.
- Content hierarchy maintained via semantic h3/p elements.
- Media slot size hints guide icon vs. image sizing for WCAG contrast.

## Related

- [Hover Card](hover-card.md) — richer overlay for item details.
- [Button](button.md) — actions typically contain buttons.

## Shadcn Reference

Mirrors shadcn's Item primitive with Variant/Size/MediaVariant enums and helper-based composition for media, content, title, description, actions, header, footer, separator.
