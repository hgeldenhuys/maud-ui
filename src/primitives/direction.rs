//! Direction component — provides a directional context (LTR/RTL) for descendant elements
//! via the HTML `dir` attribute. No JS needed — `dir` cascades natively in every browser.

use maud::{html, Markup};

/// Writing direction for descendant elements.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Dir {
    /// Left-to-right (default; English, most European languages).
    #[default]
    Ltr,
    /// Right-to-left (Arabic, Hebrew, Persian, Urdu).
    Rtl,
}

impl Dir {
    /// HTML `dir` attribute value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Dir::Ltr => "ltr",
            Dir::Rtl => "rtl",
        }
    }
}

/// Direction rendering properties.
#[derive(Debug, Default)]
pub struct Props {
    /// Writing direction for the wrapped subtree.
    pub dir: Dir,
    /// Child markup that will inherit the direction.
    pub children: Markup,
}

/// Render a direction-scoped container. The `dir` attribute cascades to all descendants.
pub fn render(props: Props) -> Markup {
    html! {
        div dir=(props.dir.as_str()) {
            (props.children)
        }
    }
}

/// Showcase LTR vs RTL panels side by side.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Writing direction" }
                p.mui-showcase__caption {
                    "The HTML "
                    code { "dir" }
                    " attribute cascades to all descendants — inputs, lists, and even scrollbars flip automatically."
                }
                div style="display: grid; grid-template-columns: repeat(2, 1fr); gap: 1rem;" {
                    div {
                        p.mui-showcase__caption { "dir=\"ltr\"" }
                        (render(Props {
                            dir: Dir::Ltr,
                            children: html! {
                                div style="padding: 1rem; border: 1px solid var(--mui-border); border-radius: 0.5rem;" {
                                    p { "Hello World" }
                                    p { "English flows left to right." }
                                }
                            },
                        }))
                    }
                    div {
                        p.mui-showcase__caption { "dir=\"rtl\"" }
                        (render(Props {
                            dir: Dir::Rtl,
                            children: html! {
                                div style="padding: 1rem; border: 1px solid var(--mui-border); border-radius: 0.5rem;" {
                                    p { "مرحبا بالعالم" }
                                    p { "تتدفق اللغة العربية من اليمين إلى اليسار." }
                                }
                            },
                        }))
                    }
                }
            }
        }
    }
}
