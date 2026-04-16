//! ButtonGroup component — groups multiple buttons with shared borders.
use maud::{html, Markup};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn class_name(self) -> &'static str {
        match self {
            Orientation::Horizontal => "mui-button-group--horizontal",
            Orientation::Vertical => "mui-button-group--vertical",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Props {
    pub children: Markup,
    pub orientation: Orientation,
    pub size: Option<String>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            children: html! {},
            orientation: Orientation::Horizontal,
            size: None,
        }
    }
}

pub fn render(props: Props) -> Markup {
    let mut class = "mui-button-group".to_string();
    class.push(' ');
    class.push_str(props.orientation.class_name());

    if let Some(size) = props.size {
        class.push(' ');
        class.push_str(&size);
    }

    html! {
        div class=(class) role="group" {
            (props.children)
        }
    }
}

pub fn showcase() -> Markup {
    let buttons_horizontal = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Left" }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Center" }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Right" }
    };

    let buttons_vertical = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Top" }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Middle" }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Bottom" }
    };

    let buttons_mixed = html! {
        button type="button" class="mui-btn mui-btn--primary mui-btn--md" { "Primary" }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Outline 1" }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" { "Outline 2" }
    };

    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Horizontal (default)" }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_horizontal.clone(),
                        orientation: Orientation::Horizontal,
                        size: None,
                    }))
                }
            }
            section {
                h2 { "Vertical" }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_vertical.clone(),
                        orientation: Orientation::Vertical,
                        size: None,
                    }))
                }
            }
            section {
                h2 { "Mixed variants" }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_mixed,
                        orientation: Orientation::Horizontal,
                        size: None,
                    }))
                }
            }
        }
    }
}
