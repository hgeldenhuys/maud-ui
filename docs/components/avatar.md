# Avatar

Image + fallback initials component for representing users, with optional status badge and grouping support.

## Import

```rust
use maud_ui::primitives::avatar::{self, Size, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::avatar;

html! {
    (avatar::render(avatar::Props {
        src: Some("https://example.com/avatar.jpg".to_string()),
        alt: "User name".to_string(),
        fallback: "JD".to_string(),
        size: avatar::Size::Md,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| src | Option<String> | `None` | Image URL; if None, renders fallback initials instead. |
| alt | String | `"Avatar"` | Accessible label exposed to screen readers; used as aria-label when src is None. |
| fallback | String | `"U"` | Initials or short text displayed when src is None (automatically uppercased). |
| size | Size | `Md` | Physical size: Sm, Md, or Lg. |

## Size Variants

| Size | Class | Use Case |
|------|-------|----------|
| Sm | `mui-avatar--sm` | Compact avatars in lists or sidebars. |
| Md | `mui-avatar--md` | Default avatar size for most contexts. |
| Lg | `mui-avatar--lg` | Large avatars for profiles or detail views. |

## Helper Functions

- `avatar::badge(children: Markup) -> Markup` — Renders a status indicator (pip) at the bottom-right of an avatar; typically used with inline style for color.
- `avatar::group(children: Markup) -> Markup` — Wraps multiple rendered avatars in a horizontally overlapping group with `role="group"`.
- `avatar::group_count(n: usize) -> Markup` — Renders a `+{n}` tail marker for avatar groups showing the count of additional members.

## Accessibility

- When `src` is present: the `<img alt>` carries the accessible name; outer span has no role.
- When `src` is None: outer span has `role="img"` and `aria-label` set to `alt` string.
- Fallback text is marked `aria-hidden="true"` to avoid duplication.
- Badge and group are marked `aria-hidden="true"` (presentational).

## Related

Badge, Tooltip, User card components.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/avatar
