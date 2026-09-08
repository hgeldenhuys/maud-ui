//! Sheet component — slide-out panel extending Dialog. Mirrors shadcn's Sheet
//! which is treated as a first-class primitive distinct from Dialog/Drawer.
//! Uses the native `<dialog>` element and slides in from a configurable edge.
use maud::{html, Markup};

/// Side from which the sheet slides in
#[derive(Clone, Debug, Default)]
pub enum Side {
    Top,
    #[default]
    Right,
    Bottom,
    Left,
}

impl Side {
    /// Short side identifier used in CSS modifier class and side tokens
    pub fn as_class(&self) -> &'static str {
        match self {
            Side::Top => "top",
            Side::Right => "right",
            Side::Bottom => "bottom",
            Side::Left => "left",
        }
    }
}

/// Sheet rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier for the sheet (used by trigger to open it)
    pub id: String,
    /// Sheet title
    pub title: String,
    /// Optional description text displayed below title
    pub description: Option<String>,
    /// Markup content displayed in sheet body
    pub children: Markup,
    /// Optional footer markup pinned at the bottom
    pub footer: Option<Markup>,
    /// Which side the sheet slides from (default Right)
    pub side: Side,
    /// Whether to render the close button (default true)
    pub show_close_button: bool,
    /// Initial open state (default false; if true renders with open attribute for SSR)
    pub open: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "sheet".to_string(),
            title: "Sheet".to_string(),
            description: None,
            children: html! {},
            footer: None,
            side: Side::Right,
            show_close_button: true,
            open: false,
        }
    }
}

/// Render a sheet trigger button that opens the sheet with the given target_id.
/// Reuses the dialog-trigger behaviour since sheets use the same native
/// `<dialog>` open/close mechanics — no dedicated sheet.js needed.
pub fn trigger(target_id: &str, label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-btn mui-btn--default mui-btn--md"
            data-mui="dialog-trigger"
            data-target=(target_id)
            aria-controls=(target_id) aria-haspopup="dialog"
        {
            (label)
        }
    }
}

/// Render a close button for use inside the sheet
pub fn close_button(label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-sheet__close"
            data-mui-close
            aria-label=(label)
        {
            "\u{00d7}"
        }
    }
}

/// Render a sheet with the given properties
pub fn render(props: Props) -> Markup {
    let title_id = format!("{}-title", props.id);
    let desc_id = format!("{}-desc", props.id);
    let has_desc = props.description.is_some();
    let side_class = format!("mui-sheet--{}", props.side.as_class());

    html! {
        dialog class={"mui-sheet " (side_class)}
            id=(props.id)
            data-mui="sheet"
            role="dialog"
            aria-labelledby=(title_id)
            aria-describedby=[if has_desc { Some(desc_id.as_str()) } else { None }]
            open[props.open]
        {
            div class="mui-sheet__header" {
                h2 class="mui-sheet__title" id=(title_id) {
                    (props.title)
                }
                @if props.show_close_button {
                    (close_button("Close"))
                }
            }
            @if let Some(desc) = props.description {
                p class="mui-sheet__description" id=(desc_id) {
                    (desc)
                }
            }
            div class="mui-sheet__body" {
                (props.children)
            }
            @if let Some(footer) = props.footer {
                div class="mui-sheet__footer" {
                    (footer)
                }
            }
        }
    }
}

/// Showcase every edge, labelled fields, and a long scrollable body.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            @for (side, key, label) in [
                (Side::Right, "right", "Edit guest details"),
                (Side::Left, "left", "Guest details"),
                (Side::Top, "top", "Arrival summary"),
                (Side::Bottom, "bottom", "Handover notes"),
            ] {
                @let id = format!("demo-sheet-{key}");
                section {
                    p.mui-showcase__caption { (key) " edge" }
                    (trigger(&id, label))
                    (render(Props {
                        id: id.clone(),
                        title: label.into(),
                        description: Some(if key == "bottom" {
                            "Read the handover. The footer stays in reach while the notes scroll."
                        } else {
                            "Review the guest information. Close or press Escape to return."
                        }.into()),
                        children: html! {
                            @if key == "bottom" {
                                @for number in 1..=10 {
                                    h3 { "Handover note " (number) }
                                    p { "Sofia arrives after 18:00. Confirm the room is ready, keep the guest's access instructions at reception, and pass any updates to the evening team." }
                                }
                            } @else {
                                div class="mui-field" {
                                    label class="mui-label" for=(format!("{id}-name")) { "Guest name" }
                                    input class="mui-input" id=(format!("{id}-name")) type="text" value="Sofia Davis";
                                }
                                div class="mui-field" {
                                    label class="mui-label" for=(format!("{id}-reference")) { "Reservation" }
                                    input class="mui-input" id=(format!("{id}-reference")) type="text" value="RS-2048" readonly;
                                }
                            }
                        },
                        footer: Some(html! {
                            button type="button" class="mui-btn mui-btn--outline mui-btn--md" data-mui-close { "Cancel" }
                            button type="button" class="mui-btn mui-btn--primary mui-btn--md" data-mui-close { "Done reviewing" }
                        }),
                        side, ..Default::default()
                    }))
                }
            }
        }
    }
}
