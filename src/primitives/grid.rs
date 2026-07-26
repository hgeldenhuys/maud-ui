//! Grid component — the two-dimensional layout container.
//!
//! Where [`stack`](super::stack) lays children along one axis, `grid` lays them
//! into columns. The two share one spacing scale and one alignment vocabulary:
//! `gap` is [`Space`] and `align` is [`Align`], both re-exported from `stack`,
//! so a manifest describes those value sets once rather than once per container.
//!
//! The default is [`Columns::AutoFit`] — `repeat(auto-fit, minmax(…, 1fr))` —
//! which fills the available width with as many columns as fit and needs no
//! media query. Fixed counts are available when a layout genuinely requires
//! exactly N columns.

use maud::{html, Markup};

pub use super::stack::{Align, Space};

/// Column track definition.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Columns {
    /// As many equal columns as fit, each at least [`MinColumn`] wide —
    /// `repeat(auto-fit, minmax(var(--mui-grid-min), 1fr))`. Responsive with no
    /// media query, and the only option that adapts to its container rather
    /// than the viewport.
    #[default]
    AutoFit,
    /// A single column.
    One,
    /// Two equal columns.
    Two,
    /// Three equal columns.
    Three,
    /// Four equal columns.
    Four,
    /// Five equal columns.
    Five,
    /// Six equal columns.
    Six,
    /// Twelve equal columns — the classic layout grid, for children that span
    /// several tracks.
    Twelve,
}

impl Columns {
    /// Returns the modifier class for this track definition, or an empty string
    /// for the default ([`Columns::AutoFit`], carried by `.mui-grid`).
    pub fn as_class(self) -> &'static str {
        match self {
            Columns::AutoFit => "",
            Columns::One => "mui-grid--cols-1",
            Columns::Two => "mui-grid--cols-2",
            Columns::Three => "mui-grid--cols-3",
            Columns::Four => "mui-grid--cols-4",
            Columns::Five => "mui-grid--cols-5",
            Columns::Six => "mui-grid--cols-6",
            Columns::Twelve => "mui-grid--cols-12",
        }
    }

    /// The number of columns this definition produces, or `None` for
    /// [`Columns::AutoFit`], whose count depends on the available width.
    pub fn count(self) -> Option<u8> {
        match self {
            Columns::AutoFit => None,
            Columns::One => Some(1),
            Columns::Two => Some(2),
            Columns::Three => Some(3),
            Columns::Four => Some(4),
            Columns::Five => Some(5),
            Columns::Six => Some(6),
            Columns::Twelve => Some(12),
        }
    }
}

/// Minimum column width for [`Columns::AutoFit`] — the knob that decides how
/// many columns fit. Ignored by fixed column counts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MinColumn {
    /// `10rem` — dense: chips, swatches, small tiles.
    Sm,
    /// `14rem` — the default: stat cards, thumbnails.
    #[default]
    Md,
    /// `18rem` — content cards.
    Lg,
    /// `24rem` — wide panels, forms.
    Xl,
}

impl MinColumn {
    /// Returns the modifier class, or an empty string for the default
    /// ([`MinColumn::Md`], carried by `.mui-grid`).
    pub fn as_class(self) -> &'static str {
        match self {
            MinColumn::Sm => "mui-grid--min-sm",
            MinColumn::Md => "",
            MinColumn::Lg => "mui-grid--min-lg",
            MinColumn::Xl => "mui-grid--min-xl",
        }
    }

    /// The CSS length this step resolves to.
    pub fn as_length(self) -> &'static str {
        match self {
            MinColumn::Sm => "10rem",
            MinColumn::Md => "14rem",
            MinColumn::Lg => "18rem",
            MinColumn::Xl => "24rem",
        }
    }
}

/// Grid rendering properties.
#[derive(Debug, Clone)]
pub struct Props {
    /// Column track definition. Default [`Columns::AutoFit`].
    pub columns: Columns,
    /// Minimum column width for [`Columns::AutoFit`]. Default [`MinColumn::Md`].
    /// Has no effect on fixed column counts.
    pub min_column: MinColumn,
    /// Space between both rows and columns. Default [`Space::Md`] (`0.75rem`).
    pub gap: Space,
    /// Space inside the container's edges. Default [`Space::None`].
    pub padding: Space,
    /// Block-axis alignment of items within their track (`align-items`).
    /// Default [`Align::Stretch`].
    pub align: Align,
    /// Collapse fixed column counts to a single column below `40rem`.
    /// Default `true` — see [`render`] for why.
    pub collapse_narrow: bool,
    /// Optional `id`, for anchor targets and `aria-labelledby` references.
    pub id: Option<String>,
    /// The grid's children — each becomes one grid item.
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            columns: Columns::default(),
            min_column: MinColumn::default(),
            gap: Space::Md,
            padding: Space::None,
            align: Align::default(),
            collapse_narrow: true,
            id: None,
            children: html! {},
        }
    }
}

