//! Sidebar primitive — collapsible app-shell sidebar modelled on shadcn's Sidebar.
//! Renders as an `<aside>` element whose state is driven by `data-state` attributes
//! (`expanded` / `collapsed`). The companion JS behaviour (`dist/behaviors/sidebar.js`)
//! toggles the state on `Cmd/Ctrl+B` and on `[data-mui="sidebar-trigger"]` click.
//!
//! Subcomponents are exposed as free functions returning `Markup`, following the
//! same pattern as `navigation_menu` / `menubar`.
use maud::{html, Markup};

/// Which edge the sidebar anchors to
#[derive(Clone, Debug, Default)]
pub enum Side {
    #[default]
    Left,
    Right,
}

impl Side {
    pub fn as_data(&self) -> &'static str {
        match self {
            Side::Left => "left",
            Side::Right => "right",
        }
    }
}

/// Visual variant of the sidebar
#[derive(Clone, Debug, Default)]
pub enum SidebarVariant {
    #[default]
    Sidebar,
    Floating,
    Inset,
}

impl SidebarVariant {
    pub fn as_data(&self) -> &'static str {
        match self {
            SidebarVariant::Sidebar => "sidebar",
            SidebarVariant::Floating => "floating",
            SidebarVariant::Inset => "inset",
        }
    }
}

/// How the sidebar collapses when toggled off
#[derive(Clone, Debug, Default)]
pub enum Collapsible {
    #[default]
    Offcanvas,
    Icon,
    None,
}

impl Collapsible {
    pub fn as_data(&self) -> &'static str {
        match self {
            Collapsible::Offcanvas => "offcanvas",
            Collapsible::Icon => "icon",
            Collapsible::None => "none",
        }
    }
}

/// Sidebar root rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier (used by trigger + behaviour to target the sidebar)
    pub id: String,
    /// Which edge the sidebar anchors to
    pub side: Side,
    /// Visual variant
    pub variant: SidebarVariant,
    /// How the sidebar collapses
    pub collapsible: Collapsible,
    /// Whether the sidebar renders in its expanded state (SSR default)
    pub default_open: bool,
    /// Markup content (typically header / content / footer helpers)
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "sidebar".to_string(),
            side: Side::default(),
            variant: SidebarVariant::default(),
            collapsible: Collapsible::default(),
            default_open: true,
            children: html! {},
        }
    }
}

/// Provider wrapper — establishes the `data-mui="sidebar-provider"` layout container
/// that positions sidebar + inset side-by-side. Use at the app-shell root.
pub fn provider(children: Markup) -> Markup {
    html! {
        div class="mui-sidebar-provider" data-mui="sidebar-provider" {
            (children)
        }
    }
}

/// Render the sidebar root with all data-attributes wired up.
pub fn render(props: Props) -> Markup {
    let state = if props.default_open {
        "expanded"
    } else {
        "collapsed"
    };
    html! {
        aside
            class="mui-sidebar"
            id=(props.id)
            data-mui="sidebar"
            data-state=(state)
            data-side=(props.side.as_data())
            data-variant=(props.variant.as_data())
            data-collapsible=(props.collapsible.as_data())
            aria-label="Sidebar"
        {
            (props.children)
        }
    }
}

/// Header region — typically brand / workspace switcher
pub fn header(children: Markup) -> Markup {
    html! { div class="mui-sidebar__header" { (children) } }
}

/// Content region — scrollable column holding groups
pub fn content(children: Markup) -> Markup {
    html! { div class="mui-sidebar__content" { (children) } }
}

/// Footer region — pinned to the bottom
pub fn footer(children: Markup) -> Markup {
    html! { div class="mui-sidebar__footer" { (children) } }
}

/// Group — a labeled cluster of menu items
pub fn group(children: Markup) -> Markup {
    html! { div class="mui-sidebar__group" { (children) } }
}

/// Group label — small-caps heading for a group
pub fn group_label(children: Markup) -> Markup {
    html! { div class="mui-sidebar__group-label" { (children) } }
}

