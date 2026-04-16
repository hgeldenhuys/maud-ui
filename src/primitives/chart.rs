//! Chart component — lightweight inline SVG bar and line charts, no JS needed.

use maud::{html, Markup, PreEscaped};

/// A single data point with a label and numeric value.
#[derive(Debug, Clone)]
pub struct DataPoint {
    pub label: String,
    pub value: f64,
}

/// Chart type: bar or line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChartType {
    Bar,
    Line,
}

/// Chart rendering properties.
#[derive(Debug, Clone)]
pub struct Props {
    /// Unique ID for the chart container
    pub id: String,
    /// Bar or Line
    pub chart_type: ChartType,
    /// Data points to plot
    pub data: Vec<DataPoint>,
    /// Optional title above the chart
    pub title: Option<String>,
    /// SVG width in px (default 400)
    pub width: u32,
    /// SVG height in px (default 200)
    pub height: u32,
    /// CSS color for bars/line/dots; defaults to var(--mui-accent)
    pub color: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "chart".into(),
            chart_type: ChartType::Bar,
            data: Vec::new(),
            title: None,
            width: 400,
            height: 200,
            color: None,
        }
    }
}

// Layout constants — padding for axis labels and breathing room
const PAD_LEFT: f64 = 48.0;
const PAD_TOP: f64 = 12.0;
const PAD_RIGHT: f64 = 12.0;
const PAD_BOTTOM: f64 = 32.0;

/// Scale a value into the drawable area (y-axis is flipped in SVG).
fn scale_y(value: f64, max_value: f64, height: u32) -> f64 {
    let plot_h = height as f64 - PAD_TOP - PAD_BOTTOM;
    if max_value <= 0.0 {
        return height as f64 - PAD_BOTTOM;
    }
    height as f64 - PAD_BOTTOM - (value / max_value) * plot_h
}

/// Build SVG markup for a bar chart.
fn render_bar(props: &Props, color: &str) -> String {
    let w = props.width;
    let h = props.height;
    let n = props.data.len();
    if n == 0 {
        return format!(
            r#"<svg viewBox="0 0 {w} {h}" class="mui-chart__svg" xmlns="http://www.w3.org/2000/svg"></svg>"#
        );
    }

    let max_value = props
        .data
        .iter()
        .map(|d| d.value)
        .fold(f64::NEG_INFINITY, f64::max)
        .max(0.0);

    let plot_w = w as f64 - PAD_LEFT - PAD_RIGHT;
    let slot_w = plot_w / n as f64;
    let gap = (slot_w * 0.2).max(2.0);
    let bar_w = slot_w - gap;
    let baseline_y = h as f64 - PAD_BOTTOM;

    let mut svg = format!(
        r#"<svg viewBox="0 0 {w} {h}" class="mui-chart__svg" xmlns="http://www.w3.org/2000/svg">"#
    );

    // Y-axis line
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="currentColor" stroke-opacity="0.2" />"#,
        PAD_LEFT, PAD_TOP, PAD_LEFT, baseline_y
    ));
    // X-axis line
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="currentColor" stroke-opacity="0.2" />"#,
        PAD_LEFT,
        baseline_y,
        w as f64 - PAD_RIGHT,
        baseline_y
    ));

    // Y-axis tick labels (0, 25%, 50%, 75%, max)
    for i in 0..=4 {
        let frac = i as f64 / 4.0;
        let val = max_value * frac;
        let y = scale_y(val, max_value, h);
        // Grid line
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{y}" x2="{}" y2="{y}" stroke="currentColor" stroke-opacity="0.1" />"#,
            PAD_LEFT,
            w as f64 - PAD_RIGHT,
        ));
        // Value label
        let label = if val >= 1000.0 {
            format!("{:.0}k", val / 1000.0)
        } else if val == val.floor() {
            format!("{:.0}", val)
        } else {
            format!("{:.1}", val)
        };
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" text-anchor="end" class="mui-chart__value">{}</text>"#,
            PAD_LEFT - 4.0,
            y + 3.0,
            label
        ));
    }

    // Bars + X labels
    for i in 0..n {
        let dp = &props.data[i];
        let bar_x = PAD_LEFT + (i as f64 * slot_w) + gap / 2.0;
        let bar_y = scale_y(dp.value, max_value, h);
        let bar_h = baseline_y - bar_y;
        let center_x = bar_x + bar_w / 2.0;

        svg.push_str(&format!(
            r#"<rect x="{bar_x}" y="{bar_y}" width="{bar_w}" height="{bar_h}" rx="3" fill="{color}" opacity="0.85" />"#
        ));
        svg.push_str(&format!(
            r#"<text x="{center_x}" y="{}" text-anchor="middle" class="mui-chart__label">{}</text>"#,
            h as f64 - 10.0,
            html_escape(&dp.label)
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Build SVG markup for a line chart.
fn render_line(props: &Props, color: &str) -> String {
    let w = props.width;
    let h = props.height;
    let n = props.data.len();
    if n == 0 {
        return format!(
            r#"<svg viewBox="0 0 {w} {h}" class="mui-chart__svg" xmlns="http://www.w3.org/2000/svg"></svg>"#
        );
    }

    let max_value = props
        .data
        .iter()
        .map(|d| d.value)
        .fold(f64::NEG_INFINITY, f64::max)
        .max(0.0);

    let plot_w = w as f64 - PAD_LEFT - PAD_RIGHT;
    let baseline_y = h as f64 - PAD_BOTTOM;

    let mut svg = format!(
        r#"<svg viewBox="0 0 {w} {h}" class="mui-chart__svg" xmlns="http://www.w3.org/2000/svg">"#
    );

    // Y-axis line
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="currentColor" stroke-opacity="0.2" />"#,
        PAD_LEFT, PAD_TOP, PAD_LEFT, baseline_y
    ));
    // X-axis line
    svg.push_str(&format!(
        r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="currentColor" stroke-opacity="0.2" />"#,
        PAD_LEFT,
        baseline_y,
        w as f64 - PAD_RIGHT,
        baseline_y
    ));

    // Y-axis tick labels
    for i in 0..=4 {
        let frac = i as f64 / 4.0;
        let val = max_value * frac;
        let y = scale_y(val, max_value, h);
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{y}" x2="{}" y2="{y}" stroke="currentColor" stroke-opacity="0.1" />"#,
            PAD_LEFT,
            w as f64 - PAD_RIGHT,
        ));
        let label = if val >= 1000.0 {
            format!("{:.0}k", val / 1000.0)
        } else if val == val.floor() {
            format!("{:.0}", val)
        } else {
            format!("{:.1}", val)
        };
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" text-anchor="end" class="mui-chart__value">{}</text>"#,
            PAD_LEFT - 4.0,
            y + 3.0,
            label
        ));
    }

    // Compute (x, y) for each data point
    let mut points: Vec<(f64, f64)> = Vec::with_capacity(n);
    for i in 0..n {
        let x = if n == 1 {
            PAD_LEFT + plot_w / 2.0
        } else {
            PAD_LEFT + (i as f64 / (n - 1) as f64) * plot_w
        };
        let y = scale_y(props.data[i].value, max_value, h);
        points.push((x, y));
    }

    // Gradient definition for area fill
    let grad_id = format!("{}-area-grad", props.id);
    svg.push_str(&format!(
        r#"<defs><linearGradient id="{grad_id}" x1="0" y1="0" x2="0" y2="1"><stop offset="0%" stop-color="{color}" stop-opacity="0.25"/><stop offset="100%" stop-color="{color}" stop-opacity="0.03"/></linearGradient></defs>"#
    ));

    // Area fill (smooth gradient)
    if points.len() >= 2 {
        let mut area = String::from("<polygon points=\"");
        // Start at baseline under first point
        area.push_str(&format!("{},{} ", points[0].0, baseline_y));
        for (x, y) in &points {
            area.push_str(&format!("{x},{y} "));
        }
        // Close back to baseline under last point
        area.push_str(&format!(
            "{},{}",
            points[points.len() - 1].0,
            baseline_y
        ));
        area.push_str(&format!(
            r#"" fill="url(#{grad_id})" />"#
        ));
        svg.push_str(&area);
    }

    // Polyline
    let pts: Vec<String> = points.iter().map(|(x, y)| format!("{x},{y}")).collect();
    svg.push_str(&format!(
        r#"<polyline points="{}" fill="none" stroke="{color}" stroke-width="2" stroke-linejoin="round" stroke-linecap="round" />"#,
        pts.join(" ")
    ));

    // Data-point circles
    for (x, y) in &points {
        svg.push_str(&format!(
            r#"<circle cx="{x}" cy="{y}" r="3" fill="{color}" />"#
        ));
    }

    // X labels
    for i in 0..n {
        let x = points[i].0;
        svg.push_str(&format!(
            r#"<text x="{x}" y="{}" text-anchor="middle" class="mui-chart__label">{}</text>"#,
            h as f64 - 10.0,
            html_escape(&props.data[i].label)
        ));
    }

    svg.push_str("</svg>");
    svg
}

