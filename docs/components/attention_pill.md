# Attention pill

A small accent-tinted pill that says "this wants a look". Two states, one component: the accent form (accent-12% fill, accent dot, accent text) draws the eye; the muted zero-state form spends no accent at all, so an empty count does not sit there glowing.

## Import

```rust
use maud_ui::primitives::attention_pill::{self, Props};
```

## Example

```rust
use maud_ui::primitives::attention_pill::{self, Props};

// Wants attention
attention_pill::render(Props { label: "2 waiting on you".into(), ..Default::default() });

// Zero state — recedes
attention_pill::render(Props { label: "all caught up".into(), muted: true });
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| label | String | `""` | Caller-supplied label text. |
| muted | bool | `false` | Render the no-accent zero-state instead of the accent form. |

## When to use each

Reach for the accent form for a *non-zero* actionable count — the one thing on a row the eye should land on. Switch to `muted: true` at zero so the surface reads as calm rather than lit up at nothing.

## CSS classes

- `mui-attention-pill` (+ `mui-attention-pill--muted`).
- `mui-attention-pill__dot`, `mui-attention-pill__label`.

## Related

Badge, Status dot.
