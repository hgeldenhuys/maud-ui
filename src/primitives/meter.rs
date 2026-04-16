//! Meter component — measurement indicator with zones (low/optimal/high).

use maud::{html, Markup};

/// Meter rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Current value of the meter
    pub value: f64,
    /// Minimum value (default 0.0)
    pub min: f64,
    /// Maximum value (default 100.0)
    pub max: f64,
    /// Threshold below which the zone is "suboptimum" (warning)
    pub low: Option<f64>,
    /// Threshold above which the zone is "suboptimum" (warning)
    pub high: Option<f64>,
    /// The ideal value; if present, determines which side is "good"
    pub optimum: Option<f64>,
    /// Label for the meter (accessibility)
    pub label: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 100.0,
            low: None,
            high: None,
            optimum: None,
            label: String::new(),
        }
    }
}

/// Determine the zone class based on value, thresholds, and optimum.
fn zone_class(value: f64, min: f64, max: f64, low: Option<f64>, high: Option<f64>, optimum: Option<f64>) -> &'static str {
    // If no optimum, always optimum
    if optimum.is_none() {
        return "optimum";
    }

    let opt = optimum.unwrap();
    let mid = (min + max) / 2.0;

    // Determine which side is ideal based on optimum position
    if opt >= mid {
        // Ideal side is "high" (e.g., battery level — higher is better)
        // low = minimum acceptable, high = good threshold
        if let Some(l) = low {
            if value < l {
                return "danger";
            }
        }
        if let Some(h) = high {
            if value >= h {
                return "optimum";
            }
        }
        "suboptimum"
    } else {
        // Ideal side is "low" (e.g., disk usage — lower is better)
        // Zones are: optimum (before midpoint), suboptimum (midpoint to high), danger (above high)
        if let Some(h) = high {
            if value > h {
                return "danger";
            }
        }
        let zone_threshold = if let (Some(l), Some(h)) = (low, high) {
            (l + h) / 2.0
        } else {
            low.unwrap_or(high.unwrap_or(mid))
        };
        if value <= zone_threshold {
            "optimum"
        } else {
            "suboptimum"
        }
    }
}

/// Render a single meter with the given properties
pub fn render(props: Props) -> Markup {
    let pct = ((props.value - props.min) / (props.max - props.min) * 100.0).clamp(0.0, 100.0);
    let zone = zone_class(props.value, props.min, props.max, props.low, props.high, props.optimum);
    let width_style = format!("width: {:.1}%", pct);

    html! {
        div class="mui-meter" role="meter" aria-valuenow=(props.value) aria-valuemin=(props.min) aria-valuemax=(props.max) aria-label=(props.label) {
            div.mui-meter__track {
                div class={
                    "mui-meter__bar "
                    "mui-meter__bar--" (zone)
                } style=(width_style) {}
            }
        }
    }
}

/// Showcase battery level and disk usage meters
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Battery level (higher is better)
            div {
                p.mui-showcase__caption { "Battery level (higher is better)" }
                div.mui-showcase__row {
                    // Low (danger)
                    div {
                        (render(Props {
                            value: 15.0,
                            min: 0.0,
                            max: 100.0,
                            low: Some(20.0),
                            high: Some(80.0),
                            optimum: Some(100.0),
                            label: "Battery 15%".into(),
                        }))
                        p.mui-showcase__label { "15%" }
                    }
                    // Medium (warning)
                    div {
                        (render(Props {
                            value: 60.0,
                            min: 0.0,
                            max: 100.0,
                            low: Some(20.0),
                            high: Some(80.0),
                            optimum: Some(100.0),
                            label: "Battery 60%".into(),
                        }))
                        p.mui-showcase__label { "60%" }
                    }
                    // High (success)
                    div {
                        (render(Props {
                            value: 90.0,
                            min: 0.0,
                            max: 100.0,
                            low: Some(20.0),
                            high: Some(80.0),
                            optimum: Some(100.0),
                            label: "Battery 90%".into(),
                        }))
                        p.mui-showcase__label { "90%" }
                    }
                }
            }

            // Disk usage (lower is better)
            div {
                p.mui-showcase__caption { "Disk usage (lower is better)" }
                div.mui-showcase__row {
                    // Low (success)
                    div {
                        (render(Props {
                            value: 25.0,
                            min: 0.0,
                            max: 100.0,
                            low: Some(20.0),
                            high: Some(80.0),
                            optimum: Some(0.0),
                            label: "Disk 25%".into(),
                        }))
                        p.mui-showcase__label { "25%" }
                    }
                    // Medium (warning)
                    div {
                        (render(Props {
                            value: 70.0,
                            min: 0.0,
                            max: 100.0,
                            low: Some(20.0),
                            high: Some(80.0),
                            optimum: Some(0.0),
                            label: "Disk 70%".into(),
                        }))
                        p.mui-showcase__label { "70%" }
                    }
                    // High (danger)
                    div {
                        (render(Props {
                            value: 95.0,
                            min: 0.0,
                            max: 100.0,
                            low: Some(20.0),
                            high: Some(80.0),
                            optimum: Some(0.0),
                            label: "Disk 95%".into(),
                        }))
                        p.mui-showcase__label { "95%" }
                    }
                }
            }
        }
    }
}
