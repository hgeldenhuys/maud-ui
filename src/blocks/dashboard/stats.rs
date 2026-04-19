//! `dashboard::stats` — a section of KPI cards with color-coded deltas,
//! an optional chart slot, and an optional recent-activity feed.
//!
//! Drop this straight into a dashboard route. Pairs naturally with
//! `shell::sidebar` but works standalone inside any container — it
//! inherits the current `--mui-*` theme.
//!
//! Composes: `card` (one per stat + optional feed). No JS.
//!
//! ## Example
//!
//! ```no_run
//! use maud::html;
//! use maud_ui::blocks::dashboard::stats;
//!
//! stats::render(stats::Props {
//!     title: Some("Overview".into()),
//!     subtitle: Some("Last 30 days".into()),
//!     cards: vec![
//!         stats::StatCard {
//!             label: "MRR".into(),
//!             value: "$42,310".into(),
//!             delta: Some(stats::Delta { value: "+12.4%".into(), positive: true }),
//!             hint: Some("vs last month".into()),
//!         },
//!     ],
//!     chart: None,
//!     activity: None,
//! });
//! ```

use maud::{html, Markup};

use crate::primitives::card;

/// Props for the stats block.
#[derive(Clone, Debug, Default)]
pub struct Props {
    /// Section title shown above the cards. `None` hides the header.
    pub title: Option<String>,
    /// Small subtitle under the title (e.g., "Last 30 days"). `None`
    /// if you don't need one.
    pub subtitle: Option<String>,
    /// The KPI cards. Rendered in an auto-fit grid (min 14rem).
    pub cards: Vec<StatCard>,
    /// Optional arbitrary chart markup rendered in a wide card under
    /// the stat row. Pair with `maud_ui::primitives::chart` or your
    /// favorite chart library.
    pub chart: Option<Markup>,
    /// Optional list of recent activity rendered under the chart.
    /// `None` hides the activity card entirely.
    pub activity: Option<Vec<ActivityItem>>,
}

/// A single KPI card.
#[derive(Clone, Debug)]
pub struct StatCard {
    /// Small uppercase label ("MRR", "Active users").
    pub label: String,
    /// Big formatted value ("$42,310", "1,429").
    pub value: String,
    /// Optional colored delta shown under the value.
    pub delta: Option<Delta>,
    /// Small gray text after the delta (e.g. "vs last month").
    pub hint: Option<String>,
}

/// Period-over-period delta, color-coded by direction.
#[derive(Clone, Debug)]
pub struct Delta {
    /// Pre-formatted string ("+12.4%", "-3.2%", "+1,204"). Shown
    /// as-is; no re-parsing.
    pub value: String,
    /// `true` → rendered with `--mui-success-text`.
    /// `false` → rendered with `--mui-danger-text`.
    /// For churn-like metrics where lower is better, set
    /// `positive: true` on a negative number.
    pub positive: bool,
}

/// A single row in the activity feed.
#[derive(Clone, Debug)]
pub struct ActivityItem {
    /// Display name of the actor (e.g. "Sofia Davis").
    pub actor: String,
    /// Past-tense action ("upgraded to the Pro plan").
    pub action: String,
    /// Time label ("2 min ago", "Yesterday").
    pub timestamp: String,
    /// Two-letter initials fallback when no avatar image.
    pub avatar_initials: String,
}

