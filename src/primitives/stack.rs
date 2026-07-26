//! Stack component — the general-purpose layout container.
//!
//! Every other primitive in this crate renders *content*. `stack` renders
//! *structure*: a flex container along one axis with a token-scale gap. It is
//! the piece that lets a page be composed as a tree — containers holding
//! leaves — instead of every block hand-writing its own `div` with an inline
//! `style="display:flex;…"`.
//!
//! All appearance props are closed enums rather than free-form strings, so a
//! machine-readable manifest (and a builder UI's dropdown) can enumerate the
//! legal values. See `tokens::spacing` for the lengths the [`Space`] scale
//! resolves to.

use maud::{html, Markup};

/// Main-axis direction of the stack.
///
/// Reversed directions (`row-reverse` / `column-reverse`) are deliberately
/// absent: they decouple visual order from DOM order, which reorders keyboard
/// focus and screen-reader output away from what a sighted user sees. Reorder
/// the children instead.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    /// Children flow top-to-bottom (`flex-direction: column`).
    #[default]
    Vertical,
    /// Children flow left-to-right (`flex-direction: row`).
    Horizontal,
}

impl Direction {
    /// Returns the modifier class for this direction, or an empty string for the default.
    pub fn as_class(self) -> &'static str {
        match self {
            Direction::Vertical => "",
            Direction::Horizontal => "mui-stack--horizontal",
        }
    }
}

/// A step on the shared spacing scale, used by both `gap` and `padding`.
///
/// The steps mirror [`crate::tokens::spacing`] exactly, and resolve through the
/// `--mui-space-*` custom properties so a consumer theme can retune the whole
/// scale in one place.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Space {
    /// No space (`0`).
    #[default]
    None,
    /// `0.25rem`
    Xs,
    /// `0.5rem`
    Sm,
    /// `0.75rem`
    Md,
    /// `1rem`
    Lg,
    /// `1.5rem`
    Xl,
    /// `2rem`
    Xxl,
}

impl Space {
    /// Returns the modifier class for this step used as the *gap*, or an empty
    /// string for the gap default ([`Space::Md`], carried by `.mui-stack`).
    pub fn gap_class(self) -> &'static str {
        match self {
            Space::None => "mui-stack--gap-none",
            Space::Xs => "mui-stack--gap-xs",
            Space::Sm => "mui-stack--gap-sm",
            Space::Md => "",
            Space::Lg => "mui-stack--gap-lg",
            Space::Xl => "mui-stack--gap-xl",
            Space::Xxl => "mui-stack--gap-xxl",
        }
    }

    /// Returns the modifier class for this step used as *padding*, or an empty
    /// string for the padding default ([`Space::None`]).
    pub fn padding_class(self) -> &'static str {
        match self {
            Space::None => "",
            Space::Xs => "mui-stack--p-xs",
            Space::Sm => "mui-stack--p-sm",
            Space::Md => "mui-stack--p-md",
            Space::Lg => "mui-stack--p-lg",
            Space::Xl => "mui-stack--p-xl",
            Space::Xxl => "mui-stack--p-xxl",
        }
    }

    /// The CSS length this step resolves to. Mirrors [`crate::tokens::spacing`];
    /// useful when composing an inline style that has to match a stack's rhythm.
    pub fn as_length(self) -> &'static str {
        match self {
            Space::None => "0",
            Space::Xs => "0.25rem",
            Space::Sm => "0.5rem",
            Space::Md => "0.75rem",
            Space::Lg => "1rem",
            Space::Xl => "1.5rem",
            Space::Xxl => "2rem",
        }
    }
}

/// Cross-axis alignment (`align-items`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    /// Children fill the cross axis (`stretch`) — the flex default.
    #[default]
    Stretch,
    /// Pack children to the cross-axis start (`flex-start`).
    Start,
    /// Centre children on the cross axis (`center`).
    Center,
    /// Pack children to the cross-axis end (`flex-end`).
    End,
    /// Align children's text baselines (`baseline`).
    Baseline,
}

impl Align {
    /// Returns the modifier class for this alignment, or an empty string for the default.
    pub fn as_class(self) -> &'static str {
        match self {
            Align::Stretch => "",
            Align::Start => "mui-stack--align-start",
            Align::Center => "mui-stack--align-center",
            Align::End => "mui-stack--align-end",
            Align::Baseline => "mui-stack--align-baseline",
        }
    }
}

