//! `shell::sidebar` — full app shell: vertical nav on the left, topbar
//! + main content slot on the right. Drop your whole app inside
//! `children`; the shell handles layout, active-state highlighting, and
//! the user footer.
//!
//! Visual reference: Linear, Vercel, Stripe dashboards. Fixed 16rem
//! sidebar with a branded header, grouped nav items (optional section
//! headings), and a user footer at the bottom. Sticky topbar spans the
//! content column with an optional title, custom right-side actions
//! slot, and `data-mui-drawer-trigger` on mobile.
//!
//! ## Example
//!
//! ```no_run
//! use maud::{html, Markup};
//! use maud_ui::blocks::shell::sidebar;
//!
//! fn dashboard_page() -> Markup {
//!     sidebar::render(sidebar::Props {
//!         brand: html! {
//!             span style="font-weight:600;" { "Acme" }
//!         },
//!         active_path: "/dashboard".into(),
//!         nav_groups: vec![
//!             sidebar::NavGroup {
//!                 label: None,
//!                 items: vec![
//!                     sidebar::NavItem {
//!                         label: "Dashboard".into(),
//!                         href: "/dashboard".into(),
//!                         icon: None,
//!                         badge: None,
//!                     },
//!                 ],
//!             },
//!         ],
//!         user: Some(sidebar::UserBlock {
//!             name: "Sofia Davis".into(),
//!             email: "sofia@acme.com".into(),
//!             avatar_initials: "SD".into(),
//!             menu_href: "/settings".into(),
//!         }),
//!         topbar_title: Some("Overview".into()),
//!         topbar_actions: html! {},
//!         children: html! { p { "Your app here." } },
//!         ..Default::default()
//!     })
//! }
//! ```

use maud::{html, Markup, PreEscaped};

/// Props for the sidebar shell.
#[derive(Clone, Debug)]
pub struct Props {
    /// Brand/logo shown at the top of the sidebar. Typically an inline
    /// SVG + product name. No default — consumers always brand their
    /// own shell.
    pub brand: Markup,

    /// Grouped nav items rendered in the sidebar body, in order.
    pub nav_groups: Vec<NavGroup>,

    /// Current page path. A nav item whose `href` matches this value
    /// gets the `--active` styling. Use exact match.
    pub active_path: String,

    /// Optional user footer at the bottom of the sidebar. `None` hides
    /// the footer entirely — useful for marketing shells or when the
    /// user isn't signed in yet.
    pub user: Option<UserBlock>,

    /// Topbar title shown in the main column's header. `None` shows
    /// just the actions row.
    pub topbar_title: Option<String>,

    /// Topbar right-side slot. Typical content: a primary action
    /// button, search input, notifications bell, command-palette
    /// trigger. Empty `html!` = no actions.
    pub topbar_actions: Markup,

    /// The page's main content. This is where your route-specific
    /// markup goes.
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            brand: html! {
                span style="font-weight:600;font-size:1rem;" { "App" }
            },
            nav_groups: Vec::new(),
            active_path: String::new(),
            user: None,
            topbar_title: None,
            topbar_actions: html! {},
            children: html! {},
        }
    }
}

/// A group of nav items under an optional section heading.
#[derive(Clone, Debug)]
pub struct NavGroup {
    /// Optional section label (e.g. "Workspace", "Analytics"). `None`
    /// renders the items directly with no heading.
    pub label: Option<String>,
    pub items: Vec<NavItem>,
}

/// A single nav link. Compare `href` to `Props.active_path` for
/// active-state styling.
#[derive(Clone, Debug)]
pub struct NavItem {
    pub label: String,
    pub href: String,
    /// Optional inline SVG icon rendered on the left. Use
    /// `stroke="currentColor"` so it matches text color and inverts
    /// in the active state.
    pub icon: Option<Markup>,
    /// Optional badge/count rendered on the right (e.g. "12" for
    /// unread notifications, "New" for a feature flag).
    pub badge: Option<String>,
}

