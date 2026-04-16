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
            section {
                h2 { "Sizes" }
                div.mui-showcase__row {
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=12".to_string()),
                        alt: "User avatar Sm".to_string(),
                        fallback: "AB".to_string(),
                        size: Size::Sm,
                    }))
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=20".to_string()),
                        alt: "User avatar Md".to_string(),
                        fallback: "CD".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=33".to_string()),
                        alt: "User avatar Lg".to_string(),
                        fallback: "EF".to_string(),
                        size: Size::Lg,
                    }))
                }
            }
            section {
                h2 { "Fallback" }
                div.mui-showcase__row {
                    (render(Props {
                        src: None,
                        alt: "Fallback avatar Sm".to_string(),
                        fallback: "AB".to_string(),
                        size: Size::Sm,
                    }))
                    (render(Props {
                        src: None,
                        alt: "Fallback avatar Md".to_string(),
                        fallback: "CD".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: None,
                        alt: "Fallback avatar Lg".to_string(),
                        fallback: "EF".to_string(),
                        size: Size::Lg,
                    }))
                }
            }
            section {
                h2 { "Avatar Group" }
                div.mui-avatar-group {
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=5".to_string()),
                        alt: "Team member 1".to_string(),
                        fallback: "A".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=8".to_string()),
                        alt: "Team member 2".to_string(),
                        fallback: "B".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: Some("https://i.pravatar.cc/120?img=15".to_string()),
                        alt: "Team member 3".to_string(),
                        fallback: "C".to_string(),
                        size: Size::Md,
                    }))
                    (render(Props {
                        src: None,
                        alt: "2 more members".to_string(),
                        fallback: "+2".to_string(),
                        size: Size::Md,
                    }))
                }
            }
        }
    }
}
