//! Card component — container with optional header, body, and footer.
use maud::{html, Markup};

/// Card size variant.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Size {
    /// Default padding (`1.5rem`).
    #[default]
    Default,
    /// Compact card — reduced padding (`0.75rem`).
    Sm,
}

impl Size {
    /// Returns the modifier class for this size, or an empty string for the default.
    pub fn as_class(self) -> &'static str {
        match self {
            Size::Default => "",
            Size::Sm => "mui-card--sm",
        }
    }
}

/// Card rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Optional card title displayed in header
    pub title: Option<String>,
    /// Optional card description displayed below title
    pub description: Option<String>,
    /// Main content markup
    pub children: Markup,
    /// Optional footer markup
    pub footer: Option<Markup>,
    /// Card size modifier.
    pub size: Size,
    /// Optional top-right header slot (shadcn `CardAction` equivalent).
    /// When `Some`, the header becomes a 2-col grid: `(title/description) | (action)`.
    pub action: Option<Markup>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            children: html! {},
            footer: None,
            size: Size::default(),
            action: None,
        }
    }
}

/// Render a card with the given properties.
pub fn render(props: Props) -> Markup {
    let size_class = props.size.as_class();
    let card_class = if size_class.is_empty() {
        "mui-card".to_string()
    } else {
        format!("mui-card {size_class}")
    };

    html! {
        div class=(card_class) {
            @if props.title.is_some() || props.description.is_some() || props.action.is_some() {
                div class="mui-card__header" {
                    div class="mui-card__header-text" {
                        @if let Some(title) = props.title {
                            h3 class="mui-card__title" {
                                (title)
                            }
                        }
                        @if let Some(desc) = props.description {
                            p class="mui-card__description" {
                                (desc)
                            }
                        }
                    }
                    @if let Some(action_markup) = props.action {
                        div class="mui-card__action" {
                            (action_markup)
                        }
                    }
                }
            }
            div class="mui-card__body" {
                (props.children)
            }
            @if let Some(footer_markup) = props.footer {
                div class="mui-card__footer" {
                    (footer_markup)
                }
            }
        }
    }
}

/// Standalone header-action helper — renders the top-right action slot. Use when composing a
/// Card manually (without `render(Props)`), or to pass into `Props::action`.
pub fn action(children: Markup) -> Markup {
    html! {
        div class="mui-card__action" {
            (children)
        }
    }
}

/// Standalone content helper — renders the card body wrapper.
pub fn content(children: Markup) -> Markup {
    html! {
        div class="mui-card__body" {
            (children)
        }
    }
}

/// Standalone footer helper — renders the card footer wrapper.
pub fn footer(children: Markup) -> Markup {
    html! {
        div class="mui-card__footer" {
            (children)
        }
    }
}

