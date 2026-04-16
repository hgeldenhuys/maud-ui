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
            // Two horizontal panels
            div {
                h3 class="mui-showcase__caption" { "Two Horizontal Panels" }
                (render(Props {
                    id: "demo-resize-h2".to_string(),
                    direction: Direction::Horizontal,
                    panels: vec![
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Sidebar" }
                                    p { "This panel takes 30% of the space by default." }
                                }
                            },
                            default_size: 30.0,
                            min_size: Some(15.0),
                        },
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Content" }
                                    p { "This panel takes 70% of the space by default." }
                                }
                            },
                            default_size: 70.0,
                            min_size: Some(30.0),
                        },
                    ],
                }))
            }

            // Three horizontal panels
            div {
                h3 class="mui-showcase__caption" { "Three Panels with Two Handles" }
                (render(Props {
                    id: "demo-resize-h3".to_string(),
                    direction: Direction::Horizontal,
                    panels: vec![
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Left" }
                                    p { "25%" }
                                }
                            },
                            default_size: 25.0,
                            min_size: Some(10.0),
                        },
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Center" }
                                    p { "50%" }
                                }
                            },
                            default_size: 50.0,
                            min_size: Some(20.0),
                        },
                        Panel {
                            content: html! {
                                div class="mui-resizable__demo-content" {
                                    h4 { "Right" }
                                    p { "25%" }
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