/// Group action — button anchored to the group header (e.g. add/plus icon)
pub fn group_action(children: Markup) -> Markup {
    html! {
        button type="button" class="mui-sidebar__group-action" {
            (children)
        }
    }
}

/// Group content — container for the menu inside a group
pub fn group_content(children: Markup) -> Markup {
    html! { div class="mui-sidebar__group-content" { (children) } }
}

/// Menu — `<ul>` wrapper for menu items
pub fn menu(children: Markup) -> Markup {
    html! { ul class="mui-sidebar__menu" { (children) } }
}

/// Menu item — `<li>` wrapper. Pass a `menu_button` (or link) as the child.
pub fn menu_item(children: Markup) -> Markup {
    html! { li class="mui-sidebar__menu-item" { (children) } }
}

/// Menu button — the interactive row. Caller supplies label/icon markup.
pub fn menu_button(children: Markup) -> Markup {
    html! {
        button type="button" class="mui-sidebar__menu-button" {
            (children)
        }
    }
}

/// Menu action — trailing action button on a menu item (e.g. kebab menu)
pub fn menu_action(children: Markup) -> Markup {
    html! {
        button type="button" class="mui-sidebar__menu-action" {
            (children)
        }
    }
}

/// Menu sub — nested sub-menu (`<ul>`) indented under a parent item
pub fn menu_sub(children: Markup) -> Markup {
    html! { ul class="mui-sidebar__menu-sub" { (children) } }
}

/// Menu sub item — `<li>` row inside a sub-menu
pub fn menu_sub_item(children: Markup) -> Markup {
    html! { li class="mui-sidebar__menu-sub-item" { (children) } }
}

/// Menu badge — small badge/count pill attached to a menu item
pub fn menu_badge(children: Markup) -> Markup {
    html! { span class="mui-sidebar__menu-badge" { (children) } }
}

/// Menu skeleton — loading placeholder for a menu row
pub fn menu_skeleton() -> Markup {
    html! {
        li class="mui-sidebar__menu-item" {
            div class="mui-sidebar__menu-skeleton" {
                div class="mui-sidebar__menu-skeleton-icon" {}
                div class="mui-sidebar__menu-skeleton-text" {}
            }
        }
    }
}

/// Trigger — button the user clicks to toggle the sidebar.
/// Pair with the `sidebar` JS behaviour which listens for
/// `[data-mui="sidebar-trigger"][data-target="<id>"]` clicks.
pub fn trigger(target_id: &str, label: &str) -> Markup {
    html! {
        button type="button"
            class="mui-btn mui-btn--ghost mui-btn--sm mui-sidebar__trigger"
            data-mui="sidebar-trigger"
            data-target=(target_id)
            aria-label=(label)
        {
            span class="mui-sidebar__trigger-icon" aria-hidden="true" {
                "\u{2630}"
            }
            span class="mui-sidebar__trigger-label" { (label) }
        }
    }
}

/// Rail — thin interactive strip along the sidebar edge that expands a
/// collapsed (icon-variant) sidebar on click. Rendered as a sibling inside
/// the `<aside>` so it positions absolutely relative to it.
pub fn rail() -> Markup {
    html! {
        button type="button"
            class="mui-sidebar__rail"
            data-mui="sidebar-rail"
            aria-label="Toggle sidebar"
            tabindex="-1"
        {}
    }
}

/// Inset — layout sibling that holds main content, reserving space next to
/// the sidebar. Use inside `provider()` alongside `render(...)`.
pub fn inset(children: Markup) -> Markup {
    html! {
        main class="mui-sidebar-inset" data-mui="sidebar-inset" {
            (children)
        }
    }
}

