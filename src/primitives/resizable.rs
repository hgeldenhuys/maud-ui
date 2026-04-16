//! Resizable component — draggable panels that resize horizontally or vertically.

use maud::{html, Markup};

/// A single resizable panel.
#[derive(Clone, Debug)]
pub struct Panel {
    pub content: Markup,
    pub default_size: f64,
    pub min_size: Option<f64>,
}

/// Layout direction for the resizable group.
#[derive(Clone, Debug)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    fn as_str(&self) -> &'static str {
        match self {
            Direction::Horizontal => "horizontal",
            Direction::Vertical => "vertical",
        }
    }

    fn aria_orientation(&self) -> &'static str {
        match self {
            Direction::Horizontal => "horizontal",
            Direction::Vertical => "vertical",
        }
    }
}

/// Resizable group rendering properties.
#[derive(Clone, Debug)]
pub struct Props {
    pub id: String,
    pub panels: Vec<Panel>,
    pub direction: Direction,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "resizable".to_string(),
            panels: vec![],
            direction: Direction::Horizontal,
        }
    }
}

/// Render a resizable panel group with the given properties.
pub fn render(props: Props) -> Markup {
    let dir = props.direction.as_str();
    let orientation = props.direction.aria_orientation();
    let panel_count = props.panels.len();

    html! {
        div class=(format!("mui-resizable mui-resizable--{}", dir))
            data-mui="resizable"
            data-direction=(dir)
            id=(props.id)
        {
            @for (i, panel) in props.panels.iter().enumerate() {
                @let min = panel.min_size.unwrap_or(10.0);
                div class="mui-resizable__panel"
                    style=(format!("flex: {} 1 0%", panel.default_size))
                    data-min-size=(format!("{}", min))
                {
                    (panel.content)
                }
                @if i < panel_count - 1 {
                    div class="mui-resizable__handle"
                        data-index=(format!("{}", i))
                        tabindex="0"
                        role="separator"
                        aria-orientation=(orientation)
                        aria-valuenow=(format!("{}", panel.default_size))
                    {
                        div class="mui-resizable__handle-bar" {}
                    }
                }
            }
        }
    }
}

/// Showcase resizable panel examples.
pub fn showcase() -> Markup {
    html! {
        div class="mui-showcase__grid" {
            // Two horizontal panels — sidebar + content
            div {
                h3 class="mui-showcase__caption" { "Sidebar + Content" }
                (render(Props {
                    id: "demo-resize-h2".to_string(),
                    direction: Direction::Horizontal,
                    panels: vec![
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Navigation" }
                                    ul class="mui-resizable__demo-nav" {
                                        li { a class="active" href="#" { "Dashboard" } }
                                        li { a href="#" { "Projects" } }
                                        li { a href="#" { "Team" } }
                                        li { a href="#" { "Settings" } }
                                        li { a href="#" { "Analytics" } }
                                    }
                                }
                            },
                            default_size: 30.0,
                            min_size: Some(15.0),
                        },
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Dashboard" }
                                    p { "Welcome back. Here is an overview of your recent activity, key metrics, and pending tasks. Drag the handle to resize the sidebar." }
                                }
                            },
                            default_size: 70.0,
                            min_size: Some(30.0),
                        },
                    ],
                }))
            }

            // Three horizontal panels — file explorer layout
            div {
                h3 class="mui-showcase__caption" { "Three-column Layout" }
                (render(Props {
                    id: "demo-resize-h3".to_string(),
                    direction: Direction::Horizontal,
                    panels: vec![
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Explorer" }
                                    ul class="mui-resizable__demo-nav" {
                                        li { a class="active" href="#" { "src/" } }
                                        li { a href="#" { "tests/" } }
                                        li { a href="#" { "docs/" } }
                                        li { a href="#" { "Cargo.toml" } }
                                    }
                                }
                            },
                            default_size: 20.0,
                            min_size: Some(10.0),
                        },
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Editor" }
                                    p { "Select a file from the explorer to view its contents here. This center panel occupies the majority of the available space." }
                                }
                            },
                            default_size: 55.0,
                            min_size: Some(20.0),
                        },
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Inspector" }
                                    p { "Properties and metadata for the selected item will appear in this panel." }
                                }
                            },
                            default_size: 25.0,
                            min_size: Some(10.0),
                        },
                    ],
                }))
            }
        }
    }
}
