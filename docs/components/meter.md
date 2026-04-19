# Meter

Visual progress indicator with configurable zones (optimal, suboptimal, danger). Displays numeric measurements with color-coded regions based on thresholds and optimum value.

## Import

```rust
use maud_ui::primitives::meter::{self, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::meter;

html! {
    // Battery level (higher is better)
    (meter::render(meter::Props {
        value: 85.0,
        min: 0.0,
        max: 100.0,
        low: Some(20.0),
        high: Some(80.0),
        optimum: Some(100.0),  // Ideal is max (right side)
        label: "Battery level".into(),
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| value | f64 | `0.0` | Current numeric value of the meter. |
| min | f64 | `0.0` | Minimum value of the scale. |
| max | f64 | `100.0` | Maximum value of the scale. |
| low | Option<f64> | `None` | Lower threshold; values below this (when optimum is high) or above this (when optimum is low) enter suboptimal/danger zones. |
| high | Option<f64> | `None` | Upper threshold; values above this (when optimum is high) or below this (when optimum is low) move towards optimal/danger zones. |
| optimum | Option<f64> | `None` | Ideal value; determines which side of the scale is "good." If None, all values are optimum. |
| label | String | `""` | Accessibility label for the meter. Announced by screen readers. |

## Zone Logic

The meter automatically determines which visual zone (optimum, suboptimum, danger) to display based on the relationship between `value`, `low`, `high`, and `optimum`:

### Higher Is Better (optimum >= midpoint)

Used for battery level, signal strength, or other metrics where higher values are desirable.

| Value | Zone | Styling |
|-------|------|---------|
| < low | Danger | Red indicator |
| >= low && < high | Suboptimal | Yellow indicator |
| >= high | Optimum | Green indicator |

**Example:** Battery with `min=0, max=100, low=20, high=80, optimum=100`
- 15% → Danger (red)
- 60% → Suboptimal (yellow)
- 90% → Optimum (green)

### Lower Is Better (optimum < midpoint)

Used for disk usage, temperature, latency, or other metrics where lower values are desirable.

| Value | Zone | Styling |
|-------|------|---------|
| <= optimum_threshold | Optimum | Green indicator |
| > optimum_threshold && <= high | Suboptimal | Yellow indicator |
| > high | Danger | Red indicator |

**Example:** Disk usage with `min=0, max=100, low=20, high=80, optimum=0`
- 25% → Optimum (green)
- 70% → Suboptimal (yellow)
- 95% → Danger (red)

### No Optimum

When `optimum` is `None`, the meter is always rendered in the optimum zone (green).

## Variants

| Variant | Configuration | Use Case |
|---------|---------------|----------|
| Battery level | Higher is better (optimum = max) | Device power, charge level. |
| Disk usage | Lower is better (optimum = min) | Storage capacity, free space. |
| Signal strength | Higher is better (optimum = max) | Network signal, WiFi strength. |
| Temperature | Lower is better (optimum = min) | CPU temp, water cooling. |
| Memory usage | Lower is better (optimum = min) | RAM, heap size. |
| Upload progress | Higher is better (optimum = max) | File transfer, data sync. |

## Helpers

None. The component uses internal `zone_class()` to determine the visual state.

## Accessibility

- **Role:** The outer container uses `role="meter"`.
- **ARIA attributes:** Sets `aria-valuenow` (current value), `aria-valuemin` (minimum), `aria-valuemax` (maximum), and `aria-label` (descriptive text).
- **Screen readers:** All numeric and descriptive information is announced; visual zone colors are supplemented by semantic ARIA.

## Related

Progress, Slider, Input, Number Field.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/progress
