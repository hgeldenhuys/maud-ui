# Chart

A lightweight inline SVG bar or line chart renderer. No JS required for static rendering; supports optional accessibility layer and custom colors. Data points are simple (label, value) tuples.

## Import

```rust
use maud_ui::primitives::chart::{self, ChartType, DataPoint, ChartConfig, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::chart;

html! {
    (chart::render(chart::Props {
        id: "sales-chart".into(),
        chart_type: chart::ChartType::Bar,
        data: vec![
            chart::DataPoint { label: "Jan".into(), value: 186.0 },
            chart::DataPoint { label: "Feb".into(), value: 305.0 },
            chart::DataPoint { label: "Mar".into(), value: 237.0 },
        ],
        title: Some("Monthly Revenue".into()),
        width: 400,
        height: 200,
        color: None,
        config: None,
        accessibility_layer: false,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `id` | `String` | `"chart"` | Unique ID for the chart container |
| `chart_type` | `ChartType` | `Bar` | Bar or Line chart type |
| `data` | `Vec<DataPoint>` | `vec![]` | Data points with label and numeric value |
| `title` | `Option<String>` | `None` | Optional title above the chart |
| `width` | `u32` | `400` | SVG width in pixels |
| `height` | `u32` | `200` | SVG height in pixels |
| `color` | `Option<String>` | `None` | CSS color for bars/line/dots (defaults to var(--mui-accent)) |
| `config` | `Option<ChartConfig>` | `None` | Series configuration (shadcn-parity stub — currently unused by renderer) |
| `accessibility_layer` | `bool` | `false` | Emits data-accessibility-layer="true" for AT hooks to enlarge hit targets, add focus rings |

## Variants / Enums

### ChartType
- `Bar`: vertical bar chart
- `Line`: line chart with area fill gradient

## Accessibility

Line and bar charts render grid lines and axis labels for reference. When `accessibility_layer` is true, emits `data-accessibility-layer="true"` on the container so CSS or AT hooks can adapt (enlarged circles, visible focus rings, etc.). Grid lines are low-opacity for visual context; axis labels use `.mui-chart__value` and `.mui-chart__label` classes.

## Related

Card (for chart containers), Button (for chart controls/legends)

## Shadcn reference

https://ui.shadcn.com/docs/components/base/chart