/// Maps a spacing step onto this component's gap modifier. `Md` is the default
/// and is carried by `.mui-grid` itself.
fn gap_class(space: Space) -> &'static str {
    match space {
        Space::None => "mui-grid--gap-none",
        Space::Xs => "mui-grid--gap-xs",
        Space::Sm => "mui-grid--gap-sm",
        Space::Md => "",
        Space::Lg => "mui-grid--gap-lg",
        Space::Xl => "mui-grid--gap-xl",
        Space::Xxl => "mui-grid--gap-xxl",
    }
}

/// Maps a spacing step onto this component's padding modifier. `None` is the
/// default and emits nothing.
fn padding_class(space: Space) -> &'static str {
    match space {
        Space::None => "",
        Space::Xs => "mui-grid--p-xs",
        Space::Sm => "mui-grid--p-sm",
        Space::Md => "mui-grid--p-md",
        Space::Lg => "mui-grid--p-lg",
        Space::Xl => "mui-grid--p-xl",
        Space::Xxl => "mui-grid--p-xxl",
    }
}

/// Maps a cross-axis alignment onto this component's modifier.
fn align_class(align: Align) -> &'static str {
    match align {
        Align::Stretch => "",
        Align::Start => "mui-grid--align-start",
        Align::Center => "mui-grid--align-center",
        Align::End => "mui-grid--align-end",
        Align::Baseline => "mui-grid--align-baseline",
    }
}

/// Render a grid with the given properties.
///
/// `collapse_narrow` defaults to `true`: below `40rem` a fixed column count
/// becomes a single column. A four-column grid on a phone is four unreadable
/// slivers, and this crate offers no class escape hatch a consumer could use to
/// fix it themselves — so the responsive behaviour has to live here. Set it to
/// `false` when the columns are genuinely small (a swatch palette, a numeric
/// keypad) and should stay side by side at every width. [`Columns::AutoFit`] is
/// unaffected either way: it already reflows on its own.
pub fn render(props: Props) -> Markup {
    let mut class = String::from("mui-grid");
    for modifier in [
        props.columns.as_class(),
        // The auto-fit minimum only means anything for auto-fit tracks.
        if props.columns == Columns::AutoFit {
            props.min_column.as_class()
        } else {
            ""
        },
        gap_class(props.gap),
        padding_class(props.padding),
        align_class(props.align),
        // Only fixed counts can collapse; auto-fit already reflows.
        if props.collapse_narrow && props.columns != Columns::AutoFit {
            "mui-grid--collapse"
        } else {
            ""
        },
    ] {
        if !modifier.is_empty() {
            class.push(' ');
            class.push_str(modifier);
        }
    }

    html! {
        div class=(class) id=[props.id.as_deref()] {
            (props.children)
        }
    }
}

/// Auto-fitting grid with the default gap and minimum column width — the common
/// case, without the `Props` ceremony.
pub fn auto(children: Markup) -> Markup {
    render(Props {
        children,
        ..Default::default()
    })
}

/// Fixed-column grid with the default gap, collapsing to one column on narrow
/// viewports.
pub fn columns(columns: Columns, children: Markup) -> Markup {
    render(Props {
        columns,
        children,
        ..Default::default()
    })
}

