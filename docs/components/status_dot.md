# Status dot

A 6px semantic dot for run state, presence, and health. Filled by default; an outline (hollow) variant reads as "observing / not yet settled" without spending a second hue.

## Import

```rust
use maud_ui::primitives::status_dot::{self, Props, Tone};
```

## Example

```rust
use maud_ui::primitives::status_dot::{self, Props, Tone};

status_dot::render(Props { tone: Tone::Success, label: Some("passing".into()), ..Default::default() })
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| tone | Tone | `Accent` | Semantic colour. |
| outline | bool | `false` | Hollow (observing) instead of filled. |
| label | Option\<String\> | `None` | When set, the dot is `role="img"` with this label; otherwise it is `aria-hidden`. |

## Tone enum

| Value | Meaning |
|-------|---------|
| Success | Healthy, passing, live. |
| Accent | Active, selected, running. |
| Warning | Attention, degraded. |
| Neutral | Idle, unknown, off. |
| Destructive | Failed, blocked. |

## Accessibility

A bare dot is decorative and hidden from assistive tech. Pass `label` when the dot is the *only* carrier of a state, so it is announced rather than silently invisible.

## CSS classes

- `mui-status-dot` + one of `mui-status-dot--success` / `--accent` / `--warning` / `--neutral` / `--destructive`.
- `mui-status-dot--outline` for the hollow form (composes with the tone class).

## Related

Badge, Attention pill, Spinner.
