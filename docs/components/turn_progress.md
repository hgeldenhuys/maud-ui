# Turn progress

A 2px full-width strip showing how far a turn has run. A single hard-edged two-stop gradient — accent from 0 to N%, then accent-10% for the remainder. No animation, no track chrome: it is a hair of colour at the top of the executing composer, not a widget.

## Import

```rust
use maud_ui::primitives::turn_progress::{self, Props};
```

## Example

```rust
use maud_ui::primitives::turn_progress::{self, Props};

turn_progress::render(Props { percent: 34 })
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| percent | u8 | `0` | Percent complete, 0–100 (clamped on render). |

## How it works

The fill is carried by an inline `--mui-turn-pct` custom property, so the same class serves every fill and a consumer can animate it by writing the variable. The strip exposes `role="progressbar"` with `aria-valuenow`.

## CSS classes

- `mui-turn-progress` — the strip. Reads `--mui-turn-pct` (default `0%`).

## Related

Progress, Meter, Composer (renders this above the executing field).
