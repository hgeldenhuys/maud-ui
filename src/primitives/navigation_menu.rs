//! Navigation Menu component — horizontal nav with dropdown content panels (mega-menu style).

use maud::{html, Markup};

/// Orientation of the navigation menu list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

/// A direct navigation link.
#[derive(Clone, Debug)]
pub struct NavLink {
    pub label: String,
    pub href: String,
    pub description: Option<String>,
}

/// A dropdown menu containing multiple links.
#[derive(Clone, Debug)]
pub struct NavMenu {
    pub label: String,
    pub links: Vec<NavLink>,
}

/// A navigation item: either a direct link or a dropdown menu.
#[derive(Clone, Debug)]
pub enum NavItem {
    Link(NavLink),
    Menu(NavMenu),
}

/// Navigation menu rendering properties.
#[derive(Clone, Debug)]
pub struct Props {
    pub id: String,
    pub items: Vec<NavItem>,
    /// Horizontal (default) or vertical layout of the top-level list.
    pub orientation: Orientation,
    /// When `true` (default), dropdown content is rendered in a shared viewport panel.
    /// When `false`, each trigger carries its own per-item popover wrapper.
    pub viewport: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "nav-menu".to_string(),
            items: vec![],
            orientation: Orientation::Horizontal,
            viewport: true,
        }
    }
}

/// Render a small chevron/arrow between active triggers (NavigationMenuIndicator).
///
/// Purely decorative — `aria-hidden="true"` so screen readers skip it.
pub fn indicator() -> Markup {
    html! {
        span class="mui-nav-menu__indicator" aria-hidden="true" { "\u{25BE}" }
    }
}

/// Render a navigation menu with the given properties.
pub fn render(props: Props) -> Markup {
    let list_class = match props.orientation {
        Orientation::Horizontal => "mui-nav-menu__list",
        Orientation::Vertical => "mui-nav-menu__list mui-nav-menu--vertical",
    };
    let viewport_attr = if props.viewport { "shared" } else { "per-item" };

    html! {
        nav class="mui-nav-menu" data-mui="nav-menu" id=(props.id) aria-label="Primary" data-viewport=(viewport_attr) {
            ul class=(list_class) {
                @for item in &props.items {
                    @match item {
                        NavItem::Link(link) => {
                            li {
                                a class="mui-nav-menu__link" href=(link.href) {
                                    (link.label)
                                }
                            }
                        }
                        NavItem::Menu(menu) => {
                            li class="mui-nav-menu__item" data-has-content {
                                button class="mui-nav-menu__trigger" aria-expanded="false" aria-haspopup="true" {
                                    (menu.label) " \u{25BE}"
                                }
                                @if props.viewport {
                                    div class="mui-nav-menu__content" hidden {
                                        ul class="mui-nav-menu__sub-list" {
                                            @for link in &menu.links {
                                                li {
                                                    a class="mui-nav-menu__sub-link" href=(link.href) {
                                                        div class="mui-nav-menu__sub-title" {
                                                            (link.label)
                                                        }
                                                        @if let Some(desc) = &link.description {
                                                            div class="mui-nav-menu__sub-desc" {
                                                                (desc)
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                } @else {
                                    div class="mui-nav-menu__popover" data-viewport="per-item" hidden {
                                        div class="mui-nav-menu__content" {
                                            ul class="mui-nav-menu__sub-list" {
                                                @for link in &menu.links {
                                                    li {
                                                        a class="mui-nav-menu__sub-link" href=(link.href) {
                                                            div class="mui-nav-menu__sub-title" {
                                                                (link.label)
                                                            }
                                                            @if let Some(desc) = &link.description {
                                                                div class="mui-nav-menu__sub-desc" {
                                                                    (desc)
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Showcase navigation menu examples.
pub fn showcase() -> Markup {
    let items = vec![
        NavItem::Menu(NavMenu {
            label: "Getting Started".to_string(),
            links: vec![
                NavLink {
                    label: "Introduction".to_string(),
                    href: "#introduction".to_string(),
                    description: Some("Re-usable components built with maud and CSS.".to_string()),
                },
                NavLink {
                    label: "Installation".to_string(),
                    href: "#installation".to_string(),
                    description: Some(
                        "How to install dependencies and structure your app.".to_string(),
                    ),
                },
                NavLink {
                    label: "Typography".to_string(),
                    href: "#typography".to_string(),
                    description: Some(
                        "Styles for headings, paragraphs, lists and more.".to_string(),
                    ),
                },
            ],
        }),
        NavItem::Menu(NavMenu {
            label: "Components".to_string(),
            links: vec![
                NavLink {
                    label: "Alert".to_string(),
                    href: "#alert".to_string(),
                    description: Some("Displays a callout for important information.".to_string()),
                },
                NavLink {
                    label: "Button".to_string(),
                    href: "#button".to_string(),
                    description: Some("Triggers an action or event when clicked.".to_string()),
                },
                NavLink {
                    label: "Card".to_string(),
                    href: "#card".to_string(),
                    description: Some("Groups related content in a container.".to_string()),
                },
            ],
        }),
        NavItem::Menu(NavMenu {
            label: "Resources".to_string(),
            links: vec![
                NavLink {
                    label: "Documentation".to_string(),
                    href: "#documentation".to_string(),
                    description: Some("Full API reference and usage guides.".to_string()),
                },
                NavLink {
                    label: "Changelog".to_string(),
                    href: "#changelog".to_string(),
                    description: Some("Latest updates and release notes.".to_string()),
                },
            ],
        }),
        NavItem::Link(NavLink {
            label: "GitHub".to_string(),
            href: "https://github.com".to_string(),
            description: None,
        }),
    ];

    let horizontal = Props {
        id: "demo-nav-menu".to_string(),
        items: items.clone(),
        ..Props::default()
    };

    let vertical_items = vec![
        NavItem::Menu(NavMenu {
            label: "Getting Started".to_string(),
            links: vec![
                NavLink {
                    label: "Introduction".to_string(),
                    href: "#introduction".to_string(),
                    description: Some("Overview of the design system.".to_string()),
                },
                NavLink {
                    label: "Installation".to_string(),
                    href: "#installation".to_string(),
                    description: None,
                },
            ],
        }),
        NavItem::Link(NavLink {
            label: "Docs".to_string(),
            href: "#docs".to_string(),
            description: None,
        }),
        NavItem::Link(NavLink {
            label: "GitHub".to_string(),
            href: "https://github.com".to_string(),
            description: None,
        }),
    ];

    let vertical = Props {
        id: "demo-nav-menu-vertical".to_string(),
        items: vertical_items,
        orientation: Orientation::Vertical,
        viewport: false,
    };

    html! {
        div class="mui-nav-menu__showcase" {
            (render(horizontal))
            div class="mui-nav-menu__showcase-indicator" {
                span { "Active" }
                (indicator())
                span { "Next" }
            }
            (render(vertical))
        }
    }
}
