//! Application shell with one sidebar node, a persistent desktop rail and a native phone drawer.
//! Masthead and footer are optional; the page header carries route context.
use super::page_header;
use crate::primitives::bottom_tab_bar;
use maud::{html, Markup, PreEscaped};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MobileNavigation {
    #[default]
    Drawer,
    /// First four destinations plus More; nested destinations also remain reachable.
    Tabs,
}

#[derive(Clone, Debug)]
pub struct Props {
    /// Explicit presentation state; Ready preserves the ordinary content.
    pub state: crate::blocks::state::State,
    /// Stable, unique ID: navigation, drawer and local preferences are scoped to it.
    pub id: String,
    pub brand: Markup,
    pub header: Option<Markup>,
    pub sidebar_footer: Option<Markup>,
    pub nav_groups: Vec<NavGroup>,
    pub active_path: String,
    pub user: Option<UserBlock>,
    pub mobile_navigation: MobileNavigation,
    pub collapsible: bool,
    pub default_collapsed: bool,
    pub topbar_title: Option<String>,
    pub topbar_actions: Markup,
    /// Overrides the title/actions topbar; the shell supplies its own menu trigger.
    pub page_header: Option<page_header::Props>,
    pub app_header: Markup,
    pub app_footer: Markup,
    /// Use a section for the content when composing inside an existing main landmark.
    pub embedded: bool,
    pub children: Markup,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            state: Default::default(),
            id: "mui-app".into(),
            brand: html! { span class="mui-block--shell__brand-name" { "App" } },
            header: None,
            sidebar_footer: None,
            nav_groups: vec![],
            active_path: String::new(),
            user: None,
            mobile_navigation: MobileNavigation::Drawer,
            collapsible: true,
            default_collapsed: false,
            topbar_title: None,
            topbar_actions: html! {},
            page_header: None,
            app_header: html! {},
            app_footer: html! {},
            embedded: false,
            children: html! {},
        }
    }
}

/// Labeled groups use native details/summary and remember their open state locally.
#[derive(Clone, Debug, Default)]
pub struct NavGroup {
    pub label: Option<String>,
    pub items: Vec<NavItem>,
}
#[derive(Clone, Debug, Default)]
pub struct NavItem {
    pub label: String,
    pub short_label: Option<String>,
    pub href: String,
    /// Decorative SVG in a consistent 16px slot. A letter fallback survives rail collapse.
    pub icon: Option<Markup>,
    pub badge: Option<String>,
    pub children: Vec<NavItem>,
}
#[derive(Clone, Debug)]
pub struct UserBlock {
    pub name: String,
    pub email: String,
    pub avatar_initials: String,
    pub menu_href: String,
}

fn flatten<'a>(items: &'a [NavItem], output: &mut Vec<&'a NavItem>) {
    for item in items {
        output.push(item);
        flatten(&item.children, output);
    }
}
fn nav_items(items: &[NavItem], current: Option<&NavItem>) -> Markup {
    html! {
        ul class="mui-block--shell__nav-list" {
            @for item in items {
                @let active = current.is_some_and(|selected| std::ptr::eq(selected, item));
                li {
                    a href=(item.href) class=(if active { "mui-block--shell__nav-item mui-block--shell__nav-item--active" } else { "mui-block--shell__nav-item" })
                        aria-current=[active.then_some("page")] title=(item.label)
                        aria-label=(if let Some(badge) = &item.badge { format!("{} — {}", item.label, badge) } else { item.label.clone() }) {
                        span class="mui-block--shell__nav-icon" aria-hidden="true" {
                            @if let Some(icon) = &item.icon { (icon) } @else { (item.label.chars().next().unwrap_or('·')) }
                        }
                        span class="mui-block--shell__nav-label" { (item.label) }
                        @if let Some(badge) = &item.badge { span class="mui-block--shell__nav-badge" { (badge) } }
                    }
                    @if !item.children.is_empty() { (nav_items(&item.children, current)) }
                }
            }
        }
    }
}

pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || render_ready(props))
}

