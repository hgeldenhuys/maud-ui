# Slider

Range input with custom styling and progressive enhancement. Supports single-thumb and multi-thumb (range) modes. New in v0.2.1: `values: Vec<f64>` for multi-thumb, `orientation` for vertical sliders.

## Import

```rust
use maud_ui::primitives::slider::{self, Orientation, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::slider;

// Single-thumb slider
html! {
    (slider::render(slider::Props {
        name: "volume".into(),
        id: "slider-volume".into(),
        value: 72.0,
        min: 0.0,
        max: 100.0,
        step: 1.0,
        label: "Volume".into(),
        show_value: true,
        ..Default::default()
    }))
}

// Multi-thumb range slider
html! {
    (slider::render(slider::Props {
        name: "price-range".into(),
        id: "slider-price".into(),
        values: vec![80.0, 320.0],
        min: 0.0,
        max: 500.0,
        step: 10.0,
        label: "Price range".into(),
        show_value: true,
        ..Default::default()
    }))
}

// Vertical slider
html! {
    (slider::render(slider::Props {
        name: "eq-mid".into(),
        id: "slider-eq".into(),
        value: 60.0,
        min: 0.0,
        max: 100.0,
        step: 1.0,
        label: "Mid frequency".into(),
        orientation: slider::Orientation::Vertical,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| name | String | (empty) | HTML name attribute for form submission. |
| id | String | (empty) | HTML id attribute for label linkage. |
| value | f64 | `50.0` | Current value (single-thumb mode when `values` is empty). |
| values | Vec<f64> | (empty) | Multiple thumb values. When non-empty, renders one thumb per value and fills between min/max. Overrides single `value`. |
| min | f64 | `0.0` | Minimum value. |
| max | f64 | `100.0` | Maximum value. |
| step | f64 | `1.0` | Step increment. |
| disabled | bool | `false` | Whether the slider is disabled. |
| label | String | (empty) | ARIA label for the slider. |
| show_value | bool | `false` | Whether to show the current value label below the track. |
| orientation | Orientation | `Horizontal` | Slider orientation: Horizontal or Vertical. |

## Orientation Enum

| Variant | Behavior |
|---------|----------|
| Horizontal | Slider track runs left-right; thumb moves horizontally. |
| Vertical | Slider track runs bottom-top; larger values appear higher (standard convention). |

## Modes

### Single-Thumb Mode

When `values` is empty, the slider renders one thumb at the position of `value`. Fill spans from 0 to the thumb.

### Multi-Thumb (Range) Mode

When `values` is non-empty, the slider renders one thumb per value (sorted). Fill spans between the minimum and maximum values in the set. Single `value` is reflected to the hidden native `<input>` for form submission; multi-thumb state is driven by JS on `data-values` for progressive enhancement.

## Accessibility

- Root has `role="slider"` on each thumb (per ARIA spec, though native `<input type="range">` is hidden).
- Each thumb has:
  - `aria-valuenow`: current thumb value.
  - `aria-valuemin` / `aria-valuemax`: min/max bounds.
  - `aria-orientation`: `horizontal` or `vertical`.
  - `aria-label`: inherited from the Slider's label field.
  - `aria-disabled`: set to `true` when disabled.
- Fill bar marked `aria-hidden="true"` (decorative).
- Native input is `aria-hidden="true"` and `tabindex="-1"` (decorative; JS handles the real interaction).

## CSS Classes

| Class | Purpose |
|-------|---------|
| `mui-slider` | Root slider container. |
| `mui-slider--vertical` | Vertical orientation variant. |
| `mui-slider__track` | Track background bar. |
| `mui-slider__fill` | Filled portion of the track (solid bar). |
| `mui-slider__thumb` | Individual thumb/handle. |
| `mui-slider__native` | Hidden native `<input type="range">` for form submission. |
| `mui-slider__value` | Value label span (if `show_value: true`). |

## Value Display

- **Single-thumb:** displays the current value (e.g., `72`).
- **Multi-thumb:** displays a range with an en-dash (e.g., `80 — 320`).
- Trailing zeros are stripped (e.g., `50.0` displays as `50`, not `50.0`).

## Related

RangeSlider, Input, Form.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/slider
