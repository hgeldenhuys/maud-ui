//! Item component — media + content + actions list row. Sibling to EmptyState
//! and designed for list-row patterns (notifications, search results, file
//! rows). Mirrors shadcn's `Item` base primitive.
//!
//! Composition: wrap rows in [`group`], then compose each row from
//! [`media`] + [`content`] (with [`title`] / [`description`]) + [`actions`].
//! Use [`header`] / [`footer`] / [`separator`] for richer layouts.
use maud::{html, Markup};

/// Visual variant — frame style around the item.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Variant {
    #[default]
    Default,
    Outline,
    Muted,
}

impl Variant {
    fn as_class(&self) -> &'static str {
        match self {
            Variant::Default => "default",
            Variant::Outline => "outline",
            Variant::Muted => "muted",
        }
    }
}

/// Density — controls padding + gap.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum Size {
    #[default]
    Default,
    Sm,
    Xs,
}

impl Size {
    fn as_class(&self) -> &'static str {
        match self {
            Size::Default => "default",
            Size::Sm => "sm",
            Size::Xs => "xs",
        }
    }
}

/// Media sub-variant — hints at whether the media slot hosts an icon, an
/// image, or generic content. Controls sizing inside `.mui-item__media`.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum MediaVariant {
    #[default]
    Default,
    Icon,
    Image,
}

impl MediaVariant {
    fn as_class(&self) -> &'static str {
        match self {
            MediaVariant::Default => "default",
            MediaVariant::Icon => "icon",
            MediaVariant::Image => "image",
        }
    }
}

/// Item rendering properties.
#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Optional DOM id (wire triggers / skip-links to this row).
    pub id: Option<String>,
    /// Visual variant — frame style.
    pub variant: Variant,
    /// Density.
    pub size: Size,
    /// Row contents — typically composed from `media`, `content`, `actions`.
    pub children: Markup,
}

/// Render a single item row.
pub fn render(props: Props) -> Markup {
    let variant_cls = format!("mui-item--{}", props.variant.as_class());
    let size_cls = format!("mui-item--{}", props.size.as_class());
    html! {
        div class={"mui-item " (variant_cls) " " (size_cls)}
            id=[props.id.as_deref()]
            data-mui="item"
        {
            (props.children)
        }
    }
}

/// Wrap a list of items with consistent spacing.
pub fn group(children: Markup) -> Markup {
    html! {
        div class="mui-item-group" role="list" {
            (children)
        }
    }
}

/// Media slot — icon, avatar, image, thumbnail.
pub fn media(variant: MediaVariant, children: Markup) -> Markup {
    let variant_cls = format!("mui-item__media--{}", variant.as_class());
    html! {
        div class={"mui-item__media " (variant_cls)} {
            (children)
        }
    }
}

/// Primary textual stack — usually holds [`title`] and [`description`].
pub fn content(children: Markup) -> Markup {
    html! {
        div class="mui-item__content" {
            (children)
        }
    }
}

/// Item title — styled as a small heading.
pub fn title(children: Markup) -> Markup {
    html! {
        h3 class="mui-item__title" {
            (children)
        }
    }
}

/// Item description — muted supporting text.
pub fn description(children: Markup) -> Markup {
    html! {
        p class="mui-item__description" {
            (children)
        }
    }
}

/// Trailing actions slot — buttons, icons, timestamps.
pub fn actions(children: Markup) -> Markup {
    html! {
        div class="mui-item__actions" {
            (children)
        }
    }
}

/// Optional header strip above the main row.
pub fn header(children: Markup) -> Markup {
    html! {
        div class="mui-item__header" {
            (children)
        }
    }
}

/// Optional footer strip below the main row.
pub fn footer(children: Markup) -> Markup {
    html! {
        div class="mui-item__footer" {
            (children)
        }
    }
}

/// Separator between grouped items.
pub fn separator() -> Markup {
    html! {
        hr class="mui-item__separator";
    }
}

/// Showcase all item variants and common composition patterns.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Item group — three variants" }
                p.mui-showcase__caption {
                    "Default with icon media, Outline with image media, Muted title-only (sm)."
                }
                (group(html! {
                    (render(Props {
                        variant: Variant::Default,
                        children: html! {
                            (media(MediaVariant::Icon, html! { "\u{1F4EC}" }))
                            (content(html! {
                                (title(html! { "New message from Jane" }))
                                (description(html! { "Hey — just a heads up that the design doc is ready for review." }))
                            }))
                            (actions(html! {
                                button class="mui-btn mui-btn--outline mui-btn--md" { "Reply" }
                            }))
                        },
                        ..Default::default()
                    }))
                    (render(Props {
                        variant: Variant::Outline,
                        children: html! {
                            (media(MediaVariant::Image, html! {
                                div style="width:100%;height:100%;background:linear-gradient(135deg,#2563eb,#60a5fa);" {}
                            }))
                            (content(html! {
                                (title(html! { "Acme Corp launch brief" }))
                                (description(html! { "Shared folder \u{00b7} 12 files \u{00b7} updated 2h ago" }))
                            }))
                            (actions(html! {
                                button class="mui-btn mui-btn--outline mui-btn--md" { "Open" }
                            }))
                        },
                        ..Default::default()
                    }))
                    (render(Props {
                        variant: Variant::Muted,
                        size: Size::Sm,
                        children: html! {
                            (content(html! {
                                (title(html! { "Drafts (3)" }))
                            }))
                        },
                        ..Default::default()
                    }))
                }))
            }

            section {
                h2 { "With header / footer / separator" }
                (group(html! {
                    (render(Props {
                        variant: Variant::Outline,
                        children: html! {
                            (header(html! {
                                span.mui-text-muted { "Today" }
                            }))
                            (media(MediaVariant::Icon, html! { "\u{1F514}" }))
                            (content(html! {
                                (title(html! { "Build finished" }))
                                (description(html! { "kapable-api \u{00b7} blue slot \u{00b7} 00:42 elapsed" }))
                            }))
                            (footer(html! {
                                span.mui-text-subtle { "deploy-daemon" }
                            }))
                        },
                        ..Default::default()
                    }))
                    (separator())
                    (render(Props {
                        variant: Variant::Outline,
                        size: Size::Xs,
                        children: html! {
                            (media(MediaVariant::Icon, html! { "\u{2713}" }))
                            (content(html! {
                                (title(html! { "Tests passed" }))
                            }))
                        },
                        ..Default::default()
                    }))
                }))
            }
        }
    }
}
