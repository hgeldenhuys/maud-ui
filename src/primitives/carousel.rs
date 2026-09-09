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
                div class="mui-carousel__container" aria-live=(if props.auto_play { "polite" } else { "off" }) {
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

/// Render one product slide: product mark, name, price, component link.
fn product_slide(name: &str, price: &str, glyph: &str) -> Markup {
    html! {
        div style="display:flex;flex-direction:column;gap: var(--mui-space-md);padding: var(--mui-space-md);" {
            // Neutral product placeholder follows the active palette.
            div style="background: var(--mui-bg-input); color: var(--mui-text-secondary); border-radius: var(--mui-radius-lg); height: 10rem; display: flex; align-items: center; justify-content: center;" {
                span data-mui-type="display" { (glyph) }
            }
            div style="display:flex;justify-content:space-between;align-items:flex-start;gap: var(--mui-space-sm);" {
                div style="min-width:0;" {
                    p data-mui-type="body" style="font-weight: var(--mui-weight-heading);margin: 0;" { (name) }
                    p data-mui-type="small" style="color:var(--mui-text-muted);margin: var(--mui-space-2xs) 0 0;" { "Free shipping over $50" }
                }
                span data-mui-type="body" style="font-weight: var(--mui-weight-heading);" { (price) }
            }
            a href="/card" class="mui-btn mui-btn--outline mui-btn--sm" style="align-self:flex-start;" {
                "View card pattern →"
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
                        product_slide("Aurora Wireless Headphones", "$149", "AU"),
                        product_slide("Orbit Smart Watch", "$249", "OR"),
                        product_slide("Nimbus Desk Lamp", "$79", "NI"),
                        product_slide("Meridian Leather Wallet", "$64", "ME"),
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
                            div style="padding: var(--mui-space-20);display:flex;flex-direction:column;gap: var(--mui-space-md);min-height:10rem;" {
                                p data-mui-type="body" style="line-height: var(--mui-leading-small);margin: 0;font-style:italic;color:var(--mui-text);" {
                                    "\u{201C}Charging speed is absurd. Went from 0 to 80% during my morning coffee.\u{201D}"
                                }
                                div style="display:flex;align-items:center;gap: var(--mui-inset-cell);margin-top:auto;" {
                                    div data-mui-type="small" style="width:2rem;height:2rem;border-radius: var(--mui-radius-full);background:var(--mui-bg-input);color:var(--mui-text-secondary);display:flex;align-items:center;justify-content:center;font-weight: var(--mui-weight-heading);" { "SM" }
                                    div {
                                        p data-mui-type="small" style="font-weight: var(--mui-weight-medium);margin: 0;" { "Sofia Martinez" }
                                        p data-mui-type="caption" style="color:var(--mui-text-muted);margin: 0;" { "Verified buyer \u{00B7} 5 stars" }
                                    }
                                }
                            }
                        },
                        html! {
                            div style="padding: var(--mui-space-20);display:flex;flex-direction:column;gap: var(--mui-space-md);min-height:10rem;" {
                                p data-mui-type="body" style="line-height: var(--mui-leading-small);margin: 0;font-style:italic;color:var(--mui-text);" {
                                    "\u{201C}Sound isolation is the best I've tried under $200. Worth every penny.\u{201D}"
                                }
                                div style="display:flex;align-items:center;gap: var(--mui-inset-cell);margin-top:auto;" {
                                    div data-mui-type="small" style="width:2rem;height:2rem;border-radius: var(--mui-radius-full);background:var(--mui-bg-input);color:var(--mui-text-secondary);display:flex;align-items:center;justify-content:center;font-weight: var(--mui-weight-heading);" { "DK" }
                                    div {
                                        p data-mui-type="small" style="font-weight: var(--mui-weight-medium);margin: 0;" { "Daniel Kim" }
                                        p data-mui-type="caption" style="color:var(--mui-text-muted);margin: 0;" { "Verified buyer \u{00B7} 5 stars" }
                                    }
                                }
                            }
                        },
                        html! {
                            div style="padding: var(--mui-space-20);display:flex;flex-direction:column;gap: var(--mui-space-md);min-height:10rem;" {
                                p data-mui-type="body" style="line-height: var(--mui-leading-small);margin: 0;font-style:italic;color:var(--mui-text);" {
                                    "\u{201C}Returned two other pairs before this one. Comfortable for full workdays.\u{201D}"
                                }
                                div style="display:flex;align-items:center;gap: var(--mui-inset-cell);margin-top:auto;" {
                                    div data-mui-type="small" style="width:2rem;height:2rem;border-radius: var(--mui-radius-full);background:var(--mui-bg-input);color:var(--mui-text-secondary);display:flex;align-items:center;justify-content:center;font-weight: var(--mui-weight-heading);" { "AP" }
                                    div {
                                        p data-mui-type="small" style="font-weight: var(--mui-weight-medium);margin: 0;" { "Amelia Park" }
                                        p data-mui-type="caption" style="color:var(--mui-text-muted);margin: 0;" { "Verified buyer \u{00B7} 4 stars" }
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
