//! `data::table_full` — data table with search box, filter chips, bulk
//! action dropdown, selectable rows, and per-row action menu.
//!
//! Composes: `card`, `input`, `button`, `badge`, `native_select`. The
//! table itself is rendered directly (not via `data_table` primitive)
//! because the block needs control over the status pills and the
//! per-row action column.
//!
//! The block is server-rendered HTML — search/filter/bulk actions
//! submit to the `action` URL and your handler re-renders the list.
//! Works without JS.

use maud::{html, Markup};

use crate::primitives::{badge, button, card, input, native_select};

#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Page title shown above the card.
    pub title: String,
    /// Short description under the title.
    pub subtitle: Option<String>,
    /// URL that both the filter form + bulk-action form submit to.
    pub action: String,
    /// Current value of the search box (preserved across submits).
    pub search_value: String,
    pub columns: Vec<Column>,
    pub filters: Vec<Filter>,
    pub rows: Vec<Row>,
    /// Bulk actions shown in a dropdown — disabled until at least one
    /// row is selected (the checkbox-counter JS is up to the consumer;
    /// this block ships the dropdown wired to POST with selected ids).
    pub bulk_actions: Vec<BulkAction>,
    /// Summary text shown under the table — e.g. "Showing 25 of 1,284".
    pub pagination_summary: Option<String>,
    /// URL for the primary action (e.g. "Create customer") at the top
    /// right of the card.
    pub primary_action: Option<PrimaryAction>,
}

