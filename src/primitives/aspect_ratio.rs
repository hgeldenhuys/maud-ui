//! AspectRatio component — maintains a fixed aspect ratio for child elements.

use maud::{html, Markup};

/// AspectRatio rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Aspect ratio as width/height (e.g., 16.0/9.0 ≈ 1.778, 1.0 for square)
    pub ratio: f64,
    /// Child markup to be contained within the aspect ratio box
    pub children: Markup,
}

impl Props {
    pub fn new(ratio: f64, children: Markup) -> Self {
        Self { ratio, children }
    }

    /// 16:9 aspect ratio (common for video)
    pub fn video(children: Markup) -> Self {
        Self {
            ratio: 16.0 / 9.0,
            children,
        }
    }

    /// 4:3 aspect ratio
    pub fn four_by_three(children: Markup) -> Self {
        Self {
            ratio: 4.0 / 3.0,
            children,
        }
    }

    /// 1:1 square aspect ratio
    pub fn square(children: Markup) -> Self {
        Self {
            ratio: 1.0,
            children,
        }
    }
}

/// Render an aspect ratio container that constrains its children to maintain a fixed ratio
pub fn render(props: Props) -> Markup {
    let style = format!("--mui-aspect-ratio: {}", props.ratio);

    html! {
        div.mui-aspect-ratio style=(style) {
            (props.children)
        }
    }
}

/// Showcase all aspect ratio variants
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Common ratios" }
                div style="display: grid; grid-template-columns: repeat(3, 1fr); gap: 1rem;" {
                    div {
                        p.mui-showcase__caption { "16:9" }
                        (render(Props::video(
                            html! {
                                div style="background: linear-gradient(135deg, var(--mui-accent) 0%, #764ba2 100%); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 0.875rem;" {
                                    "16:9"
                                }
                            }
                        )))
                    }
                    div {
                        p.mui-showcase__caption { "4:3" }
                        (render(Props::four_by_three(
                            html! {
                                div style="background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 0.875rem;" {
                                    "4:3"
                                }
                            }
                        )))
                    }
                    div {
                        p.mui-showcase__caption { "1:1" }
                        (render(Props::square(
                            html! {
                                div style="background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%); width: 100%; height: 100%; display: flex; align-items: center; justify-content: center; color: white; font-weight: 600; font-size: 0.875rem;" {
                                    "1:1"
                                }
                            }
                        )))
                    }
                }
            }
            section {
                h2 { "With image" }
                div style="max-width: 24rem;" {
                    (render(Props::video(
                        html! {
                            img src="https://images.unsplash.com/photo-1588345921523-c2dcdb7f1dcd?w=800&dpr=2&q=80" alt="Minimalist white shapes" style="width: 100%; height: 100%; object-fit: cover;" {}
                        }
                    )))
                }
            }
        }
    }
}
