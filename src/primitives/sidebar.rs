//! Sidebar primitive — collapsible app-shell sidebar modelled on shadcn's Sidebar.
//! Renders as an `<aside>` element whose state is driven by `data-state` attributes
//! (`expanded` / `collapsed`). The companion JS behaviour (`static/behaviors/sidebar.js`)
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

/// What ground the sidebar sits on. `Card` (the default) paints the card
/// surface and reserves the `--mui-sidebar-w` column; `Transparent` is a bare
/// cell — no background, no reserved width, its box comes from the parent
/// grid/flex layout (demo-frame and canvas layouts).
#[derive(Clone, Debug, Default)]
pub enum Surface {
    #[default]
    Card,
    Transparent,
}

impl Surface {
    pub fn as_data(&self) -> &'static str {
        match self {
            Surface::Card => "card",
            Surface::Transparent => "transparent",
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
    /// What ground the sidebar sits on. See [`Surface`].
    pub surface: Surface,
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
            surface: Surface::default(),
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
            id=(&props.id)
            data-mui="sidebar"
            data-state=(state)
            data-side=(props.side.as_data())
            data-variant=(props.variant.as_data())
            data-surface=(props.surface.as_data())
            data-collapsible=(props.collapsible.as_data())
            aria-label="Sidebar"
        {
            (props.children)
        }
        dialog class="mui-navigation-dialog" id=(format!("{}-drawer", props.id)) aria-label="Sidebar navigation" {
            form method="dialog" { button class="mui-btn mui-btn--outline mui-btn--sm" { "Close navigation" } }
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

/// Native collapsible group with local state keyed by a stable, page-unique ID.
pub fn collapsible_group(id: &str, label: &str, children: Markup, default_open: bool) -> Markup {
    let contains_current = children.0.contains("aria-current=\"page\"");
    html! { details class="mui-sidebar__group" id=(id) data-mui="nav-group" data-nav-key=(id) data-contains-current=[contains_current.then_some("true")] open[default_open] {
        summary class="mui-sidebar__group-label" { (label) }
        (children)
    } }
}

/// Navigation row with a full accessible name retained in an icon rail.
pub fn menu_link(href: &str, label: &str, icon: Option<Markup>, active: bool) -> Markup {
    html! { a class="mui-sidebar__menu-button" href=(href) aria-label=(label) title=(label) aria-current=[active.then_some("page")] {
        span class="mui-sidebar__menu-icon" aria-hidden="true" { @if let Some(icon) = icon { (icon) } @else { (label.chars().next().unwrap_or('·')) } }
        span { (label) }
    } }
}

/// Group label — small-caps heading for a group
pub fn group_label(children: Markup) -> Markup {
    html! { div class="mui-sidebar__group-label" { (children) } }
}

/// Group label as a collapsible row — a 24px flex row with a chevron slot
/// (Linear's group header), instead of the small-caps heading. `chevron` is
/// the trailing glyph markup (a chevron/caret icon); `None` renders the row
/// with no chevron slot content.
pub fn group_label_collapsible(children: Markup, chevron: Option<Markup>) -> Markup {
    html! { div class="mui-sidebar__group-label mui-sidebar__group-label--collapsible" {
        (children)
        @if let Some(chevron) = chevron {
            span class="mui-sidebar__group-chevron" aria-hidden="true" { (chevron) }
        }
    } }
}

/// How a current (`current: true`) menu row is painted.
///
/// [`CurrentVariant::Default`] is the Linear recipe: a 4% ground with the ink
/// and weight unchanged. [`CurrentVariant::Accent`] is the legacy library
/// look — accent-soft ground, accent ink, heavier weight and the inset
/// accent bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CurrentVariant {
    /// 4% ground, ink unchanged (the default current style).
    #[default]
    Default,
    /// Legacy accent-soft recolour + inset accent bar.
    Accent,
}

/// Menu button properties — [`menu_button_props`] renders the interactive
/// row from these.
#[derive(Debug, Clone)]
pub struct MenuButtonProps {
    /// Row content: the icon span and the label.
    pub children: Markup,
    /// Mark the row as the current page (white-4% ground by default; see
    /// [`CurrentVariant`]).
    pub current: bool,
    /// Which current-row treatment to use when `current` is set.
    pub current_variant: CurrentVariant,
    /// Extra class names appended to the class list — the escape hatch for
    /// per-instance overrides that a closed enum cannot express.
    pub class: Option<String>,
}

impl Default for MenuButtonProps {
    fn default() -> Self {
        Self {
            children: html! {},
            current: false,
            current_variant: CurrentVariant::default(),
            class: None,
        }
    }
}

/// Menu button — the interactive row, from props (current state, current
/// variant, class passthrough). The plain [`menu_button`] wrapper still
/// exists for children-only rows.
pub fn menu_button_props(props: MenuButtonProps) -> Markup {
    let mut class = String::from("mui-sidebar__menu-button");
    if props.current && props.current_variant == CurrentVariant::Accent {
        class.push_str(" mui-sidebar__menu-button--accent");
    }
    if let Some(extra) = &props.class {
        class.push(' ');
        class.push_str(extra);
    }
    html! {
        button type="button" class=(class) aria-current=[props.current.then_some("page")] {
            (props.children)
        }
    }
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
    html! { div { (children) } }
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
    html! { li { (children) } }
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
            span aria-hidden="true" {
                "\u{2630}"
            }
            span { (label) }
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
    // Consistent stroke icons; menu_link supplies their decorative slot.
    let ico = |glyph: &'static str| -> Markup {
        let path = match glyph {
            "□" => "M3 3h7v7H3zM14 3h7v7h-7zM3 14h7v7H3zM14 14h7v7h-7z",
            "≡" => "M3 6h7l2 2h9v12H3z",
            "↑" => "M12 16V3m-5 5 5-5 5 5M4 16v5h16v-5",
            "○" => "M16 7a4 4 0 1 1-8 0 4 4 0 0 1 8 0M4 21v-3a8 8 0 0 1 16 0v3",
            "◇" => "M3 5h18v14H3zM3 10h18M6 15h4",
            _ => "M16 12a4 4 0 1 1-8 0 4 4 0 0 1 8 0M12 2v3m0 14v3M2 12h3m14 0h3M5 5l2 2m10 10 2 2M5 19l2-2M17 7l2-2",
        };
        html! { svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" aria-hidden="true" { path d=(path); } }
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
                            surface: Surface::Card,
                            children: html! {
                                (header(html! {
                                    div class="mui-sidebar__brand" {
                                        span class="mui-sidebar__brand-mark" aria-hidden="true" { "M" }
                                        span class="mui-sidebar__brand-name" { "maud-ui" }
                                    }
                                }))
                                (content(html! {
                                    (collapsible_group("demo-platform", "Platform", menu(html! {
                                        (menu_item(menu_link("/blocks/dashboard-stats", "Dashboard", Some(ico("□")), true)))
                                        (menu_item(html! {
                                            (menu_link("/blocks/worklist-header", "Projects", Some(ico("≡")), false))
                                            (menu_sub(menu_sub_item(menu_link("/blocks/record-header", "Active project", Some(ico("·")), false))))
                                        }))
                                        (menu_item(menu_link("/blocks/task-grid", "Deployments", Some(ico("↑")), false)))
                                    }), true))
                                    (collapsible_group("demo-workspace", "Workspace", menu(html! {
                                        (menu_item(menu_link("/blocks/settings-team", "Members", Some(ico("○")), false)))
                                        (menu_item(menu_link("/blocks/settings-billing", "Billing", Some(ico("◇")), false)))
                                        (menu_item(menu_link("/blocks/settings-profile", "Settings", Some(ico("⚙")), false)))
                                    }), true))
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
                        (html! { section class="mui-sidebar-inset" {
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
                                    kbd { "15rem" } " and " kbd { "4rem" } " via "
                                    kbd { "data-state" } "."
                                }
                            }
                        } })
                    }))
                }
            }
        }
    }
}
