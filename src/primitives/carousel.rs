//! Carousel component — horizontal slide viewer with arrow/dot navigation.

use maud::{html, Markup};

/// Carousel rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique element id
    pub id: String,
    /// Each slide's content as Markup
    pub items: Vec<Markup>,
    /// Show dot indicators at bottom (default true)
    pub show_dots: bool,
    /// Show prev/next arrows (default true)
    pub show_arrows: bool,
    /// Wrap around when reaching the end (default false)
    pub loop_slides: bool,
    /// Auto-advance slides (default false)
    pub auto_play: bool,
    /// Accessible label for the carousel region
    pub aria_label: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "carousel".to_string(),
            items: vec![],
            show_dots: true,
            show_arrows: true,
            loop_slides: false,
            auto_play: false,
            aria_label: "Carousel".to_string(),
        }
    }
}

/// Render a carousel with the given properties
pub fn render(props: Props) -> Markup {
    let total = props.items.len();
    let loop_str = if props.loop_slides { "true" } else { "false" };
    let autoplay_str = if props.auto_play { "true" } else { "false" };

    html! {
        div class="mui-carousel"
            data-mui="carousel"
            id=(props.id)
            role="region"
            aria-roledescription="carousel"
            aria-label=(props.aria_label)
            data-loop=(loop_str)
            data-autoplay=(autoplay_str)
        {
            div class="mui-carousel__viewport" {
                div class="mui-carousel__container" aria-live="off" {
                    @for (i, item) in props.items.iter().enumerate() {
                        div class="mui-carousel__slide"
                            role="group"
                            aria-roledescription="slide"
                            aria-label=(format!("Slide {} of {}", i + 1, total))
                        {
                            (item)
                        }
                    }
                }
            }
            @if props.show_arrows {
                button type="button" class="mui-carousel__prev" aria-label="Previous slide" {
                    "\u{2039}"
                }
                button type="button" class="mui-carousel__next" aria-label="Next slide" {
                    "\u{203A}"
                }
            }
            @if props.show_dots {
                div class="mui-carousel__dots" role="tablist" {
                    @for i in 0..total {
                        button type="button"
                            class=(if i == 0 { "mui-carousel__dot mui-carousel__dot--active" } else { "mui-carousel__dot" })
                            role="tab"
                            aria-selected=(if i == 0 { "true" } else { "false" })
                            aria-label=(format!("Go to slide {}", i + 1))
                        {}
                    }
                }
            }
        }
    }
}

/// Showcase carousel variants
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Default: arrows + dots, 5 colored slides
            div {
                h3 class="mui-showcase__caption" { "Default (arrows + dots)" }
                (render(Props {
                    id: "demo-carousel-1".to_string(),
                    items: vec![
                        html! {
                            div style="background: linear-gradient(135deg, #3b82f6, #6366f1); color: #fff; display: flex; align-items: center; justify-content: center; height: 12rem; font-size: 1.25rem; font-weight: 600; border-radius: var(--mui-radius-lg);" {
                                "Slide 1"
                            }
                        },
                        html! {
                            div style="background: linear-gradient(135deg, #8b5cf6, #ec4899); color: #fff; display: flex; align-items: center; justify-content: center; height: 12rem; font-size: 1.25rem; font-weight: 600; border-radius: var(--mui-radius-lg);" {
                                "Slide 2"
                            }
                        },
                        html! {
                            div style="background: linear-gradient(135deg, #f59e0b, #ef4444); color: #fff; display: flex; align-items: center; justify-content: center; height: 12rem; font-size: 1.25rem; font-weight: 600; border-radius: var(--mui-radius-lg);" {
                                "Slide 3"
                            }
                        },
                        html! {
                            div style="background: linear-gradient(135deg, #10b981, #3b82f6); color: #fff; display: flex; align-items: center; justify-content: center; height: 12rem; font-size: 1.25rem; font-weight: 600; border-radius: var(--mui-radius-lg);" {
                                "Slide 4"
                            }
                        },
                        html! {
                            div style="background: linear-gradient(135deg, #f43f5e, #a855f7); color: #fff; display: flex; align-items: center; justify-content: center; height: 12rem; font-size: 1.25rem; font-weight: 600; border-radius: var(--mui-radius-lg);" {
                                "Slide 5"
                            }
                        },
                    ],
                    show_dots: true,
                    show_arrows: true,
                    loop_slides: false,
                    auto_play: false,
                    aria_label: "Demo carousel with arrows and dots".to_string(),
                }))
            }

            // Without dots
            div {
                h3 class="mui-showcase__caption" { "Arrows only (no dots)" }
                (render(Props {
                    id: "demo-carousel-2".to_string(),
                    items: vec![
                        html! {
                            div style="background: linear-gradient(135deg, #0ea5e9, #06b6d4); color: #fff; display: flex; align-items: center; justify-content: center; height: 12rem; font-size: 1.25rem; font-weight: 600; border-radius: var(--mui-radius-lg);" {
                                "Slide A"
                            }
                        },
                        html! {
                            div style="background: linear-gradient(135deg, #14b8a6, #22c55e); color: #fff; display: flex; align-items: center; justify-content: center; height: 12rem; font-size: 1.25rem; font-weight: 600; border-radius: var(--mui-radius-lg);" {
                                "Slide B"
                            }
                        },
                        html! {
                            div style="background: linear-gradient(135deg, #a855f7, #6366f1); color: #fff; display: flex; align-items: center; justify-content: center; height: 12rem; font-size: 1.25rem; font-weight: 600; border-radius: var(--mui-radius-lg);" {
                                "Slide C"
                            }
                        },
                    ],
                    show_dots: false,
                    show_arrows: true,
                    loop_slides: true,
                    auto_play: false,
                    aria_label: "Demo carousel with arrows only".to_string(),
                }))
            }
        }
    }
}
