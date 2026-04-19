//! Navigation Menu component — horizontal nav with dropdown content panels (mega-menu style).

use maud::{html, Markup};

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
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "nav-menu".to_string(),
            items: vec![],
        }
    }
}

/// Render a navigation menu with the given properties.
pub fn render(props: Props) -> Markup {
    html! {
        nav class="mui-nav-menu" data-mui="nav-menu" id=(props.id) aria-label="Primary" {
            ul class="mui-nav-menu__list" {
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
                                button class="mui-nav-menu__trigger" aria-expanded="false" {
                                    (menu.label) " \u{25BE}"
                                }
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

    let props = Props {
        id: "demo-nav-menu".to_string(),
        items,
    };

    render(props)
}
