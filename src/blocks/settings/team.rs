//! `settings::team` — team roster with invite row and per-member role +
//! remove controls.
//!
//! Composes: `card`, `button`, `badge`, `input`, `native_select`. No
//! dialogs (the real app wires the Invite button to a dialog or route;
//! this block just renders the table + invite input row).
//!
//! ## Example
//!
//! ```no_run
//! use maud_ui::blocks::settings::team;
//!
//! team::render(team::Props {
//!     invite_action: "/settings/team/invite".into(),
//!     roles: vec!["Owner".into(), "Admin".into(), "Member".into()],
//!     members: vec![
//!         team::Member {
//!             name: "Sofia Davis".into(),
//!             email: "sofia@acme.com".into(),
//!             avatar_initials: "SD".into(),
//!             role: "Owner".into(),
//!             status: team::Status::Active,
//!             last_active: "2 minutes ago".into(),
//!             change_role_action: None, // owner can't be demoted from here
//!             remove_action: None,      // owner can't be removed from here
//!         },
//!     ],
//! });
//! ```

use maud::{html, Markup};

use crate::primitives::{badge, button, card, input, native_select};

/// Props for the team block.
#[derive(Clone, Debug, Default)]
pub struct Props {
    /// POST target for the invite form — fields: `email`, `role`.
    pub invite_action: String,
    /// Available role options shown in the invite dropdown and the
    /// per-member role select.
    pub roles: Vec<String>,
    /// Team members, rendered in order.
    pub members: Vec<Member>,
}

#[derive(Clone, Debug)]
pub struct Member {
    pub name: String,
    pub email: String,
    pub avatar_initials: String,
    /// Current role (must be one of `Props.roles` for the select to
    /// show it as selected).
    pub role: String,
    pub status: Status,
    /// Human time ("2 minutes ago", "Never") — shown in the table.
    pub last_active: String,
    /// POST URL for changing this member's role. `None` disables the
    /// select (e.g. for the owner).
    pub change_role_action: Option<String>,
    /// POST URL to remove this member. `None` hides the remove
    /// button.
    pub remove_action: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Active,
    Pending,
    Suspended,
}