/// Main-axis distribution (`justify-content`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Justify {
    /// Pack children to the main-axis start (`flex-start`) — the flex default.
    #[default]
    Start,
    /// Centre children on the main axis (`center`).
    Center,
    /// Pack children to the main-axis end (`flex-end`).
    End,
    /// Equal space *between* children, none at the edges (`space-between`).
    Between,
    /// Equal space around each child (`space-around`).
    Around,
    /// Equal space between children *and* at the edges (`space-evenly`).
    Evenly,
}

impl Justify {
    /// Returns the modifier class for this distribution, or an empty string for the default.
    pub fn as_class(self) -> &'static str {
        match self {
            Justify::Start => "",
            Justify::Center => "mui-stack--justify-center",
            Justify::End => "mui-stack--justify-end",
            Justify::Between => "mui-stack--justify-between",
            Justify::Around => "mui-stack--justify-around",
            Justify::Evenly => "mui-stack--justify-evenly",
        }
    }
}

/// The HTML element the stack renders as.
///
/// A page assembled entirely from `div`s has no landmarks, so assistive tech
/// gets one undifferentiated blob. Choosing the element here keeps the document
/// outline intact without needing a second wrapper primitive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tag {
    /// `<div>` — no semantics.
    #[default]
    Div,
    /// `<section>` — a thematic grouping. **Requires `aria_label`**: an unnamed
    /// `<section>` is not exposed as a landmark at all.
    Section,
    /// `<article>` — a self-contained composition.
    Article,
    /// `<aside>` — tangentially related content (a `complementary` landmark).
    Aside,
    /// `<nav>` — a block of navigation links (a `navigation` landmark).
    Nav,
    /// `<header>` — introductory content (a `banner` landmark at page scope).
    Header,
    /// `<footer>` — closing content (a `contentinfo` landmark at page scope).
    Footer,
    /// `<main>` — the document's primary content. At most one per page.
    Main,
}

impl Tag {
    /// The element name this tag renders as.
    pub fn as_element(self) -> &'static str {
        match self {
            Tag::Div => "div",
            Tag::Section => "section",
            Tag::Article => "article",
            Tag::Aside => "aside",
            Tag::Nav => "nav",
            Tag::Header => "header",
            Tag::Footer => "footer",
            Tag::Main => "main",
        }
    }
}

/// Stack rendering properties.
#[derive(Debug, Clone)]
pub struct Props {
    /// Main-axis direction. Default [`Direction::Vertical`].
    pub direction: Direction,
    /// Space between children. Default [`Space::Md`] (`0.75rem`).
    pub gap: Space,
    /// Space inside the container's edges. Default [`Space::None`], so a stack
    /// is a drop-in wrapper that adds no inset of its own.
    pub padding: Space,
    /// Cross-axis alignment. Default [`Align::Stretch`].
    pub align: Align,
    /// Main-axis distribution. Default [`Justify::Start`].
    pub justify: Justify,
    /// Allow children to wrap onto additional lines (`flex-wrap: wrap`).
    /// Default `false`.
    pub wrap: bool,
    /// The element to render as. Default [`Tag::Div`].
    pub tag: Tag,
    /// Optional `id`, for anchor targets and `aria-labelledby` references.
    pub id: Option<String>,
    /// Accessible name for the container. Required when `tag` is
    /// [`Tag::Section`]; recommended when a page has more than one
    /// [`Tag::Nav`] or [`Tag::Aside`] so each landmark is distinguishable.
    pub aria_label: Option<String>,
    /// The stacked content.
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            direction: Direction::default(),
            gap: Space::Md,
            padding: Space::None,
            align: Align::default(),
            justify: Justify::default(),
            wrap: false,
            tag: Tag::default(),
            id: None,
            aria_label: None,
            children: html! {},
        }
    }
}

/// Builds the full class attribute for a set of props — base class plus every
/// non-default modifier, in a stable order.
fn class_for(props: &Props) -> String {
    let mut class = String::from("mui-stack");
    for modifier in [
        props.direction.as_class(),
        props.gap.gap_class(),
        props.padding.padding_class(),
        props.align.as_class(),
        props.justify.as_class(),
        if props.wrap { "mui-stack--wrap" } else { "" },
    ] {
        if !modifier.is_empty() {
            class.push(' ');
            class.push_str(modifier);
        }
    }
    class
}

/// `html!` needs a literal element name, so each [`Tag`] gets its own arm. The
/// attribute list is written once here rather than eight times.
macro_rules! stack_markup {
    ($props:expr, $class:expr, $($variant:ident => $el:ident),* $(,)?) => {
        match $props.tag {
            $(
                Tag::$variant => html! {
                    $el class=($class) id=[$props.id.as_deref()] aria-label=[$props.aria_label.as_deref()] {
                        ($props.children)
                    }
                },
            )*
        }
    };
}

