//! `pricing::tiers` — 3-column pricing comparison card row.
//!
//! Highlighted middle tier ("Most popular") with subtle accent border +
//! badge. Each tier has heading, short description, price, billing
//! period, feature list, CTA button.
//!
//! Composes: `card`, `badge`, `button`. No JS.

use maud::{html, Markup, PreEscaped};

use crate::primitives::{badge, button};

#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Section heading (above the cards). `None` hides the header row.
    pub heading: Option<String>,
    pub subheading: Option<String>,
    /// The tiers, rendered in order. 3 is the usual count but any
    /// number works — the grid adjusts.
    pub tiers: Vec<Tier>,
    /// Optional fine-print line below the cards (e.g. "All plans
    /// include a 14-day free trial. No credit card required.").
    pub fine_print: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Tier {
    pub name: String,
    pub tagline: String,
    /// Big price display — pre-formatted ("$29", "$290"). Keep the
    /// currency symbol here; the period gets appended in the UI.
    pub price: String,
    /// Billing period word — shown after the price ("/month",
    /// "/year"). Typical values: "/month", "/year", "forever" (for
    /// free tiers; price "$0" + period "forever" reads naturally).
    pub price_period: String,
    /// Bullet list of features. Each renders with a check glyph.
    pub features: Vec<String>,
    /// Call-to-action button label.
    pub cta_label: String,
    /// CTA URL (GET) or form action. When `cta_form` is `true`, the
    /// block wraps the button in a POST form — useful for "Start
    /// trial" that needs CSRF.
    pub cta_href: String,
    pub cta_form: bool,
    /// Mark this tier as the highlighted one. Visual bump +
    /// "Most popular" badge. Only one tier should have this true.
    pub highlighted: bool,
}

pub fn render(props: Props) -> Markup {
    html! {
        section class="mui-block mui-block--pricing" {
            @if props.heading.is_some() || props.subheading.is_some() {
                header class="mui-block--pricing__header" {
                    @if let Some(h) = &props.heading {
                        h2 class="mui-block--pricing__heading" { (h) }
                    }
                    @if let Some(s) = &props.subheading {
                        p class="mui-block--pricing__subheading" { (s) }
                    }
                }
            }

            div class="mui-block--pricing__grid" {
                @for tier in &props.tiers {
                    div class=(if tier.highlighted {
                        "mui-block--pricing__tier mui-block--pricing__tier--highlighted"
                    } else {
                        "mui-block--pricing__tier"
                    }) {
                        @if tier.highlighted {
                            div class="mui-block--pricing__tier-badge" {
                                (badge::render(badge::Props {
                                    label: "Most popular".into(),
                                    variant: badge::Variant::Default,
                                }))
                            }
                        }

                        h3 class="mui-block--pricing__tier-name" { (tier.name) }
                        p class="mui-block--pricing__tier-tagline" { (tier.tagline) }

                        div class="mui-block--pricing__tier-price" {
                            span class="mui-block--pricing__tier-price-amount" { (tier.price) }
                            span class="mui-block--pricing__tier-price-period" {
                                " " (tier.price_period)
                            }
                        }

                        ul class="mui-block--pricing__tier-features" {
                            @for f in &tier.features {
                                li {
                                    span class="mui-block--pricing__tier-check" aria-hidden="true" {
                                        (icon_check())
                                    }
                                    span { (f) }
                                }
                            }
                        }

                        @if tier.cta_form {
                            form action=(tier.cta_href) method="post"
                                 class="mui-block--pricing__tier-cta" {
                                (button::render(button::Props {
                                    label: tier.cta_label.clone(),
                                    variant: if tier.highlighted {
                                        button::Variant::Primary
                                    } else {
                                        button::Variant::Outline
                                    },
                                    size: button::Size::Md,
                                    button_type: "submit",
                                    ..Default::default()
                                }))
                            }
                        } @else {
                            a href=(tier.cta_href)
                              class=(if tier.highlighted {
                                  "mui-btn mui-btn--primary mui-btn--md mui-block--pricing__tier-cta-link"
                              } else {
                                  "mui-btn mui-btn--outline mui-btn--md mui-block--pricing__tier-cta-link"
                              })
                              style="text-decoration:none;justify-content:center;" {
                                (tier.cta_label)
                            }
                        }
                    }
                }
            }

            @if let Some(fp) = &props.fine_print {
                p class="mui-block--pricing__fine-print" { (fp) }
            }
        }
    }
}

pub fn preview() -> Markup {
    render(Props {
        heading: Some("Simple, transparent pricing".into()),
        subheading: Some("Start free, upgrade when you\u{2019}re ready. No hidden fees.".into()),
        tiers: vec![
            Tier {
                name: "Starter".into(),
                tagline: "For individuals getting started.".into(),
                price: "$0".into(),
                price_period: "forever".into(),
                features: vec![
                    "1 project".into(),
                    "Up to 3 teammates".into(),
                    "Community support".into(),
                    "Basic analytics".into(),
                ],
                cta_label: "Get started".into(),
                cta_href: "/signup?plan=starter".into(),
                cta_form: false,
                highlighted: false,
            },
            Tier {
                name: "Pro".into(),
                tagline: "For growing teams that ship.".into(),
                price: "$79".into(),
                price_period: "/month".into(),
                features: vec![
                    "Unlimited projects".into(),
                    "Up to 10 teammates".into(),
                    "Priority email support".into(),
                    "Advanced analytics + exports".into(),
                    "SSO via Google Workspace".into(),
                ],
                cta_label: "Start 14-day trial".into(),
                cta_href: "/signup?plan=pro".into(),
                cta_form: true,
                highlighted: true,
            },
            Tier {
                name: "Enterprise".into(),
                tagline: "For companies with serious scale.".into(),
                price: "Custom".into(),
                price_period: String::new(),
                features: vec![
                    "Everything in Pro".into(),
                    "Unlimited teammates".into(),
                    "Dedicated account manager".into(),
                    "SAML SSO + SCIM".into(),
                    "99.99% uptime SLA".into(),
                    "Custom terms".into(),
                ],
                cta_label: "Contact sales".into(),
                cta_href: "/contact".into(),
                cta_form: false,
                highlighted: false,
            },
        ],
        fine_print: Some("All plans include a 14-day free trial. Cancel anytime. No credit card required to start.".into()),
    })
}

fn icon_check() -> Markup {
    PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"><polyline points="20 6 9 17 4 12"/></svg>"##.to_string())
}
