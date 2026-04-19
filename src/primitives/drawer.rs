//! Drawer component — sliding panel that anchors to a screen edge using native <dialog> element.
use maud::{html, Markup};

use super::{field, native_select, separator, switch};

/// Side where the drawer slides in from
#[derive(Clone, Debug)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

impl Side {
    fn class_name(&self) -> &'static str {
        match self {
            Side::Left => "mui-drawer--left",
            Side::Right => "mui-drawer--right",
            Side::Top => "mui-drawer--top",
            Side::Bottom => "mui-drawer--bottom",
        }
    }
}

impl Default for Side {
    fn default() -> Self {
        Side::Right
    }
}

/// Drawer rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier for the drawer (used by trigger to open it)
    pub id: String,
    /// Drawer title
    pub title: String,
    /// Optional description text displayed below title
    pub description: Option<String>,
    /// Markup content displayed in drawer body
    pub children: Markup,
    /// Optional footer markup pinned at the bottom
    pub footer: Option<Markup>,
    /// Which side the drawer slides from (default Right)
    pub side: Side,
    /// Whether the page background should scale down when the drawer opens
    /// (mirrors Vaul's `shouldScaleBackground`). Emits
    /// `data-scale-background="true"` on the root so a script/CSS hook can
    /// toggle `body[data-drawer-scaling]`. Default: false.
    pub should_scale_background: bool,
    /// Whether the built-in close (×) button renders in the header.
    /// Default: true.
    pub show_close_button: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "drawer".to_string(),
            title: "Drawer".to_string(),
            description: None,
            children: html! {},
            footer: None,
            side: Side::Right,
            should_scale_background: false,
            show_close_button: true,
        }
    }
}

/// Render a drawer trigger button that opens the drawer with the given target_id
pub fn trigger(target_id: &str, label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-btn mui-btn--default mui-btn--md"
            data-mui="drawer-trigger"
            data-target=(target_id)
        {
            (label)
        }
    }
}

/// Render a close button for use inside the drawer
pub fn close_button(label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-drawer__close"
            data-mui-close
            aria-label=(label)
        {
            "×"
        }
    }
}

/// Render a drawer with the given properties
pub fn render(props: Props) -> Markup {
    let title_id = format!("{}-title", props.id);
    let desc_id = format!("{}-desc", props.id);
    let has_desc = props.description.is_some();
    let show_handle = matches!(props.side, Side::Bottom | Side::Top);
    let scale_attr = if props.should_scale_background {
        Some("true")
    } else {
        None
    };

    html! {
        dialog class={"mui-drawer " (props.side.class_name())}
            id=(props.id)
            data-mui="drawer"
            data-scale-background=[scale_attr]
            aria-modal="true"
            aria-labelledby=(title_id)
            aria-describedby=[if has_desc { Some(desc_id.as_str()) } else { None }]
        {
            @if show_handle {
                div class="mui-drawer__handle" {
                    div class="mui-drawer__handle-bar" {}
                }
            }
            div class="mui-drawer__header" {
                h2 class="mui-drawer__title" id=(title_id) {
                    (props.title)
                }
                @if props.show_close_button {
                    (close_button("Close"))
                }
            }
            @if let Some(desc) = props.description {
                p class="mui-drawer__description" id=(desc_id) {
                    (desc)
                }
            }
            div class="mui-drawer__body" {
                (props.children)
            }
            @if let Some(footer) = props.footer {
                div class="mui-drawer__footer" {
                    (footer)
                }
            }
        }
    }
}