/// Render a stack with the given properties.
///
/// Accessibility contract: [`Tag::Section`] MUST be paired with a non-`None`
/// `aria_label`. A `<section>` with no accessible name is stripped of its
/// `region` role by the accessibility mapping, so the grouping the tag was
/// chosen for silently does not exist for screen-reader users. Enforced via
/// `debug_assert!` in debug builds, matching `button`'s icon-only rule.
pub fn render(props: Props) -> Markup {
    debug_assert!(
        props.tag != Tag::Section || props.aria_label.is_some(),
        "Stack with Tag::Section requires aria_label — an unnamed <section> is not exposed as a landmark"
    );

    let class = class_for(&props);

    stack_markup! {
        props, class,
        Div => div,
        Section => section,
        Article => article,
        Aside => aside,
        Nav => nav,
        Header => header,
        Footer => footer,
        Main => main,
    }
}

/// Vertical stack with the default gap — the common case, without the `Props`
/// ceremony. Equivalent to `render(Props { children, ..Default::default() })`.
pub fn vertical(children: Markup) -> Markup {
    render(Props {
        children,
        ..Default::default()
    })
}

/// Horizontal stack with the default gap, vertically centred — the other common
/// case (a toolbar row, a button pair, an icon beside a label).
pub fn horizontal(children: Markup) -> Markup {
    render(Props {
        direction: Direction::Horizontal,
        align: Align::Center,
        children,
        ..Default::default()
    })
}