/// Showcase all grid use cases.
pub fn showcase() -> Markup {
    use crate::primitives::{badge, card, stack};

    let cell = |text: &str| -> Markup {
        html! {
            div style="background:var(--mui-bg-input);border:1px solid var(--mui-border);border-radius:var(--mui-radius-md);padding:0.75rem;font-size:0.8125rem;color:var(--mui-text-muted);text-align:center;" {
                (text)
            }
        }
    };

    html! {
        div.mui-showcase__grid {
            // 1. Auto-fit — the default
            section {
                h2 { "Auto-fit (default)" }
                p.mui-showcase__caption {
                    "As many equal columns as fit, each at least 14rem. Resize the window \u{2014} no media query involved."
                }
                (auto(html! {
                    @for n in 1..=6 {
                        (cell(&format!("Item {n}")))
                    }
                }))
            }

            // 2. Minimum column width
            section {
                h2 { "Minimum column width" }
                @for (min, name) in [
                    (MinColumn::Sm, "Sm \u{2014} 10rem (dense)"),
                    (MinColumn::Md, "Md \u{2014} 14rem (default)"),
                    (MinColumn::Lg, "Lg \u{2014} 18rem"),
                    (MinColumn::Xl, "Xl \u{2014} 24rem"),
                ] {
                    p.mui-showcase__caption { (name) }
                    div style="margin-bottom:1rem;" {
                        (render(Props {
                            min_column: min,
                            children: html! {
                                @for n in 1..=4 { (cell(&format!("{n}"))) }
                            },
                            ..Default::default()
                        }))
                    }
                }
            }

            // 3. Fixed column counts
            section {
                h2 { "Fixed columns" }
                p.mui-showcase__caption {
                    "Collapse to one column below 40rem by default \u{2014} four columns on a phone is four unreadable slivers."
                }
                @for (cols, name) in [
                    (Columns::Two, "Two"),
                    (Columns::Three, "Three"),
                    (Columns::Four, "Four"),
                    (Columns::Six, "Six"),
                ] {
                    p.mui-showcase__caption { (name) }
                    div style="margin-bottom:1rem;" {
                        (columns(cols, html! {
                            @for n in 1..=cols.count().unwrap_or(3) { (cell(&format!("{n}"))) }
                        }))
                    }
                }
            }

            // 4. collapse_narrow: false
            section {
                h2 { "collapse_narrow: false" }
                p.mui-showcase__caption {
                    "For genuinely small cells that should stay side by side at every width \u{2014} a swatch palette, a keypad."
                }
                (render(Props {
                    columns: Columns::Six,
                    collapse_narrow: false,
                    gap: Space::Sm,
                    children: html! {
                        @for hex in ["#2563eb", "#15803d", "#a16207", "#dc2626", "#6d28d9", "#be123c"] {
                            div style=(format!("background:{hex};height:2.5rem;border-radius:var(--mui-radius-md);")) {}
                        }
                    },
                    ..Default::default()
                }))
            }

            // 5. Gap scale
            section {
                h2 { "Gap scale" }
                p.mui-showcase__caption { "The same Space scale as stack \u{2014} one vocabulary across both containers." }
                @for (gap, name) in [
                    (Space::None, "None"),
                    (Space::Sm, "Sm"),
                    (Space::Md, "Md (default)"),
                    (Space::Xl, "Xl"),
                ] {
                    p.mui-showcase__caption { (name) }
                    div style="margin-bottom:1rem;" {
                        (render(Props {
                            columns: Columns::Four,
                            collapse_narrow: false,
                            gap,
                            children: html! {
                                @for n in 1..=4 { (cell(&format!("{n}"))) }
                            },
                            ..Default::default()
                        }))
                    }
                }
            }

            // 6. Align
            section {
                h2 { "Align (block axis)" }
                @for (align, name) in [
                    (Align::Stretch, "Stretch (default) \u{2014} items fill the row"),
                    (Align::Start, "Start"),
                    (Align::Center, "Center"),
                    (Align::End, "End"),
                ] {
                    p.mui-showcase__caption { (name) }
                    div style="border:1px dashed var(--mui-border);border-radius:var(--mui-radius-md);margin-bottom:1rem;" {
                        (render(Props {
                            columns: Columns::Three,
                            collapse_narrow: false,
                            align,
                            padding: Space::Sm,
                            children: html! {
                                (cell("Short"))
                                div style="background:var(--mui-bg-input);border:1px solid var(--mui-border);border-radius:var(--mui-radius-md);padding:2rem 0.75rem;font-size:0.8125rem;color:var(--mui-text-muted);text-align:center;" { "Tall" }
                                (cell("Short"))
                            },
                            ..Default::default()
                        }))
                    }
                }
            }

            // 7. Real composition — a card grid
            section {
                h2 { "Composition" }
                p.mui-showcase__caption { "A dashboard card grid \u{2014} grid for the columns, stack for each card's rows." }
                (render(Props {
                    min_column: MinColumn::Lg,
                    gap: Space::Lg,
                    children: html! {
                        @for (title, value, delta, tone) in [
                            ("Total revenue", "$45,231.89", "+20.1%", badge::Variant::Success),
                            ("Subscriptions", "2,350", "+180.1%", badge::Variant::Success),
                            ("Active now", "573", "-4.3%", badge::Variant::Danger),
                        ] {
                            (card::render(card::Props {
                                title: Some(title.into()),
                                size: card::Size::Sm,
                                children: stack::render(stack::Props {
                                    gap: Space::Xs,
                                    // Align::Start, not the Stretch default: a badge
                                    // stretched across the cross axis becomes a
                                    // full-width bar instead of a chip.
                                    align: Align::Start,
                                    children: html! {
                                        p style="font-size:1.5rem;font-weight:700;margin:0;" { (value) }
                                        (badge::render(badge::Props {
                                            label: format!("{delta} from last month"),
                                            variant: tone,
                                            ..Default::default()
                                        }))
                                    },
                                    ..Default::default()
                                }),
                                ..Default::default()
                            }))
                        }
                    },
                    ..Default::default()
                }))
            }
        }
    }
}
