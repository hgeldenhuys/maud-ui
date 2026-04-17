//! Carousel component — horizontal slide viewer with arrow/dot navigation.

use maud::{html, Markup, PreEscaped};

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
                    (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>"#))
                }
                button type="button" class="mui-carousel__next" aria-label="Next slide" {
                    (PreEscaped(r#"<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#))
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

/// Render one product slide: SVG placeholder, name, price, CTA.
fn product_slide(name: &str, price: &str, gradient: &str, glyph: &str) -> Markup {
    html! {
        div style="display:flex;flex-direction:column;gap:0.75rem;padding:0.75rem;" {
            // Product image — SVG placeholder inside gradient
            div style=(format!("background: linear-gradient(135deg, {}); border-radius: var(--mui-radius-lg); height: 10rem; display: flex; align-items: center; justify-content: center;", gradient)) {
                span style="font-size:3rem;filter:drop-shadow(0 2px 8px rgba(0,0,0,0.2));" { (glyph) }
            }
            div style="display:flex;justify-content:space-between;align-items:flex-start;gap:0.5rem;" {
                div style="min-width:0;" {
                    p style="font-size:0.9375rem;font-weight:600;margin:0;" { (name) }
                    p style="font-size:0.8125rem;color:var(--mui-text-muted);margin:0.125rem 0 0;" { "Free shipping over $50" }
                }
                span style="font-size:0.9375rem;font-weight:600;" { (price) }
            }
            button type="button" style="align-self:flex-start;font-size:0.8125rem;font-weight:500;color:var(--mui-text);background:transparent;border:1px solid var(--mui-border,#e5e7eb);border-radius:var(--mui-radius-md);padding:0.375rem 0.75rem;cursor:pointer;" {
                "View details →"
            }
        }
    }
}

/// Showcase carousel variants
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // Product gallery
            div {
                h3 class="mui-showcase__caption" { "Featured products" }
                (render(Props {
                    id: "demo-carousel-products".to_string(),
                    items: vec![
                        product_slide("Aurora Wireless Headphones", "$149", "#3b82f6, #6366f1", "\u{1F3A7}"),
                        product_slide("Orbit Smart Watch", "$249", "#8b5cf6, #ec4899", "\u{231A}"),
                        product_slide("Nimbus Desk Lamp", "$79", "#f59e0b, #ef4444", "\u{1F4A1}"),
                        product_slide("Meridian Leather Wallet", "$64", "#10b981, #3b82f6", "\u{1F45B}"),
                    ],
                    show_dots: true,
                    show_arrows: true,
                    loop_slides: true,
                    auto_play: false,
                    aria_label: "Featured products".to_string(),
                }))
            }

            // Testimonials carousel — arrows only
            div {
                h3 class="mui-showcase__caption" { "What customers say" }
                (render(Props {
                    id: "demo-carousel-reviews".to_string(),
                    items: vec![
                        html! {
                            div style="padding:1.25rem;display:flex;flex-direction:column;gap:0.75rem;min-height:10rem;" {
                                p style="font-size:1rem;line-height:1.5;margin:0;font-style:italic;color:var(--mui-text);" {
                                    "\u{201C}Charging speed is absurd. Went from 0 to 80% during my morning coffee.\u{201D}"
                                }
                                div style="display:flex;align-items:center;gap:0.625rem;margin-top:auto;" {
                                    div style="width:2rem;height:2rem;border-radius:9999px;background:linear-gradient(135deg,#14b8a6,#22c55e);color:#fff;display:flex;align-items:center;justify-content:center;font-size:0.8125rem;font-weight:600;" { "SM" }
                                    div {
                                        p style="font-size:0.8125rem;font-weight:500;margin:0;" { "Sofia Martinez" }
                                        p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" { "Verified buyer \u{00B7} 5 stars" }
                                    }
                                }
                            }
                        },
                        html! {
                            div style="padding:1.25rem;display:flex;flex-direction:column;gap:0.75rem;min-height:10rem;" {
                                p style="font-size:1rem;line-height:1.5;margin:0;font-style:italic;color:var(--mui-text);" {
                                    "\u{201C}Sound isolation is the best I've tried under $200. Worth every penny.\u{201D}"
                                }
                                div style="display:flex;align-items:center;gap:0.625rem;margin-top:auto;" {
                                    div style="width:2rem;height:2rem;border-radius:9999px;background:linear-gradient(135deg,#a855f7,#6366f1);color:#fff;display:flex;align-items:center;justify-content:center;font-size:0.8125rem;font-weight:600;" { "DK" }
                                    div {
                                        p style="font-size:0.8125rem;font-weight:500;margin:0;" { "Daniel Kim" }
                                        p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" { "Verified buyer \u{00B7} 5 stars" }
                                    }
                                }
                            }
                        },
                        html! {
                            div style="padding:1.25rem;display:flex;flex-direction:column;gap:0.75rem;min-height:10rem;" {
                                p style="font-size:1rem;line-height:1.5;margin:0;font-style:italic;color:var(--mui-text);" {
                                    "\u{201C}Returned two other pairs before this one. Comfortable for full workdays.\u{201D}"
                                }
                                div style="display:flex;align-items:center;gap:0.625rem;margin-top:auto;" {
                                    div style="width:2rem;height:2rem;border-radius:9999px;background:linear-gradient(135deg,#f43f5e,#f59e0b);color:#fff;display:flex;align-items:center;justify-content:center;font-size:0.8125rem;font-weight:600;" { "AP" }
                                    div {
                                        p style="font-size:0.8125rem;font-weight:500;margin:0;" { "Amelia Park" }
                                        p style="font-size:0.75rem;color:var(--mui-text-muted);margin:0;" { "Verified buyer \u{00B7} 4 stars" }
                                    }
                                }
                            }
                        },
                    ],
                    show_dots: false,
                    show_arrows: true,
                    loop_slides: true,
                    auto_play: false,
                    aria_label: "Customer testimonials".to_string(),
                }))
            }
        }
    }
}