fn render_ready(props: Props) -> Markup {
    let nav_id = format!("{}-navigation", props.id);
    let drawer_id = format!("{}-drawer", props.id);
    let mut items = Vec::new();
    for group in &props.nav_groups {
        flatten(&group.items, &mut items);
    }
    let current = items.iter().position(|item| item.href == props.active_path);
    let selected = current.map(|i| items[i]);
    let mobile_tabs = props.mobile_navigation == MobileNavigation::Tabs;
    let trigger = html! {
        a class="mui-block--shell__trigger mui-btn mui-btn--outline mui-btn--sm" href=(format!("#{nav_id}"))
            data-mui="navigation-trigger" aria-controls=(&drawer_id) aria-haspopup="dialog" { "Menu" }
        @if props.collapsible {
            button class="mui-block--shell__collapse mui-btn mui-btn--ghost mui-btn--icon" type="button"
                data-mui="shell-rail" aria-controls=(&nav_id) aria-expanded=((!props.default_collapsed).to_string()) aria-label="Toggle sidebar" title="Toggle sidebar" {
                svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" aria-hidden="true" {
                    rect x="3" y="4" width="18" height="16" rx="2"; path d="M9 4v16";
                }
            }
        }
    };
    html! {
        div class="mui-block mui-block--shell" id=(props.id) data-mui="shell-navigation"
            data-collapsed=(props.default_collapsed.to_string()) data-embedded=(props.embedded.to_string())
            data-mobile-navigation=(if mobile_tabs { "tabs" } else { "drawer" }) {
            (props.app_header)
            aside class="mui-block--shell__sidebar" id=(&nav_id) aria-label="Application navigation" tabindex="-1" {
                div class="mui-block--shell__brand" { (props.brand) }
                @if let Some(header) = props.header { div class="mui-block--shell__header" { (header) } }
                div class="mui-block--shell__mobile-controls" {}
                nav class="mui-block--shell__nav" aria-label="Primary" {
                    @for (index, group) in props.nav_groups.iter().enumerate() {
                        @if let Some(label) = &group.label {
                            details class="mui-block--shell__nav-group" role="group" aria-label=(label) open
                                data-mui="nav-group" data-nav-key=(format!("{}:{}:{}", nav_id, index, label)) {
                                summary class="mui-block--shell__nav-group-label" { (label) }
                                (nav_items(&group.items, selected))
                            }
                        } @else { div class="mui-block--shell__nav-group" { (nav_items(&group.items, selected)) } }
                    }
                }
                @if let Some(footer) = props.sidebar_footer { div class="mui-block--shell__footer" { (footer) } }
                @else if let Some(user) = &props.user {
                    a href=(user.menu_href) class="mui-block--shell__user" title=(user.name) aria-label=(user.name) {
                        span class="mui-block--shell__user-avatar" aria-hidden="true" { (user.avatar_initials) }
                        span class="mui-block--shell__user-text" {
                            span class="mui-block--shell__user-name" { (user.name) }
                            span class="mui-block--shell__user-email" { (user.email) }
                        }
                        span class="mui-block--shell__user-caret" aria-hidden="true" { (icon_chevron_right()) }
                    }
                }
            }
            div class="mui-block--shell__main" {
                @if let Some(mut page) = props.page_header {
                    @let custom_leading = page.leading;
                    @let () = { page.leading = html! { (trigger) (custom_leading) }; };
                    (page_header::render(page))
                } @else {
                    header class="mui-block--shell__topbar" {
                        (trigger)
                        @if let Some(title) = &props.topbar_title { span class="mui-block--shell__topbar-title" { (title) } }
                        div class="mui-block--shell__topbar-actions" { (props.topbar_actions) }
                    }
                }
                @if props.embedded { section class="mui-block--shell__content" aria-label="Workspace content" { (props.children) } }
                @else { main class="mui-block--shell__content" { (props.children) } }
            }
            (props.app_footer)
            dialog class="mui-navigation-dialog" id=(&drawer_id) aria-label="Application navigation" {
                form method="dialog" { button class="mui-btn mui-btn--outline mui-btn--sm" { "Close navigation" } }
            }
            @if mobile_tabs {
                (bottom_tab_bar::render(bottom_tab_bar::Props {
                    items: items.iter().take(4).map(|item| bottom_tab_bar::Item { label: item.label.clone(), short_label: item.short_label.clone(), href: item.href.clone(), icon: item.icon.clone() }).collect(),
                    current_href: Some(props.active_path.clone()),
                    more: Some(bottom_tab_bar::More { label: "More".into(), target_id: drawer_id, fallback_href: format!("#{nav_id}"), current: current.is_some_and(|i| i >= 4) }),
                    position: if props.embedded { bottom_tab_bar::Position::Inline } else { bottom_tab_bar::Position::Fixed },
                    ..Default::default()
                }))
            }
        }
    }
}

/// Same application in both palettes, with the optional masthead and footer.
pub fn preview() -> Markup {
    html! {
        div class="mui-shell-previews" {
            @for theme in ["light", "dark"] {
                section data-theme=(theme) class="mui-shell-preview" {
                    p class="mui-eyebrow" { (theme) " · optional header and footer" }
                    (example(&format!("showcase-shell-{theme}")))
                }
            }
        }
    }
}