/// Render a chart with the given properties.
pub fn render(props: Props) -> Markup {
    let color = props
        .color
        .clone()
        .unwrap_or_else(|| "var(--mui-accent)".into());

    let svg = match props.chart_type {
        ChartType::Bar => render_bar(&props, &color),
        ChartType::Line => render_line(&props, &color),
    };

    html! {
        div.mui-chart id=(props.id) {
            @if let Some(ref title) = props.title {
                p.mui-chart__title { (title) }
            }
            (PreEscaped(svg))
        }
    }
}

/// Minimal HTML entity escaping for label text inside SVG.
fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Showcase bar and line charts with sample data.
pub fn showcase() -> Markup {
    let monthly_data = vec![
        DataPoint { label: "Jan".into(), value: 186.0 },
        DataPoint { label: "Feb".into(), value: 305.0 },
        DataPoint { label: "Mar".into(), value: 237.0 },
        DataPoint { label: "Apr".into(), value: 73.0 },
        DataPoint { label: "May".into(), value: 209.0 },
        DataPoint { label: "Jun".into(), value: 214.0 },
    ];

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Bar chart" }
                (render(Props {
                    id: "chart-bar-demo".into(),
                    chart_type: ChartType::Bar,
                    data: monthly_data.clone(),
                    title: Some("Monthly Revenue (USD)".into()),
                    ..Default::default()
                }))
            }
            div {
                p.mui-showcase__caption { "Line chart with area fill" }
                (render(Props {
                    id: "chart-line-demo".into(),
                    chart_type: ChartType::Line,
                    data: monthly_data.clone(),
                    title: Some("Active Users Over Time".into()),
                    ..Default::default()
                }))
            }
            div {
                p.mui-showcase__caption { "Custom color" }
                (render(Props {
                    id: "chart-custom-color".into(),
                    chart_type: ChartType::Bar,
                    data: monthly_data.clone(),
                    title: Some("Conversion Rate by Month".into()),
                    color: Some("var(--mui-success)".into()),
                    ..Default::default()
                }))
            }
            div {
                p.mui-showcase__caption { "Wide line chart" }
                (render(Props {
                    id: "chart-line-wide".into(),
                    chart_type: ChartType::Line,
                    data: monthly_data,
                    title: Some("Pageviews Trend (6 months)".into()),
                    width: 600,
                    height: 250,
                    color: Some("var(--mui-warning)".into()),
                }))
            }
        }
    }
}
