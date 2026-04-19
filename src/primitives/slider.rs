//! Slider component — range input with custom styling and progressive enhancement.

use maud::{html, Markup};

/// Orientation of the slider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// Slider rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// HTML name attribute for form submission
    pub name: String,
    /// HTML id attribute for label linkage
    pub id: String,
    /// Current value (used when `values` is empty — single-thumb mode)
    pub value: f64,
    /// Multiple thumb values. When non-empty, renders one thumb per value
    /// and fills between the min and max of the set (range/multi-thumb mode).
    /// When empty, falls back to single-thumb mode using `value`.
    pub values: Vec<f64>,
    /// Minimum value (default 0.0)
    pub min: f64,
    /// Maximum value (default 100.0)
    pub max: f64,
    /// Step increment (default 1.0)
    pub step: f64,
    /// Whether the slider is disabled
    pub disabled: bool,
    /// ARIA label for the slider
    pub label: String,
    /// Whether to show the current value label
    pub show_value: bool,
    /// Orientation of the slider (default Horizontal)
    pub orientation: Orientation,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            value: 50.0,
            values: Vec::new(),
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            label: String::new(),
            show_value: false,
            orientation: Orientation::Horizontal,
        }
    }
}

/// Format a value cleanly: strip trailing zeros after decimal point.
fn format_value(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{}", value)
    }
}

/// Convert a value to a percentage position along the track.
fn to_pct(value: f64, min: f64, max: f64) -> f64 {
    if max != min {
        ((value - min) / (max - min)) * 100.0
    } else {
        0.0
    }
}

/// Position style for a thumb. Vertical sliders anchor from the bottom
/// so that larger values appear higher on the track (standard convention).
fn thumb_style(pct: f64, orientation: Orientation) -> String {
    match orientation {
        Orientation::Horizontal => format!("left: {}%", pct),
        Orientation::Vertical => format!("bottom: {}%", pct),
    }
}

/// Fill style — for horizontal a left-anchored width bar, for vertical a
/// bottom-anchored height bar. When multi-thumb, fill starts at the min
/// thumb offset.
fn fill_style(start_pct: f64, extent_pct: f64, orientation: Orientation) -> String {
    match orientation {
        Orientation::Horizontal => {
            format!("left: {}%; width: {}%", start_pct, extent_pct)
        }
        Orientation::Vertical => {
            format!("bottom: {}%; height: {}%", start_pct, extent_pct)
        }
    }
}

/// Render a slider with the given properties
pub fn render(props: Props) -> Markup {
    // Collect the thumb values. Multi-thumb mode is active when `values`
    // is non-empty; otherwise fall back to the single `value`.
    let mut thumbs: Vec<f64> = if props.values.is_empty() {
        vec![props.value]
    } else {
        props.values.clone()
    };
    thumbs.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    // Fill extent. For a single thumb, fill spans from 0 to the thumb.
    // For multiple thumbs, fill spans between the min and max of the set.
    let (fill_start_pct, fill_extent_pct) = if thumbs.len() > 1 {
        let lo = *thumbs.first().unwrap();
        let hi = *thumbs.last().unwrap();
        let lo_pct = to_pct(lo, props.min, props.max);
        let hi_pct = to_pct(hi, props.min, props.max);
        (lo_pct, (hi_pct - lo_pct).max(0.0))
    } else {
        let pct = to_pct(thumbs[0], props.min, props.max);
        (0.0, pct)
    };

    let is_vertical = props.orientation == Orientation::Vertical;
    let orientation_class = if is_vertical {
        " mui-slider--vertical"
    } else {
        ""
    };
    let aria_orientation = if is_vertical {
        "vertical"
    } else {
        "horizontal"
    };
    let root_class = format!("mui-slider{}", orientation_class);

    // Native input always reflects the single-thumb `value` for form
    // submission compatibility. Multi-thumb state is driven by JS on top
    // of `data-values` for progressive enhancement.
    let data_values_attr = if !props.values.is_empty() {
        let joined: Vec<String> = thumbs.iter().map(|v| format_value(*v)).collect();
        Some(joined.join(","))
    } else {
        None
    };

    // Value label — show the range `lo — hi` when multi-thumb.
    let display_value = if thumbs.len() > 1 {
        format!(
            "{} \u{2014} {}",
            format_value(*thumbs.first().unwrap()),
            format_value(*thumbs.last().unwrap())
        )
    } else {
        format_value(thumbs[0])
    };

    let fill_s = fill_style(fill_start_pct, fill_extent_pct, props.orientation);

    html! {
        @if props.disabled {
            div class=(root_class) data-mui="slider" data-min=(props.min) data-max=(props.max) data-step=(props.step) data-orientation=(aria_orientation) data-values=[data_values_attr.as_deref()] data-disabled="true" {
                div.mui-slider__track {
                    div.mui-slider__fill style=(fill_s) aria-hidden="true" {}
                    @for (i, v) in thumbs.iter().enumerate() {
                        div.mui-slider__thumb
                            role="slider"
                            tabindex="0"
                            data-thumb-index=(i)
                            aria-valuenow=(v)
                            aria-valuemin=(props.min)
                            aria-valuemax=(props.max)
                            aria-orientation=(aria_orientation)
                            aria-label=(props.label)
                            aria-disabled="true"
                            style=(thumb_style(to_pct(*v, props.min, props.max), props.orientation)) {}
                    }
                }
                input.mui-slider__native
                    type="range"
                    name=(props.name)
                    id=(props.id)
                    value=(props.value)
                    min=(props.min)
                    max=(props.max)
                    step=(props.step)
                    disabled
                    aria-hidden="true"
                    tabindex="-1";
                @if props.show_value {
                    span.mui-slider__value { (display_value) }
                }
            }
        } @else {
            div class=(root_class) data-mui="slider" data-min=(props.min) data-max=(props.max) data-step=(props.step) data-orientation=(aria_orientation) data-values=[data_values_attr.as_deref()] {
                div.mui-slider__track {
                    div.mui-slider__fill style=(fill_s) aria-hidden="true" {}
                    @for (i, v) in thumbs.iter().enumerate() {
                        div.mui-slider__thumb
                            role="slider"
                            tabindex="0"
                            data-thumb-index=(i)
                            aria-valuenow=(v)
                            aria-valuemin=(props.min)
                            aria-valuemax=(props.max)
                            aria-orientation=(aria_orientation)
                            aria-label=(props.label)
                            style=(thumb_style(to_pct(*v, props.min, props.max), props.orientation)) {}
                    }
                }
                input.mui-slider__native
                    type="range"
                    name=(props.name)
                    id=(props.id)
                    value=(props.value)
                    min=(props.min)
                    max=(props.max)
                    step=(props.step)
                    aria-hidden="true"
                    tabindex="-1";
                @if props.show_value {
                    span.mui-slider__value { (display_value) }
                }
            }
        }
    }
}