/// Showcase all stack use cases.
pub fn showcase() -> Markup {
    use crate::primitives::{badge, button, card};

    // A visible box so the layout itself is legible in the gallery.
    let cell = |text: &str| -> Markup {
        html! {
            div style="background:var(--mui-bg-input);border:1px solid var(--mui-border);border-radius:var(--mui-radius-md);padding:0.5rem 0.75rem;font-size:0.8125rem;color:var(--mui-text-muted);" {
                (text)
            }
        }
    };

    html! {
        div.mui-showcase__grid {
            // 1. Direction
            section {
                h2 { "Direction" }
                p.mui-showcase__caption { "Vertical (default)" }
                (render(Props {
                    children: html! { (cell("First")) (cell("Second")) (cell("Third")) },
                    ..Default::default()
                }))
                p.mui-showcase__caption style="margin-top:1rem;" { "Horizontal" }
                (render(Props {
                    direction: Direction::Horizontal,
                    children: html! { (cell("First")) (cell("Second")) (cell("Third")) },
                    ..Default::default()
                }))
            }

            // 2. Gap scale
            section {
                h2 { "Gap scale" }
                @for (gap, name) in [
                    (Space::None, "None \u{2014} 0"),
                    (Space::Xs, "Xs \u{2014} 0.25rem"),
                    (Space::Sm, "Sm \u{2014} 0.5rem"),
                    (Space::Md, "Md \u{2014} 0.75rem (default)"),
                    (Space::Lg, "Lg \u{2014} 1rem"),
                    (Space::Xl, "Xl \u{2014} 1.5rem"),
                    (Space::Xxl, "Xxl \u{2014} 2rem"),
                ] {
                    p.mui-showcase__caption { (name) }
                    div style="margin-bottom:0.75rem;" {
                        (render(Props {
                            direction: Direction::Horizontal,
                            gap,
                            children: html! { (cell("A")) (cell("B")) (cell("C")) },
                            ..Default::default()
                        }))
                    }
                }
            }

            // 3. Cross-axis alignment
            section {
                h2 { "Align (cross axis)" }
                @for (align, name) in [
                    (Align::Stretch, "Stretch (default)"),
                    (Align::Start, "Start"),
                    (Align::Center, "Center"),
                    (Align::End, "End"),
                    (Align::Baseline, "Baseline"),
                ] {
                    p.mui-showcase__caption { (name) }
                    div style="height:5rem;border:1px dashed var(--mui-border);border-radius:var(--mui-radius-md);margin-bottom:0.75rem;" {
                        (render(Props {
                            direction: Direction::Horizontal,
                            align,
                            padding: Space::Sm,
                            children: html! {
                                (cell("Short"))
                                div style="background:var(--mui-bg-input);border:1px solid var(--mui-border);border-radius:var(--mui-radius-md);padding:1.25rem 0.75rem;font-size:1.25rem;color:var(--mui-text-muted);" { "Tall" }
                                (cell("Short"))
                            },
                            ..Default::default()
                        }))
                    }
                }
            }

            // 4. Main-axis distribution
            section {
                h2 { "Justify (main axis)" }
                @for (justify, name) in [
                    (Justify::Start, "Start (default)"),
                    (Justify::Center, "Center"),
                    (Justify::End, "End"),
                    (Justify::Between, "Between"),
                    (Justify::Around, "Around"),
                    (Justify::Evenly, "Evenly"),
                ] {
                    p.mui-showcase__caption { (name) }
                    div style="border:1px dashed var(--mui-border);border-radius:var(--mui-radius-md);margin-bottom:0.75rem;" {
                        (render(Props {
                            direction: Direction::Horizontal,
                            justify,
                            padding: Space::Sm,
                            children: html! { (cell("A")) (cell("B")) (cell("C")) },
                            ..Default::default()
                        }))
                    }
                }
            }

            // 5. Wrap
            section {
                h2 { "Wrap" }
                p.mui-showcase__caption { "wrap: true \u{2014} children flow onto new lines instead of overflowing" }
                div style="max-width:22rem;border:1px dashed var(--mui-border);border-radius:var(--mui-radius-md);" {
                    (render(Props {
                        direction: Direction::Horizontal,
                        wrap: true,
                        padding: Space::Sm,
                        children: html! {
                            @for tag in ["rust", "maud", "htmx", "axum", "tokio", "serde", "esbuild"] {
                                (badge::render(badge::Props {
                                    label: tag.into(),
                                    ..Default::default()
                                }))
                            }
                        },
                        ..Default::default()
                    }))
                }
            }

            // 6. Padding
            section {
                h2 { "Padding" }
                p.mui-showcase__caption { "padding: Xl \u{2014} inset applied by the container itself" }
                div style="border:1px dashed var(--mui-border);border-radius:var(--mui-radius-md);" {
                    (render(Props {
                        padding: Space::Xl,
                        children: html! { (cell("Inset content")) },
                        ..Default::default()
                    }))
                }
            }

            // 7. Helpers + real composition
            section {
                h2 { "Helpers" }
                p.mui-showcase__caption { "stack::horizontal(\u{2026}) \u{2014} row, centred, default gap" }
                (horizontal(html! {
                    (button::render(button::Props {
                        label: "Cancel".into(),
                        variant: button::Variant::Outline,
                        ..Default::default()
                    }))
                    (button::render(button::Props {
                        label: "Save changes".into(),
                        variant: button::Variant::Primary,
                        ..Default::default()
                    }))
                }))
                p.mui-showcase__caption style="margin-top:1rem;" { "stack::vertical(\u{2026}) inside a card body" }
                (card::render(card::Props {
                    title: Some("Deployment".into()),
                    description: Some("Composed with nested stacks \u{2014} no inline flex styles.".into()),
                    children: vertical(html! {
                        (render(Props {
                            direction: Direction::Horizontal,
                            justify: Justify::Between,
                            align: Align::Center,
                            children: html! {
                                span style="font-size:0.875rem;" { "Region" }
                                (badge::render(badge::Props { label: "eu-central".into(), ..Default::default() }))
                            },
                            ..Default::default()
                        }))
                        (render(Props {
                            direction: Direction::Horizontal,
                            justify: Justify::Between,
                            align: Align::Center,
                            children: html! {
                                span style="font-size:0.875rem;" { "Status" }
                                (badge::render(badge::Props {
                                    label: "Healthy".into(),
                                    variant: badge::Variant::Success,
                                    ..Default::default()
                                }))
                            },
                            ..Default::default()
                        }))
                    }),
                    ..Default::default()
                }))
            }

            // 8. Semantic tags
            section {
                h2 { "Semantic tags" }
                p.mui-showcase__caption { "tag: Tag::Nav \u{2014} renders <nav>, keeping the landmark intact" }
                (render(Props {
                    tag: Tag::Nav,
                    direction: Direction::Horizontal,
                    gap: Space::Lg,
                    aria_label: Some("Stack example navigation".into()),
                    children: html! {
                        a href="#" style="font-size:0.875rem;color:var(--mui-accent-text);" { "Overview" }
                        a href="#" style="font-size:0.875rem;color:var(--mui-accent-text);" { "Deployments" }
                        a href="#" style="font-size:0.875rem;color:var(--mui-accent-text);" { "Settings" }
                    },
                    ..Default::default()
                }))
                p.mui-showcase__caption style="margin-top:1rem;" { "tag: Tag::Section \u{2014} requires aria_label (debug_assert)" }
                (render(Props {
                    tag: Tag::Section,
                    padding: Space::Md,
                    aria_label: Some("Release notes".into()),
                    children: html! { (cell("A named region, exposed as a landmark")) },
                    ..Default::default()
                }))
            }
        }
    }
}
