//! Slider component — range input with custom styling and progressive enhancement.

use maud::{html, Markup};

/// Slider rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// HTML name attribute for form submission
    pub name: String,
    /// HTML id attribute for label linkage
    pub id: String,
    /// Current value
    pub value: f64,
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
}

impl Default for Props {
    fn default() -> Self {
        Self {
            name: String::new(),
            id: String::new(),
            value: 50.0,
            min: 0.0,
            max: 100.0,
            step: 1.0,
            disabled: false,
            label: String::new(),
            show_value: false,
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

/// Render a slider with the given properties
pub fn render(props: Props) -> Markup {
    let pct = if props.max != props.min {
        ((props.value - props.min) / (props.max - props.min)) * 100.0
    } else {
        0.0
    };
    let display_value = format_value(props.value);

    html! {
        @if props.disabled {
            div.mui-slider data-mui="slider" data-min=(props.min) data-max=(props.max) data-step=(props.step) data-disabled="true" {
                div.mui-slider__track {
                    div.mui-slider__fill style=(format!("width: {}%", pct)) aria-hidden="true" {}
                    div.mui-slider__thumb
                        role="slider"
                        tabindex="0"
                        aria-valuenow=(props.value)
                        aria-valuemin=(props.min)
                        aria-valuemax=(props.max)
                        aria-label=(props.label)
                        aria-disabled="true"
                        style=(format!("left: {}%", pct)) {}
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
            div.mui-slider data-mui="slider" data-min=(props.min) data-max=(props.max) data-step=(props.step) {
                div.mui-slider__track {
                    div.mui-slider__fill style=(format!("width: {}%", pct)) aria-hidden="true" {}
                    div.mui-slider__thumb
                        role="slider"
                        tabindex="0"
                        aria-valuenow=(props.value)
                        aria-valuemin=(props.min)
                        aria-valuemax=(props.max)
                        aria-label=(props.label)
                        style=(format!("left: {}%", pct)) {}
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
        }
    }
}
