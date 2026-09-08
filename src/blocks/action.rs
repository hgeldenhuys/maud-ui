//! A single, named action with native navigation or POST submission.
use crate::primitives::button;
use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Link {
    pub label: String,
    pub href: String,
}

#[derive(Clone, Debug)]
pub enum Target {
    Link(String),
    /// Include CSRF tokens and other server-owned values in hidden_fields.
    Post {
        action: String,
        hidden_fields: Vec<(String, String)>,
    },
}

#[derive(Clone, Debug)]
pub struct Action {
    pub label: String,
    pub target: Target,
}

impl Action {
    pub fn link(label: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            target: Target::Link(href.into()),
        }
    }

    pub(crate) fn render(&self, primary: bool) -> Markup {
        match &self.target {
            Target::Link(href) => html! {
                a class=(if primary { "mui-btn mui-btn--primary mui-btn--md" } else { "mui-btn mui-btn--outline mui-btn--row" }) href=(href) { (&self.label) }
            },
            Target::Post {
                action,
                hidden_fields,
            } => html! {
                form method="post" action=(action) class="mui-block-action" {
                    @for (name, value) in hidden_fields { input type="hidden" name=(name) value=(value); }
                    (button::render(button::Props {
                        label: self.label.clone(), button_type: "submit",
                        variant: if primary { button::Variant::Primary } else { button::Variant::Outline },
                        size: if primary { button::Size::Md } else { button::Size::Row },
                        ..Default::default()
                    }))
                }
            },
        }
    }
}

/// Choose a heading appropriate to the surrounding page, including nested previews.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Heading {
    H1,
    #[default]
    H2,
    H3,
}

impl Heading {
    pub(crate) fn render(self, title: &str, class: &str) -> Markup {
        match self {
            Self::H1 => html! { h1 class=(class) { (title) } },
            Self::H2 => html! { h2 class=(class) { (title) } },
            Self::H3 => html! { h3 class=(class) { (title) } },
        }
    }
}