/// Showcase all card use cases
pub fn showcase() -> Markup {
    use crate::primitives::{button, input, label, select, switch};

    html! {
        div.mui-showcase__grid {
            // 1. Notifications card with switches
            div {
                p.mui-showcase__caption { "Notifications" }
                (render(Props {
                    title: Some("Notifications".into()),
                    description: Some("Choose what you want to be notified about.".into()),
                    children: html! {
                        div style="display:flex;flex-direction:column;gap:1rem;" {
                            div style="display:flex;align-items:flex-start;justify-content:space-between;gap:1rem;" {
                                div {
                                    p style="font-size:0.875rem;font-weight:500;margin:0;" { "Push Notifications" }
                                    p style="font-size:0.8125rem;color:var(--mui-text-muted);margin:0.125rem 0 0;" {
                                        "Send notifications to device."
                                    }
                                }
                                (switch::render(switch::Props {
                                    name: "card-push".into(),
                                    id: "card-push".into(),
                                    label: String::new(),
                                    checked: false,
                                    disabled: false,
                                    aria_label: Some("Push notifications".into()),
                                }))
                            }
                            div style="display:flex;align-items:flex-start;justify-content:space-between;gap:1rem;" {
                                div {
                                    p style="font-size:0.875rem;font-weight:500;margin:0;" { "Email Notifications" }
                                    p style="font-size:0.8125rem;color:var(--mui-text-muted);margin:0.125rem 0 0;" {
                                        "Receive emails for activity updates."
                                    }
                                }
                                (switch::render(switch::Props {
                                    name: "card-email".into(),
                                    id: "card-email".into(),
                                    label: String::new(),
                                    checked: true,
                                    disabled: false,
                                    aria_label: Some("Email notifications".into()),
                                }))
                            }
                        }
                    },
                    footer: Some(html! {
                        div style="display:flex;gap:0.5rem;margin-left:auto;" {
                            (button::render(button::Props {
                                label: "Save preferences".into(),
                                variant: button::Variant::Primary,
                                size: button::Size::Md,
                                ..Default::default()
                            }))
                        }
                    }),
                    ..Default::default()
                }))
            }

            // 2. Simple stat card
            div {
                p.mui-showcase__caption { "Stat card" }
                (render(Props {
                    title: Some("Total Revenue".into()),
                    description: None,
                    children: html! {
                        div {
                            p style="font-size:1.75rem;font-weight:700;margin:0;" { "$45,231.89" }
                            p style="font-size:0.8125rem;color:var(--mui-text-muted);margin:0.25rem 0 0;" {
                                "+20.1% from last month"
                            }
                        }
                    },
                    footer: None,
                    ..Default::default()
                }))
            }

            // 3. Create project card with form
            div {
                p.mui-showcase__caption { "Create project" }
                (render(Props {
                    title: Some("Create project".into()),
                    description: Some("Deploy your new project in one click.".into()),
                    children: html! {
                        div style="display:flex;flex-direction:column;gap:0.75rem;" {
                            div class="mui-field" {
                                (label::render(label::Props {
                                    text: "Name".into(),
                                    html_for: Some("card-project-name".into()),
                                    ..Default::default()
                                }))
                                (input::render(input::Props {
                                    name: "project_name".into(),
                                    id: "card-project-name".into(),
                                    input_type: input::InputType::Text,
                                    placeholder: "Name of your project".into(),
                                    ..Default::default()
                                }))
                            }
                            div class="mui-field" {
                                (label::render(label::Props {
                                    text: "Framework".into(),
                                    html_for: Some("card-framework".into()),
                                    ..Default::default()
                                }))
                                (select::render(select::Props {
                                    name: "framework".into(),
                                    id: "card-framework".into(),
                                    options: vec![
                                        select::SelectOption { value: "next".into(), label: "Next.js".into(), disabled: false },
                                        select::SelectOption { value: "remix".into(), label: "Remix".into(), disabled: false },
                                        select::SelectOption { value: "astro".into(), label: "Astro".into(), disabled: false },
                                        select::SelectOption { value: "nuxt".into(), label: "Nuxt".into(), disabled: false },
                                    ],
                                    selected: None,
                                    placeholder: "Select a framework\u{2026}".into(),
                                    disabled: false,
                                }))
                            }
                        }
                    },
                    footer: Some(html! {
                        div style="display:flex;gap:0.5rem;justify-content:space-between;width:100%;" {
                            (button::render(button::Props {
                                label: "Cancel".into(),
                                variant: button::Variant::Outline,
                                size: button::Size::Md,
                                ..Default::default()
                            }))
                            (button::render(button::Props {
                                label: "Create".into(),
                                variant: button::Variant::Primary,
                                size: button::Size::Md,
                                ..Default::default()
                            }))
                        }
                    }),
                    ..Default::default()
                }))
            }

            // 4. Compact (Sm) card
            div {
                p.mui-showcase__caption { "Compact card (size: Sm)" }
                (render(Props {
                    title: Some("Storage".into()),
                    description: Some("6.2 GB of 15 GB used.".into()),
                    size: Size::Sm,
                    children: html! {
                        div style="height:0.5rem;background:var(--mui-border);border-radius:9999px;overflow:hidden;" {
                            div style="width:41%;height:100%;background:var(--mui-primary, #2563eb);" {}
                        }
                    },
                    ..Default::default()
                }))
            }

            // 5. Card with top-right action slot
            div {
                p.mui-showcase__caption { "Card with action slot" }
                (render(Props {
                    title: Some("Team members".into()),
                    description: Some("Manage who has access to this project.".into()),
                    action: Some(html! {
                        (button::render(button::Props {
                            label: "Invite".into(),
                            variant: button::Variant::Outline,
                            size: button::Size::Sm,
                            ..Default::default()
                        }))
                    }),
                    children: html! {
                        ul style="list-style:none;padding:0;margin:0;display:flex;flex-direction:column;gap:0.5rem;" {
                            li style="font-size:0.875rem;" { "Alice Johnson — Owner" }
                            li style="font-size:0.875rem;" { "Bob Smith — Editor" }
                            li style="font-size:0.875rem;" { "Carol Davis — Viewer" }
                        }
                    },
                    ..Default::default()
                }))
            }
        }
    }
}