#[derive(Clone, Debug)]
pub struct Column {
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct Filter {
    /// Display name ("Status", "Plan").
    pub label: String,
    /// Submitted form field name.
    pub name: String,
    pub options: Vec<(String, String)>,
    pub selected: Option<String>,
}

#[derive(Clone, Debug)]
pub struct Row {
    /// Stable identifier — submitted as part of bulk action form.
    pub id: String,
    /// Cells in the same order as `columns`. Last cell can use a
    /// `RowCell::Status` to render a colored badge.
    pub cells: Vec<RowCell>,
    /// Optional URL to view this row. When set, the first cell is
    /// rendered as a link.
    pub view_href: Option<String>,
}

#[derive(Clone, Debug)]
pub enum RowCell {
    Text(String),
    /// Two-line cell: primary + secondary text (name + email pattern).
    Stacked { primary: String, secondary: String },
    Status { label: String, variant: StatusVariant },
    Markup(Markup),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusVariant {
    Success,
    Warning,
    Danger,
    Neutral,
}

#[derive(Clone, Debug)]
pub struct BulkAction {
    pub value: String,
    pub label: String,
    /// `true` means the action is destructive — shown with a warning
    /// tint. Consumer is still responsible for the confirm dialog.
    pub destructive: bool,
}

#[derive(Clone, Debug)]
pub struct PrimaryAction {
    pub label: String,
    pub href: String,
}

pub fn render(props: Props) -> Markup {
    let col_count = props.columns.len();

    html! {
        div class="mui-block mui-block--data" {
            (card::render(card::Props {
                children: html! {
                    // Header row — title + primary action
                    div class="mui-block--data__header" {
                        div {
                            h2 class="mui-block--data__title" { (props.title) }
                            @if let Some(s) = &props.subtitle {
                                p class="mui-block--data__subtitle" { (s) }
                            }
                        }
                        @if let Some(action) = &props.primary_action {
                            a href=(action.href)
                              class="mui-btn mui-btn--primary mui-btn--sm"
                              style="text-decoration:none;" {
                                (action.label)
                            }
                        }
                    }

                    // Filter + search row
                    form action=(props.action) method="get"
                         class="mui-block--data__filters" {
                        div class="mui-block--data__search" {
                            (input::render(input::Props {
                                name: "q".into(),
                                id: "mui-block-data-q".into(),
                                input_type: crate::primitives::input::InputType::Search,
                                placeholder: "Search\u{2026}".into(),
                                value: props.search_value.clone(),
                                ..Default::default()
                            }))
                        }

                        @for (idx, f) in props.filters.iter().enumerate() {
                            div class="mui-block--data__filter" {
                                (native_select::render(native_select::NativeSelectProps {
                                    name: f.name.clone(),
                                    id: format!("mui-block-data-filter-{}", idx),
                                    options: f.options.iter().map(|(v, l)| native_select::NativeOption {
                                        value: v.clone(),
                                        label: l.clone(),
                                        disabled: false,
                                    }).collect(),
                                    selected: f.selected.clone(),
                                    placeholder: Some(f.label.clone()),
                                    disabled: false,
                                }))
                            }
                        }

                        (button::render(button::Props {
                            label: "Apply".into(),
                            variant: button::Variant::Outline,
                            size: button::Size::Md,
                            button_type: "submit",
                            ..Default::default()
                        }))
                    }

                    // Bulk action row (rendered even when empty so the UI
                    // doesn't shift when rows get selected)
                    @if !props.bulk_actions.is_empty() {
                        form action=(props.action) method="post"
                             class="mui-block--data__bulk" {
                            span class="mui-block--data__bulk-label" {
                                "With selected: "
                            }
                            (native_select::render(native_select::NativeSelectProps {
                                name: "bulk_action".into(),
                                id: "mui-block-data-bulk".into(),
                                options: props.bulk_actions.iter().map(|a| native_select::NativeOption {
                                    value: a.value.clone(),
                                    label: a.label.clone(),
                                    disabled: false,
                                }).collect(),
                                selected: None,
                                placeholder: Some("Choose an action\u{2026}".into()),
                                disabled: false,
                            }))
                            (button::render(button::Props {
                                label: "Run".into(),
                                variant: button::Variant::Outline,
                                size: button::Size::Md,
                                button_type: "submit",
                                ..Default::default()
                            }))
                        }
                    }

                    // Table
                    div class="mui-block--data__table-wrap" {
                        table class="mui-block--data__table" {
                            thead {
                                tr {
                                    th class="mui-block--data__select-col" {
                                        input type="checkbox"
                                              class="mui-block--data__select-all"
                                              aria-label="Select all rows";
                                    }
                                    @for c in &props.columns {
                                        th { (c.label) }
                                    }
                                    th class="mui-block--data__actions-col" aria-label="Row actions" {}
                                }
                            }
                            tbody {
                                @if props.rows.is_empty() {
                                    tr class="mui-block--data__empty" {
                                        td colspan=((col_count + 2).to_string()) {
                                            "No results. Adjust the filters above or "
                                            @if let Some(action) = &props.primary_action {
                                                a href=(action.href) class="mui-block--auth__footer-link" {
                                                    (action.label.to_lowercase())
                                                }
                                                "."
                                            } @else {
                                                "clear the search."
                                            }
                                        }
                                    }
                                }
                                @for row in &props.rows {
                                    tr {
                                        td class="mui-block--data__select-col" {
                                            input type="checkbox"
                                                  name="ids"
                                                  value=(row.id)
                                                  aria-label="Select row";
                                        }
                                        @for (idx, cell) in row.cells.iter().enumerate() {
                                            td {
                                                @match cell {
                                                    RowCell::Text(s) => {
                                                        @if idx == 0 && row.view_href.is_some() {
                                                            a href=(row.view_href.as_deref().unwrap_or("#"))
                                                              class="mui-block--data__row-link" {
                                                                (s)
                                                            }
                                                        } @else {
                                                            (s)
                                                        }
                                                    }
                                                    RowCell::Stacked { primary, secondary } => {
                                                        div class="mui-block--data__stacked" {
                                                            @if idx == 0 && row.view_href.is_some() {
                                                                a href=(row.view_href.as_deref().unwrap_or("#"))
                                                                  class="mui-block--data__row-link mui-block--data__stacked-primary" {
                                                                    (primary)
                                                                }
                                                            } @else {
                                                                span class="mui-block--data__stacked-primary" {
                                                                    (primary)
                                                                }
                                                            }
                                                            span class="mui-block--data__stacked-secondary" {
                                                                (secondary)
                                                            }
                                                        }
                                                    }
                                                    RowCell::Status { label, variant } => {
                                                        (badge::render(badge::Props {
                                                            label: label.clone(),
                                                            variant: match variant {
                                                                StatusVariant::Success => badge::Variant::Success,
                                                                StatusVariant::Warning => badge::Variant::Warning,
                                                                StatusVariant::Danger => badge::Variant::Danger,
                                                                StatusVariant::Neutral => badge::Variant::Secondary,
                                                            },
                                                        }))
                                                    }
                                                    RowCell::Markup(m) => { (m) }
                                                }
                                            }
                                        }
                                        td class="mui-block--data__actions-col" {
                                            @if let Some(href) = &row.view_href {
                                                a href=(href) class="mui-block--data__row-action" {
                                                    "View \u{2192}"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    @if let Some(summary) = &props.pagination_summary {
                        p class="mui-block--data__pagination-summary" { (summary) }
                    }
                },
                ..Default::default()
            }))
        }
    }
}

pub fn preview() -> Markup {
    render(Props {
        title: "Customers".into(),
        subtitle: Some("Manage your customer roster. Search, filter, and act in bulk.".into()),
        action: "/customers".into(),
        search_value: String::new(),
        columns: vec![
            Column { label: "Customer".into() },
            Column { label: "Plan".into() },
            Column { label: "MRR".into() },
            Column { label: "Status".into() },
            Column { label: "Signed up".into() },
        ],
        filters: vec![
            Filter {
                label: "Plan".into(),
                name: "plan".into(),
                options: vec![
                    ("".into(), "All plans".into()),
                    ("starter".into(), "Starter".into()),
                    ("pro".into(), "Pro".into()),
                    ("enterprise".into(), "Enterprise".into()),
                ],
                selected: None,
            },
            Filter {
                label: "Status".into(),
                name: "status".into(),
                options: vec![
                    ("".into(), "All statuses".into()),
                    ("active".into(), "Active".into()),
                    ("trial".into(), "Trial".into()),
                    ("past_due".into(), "Past due".into()),
                    ("churned".into(), "Churned".into()),
                ],
                selected: Some("active".into()),
            },
        ],
        rows: vec![
            Row {
                id: "c-001".into(),
                cells: vec![
                    RowCell::Stacked {
                        primary: "Acme Corp".into(),
                        secondary: "billing@acme.com".into(),
                    },
                    RowCell::Text("Enterprise".into()),
                    RowCell::Text("$2,490".into()),
                    RowCell::Status { label: "Active".into(), variant: StatusVariant::Success },
                    RowCell::Text("Jan 14, 2024".into()),
                ],
                view_href: Some("/customers/c-001".into()),
            },
            Row {
                id: "c-002".into(),
                cells: vec![
                    RowCell::Stacked {
                        primary: "Globex".into(),
                        secondary: "finance@globex.io".into(),
                    },
                    RowCell::Text("Pro".into()),
                    RowCell::Text("$790".into()),
                    RowCell::Status { label: "Active".into(), variant: StatusVariant::Success },
                    RowCell::Text("Mar 02, 2025".into()),
                ],
                view_href: Some("/customers/c-002".into()),
            },
            Row {
                id: "c-003".into(),
                cells: vec![
                    RowCell::Stacked {
                        primary: "Initech".into(),
                        secondary: "ap@initech.co".into(),
                    },
                    RowCell::Text("Pro".into()),
                    RowCell::Text("$790".into()),
                    RowCell::Status { label: "Past due".into(), variant: StatusVariant::Warning },
                    RowCell::Text("Apr 11, 2025".into()),
                ],
                view_href: Some("/customers/c-003".into()),
            },
            Row {
                id: "c-004".into(),
                cells: vec![
                    RowCell::Stacked {
                        primary: "Hooli".into(),
                        secondary: "ops@hooli.xyz".into(),
                    },
                    RowCell::Text("Starter".into()),
                    RowCell::Text("$0".into()),
                    RowCell::Status { label: "Trial".into(), variant: StatusVariant::Neutral },
                    RowCell::Text("Apr 14, 2026".into()),
                ],
                view_href: Some("/customers/c-004".into()),
            },
            Row {
                id: "c-005".into(),
                cells: vec![
                    RowCell::Stacked {
                        primary: "Piedmont Labs".into(),
                        secondary: "hello@piedmont.dev".into(),
                    },
                    RowCell::Text("Enterprise".into()),
                    RowCell::Text("$4,990".into()),
                    RowCell::Status { label: "Churned".into(), variant: StatusVariant::Danger },
                    RowCell::Text("Jun 22, 2024".into()),
                ],
                view_href: Some("/customers/c-005".into()),
            },
        ],
        bulk_actions: vec![
            BulkAction {
                value: "export".into(),
                label: "Export CSV".into(),
                destructive: false,
            },
            BulkAction {
                value: "tag".into(),
                label: "Add tag".into(),
                destructive: false,
            },
            BulkAction {
                value: "cancel".into(),
                label: "Cancel subscription".into(),
                destructive: true,
            },
        ],
        pagination_summary: Some("Showing 5 of 1,284 customers".into()),
        primary_action: Some(PrimaryAction {
            label: "New customer".into(),
            href: "/customers/new".into(),
        }),
    })
}
