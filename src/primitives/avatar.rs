//! Avatar component — maud-ui Wave 1

use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Props {
    pub src: Option<String>,
    pub alt: String,
    pub fallback: String,
    pub size: Size,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            src: None,
            alt: "Avatar".to_string(),
            fallback: "U".to_string(),
            size: Size::Md,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Size {
    Sm,
    Md,
    Lg,
}

impl Size {
    fn class_name(self) -> &'static str {
        match self {
            Size::Sm => "mui-avatar--sm",
            Size::Md => "mui-avatar--md",
            Size::Lg => "mui-avatar--lg",
        }
    }
}

pub fn render(props: Props) -> Markup {
    let size_class = props.size.class_name();
    let class = format!("mui-avatar {}", size_class);

    html! {
        span class=(class) role="img" aria-label=(props.alt) {
            @if let Some(src) = &props.src {
                img class="mui-avatar__img" src=(src) alt="" {}
            } @else {
                span class="mui-avatar__fallback" aria-hidden="true" {
                    (props.fallback.to_uppercase())
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            // User card — avatar with name and email
            section {
                h2 { "User Card" }
                div style="display:flex;align-items:center;gap:0.75rem;" {
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=12".to_string()),
                        alt: "Sofia Davis".to_string(),
                        fallback: "SD".to_string(),
                        size: Size::Lg,
                    }))
                    div {
                        div style="font-size:0.875rem;font-weight:500;color:var(--mui-text);" {
                            "Sofia Davis"
                        }
                        div style="font-size:0.8125rem;color:var(--mui-text-muted);" {
                            "sofia@example.com"
                        }
                    }
                }
            }

            // Avatar with status indicator
            section {
                h2 { "Status Indicator" }
                div.mui-showcase__row {
                    // Online
                    span style="position:relative;display:inline-block;" {
                        (render(Props {
                            src: Some("https://i.pravatar.cc/120?img=20".to_string()),
                            alt: "Online user".to_string(),
                            fallback: "ON".to_string(),
                            size: Size::Md,
                        }))
                        span style="position:absolute;bottom:0;right:0;width:0.625rem;height:0.625rem;background:#22c55e;border:2px solid var(--mui-bg);border-radius:var(--mui-radius-full);" {}
                    }
                    // Away
                    span style="position:relative;display:inline-block;" {
                        (render(Props {
                            src: Some("https://i.pravatar.cc/120?img=33".to_string()),
                            alt: "Away user".to_string(),
                            fallback: "AW".to_string(),
                            size: Size::Md,
                        }))
                        span style="position:absolute;bottom:0;right:0;width:0.625rem;height:0.625rem;background:#eab308;border:2px solid var(--mui-bg);border-radius:var(--mui-radius-full);" {}
                    }
                    // Offline (fallback)
                    span style="position:relative;display:inline-block;" {
                        (render(Props {
                            src: None,
                            alt: "Offline user".to_string(),
                            fallback: "JD".to_string(),
                            size: Size::Md,
                        }))
                        span style="position:absolute;bottom:0;right:0;width:0.625rem;height:0.625rem;background:var(--mui-text-muted);border:2px solid var(--mui-bg);border-radius:var(--mui-radius-full);" {}
                    }
                }
            }

            // Team avatar group
            section {
                h2 { "Team" }
                div.mui-avatar-group {
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=5".to_string()),
                        alt: "Alice".to_string(),
                        fallback: "A".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=8".to_string()),
                        alt: "Bob".to_string(),
                        fallback: "B".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=15".to_string()),
                        alt: "Carol".to_string(),
                        fallback: "C".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: None,
                        alt: "3 more members".to_string(),
                        fallback: "+3".to_string(),
                        size: Size::Md,
                    }))
                }
            }

            // Sizes and fallbacks
            section {
                h2 { "Sizes" }
                div.mui-showcase__row {
                    span.mui-showcase__label { "sm" }
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=12".to_string()),
                        alt: "Small avatar".to_string(),
                        fallback: "SM".to_string(),
                        size: Size::Sm,
                    }))
                    (render(Props {
                        src: None,
                        alt: "Small fallback".to_string(),
                        fallback: "SM".to_string(),
                        size: Size::Sm,
                    }))
                }
                div.mui-showcase__row {
                    span.mui-showcase__label { "md" }
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=20".to_string()),
                        alt: "Medium avatar".to_string(),
                        fallback: "MD".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: None,
                        alt: "Medium fallback".to_string(),
                        fallback: "MD".to_string(),
                        size: Size::Md,
                    }))
                }
                div.mui-showcase__row {
                    span.mui-showcase__label { "lg" }
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=33".to_string()),
                        alt: "Large avatar".to_string(),
                        fallback: "LG".to_string(),
                        size: Size::Lg,
                    }))
                    (render(Props {
                        src: None,
                        alt: "Large fallback".to_string(),
                        fallback: "LG".to_string(),
                        size: Size::Lg,
                    }))
                }
            }
        }
    }
}