/// Showcase all slider variants and use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Price range — min + max sliders stacked
            div {
                p.mui-showcase__caption { "Price range" }
                div style="display:flex;flex-direction:column;gap:0.75rem;max-width:22rem;" {
                    div style="display:flex;justify-content:space-between;align-items:center;" {
                        label style="font-size:0.875rem;color:var(--mui-text);font-weight:500;" { "Price" }
                        span style="font-size:0.875rem;font-weight:500;color:var(--mui-text);" { "$80 \u{2014} $320" }
                    }
                    div style="display:flex;flex-direction:column;gap:0.625rem;" {
                        div style="display:flex;align-items:center;gap:0.75rem;" {
                            span style="font-size:0.75rem;color:var(--mui-text-muted);min-width:2.25rem;" { "Min" }
                            div style="flex:1;" {
                                (render(Props {
                                    name: "price-min".into(),
                                    id: "slider-price-min".into(),
                                    value: 80.0,
                                    min: 0.0,
                                    max: 500.0,
                                    step: 10.0,
                                    label: "Minimum price".into(),
                                    show_value: false,
                                    ..Default::default()
                                }))
                            }
                        }
                        div style="display:flex;align-items:center;gap:0.75rem;" {
                            span style="font-size:0.75rem;color:var(--mui-text-muted);min-width:2.25rem;" { "Max" }
                            div style="flex:1;" {
                                (render(Props {
                                    name: "price-max".into(),
                                    id: "slider-price-max".into(),
                                    value: 320.0,
                                    min: 0.0,
                                    max: 500.0,
                                    step: 10.0,
                                    label: "Maximum price".into(),
                                    show_value: false,
                                    ..Default::default()
                                }))
                            }
                        }
                    }
                    div style="display:flex;justify-content:space-between;font-size:0.75rem;color:var(--mui-text-muted);" {
                        span { "$0" }
                        span { "$500" }
                    }
                }
            }

            // Range slider — single track, two thumbs (P1 demo)
            div {
                p.mui-showcase__caption { "Range slider (two thumbs)" }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:22rem;" {
                    div style="display:flex;justify-content:space-between;align-items:center;" {
                        label style="font-size:0.875rem;color:var(--mui-text);font-weight:500;" { "Budget range" }
                        span style="font-size:0.875rem;font-weight:500;color:var(--mui-text);" { "$120 \u{2014} $360" }
                    }
                    (render(Props {
                        name: "budget-range".into(),
                        id: "slider-budget-range".into(),
                        values: vec![120.0, 360.0],
                        min: 0.0,
                        max: 500.0,
                        step: 10.0,
                        label: "Budget range".into(),
                        show_value: true,
                        ..Default::default()
                    }))
                    div style="display:flex;justify-content:space-between;font-size:0.75rem;color:var(--mui-text-muted);" {
                        span { "$0" }
                        span { "$500" }
                    }
                }
            }

            // Volume — with speaker icons on ends
            div {
                p.mui-showcase__caption { "Volume" }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:22rem;" {
                    div style="display:flex;justify-content:space-between;align-items:center;" {
                        label for="slider-volume" style="font-size:0.875rem;color:var(--mui-text);font-weight:500;" { "Volume" }
                        span style="font-size:0.875rem;font-weight:500;color:var(--mui-text);" { "72" }
                    }
                    div style="display:flex;align-items:center;gap:0.75rem;" {
                        // Speaker muted icon
                        svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color:var(--mui-text-muted);flex-shrink:0" {
                            path d="M11 5 6 9H2v6h4l5 4z" {}
                            line x1="22" y1="9" x2="16" y2="15" {}
                            line x1="16" y1="9" x2="22" y2="15" {}
                        }
                        div style="flex:1;" {
                            (render(Props {
                                name: "volume".into(),
                                id: "slider-volume".into(),
                                value: 72.0,
                                min: 0.0,
                                max: 100.0,
                                step: 1.0,
                                label: "Volume".into(),
                                show_value: false,
                                ..Default::default()
                            }))
                        }
                        // Speaker loud icon
                        svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color:var(--mui-text);flex-shrink:0" {
                            path d="M11 5 6 9H2v6h4l5 4z" {}
                            path d="M15.54 8.46a5 5 0 0 1 0 7.07" {}
                            path d="M19.07 4.93a10 10 0 0 1 0 14.14" {}
                        }
                    }
                }
            }

            // Brightness
            div {
                p.mui-showcase__caption { "Brightness" }
                div style="display:flex;flex-direction:column;gap:0.5rem;max-width:22rem;" {
                    div style="display:flex;justify-content:space-between;align-items:center;" {
                        label for="slider-brightness" style="font-size:0.875rem;color:var(--mui-text);font-weight:500;" { "Brightness" }
                        span style="font-size:0.875rem;font-weight:500;color:var(--mui-text);" { "40%" }
                    }
                    div style="display:flex;align-items:center;gap:0.75rem;" {
                        // Sun dim icon
                        svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color:var(--mui-text-muted);flex-shrink:0" {
                            circle cx="12" cy="12" r="4" {}
                            path d="M12 2v2" {}
                            path d="M12 20v2" {}
                            path d="m4.93 4.93 1.41 1.41" {}
                            path d="m17.66 17.66 1.41 1.41" {}
                            path d="M2 12h2" {}
                            path d="M20 12h2" {}
                            path d="m6.34 17.66-1.41 1.41" {}
                            path d="m19.07 4.93-1.41 1.41" {}
                        }
                        div style="flex:1;" {
                            (render(Props {
                                name: "brightness".into(),
                                id: "slider-brightness".into(),
                                value: 40.0,
                                min: 0.0,
                                max: 100.0,
                                step: 1.0,
                                label: "Brightness".into(),
                                show_value: false,
                                ..Default::default()
                            }))
                        }
                        // Sun bright icon
                        svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" style="color:var(--mui-text);flex-shrink:0" {
                            circle cx="12" cy="12" r="4" {}
                            path d="M12 2v2" {}
                            path d="M12 20v2" {}
                            path d="m4.93 4.93 1.41 1.41" {}
                            path d="m17.66 17.66 1.41 1.41" {}
                            path d="M2 12h2" {}
                            path d="M20 12h2" {}
                            path d="m6.34 17.66-1.41 1.41" {}
                            path d="m19.07 4.93-1.41 1.41" {}
                        }
                    }
                }
            }

            // Vertical slider (P2 demo)
            div {
                p.mui-showcase__caption { "Vertical slider" }
                div style="display:flex;align-items:flex-end;gap:1.5rem;" {
                    div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;" {
                        (render(Props {
                            name: "equalizer-low".into(),
                            id: "slider-eq-low".into(),
                            value: 35.0,
                            min: 0.0,
                            max: 100.0,
                            step: 1.0,
                            label: "Low frequency".into(),
                            orientation: Orientation::Vertical,
                            ..Default::default()
                        }))
                        span style="font-size:0.75rem;color:var(--mui-text-muted);" { "Low" }
                    }
                    div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;" {
                        (render(Props {
                            name: "equalizer-mid".into(),
                            id: "slider-eq-mid".into(),
                            value: 60.0,
                            min: 0.0,
                            max: 100.0,
                            step: 1.0,
                            label: "Mid frequency".into(),
                            orientation: Orientation::Vertical,
                            ..Default::default()
                        }))
                        span style="font-size:0.75rem;color:var(--mui-text-muted);" { "Mid" }
                    }
                    div style="display:flex;flex-direction:column;align-items:center;gap:0.5rem;" {
                        (render(Props {
                            name: "equalizer-high".into(),
                            id: "slider-eq-high".into(),
                            value: 80.0,
                            min: 0.0,
                            max: 100.0,
                            step: 1.0,
                            label: "High frequency".into(),
                            orientation: Orientation::Vertical,
                            ..Default::default()
                        }))
                        span style="font-size:0.75rem;color:var(--mui-text-muted);" { "High" }
                    }
                }
            }
        }
    }
}