/// User information shown in the sidebar footer.
#[derive(Clone, Debug)]
pub struct UserBlock {
    pub name: String,
    pub email: String,
    /// Two-letter fallback shown when there's no avatar image.
    /// Typically first letters of given + family name.
    pub avatar_initials: String,
    /// Click target for the user footer — usually a settings or
    /// account page.
    pub menu_href: String,
}

/// Render the sidebar shell.
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-block mui-block--shell" {
            aside class="mui-block--shell__sidebar" {
                div class="mui-block--shell__brand" {
                    (props.brand)
                }

                nav class="mui-block--shell__nav" aria-label="Primary" {
                    @for group in &props.nav_groups {
                        div class="mui-block--shell__nav-group" {
                            @if let Some(label) = &group.label {
                                div class="mui-block--shell__nav-group-label" {
                                    (label)
                                }
                            }
                            ul class="mui-block--shell__nav-list" {
                                @for item in &group.items {
                                    li {
                                        @let active = item.href == props.active_path;
                                        a href=(item.href)
                                          class=(if active {
                                              "mui-block--shell__nav-item mui-block--shell__nav-item--active"
                                          } else {
                                              "mui-block--shell__nav-item"
                                          })
                                          aria-current=[if active { Some("page") } else { None }] {
                                            @if let Some(icon) = &item.icon {
                                                span class="mui-block--shell__nav-icon" aria-hidden="true" {
                                                    (icon)
                                                }
                                            }
                                            span class="mui-block--shell__nav-label" { (item.label) }
                                            @if let Some(badge) = &item.badge {
                                                span class="mui-block--shell__nav-badge" { (badge) }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                @if let Some(user) = &props.user {
                    a href=(user.menu_href) class="mui-block--shell__user" {
                        span class="mui-block--shell__user-avatar" aria-hidden="true" {
                            (user.avatar_initials)
                        }
                        span class="mui-block--shell__user-text" {
                            span class="mui-block--shell__user-name" { (user.name) }
                            span class="mui-block--shell__user-email" { (user.email) }
                        }
                        span class="mui-block--shell__user-caret" aria-hidden="true" {
                            (icon_chevron_right())
                        }
                    }
                }
            }

            div class="mui-block--shell__main" {
                header class="mui-block--shell__topbar" {
                    @if let Some(title) = &props.topbar_title {
                        h1 class="mui-block--shell__topbar-title" { (title) }
                    } @else {
                        span {}
                    }
                    div class="mui-block--shell__topbar-actions" {
                        (props.topbar_actions)
                    }
                }

                main class="mui-block--shell__content" {
                    (props.children)
                }
            }
        }
    }
}

/// Realistic filled-in preview for the showcase — a small admin
/// dashboard with 3 nav groups, a user footer, and 4 stat cards in
/// the content slot.
pub fn preview() -> Markup {
    use crate::primitives::{button, card};

    let body = html! {
        div style="display:grid;grid-template-columns:repeat(auto-fit,minmax(14rem,1fr));gap:1rem;margin-bottom:1.5rem;" {
            @for (label, value, delta) in &[
                ("MRR",             "$42,310", "+12.4%"),
                ("New customers",   "284",     "+8.1%"),
                ("Active sessions", "1,429",   "-3.2%"),
                ("Churn",           "2.1%",    "-0.4%"),
            ] {
                (card::render(card::Props {
                    children: html! {
                        p style="font-size:0.8125rem;color:var(--mui-text-muted);margin:0;text-transform:uppercase;letter-spacing:0.04em;" {
                            (*label)
                        }
                        p style="font-size:1.5rem;font-weight:600;margin:0.375rem 0 0.25rem;color:var(--mui-text);" {
                            (*value)
                        }
                        p style="font-size:0.8125rem;color:var(--mui-text-muted);margin:0;" {
                            @if delta.starts_with('-') {
                                span style="color:var(--mui-danger-text);font-weight:500;" { (*delta) }
                            } @else {
                                span style="color:var(--mui-success-text);font-weight:500;" { (*delta) }
                            }
                            " vs last month"
                        }
                    },
                    ..Default::default()
                }))
            }
        }

        (card::render(card::Props {
            title: Some("Recent activity".into()),
            children: html! {
                ul style="list-style:none;margin:0;padding:0;display:flex;flex-direction:column;gap:0.5rem;" {
                    @for (who, action, when) in &[
                        ("Sofia Davis", "upgraded to the Pro plan",      "2 min ago"),
                        ("Mateo Ortega", "invited 3 teammates",          "14 min ago"),
                        ("Jin-Ho Lee",   "published a new workflow",     "37 min ago"),
                        ("Amira Khan",   "exported the Q1 revenue report","1 hour ago"),
                    ] {
                        li style="display:flex;align-items:center;gap:0.75rem;padding:0.5rem 0;border-bottom:1px solid var(--mui-border);" {
                            span style="width:1.75rem;height:1.75rem;border-radius:var(--mui-radius-full);background:var(--mui-bg-input);display:inline-flex;align-items:center;justify-content:center;font-size:0.75rem;font-weight:600;color:var(--mui-text-muted);flex-shrink:0;" {
                                (who.chars().next().unwrap_or('?'))
                            }
                            span style="flex:1;font-size:0.875rem;color:var(--mui-text);" {
                                span style="font-weight:500;" { (*who) }
                                " " (*action)
                            }
                            span style="font-size:0.75rem;color:var(--mui-text-subtle);" { (*when) }
                        }
                    }
                }
            },
            ..Default::default()
        }))
    };

    let topbar_actions = html! {
        (button::render(button::Props {
            label: "Invite".into(),
            variant: crate::primitives::button::Variant::Outline,
            size: crate::primitives::button::Size::Sm,
            ..Default::default()
        }))
        (button::render(button::Props {
            label: "New project".into(),
            variant: crate::primitives::button::Variant::Primary,
            size: crate::primitives::button::Size::Sm,
            ..Default::default()
        }))
    };

    render(Props {
        brand: html! {
            span class="mui-block--shell__brand-mark" aria-hidden="true" { (logo_mark()) }
            span class="mui-block--shell__brand-name" { "Acme" }
        },
        active_path: "/dashboard".into(),
        nav_groups: vec![
            NavGroup {
                label: None,
                items: vec![
                    NavItem {
                        label: "Dashboard".into(),
                        href: "/dashboard".into(),
                        icon: Some(icon_grid()),
                        badge: None,
                    },
                    NavItem {
                        label: "Projects".into(),
                        href: "/projects".into(),
                        icon: Some(icon_folder()),
                        badge: None,
                    },
                    NavItem {
                        label: "Inbox".into(),
                        href: "/inbox".into(),
                        icon: Some(icon_inbox()),
                        badge: Some("12".into()),
                    },
                ],
            },
            NavGroup {
                label: Some("Workspace".into()),
                items: vec![
                    NavItem {
                        label: "Team".into(),
                        href: "/team".into(),
                        icon: Some(icon_users()),
                        badge: None,
                    },
                    NavItem {
                        label: "Billing".into(),
                        href: "/billing".into(),
                        icon: Some(icon_card()),
                        badge: None,
                    },
                    NavItem {
                        label: "Integrations".into(),
                        href: "/integrations".into(),
                        icon: Some(icon_plug()),
                        badge: Some("New".into()),
                    },
                ],
            },
            NavGroup {
                label: Some("Account".into()),
                items: vec![NavItem {
                    label: "Settings".into(),
                    href: "/settings".into(),
                    icon: Some(icon_settings()),
                    badge: None,
                }],
            },
        ],
        user: Some(UserBlock {
            name: "Sofia Davis".into(),
            email: "sofia@acme.com".into(),
            avatar_initials: "SD".into(),
            menu_href: "/settings".into(),
        }),
        topbar_title: Some("Overview".into()),
        topbar_actions,
        children: body,
    })
}

// ── Inline SVG icons (shipped in-block so consumers copy-paste the
//    whole module). stroke="currentColor" so they inherit text color.

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