/// Render the stats block.
pub fn render(props: Props) -> Markup {
    html! {
        section class="mui-block mui-block--stats" {
            @if props.title.is_some() || props.subtitle.is_some() {
                header class="mui-block--stats__header" {
                    @if let Some(title) = &props.title {
                        h2 class="mui-block--stats__title" { (title) }
                    }
                    @if let Some(subtitle) = &props.subtitle {
                        p class="mui-block--stats__subtitle" { (subtitle) }
                    }
                }
            }

            div class="mui-block--stats__grid" {
                @for c in &props.cards {
                    (card::render(card::Props {
                        children: html! {
                            div class="mui-block--stats__card" {
                                p class="mui-block--stats__label" { (c.label) }
                                p class="mui-block--stats__value" { (c.value) }
                                @if c.delta.is_some() || c.hint.is_some() {
                                    p class="mui-block--stats__delta-row" {
                                        @if let Some(d) = &c.delta {
                                            span class=(if d.positive {
                                                "mui-block--stats__delta mui-block--stats__delta--up"
                                            } else {
                                                "mui-block--stats__delta mui-block--stats__delta--down"
                                            }) { (d.value) }
                                        }
                                        @if let Some(hint) = &c.hint {
                                            span class="mui-block--stats__hint" { (hint) }
                                        }
                                    }
                                }
                            }
                        },
                        ..Default::default()
                    }))
                }
            }

            @if let Some(chart) = &props.chart {
                div class="mui-block--stats__chart" {
                    (card::render(card::Props {
                        children: html! { (chart) },
                        ..Default::default()
                    }))
                }
            }

            @if let Some(items) = &props.activity {
                div class="mui-block--stats__activity" {
                    (card::render(card::Props {
                        title: Some("Recent activity".into()),
                        children: html! {
                            ul class="mui-block--stats__activity-list" {
                                @for item in items {
                                    li class="mui-block--stats__activity-item" {
                                        span class="mui-block--stats__activity-avatar" aria-hidden="true" {
                                            (item.avatar_initials)
                                        }
                                        span class="mui-block--stats__activity-text" {
                                            span class="mui-block--stats__activity-actor" { (item.actor) }
                                            " " (item.action)
                                        }
                                        span class="mui-block--stats__activity-time" { (item.timestamp) }
                                    }
                                }
                            }
                        },
                        ..Default::default()
                    }))
                }
            }
        }
    }
}

/// Realistic filled-in preview for the showcase.
pub fn preview() -> Markup {
    render(Props {
        title: Some("Overview".into()),
        subtitle: Some("Last 30 days compared to the previous period.".into()),
        cards: vec![
            StatCard {
                label: "MRR".into(),
                value: "$42,310".into(),
                delta: Some(Delta {
                    value: "+12.4%".into(),
                    positive: true,
                }),
                hint: Some("vs last month".into()),
            },
            StatCard {
                label: "New customers".into(),
                value: "284".into(),
                delta: Some(Delta {
                    value: "+8.1%".into(),
                    positive: true,
                }),
                hint: Some("vs last month".into()),
            },
            StatCard {
                label: "Active sessions".into(),
                value: "1,429".into(),
                delta: Some(Delta {
                    value: "-3.2%".into(),
                    positive: false,
                }),
                hint: Some("vs last month".into()),
            },
            StatCard {
                label: "Churn".into(),
                value: "2.1%".into(),
                delta: Some(Delta {
                    value: "-0.4%".into(),
                    positive: true,
                }),
                hint: Some("lower is better".into()),
            },
        ],
        chart: None,
        activity: Some(vec![
            ActivityItem {
                actor: "Sofia Davis".into(),
                action: "upgraded to the Pro plan".into(),
                timestamp: "2 min ago".into(),
                avatar_initials: "SD".into(),
            },
            ActivityItem {
                actor: "Mateo Ortega".into(),
                action: "invited 3 teammates".into(),
                timestamp: "14 min ago".into(),
                avatar_initials: "MO".into(),
            },
            ActivityItem {
                actor: "Jin-Ho Lee".into(),
                action: "published a new workflow".into(),
                timestamp: "37 min ago".into(),
                avatar_initials: "JL".into(),
            },
            ActivityItem {
                actor: "Amira Khan".into(),
                action: "exported the Q1 revenue report".into(),
                timestamp: "1 hour ago".into(),
                avatar_initials: "AK".into(),
            },
        ]),
    })
}
