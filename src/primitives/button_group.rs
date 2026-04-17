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
    // Text alignment — Left/Center/Right with glyph icons
    let buttons_align = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-pressed="true" aria-label="Align left" {
            span aria-hidden="true" { "\u{2630}" }
            " Left"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-label="Align center" {
            span aria-hidden="true" { "\u{2261}" }
            " Center"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-label="Align right" {
            span aria-hidden="true" { "\u{268c}" }
            " Right"
        }
    };

    // Text formatting — Bold/Italic/Underline
    let buttons_format = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-pressed="true" aria-label="Bold" {
            strong { "B" }
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-label="Italic" {
            em { "I" }
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-label="Underline" {
            span style="text-decoration:underline;" { "U" }
        }
    };

    // View switcher — Grid/List/Board
    let buttons_view = html! {
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" aria-pressed="true" {
            span aria-hidden="true" { "\u{25a6}" }
            " Grid"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" {
            span aria-hidden="true" { "\u{2630}" }
            " List"
        }
        button type="button" class="mui-btn mui-btn--outline mui-btn--md" {
            span aria-hidden="true" { "\u{25a7}" }
            " Board"
        }
    };

    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Text alignment toolbar" }
                p.mui-showcase__caption { "Editor toolbar — exactly one alignment active at a time." }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_align,
                        orientation: Orientation::Horizontal,
                        size: None,
                    }))
                }
            }
            section {
                h2 { "Text formatting" }
                p.mui-showcase__caption { "Rich-text controls — each toggle independent (Bold currently on)." }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_format,
                        orientation: Orientation::Horizontal,
                        size: None,
                    }))
                }
            }
            section {
                h2 { "View switcher" }
                p.mui-showcase__caption { "Project listing view — Grid / List / Board (Grid active)." }
                div.mui-showcase__row {
                    (render(Props {
                        children: buttons_view,
                        orientation: Orientation::Horizontal,
                        size: None,
                    }))
                }
            }
        }
    }
}