/// Render the team management block.
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-block mui-block--settings mui-block--team" {
            (card::render(card::Props {
                title: Some("Team".into()),
                description: Some(format!("{} {}",
                    props.members.len(),
                    if props.members.len() == 1 { "member" } else { "members" })),
                children: html! {
                    // Invite row
                    form action=(props.invite_action) method="post"
                         class="mui-block--team__invite" {
                        (input::render(input::Props {
                            name: "email".into(),
                            id: "mui-block-team-invite-email".into(),
                            input_type: crate::primitives::input::InputType::Email,
                            placeholder: "teammate@example.com".into(),
                            required: true,
                            ..Default::default()
                        }))
                        (native_select::render(native_select::NativeSelectProps {
                            name: "role".into(),
                            id: "mui-block-team-invite-role".into(),
                            options: props.roles.iter().map(|r| native_select::NativeOption {
                                value: r.clone(),
                                label: r.clone(),
                                disabled: false,
                            }).collect(),
                            selected: props.roles.iter().find(|r| r.as_str() == "Member")
                                .or_else(|| props.roles.first()).cloned(),
                            placeholder: Some("Role".into()),
                            disabled: false,
                        }))
                        (button::render(button::Props {
                            label: "Invite".into(),
                            variant: button::Variant::Primary,
                            size: button::Size::Md,
                            button_type: "submit",
                            ..Default::default()
                        }))
                    }

                    // Members table
                    div class="mui-block--team__table-wrap" {
                        table class="mui-block--team__table" {
                            thead {
                                tr {
                                    th { "Member" }
                                    th { "Role" }
                                    th { "Status" }
                                    th { "Last active" }
                                    th class="mui-block--team__actions-col" aria-label="Actions" {}
                                }
                            }
                            tbody {
                                @for m in &props.members {
                                    tr {
                                        td {
                                            div class="mui-block--team__member" {
                                                span class="mui-block--team__avatar" aria-hidden="true" {
                                                    (m.avatar_initials)
                                                }
                                                div class="mui-block--team__member-text" {
                                                    span class="mui-block--team__member-name" {
                                                        (m.name)
                                                    }
                                                    span class="mui-block--team__member-email" {
                                                        (m.email)
                                                    }
                                                }
                                            }
                                        }
                                        td {
                                            @match &m.change_role_action {
                                                Some(action) => {
                                                    form action=(action) method="post" class="mui-block--team__role-form" {
                                                        (native_select::render(native_select::NativeSelectProps {
                                                            name: "role".into(),
                                                            id: format!("role-{}", m.email),
                                                            options: props.roles.iter().map(|r| native_select::NativeOption {
                                                                value: r.clone(),
                                                                label: r.clone(),
                                                                disabled: false,
                                                            }).collect(),
                                                            selected: Some(m.role.clone()),
                                                            placeholder: None,
                                                            disabled: false,
                                                        }))
                                                    }
                                                }
                                                None => {
                                                    span class="mui-block--team__role-static" { (m.role) }
                                                }
                                            }
                                        }
                                        td {
                                            @match m.status {
                                                Status::Active => (badge::render(badge::Props {
                                                    label: "Active".into(),
                                                    variant: badge::Variant::Success,
                                                })),
                                                Status::Pending => (badge::render(badge::Props {
                                                    label: "Pending".into(),
                                                    variant: badge::Variant::Warning,
                                                })),
                                                Status::Suspended => (badge::render(badge::Props {
                                                    label: "Suspended".into(),
                                                    variant: badge::Variant::Secondary,
                                                })),
                                            }
                                        }
                                        td class="mui-block--team__last-active" {
                                            (m.last_active)
                                        }
                                        td class="mui-block--team__row-actions" {
                                            @if let Some(url) = &m.remove_action {
                                                form action=(url) method="post" {
                                                    (button::render(button::Props {
                                                        label: "Remove".into(),
                                                        variant: button::Variant::Ghost,
                                                        size: button::Size::Sm,
                                                        button_type: "submit",
                                                        ..Default::default()
                                                    }))
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                ..Default::default()
            }))
        }
    }
}

/// Realistic filled-in preview for the showcase.
pub fn preview() -> Markup {
    render(Props {
        invite_action: "/settings/team/invite".into(),
        roles: vec![
            "Owner".into(),
            "Admin".into(),
            "Member".into(),
            "Viewer".into(),
        ],
        members: vec![
            Member {
                name: "Sofia Davis".into(),
                email: "sofia@acme.com".into(),
                avatar_initials: "SD".into(),
                role: "Owner".into(),
                status: Status::Active,
                last_active: "2 min ago".into(),
                change_role_action: None,
                remove_action: None,
            },
            Member {
                name: "Mateo Ortega".into(),
                email: "mateo@acme.com".into(),
                avatar_initials: "MO".into(),
                role: "Admin".into(),
                status: Status::Active,
                last_active: "14 min ago".into(),
                change_role_action: Some("/settings/team/mateo/role".into()),
                remove_action: Some("/settings/team/mateo/remove".into()),
            },
            Member {
                name: "Jin-Ho Lee".into(),
                email: "jin-ho@acme.com".into(),
                avatar_initials: "JL".into(),
                role: "Member".into(),
                status: Status::Active,
                last_active: "Yesterday".into(),
                change_role_action: Some("/settings/team/jin-ho/role".into()),
                remove_action: Some("/settings/team/jin-ho/remove".into()),
            },
            Member {
                name: "Amira Khan".into(),
                email: "amira@acme.com".into(),
                avatar_initials: "AK".into(),
                role: "Member".into(),
                status: Status::Pending,
                last_active: "Never".into(),
                change_role_action: Some("/settings/team/amira/role".into()),
                remove_action: Some("/settings/team/amira/remove".into()),
            },
        ],
    })
}