/// Showcase — small left sidebar with header, two groups, and a footer.
///
/// Note: each menu_button gets a leading icon span marked `aria-hidden="true"`
/// so CSS `span:not([aria-hidden="true"])` hides the label in icon-collapsed
/// mode but keeps the icon visible. Without the icon, icon-collapsed is blank.
pub fn showcase() -> Markup {
    // Tiny helper — a decorative glyph slot consistent with the group-label
    // hiding selector in sidebar.css.
    let ico = |glyph: &'static str| -> Markup {
        html! { span class="mui-sidebar__menu-icon" aria-hidden="true" { (glyph) } }
    };
    html! {
        div class="mui-showcase__grid" {
            section {
                h2 { "App shell (left, icon-collapsible)" }
                p class="mui-showcase__caption" {
                    "Press " kbd { "Cmd/Ctrl" } " + " kbd { "B" } " to toggle, or click the menu button."
                }
                div class="mui-sidebar-showcase" {
                    (provider(html! {
                        (render(Props {
                            id: "demo-sidebar".to_string(),
                            side: Side::Left,
                            variant: SidebarVariant::Sidebar,
                            collapsible: Collapsible::Icon,
                            default_open: true,
                            children: html! {
                                (header(html! {
                                    div class="mui-sidebar__brand" {
                                        span class="mui-sidebar__brand-mark" aria-hidden="true" { "M" }
                                        span class="mui-sidebar__brand-name" { "maud-ui" }
                                    }
                                }))
                                (content(html! {
                                    (group(html! {
                                        (group_label(html! { "Platform" }))
                                        (group_content(html! {
                                            (menu(html! {
                                                (menu_item(menu_button(html! {
                                                    (ico("\u{25a0}"))
                                                    span { "Dashboard" }
                                                })))
                                                (menu_item(menu_button(html! {
                                                    (ico("\u{2630}"))
                                                    span { "Projects" }
                                                    (menu_badge(html! { "12" }))
                                                })))
                                                (menu_item(menu_button(html! {
                                                    (ico("\u{2191}"))
                                                    span { "Deployments" }
                                                })))
                                            }))
                                        }))
                                    }))
                                    (group(html! {
                                        (group_label(html! { "Workspace" }))
                                        (group_content(html! {
                                            (menu(html! {
                                                (menu_item(menu_button(html! {
                                                    (ico("\u{25cb}"))
                                                    span { "Members" }
                                                })))
                                                (menu_item(menu_button(html! {
                                                    (ico("\u{25c6}"))
                                                    span { "Billing" }
                                                })))
                                                (menu_item(menu_button(html! {
                                                    (ico("\u{2699}"))
                                                    span { "Settings" }
                                                })))
                                            }))
                                        }))
                                    }))
                                }))
                                (footer(html! {
                                    div class="mui-sidebar__user" {
                                        span class="mui-sidebar__user-avatar" aria-hidden="true" { "JD" }
                                        span class="mui-sidebar__user-name" { "Jane Doe" }
                                    }
                                }))
                                (rail())
                            },
                        }))
                        (inset(html! {
                            div class="mui-sidebar-inset__bar" {
                                (trigger("demo-sidebar", "Toggle sidebar"))
                                span class="mui-sidebar-inset__title" { "Dashboard" }
                            }
                            div class="mui-sidebar-inset__body" {
                                div class="mui-sidebar-demo__stats" {
                                    div class="mui-sidebar-demo__stat" {
                                        span class="mui-sidebar-demo__stat-label" { "Build status" }
                                        span class="mui-sidebar-demo__stat-value" { "Passing" }
                                    }
                                    div class="mui-sidebar-demo__stat" {
                                        span class="mui-sidebar-demo__stat-label" { "Active deploys" }
                                        span class="mui-sidebar-demo__stat-value" { "3" }
                                    }
                                    div class="mui-sidebar-demo__stat" {
                                        span class="mui-sidebar-demo__stat-label" { "Last release" }
                                        span class="mui-sidebar-demo__stat-value" { "v0.2.1" }
                                    }
                                }
                                p class="mui-sidebar-demo__note" {
                                    "Replace this inset with your app's content. The sidebar width animates between "
                                    kbd { "16rem" } " and " kbd { "3rem" } " via "
                                    kbd { "data-state" } "."
                                }
                            }
                        }))
                    }))
                }
            }
        }
    }
}