/// The landing hero and the shell showcase share a real component composition.
pub(crate) fn example(id: &str) -> Markup {
    use super::{app_footer, app_header};
    use crate::primitives::breadcrumb::BreadcrumbItem;
    render(Props {
        id: id.into(),
        embedded: true,
        app_header: app_header::render(app_header::Props {
            brand: Some(html! { a href="/blocks/shell-sidebar" { "Garden House" } }),
            actions: Some(html! { span class="mui-app-header__note" { "Example workspace" } }),
            ..Default::default()
        }),
        app_footer: app_footer::render(app_footer::Props {
            line: Some("Garden House · Independent hospitality".into()),
            links: vec![crate::blocks::action::Link {
                label: "About this example".into(),
                href: "/blocks/shell-sidebar".into(),
            }],
            ..Default::default()
        }),
        brand: html! { span class="mui-block--shell__brand-mark" aria-hidden="true" { (logo_mark()) } span class="mui-block--shell__brand-name" { "Front desk" } },
        active_path: "/blocks/worklist-header".into(),
        nav_groups: vec![
            NavGroup {
                label: Some("Workspace".into()),
                items: vec![
                    NavItem {
                        label: "Overview".into(),
                        href: "/blocks/dashboard-stats".into(),
                        icon: Some(icon_grid()),
                        ..Default::default()
                    },
                    NavItem {
                        label: "Reservations".into(),
                        short_label: Some("Stays".into()),
                        href: "/blocks/worklist-header".into(),
                        icon: Some(icon_folder()),
                        badge: Some("4".into()),
                        ..Default::default()
                    },
                    NavItem {
                        label: "Guest inbox".into(),
                        href: "/blocks/record-header".into(),
                        icon: Some(icon_inbox()),
                        ..Default::default()
                    },
                ],
            },
            NavGroup {
                label: Some("Manage".into()),
                items: vec![
                    NavItem {
                        label: "Team".into(),
                        href: "/blocks/settings-team".into(),
                        icon: Some(icon_users()),
                        ..Default::default()
                    },
                    NavItem {
                        label: "Settings".into(),
                        href: "/blocks/settings-profile".into(),
                        icon: Some(icon_settings()),
                        children: vec![
                            NavItem {
                                label: "Billing".into(),
                                href: "/blocks/settings-billing".into(),
                                icon: Some(icon_card()),
                                ..Default::default()
                            },
                            NavItem {
                                label: "Integrations".into(),
                                href: "/integrations/mermaid".into(),
                                icon: Some(icon_plug()),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    },
                ],
            },
        ],
        user: Some(UserBlock {
            name: "Sofia Davis".into(),
            email: "Front desk manager".into(),
            avatar_initials: "SD".into(),
            menu_href: "/blocks/settings-profile".into(),
        }),
        page_header: Some(page_header::Props {
            breadcrumbs: vec![
                BreadcrumbItem {
                    label: "Workspace".into(),
                    href: Some("/blocks/shell-sidebar".into()),
                },
                BreadcrumbItem {
                    label: "Reservations".into(),
                    href: None,
                },
            ],
            search_markup: Some(
                html! { button class="mui-page-header__search-trigger" type="button" data-mui="workspace-search" aria-label="Search reservations" { span { "Search…" } kbd aria-hidden="true" { "⌘ K" } } },
            ),
            switchers: html! { label { span class="mui-sr-only" { "Language" } select class="mui-native-select" aria-label="Language" { option value="en" { "EN" } option value="fr" { "FR" } } } },
            ..Default::default()
        }),
        children: super::example::content(id),
        ..Default::default()
    })
}

fn logo_mark() -> Markup {
    PreEscaped(r##"<svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" aria-hidden="true"><path d="M4 18L12 4L20 18H16L12 12L8 18H4Z" fill="currentColor"/></svg>"##.to_string())
}

fn icon_chevron_right() -> Markup {
    PreEscaped(r##"<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg" aria-hidden="true"><path d="m9 18 6-6-6-6"/></svg>"##.to_string())
}

fn icon_grid() -> Markup {
    PreEscaped(r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"><rect x="3" y="3" width="7" height="7"/><rect x="14" y="3" width="7" height="7"/><rect x="14" y="14" width="7" height="7"/><rect x="3" y="14" width="7" height="7"/></svg>"##.to_string())
}

fn icon_folder() -> Markup {
    PreEscaped(r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"><path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/></svg>"##.to_string())
}

fn icon_inbox() -> Markup {
    PreEscaped(r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"><polyline points="22 12 16 12 14 15 10 15 8 12 2 12"/><path d="M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"/></svg>"##.to_string())
}

fn icon_users() -> Markup {
    PreEscaped(r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"><path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"/><circle cx="9" cy="7" r="4"/><path d="M22 21v-2a4 4 0 0 0-3-3.87"/><path d="M16 3.13a4 4 0 0 1 0 7.75"/></svg>"##.to_string())
}

fn icon_card() -> Markup {
    PreEscaped(r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"><rect width="20" height="14" x="2" y="5" rx="2"/><line x1="2" x2="22" y1="10" y2="10"/></svg>"##.to_string())
}

fn icon_plug() -> Markup {
    PreEscaped(r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"><path d="M12 22v-5"/><path d="M9 7V2"/><path d="M15 7V2"/><path d="M6 13V8h12v5a4 4 0 0 1-4 4h-4a4 4 0 0 1-4-4Z"/></svg>"##.to_string())
}

fn icon_settings() -> Markup {
    PreEscaped(r##"<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" xmlns="http://www.w3.org/2000/svg"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2Z"/><circle cx="12" cy="12" r="3"/></svg>"##.to_string())
}
