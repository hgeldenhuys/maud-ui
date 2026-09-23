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
pub struct Props<'a> {
    /// List of breadcrumb items (last item has no href). Blank labels are discarded.
    pub items: Vec<BreadcrumbItem>,
    /// Separator between items. `None` renders NO separator at all — the
    /// list gap alone separates the trail (a trail with no visible divider).
    /// Defaults to `Some("/")`.
    pub separator: Option<&'a str>,
}

// Written by hand: a derived Default gives `separator: None`, which silently
// dropped the "/" from every trail built with `..Default::default()` (0.19.0).
impl Default for Props<'_> {
    fn default() -> Self {
        Self { items: Vec::new(), separator: Some("/") }
    }
}


/// Render breadcrumb navigation
pub fn render(props: Props) -> Markup {
    let items = visible_items(props.items);
    if items.is_empty() { return html! {}; }

    html! {
        nav class="mui-breadcrumb" aria-label="Breadcrumb" {
            ol class="mui-breadcrumb__list" {
                @for (idx, item) in items.iter().enumerate() {
                    @if idx > 0 {
                        @if let Some(sep) = props.separator {
                            li class="mui-breadcrumb__separator" aria-hidden="true" {
                                (sep)
                            }
                        }
                    }
                    @if item.href.is_some() {
                        li class="mui-breadcrumb__item" data-position=(if idx == items.len() - 1 { "current" } else if idx == 0 { "root" } else { "middle" }) {
                            a href=(item.href.as_ref().unwrap()) title=(&item.label) {
                                (item.label)
                            }
                        }
                    } @else {
                        li class="mui-breadcrumb__item mui-breadcrumb__item--current" data-position="current" {
                            // role="link" + aria-disabled="true" + aria-current="page":
                            // gives SR users link-like semantics on the current page
                            // without making the element activatable. Matches shadcn/Radix.
                            span role="link" aria-disabled="true" aria-current="page" {
                                (item.label)
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Shared with page-header title derivation: empty labels never create trail positions.
pub(crate) fn visible_items(items: Vec<BreadcrumbItem>) -> Vec<BreadcrumbItem> {
    let items: Vec<_> = items.into_iter().filter(|item| !item.label.trim().is_empty()).collect();
    debug_assert!(items.iter().all(|item| !item.label.trim().is_empty()), "Rendered breadcrumbs must have labels");
    items
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
                    separator: Some("/"),
                }))
            }

            div {
                p.mui-showcase__caption { "Product navigation" }
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
                            label: "Phones".into(),
                            href: Some("/products/phones".into()),
                        },
                        BreadcrumbItem {
                            label: "iPhone 15 Pro".into(),
                            href: None,
                        },
                    ],
                    separator: Some("\u{203a}"),
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
                    separator: Some("/"),
                }))
            }

            div {
                p.mui-showcase__caption { "No separator" }
                (render(Props {
                    items: vec![
                        BreadcrumbItem {
                            label: "DRV-8852".into(),
                            href: Some("#".into()),
                        },
                        BreadcrumbItem {
                            label: "Faster app launch".into(),
                            href: None,
                        },
                    ],
                    separator: None,
                }))
            }
        }
    }
}