/// Showcase all drawer use cases
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Right (default)" }
                div.mui-showcase__row {
                    (trigger("demo-drawer-1", "Open drawer"))
                }
            }
            (render(Props {
                id: "demo-drawer-1".to_string(),
                title: "Settings".to_string(),
                description: Some("Adjust your preferences here.".to_string()),
                children: html! {
                    div style="display: flex; flex-direction: column; gap: 1rem;" {
                        (field::render(field::Props {
                            label: "Theme".to_string(),
                            id: "demo-theme".to_string(),
                            description: Some("Choose your preferred appearance.".to_string()),
                            children: html! {
                                (native_select::render(native_select::NativeSelectProps {
                                    name: "theme".to_string(),
                                    id: "demo-theme".to_string(),
                                    options: vec![
                                        native_select::NativeOption { value: "light".to_string(), label: "Light".to_string(), disabled: false },
                                        native_select::NativeOption { value: "dark".to_string(), label: "Dark".to_string(), disabled: false },
                                        native_select::NativeOption { value: "auto".to_string(), label: "System".to_string(), disabled: false },
                                    ],
                                    selected: Some("auto".to_string()),
                                    disabled: false,
                                    placeholder: None,
                                }))
                            },
                            ..Default::default()
                        }))
                        (separator::render(separator::Props {
                            orientation: separator::Orientation::Horizontal,
                            decorative: true,
                        }))
                        (switch::render(switch::Props {
                            name: "notifications".to_string(),
                            id: "demo-notifications".to_string(),
                            label: "Enable notifications".to_string(),
                            checked: true,
                            disabled: false,
                            aria_label: None,
                            ..Default::default()
                        }))
                        (switch::render(switch::Props {
                            name: "sounds".to_string(),
                            id: "demo-sounds".to_string(),
                            label: "Sound effects".to_string(),
                            checked: false,
                            disabled: false,
                            aria_label: None,
                            ..Default::default()
                        }))
                    }
                },
                footer: Some(html! {
                    button class="mui-btn mui-btn--default mui-btn--md" data-mui-close { "Cancel" }
                    button class="mui-btn mui-btn--primary mui-btn--md" { "Save changes" }
                }),
                side: Side::Right,
                ..Default::default()
            }))

            section {
                h2 { "Left (navigation)" }
                div.mui-showcase__row {
                    (trigger("demo-drawer-2", "Open drawer"))
                }
            }
            (render(Props {
                id: "demo-drawer-2".to_string(),
                title: "Navigation".to_string(),
                description: None,
                children: html! {
                    nav style="display: flex; flex-direction: column; gap: 0.25rem;" {
                        a class="mui-btn mui-btn--ghost mui-btn--md" style="justify-content: flex-start; width: 100%;" href="#" { "Home" }
                        a class="mui-btn mui-btn--ghost mui-btn--md" style="justify-content: flex-start; width: 100%;" href="#" { "Products" }
                        a class="mui-btn mui-btn--ghost mui-btn--md" style="justify-content: flex-start; width: 100%;" href="#" { "Documentation" }
                        (separator::render(separator::Props {
                            orientation: separator::Orientation::Horizontal,
                            decorative: true,
                        }))
                        a class="mui-btn mui-btn--ghost mui-btn--md" style="justify-content: flex-start; width: 100%;" href="#" { "Settings" }
                        a class="mui-btn mui-btn--ghost mui-btn--md" style="justify-content: flex-start; width: 100%;" href="#" { "Contact" }
                    }
                },
                footer: None,
                side: Side::Left,
                ..Default::default()
            }))

            section {
                h2 { "Bottom (sheet with grab handle)" }
                div.mui-showcase__row {
                    (trigger("demo-drawer-3", "Open drawer"))
                }
            }
            (render(Props {
                id: "demo-drawer-3".to_string(),
                title: "Share".to_string(),
                description: Some("Share this document with others.".to_string()),
                children: html! {
                    (field::render(field::Props {
                        label: "Email address".to_string(),
                        id: "demo-share-email".to_string(),
                        description: Some("Enter the recipient's email.".to_string()),
                        children: html! {
                            input.mui-input type="email" id="demo-share-email" name="email"
                                placeholder="colleague@example.com";
                        },
                        ..Default::default()
                    }))
                },
                footer: Some(html! {
                    button class="mui-btn mui-btn--default mui-btn--md" data-mui-close { "Cancel" }
                    button class="mui-btn mui-btn--primary mui-btn--md" { "Send invite" }
                }),
                side: Side::Bottom,
                ..Default::default()
            }))

            section {
                h2 { "Scaled background (Vaul-style)" }
                div.mui-showcase__row {
                    (trigger("demo-drawer-4", "Open drawer"))
                }
            }
            (render(Props {
                id: "demo-drawer-4".to_string(),
                title: "Confirm action".to_string(),
                description: Some("The page behind scales down while this drawer is open.".to_string()),
                children: html! {
                    p { "Renders with " code { "data-scale-background=\"true\"" } " on the dialog root. Pair with a small script that toggles " code { "body[data-drawer-scaling]" } " to get the Vaul scale-down effect." }
                },
                footer: Some(html! {
                    button class="mui-btn mui-btn--default mui-btn--md" data-mui-close { "Cancel" }
                    button class="mui-btn mui-btn--primary mui-btn--md" { "Confirm" }
                }),
                side: Side::Bottom,
                should_scale_background: true,
                ..Default::default()
            }))

            section {
                h2 { "No close button (footer-only dismiss)" }
                div.mui-showcase__row {
                    (trigger("demo-drawer-5", "Open drawer"))
                }
            }
            (render(Props {
                id: "demo-drawer-5".to_string(),
                title: "Terms of service".to_string(),
                description: Some("You must choose an action below to dismiss this drawer.".to_string()),
                children: html! {
                    p { "The built-in × close button is hidden when " code { "show_close_button" } " is false. The footer actions are the only way out." }
                },
                footer: Some(html! {
                    button class="mui-btn mui-btn--default mui-btn--md" data-mui-close { "Decline" }
                    button class="mui-btn mui-btn--primary mui-btn--md" data-mui-close { "Accept" }
                }),
                side: Side::Right,
                show_close_button: false,
                ..Default::default()
            }))
        }
    }
}
