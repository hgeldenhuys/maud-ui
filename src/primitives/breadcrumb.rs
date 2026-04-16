//! Breadcrumb component — navigation breadcrumb trail showing hierarchy.
use maud::{html, Markup};

/// Breadcrumb item with label and optional href
#[derive(Debug, Clone)]
pub struct BreadcrumbItem {
    /// Display text for the breadcrumb
    pub label: String,
    /// Optional href (None for current page, the last item)
    pub href: Option<String>,
}

/// Breadcrumb rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// List of breadcrumb items (last item has no href)
    pub items: Vec<BreadcrumbItem>,
    /// Separator character (default "/")
    pub separator: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            separator: None,
        }
    }
}

/// Render breadcrumb navigation
pub fn render(props: Props) -> Markup {
    let sep = props.separator.as_deref().unwrap_or("/");

    html! {
        nav class="mui-breadcrumb" aria-label="Breadcrumb" {
            ol class="mui-breadcrumb__list" {
                @for (idx, item) in props.items.iter().enumerate() {
                    @if idx > 0 {
                        li class="mui-breadcrumb__separator" aria-hidden="true" {
                            (sep)
                        }
                    }
                    @if item.href.is_some() {
                        li class="mui-breadcrumb__item" {
                            a href=(item.href.as_ref().unwrap()) {
                                (item.label)
                            }
                        }
                    } @else {
                        li class="mui-breadcrumb__item mui-breadcrumb__item--current" aria-current="page" {
                            span { (item.label) }
                        }
                    }
                }
            }
        }
    }
}

/// Showcase all breadcrumb use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Default separator" }
                (render(Props {
                    items: vec![
                        BreadcrumbItem {
                            label: "Home".into(),
                            href: Some("/".into()),
                        },
                        BreadcrumbItem {
                            label: "Components".into(),
                            href: Some("/docs/components".into()),
                        },
                        BreadcrumbItem {
                            label: "Breadcrumb".into(),
                            href: None,
                        },
                    ],
                    separator: None,
                }))
            }

            div {
                p.mui-showcase__caption { "Chevron separator" }
                (render(Props {
                    items: vec![
                        BreadcrumbItem {
                            label: "Home".into(),
                            href: Some("/".into()),
                        },
                        BreadcrumbItem {
                            label: "Products".into(),
                            href: Some("/products".into()),
                        },
                        BreadcrumbItem {
                            label: "Electronics".into(),
                            href: Some("/products/electronics".into()),
                        },
                        BreadcrumbItem {
                            label: "Headphones".into(),
                            href: None,
                        },
                    ],
                    separator: Some("\u{203a}".into()),
                }))
            }

            div {
                p.mui-showcase__caption { "Two levels" }
                (render(Props {
                    items: vec![
                        BreadcrumbItem {
                            label: "Docs".into(),
                            href: Some("/docs".into()),
                        },
                        BreadcrumbItem {
                            label: "Getting Started".into(),
                            href: None,
                        },
                    ],
                    separator: None,
                }))
            }
        }
    }
}
