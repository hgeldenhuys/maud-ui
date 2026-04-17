//! Calendar component — maud-ui (shadcn/ui-grade)
//!
//! Static HTML render with JS progressive enhancement for month navigation,
//! day selection, and keyboard navigation. Date math uses Tomohiko Sakamoto's
//! algorithm — no external crate required.

use maud::{html, Markup, PreEscaped};

/// Calendar rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// HTML id attribute (also used as hidden input name)
    pub id: String,
    /// Currently selected date as (year, month, day) — None means no selection
    pub selected: Option<(u32, u32, u32)>,
    /// Initial displayed month (1-12)
    pub month: u32,
    /// Initial displayed year
    pub year: u32,
    /// Earliest selectable date
    pub min_date: Option<(u32, u32, u32)>,
    /// Latest selectable date
    pub max_date: Option<(u32, u32, u32)>,
    /// Show days from previous/next month (default true)
    pub show_outside_days: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "calendar".to_string(),
            selected: None,
            month: 1,
            year: 2026,
            min_date: None,
            max_date: None,
            show_outside_days: true,
        }
    }
}

// ── Date math ──────────────────────────────────────────────────────────

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Day of week (0=Sunday, 6=Saturday) using Tomohiko Sakamoto's algorithm.
fn day_of_week(year: u32, month: u32, day: u32) -> u32 {
    let t: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    ((y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + day) % 7) as u32
}

fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "January",
    }
}

/// Format date triple as "YYYY-MM-DD"
fn fmt_date(year: u32, month: u32, day: u32) -> String {
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Compare two date triples: -1, 0, or 1
fn date_cmp(a: (u32, u32, u32), b: (u32, u32, u32)) -> i32 {
    if a.0 != b.0 {
        return if a.0 < b.0 { -1 } else { 1 };
    }
    if a.1 != b.1 {
        return if a.1 < b.1 { -1 } else { 1 };
    }
    if a.2 != b.2 {
        return if a.2 < b.2 { -1 } else { 1 };
    }
    0
}

fn is_disabled(
    year: u32,
    month: u32,
    day: u32,
    min_date: &Option<(u32, u32, u32)>,
    max_date: &Option<(u32, u32, u32)>,
) -> bool {
    let d = (year, month, day);
    if let Some(min) = min_date {
        if date_cmp(d, *min) < 0 {
            return true;
        }
    }
    if let Some(max) = max_date {
        if date_cmp(d, *max) > 0 {
            return true;
        }
    }
    false
}

// ── Cell descriptor for building the grid ──────────────────────────────

struct DayCell {
    year: u32,
    month: u32,
    day: u32,
    is_outside: bool,
    is_today: bool,
    is_selected: bool,
    is_disabled: bool,
}

/// Build a flat Vec of 42 DayCells (6 weeks x 7 days) for the given month view.
fn build_grid(props: &Props) -> Vec<DayCell> {
    let first_dow = day_of_week(props.year, props.month, 1);
    let dim = days_in_month(props.year, props.month);

    // Previous month info
    let (prev_year, prev_month) = if props.month == 1 {
        (props.year - 1, 12)
    } else {
        (props.year, props.month - 1)
    };
    let prev_dim = days_in_month(prev_year, prev_month);

    // Next month info
    let (next_year, next_month) = if props.month == 12 {
        (props.year + 1, 1)
    } else {
        (props.year, props.month + 1)
    };

    // Today: hardcoded to compile-time-neutral — JS will mark actual today
    // We still support a static "today" marker if the rendered month matches
    // Use a sentinel: 0,0,0 means "no today highlight from server"
    // The JS behavior will add the today class dynamically.
    let today = (0u32, 0u32, 0u32);

    let mut cells: Vec<DayCell> = Vec::with_capacity(42);

    // Previous month trailing days
    for i in 0..first_dow {
        let d = prev_dim - first_dow + 1 + i;
        cells.push(DayCell {
            year: prev_year,
            month: prev_month,
            day: d,
            is_outside: true,
            is_today: (prev_year, prev_month, d) == today,
            is_selected: props.selected == Some((prev_year, prev_month, d)),
            is_disabled: is_disabled(prev_year, prev_month, d, &props.min_date, &props.max_date),
        });
    }

    // Current month days
    for d in 1..=dim {
        cells.push(DayCell {
            year: props.year,
            month: props.month,
            day: d,
            is_outside: false,
            is_today: (props.year, props.month, d) == today,
            is_selected: props.selected == Some((props.year, props.month, d)),
            is_disabled: is_disabled(props.year, props.month, d, &props.min_date, &props.max_date),
        });
    }

    // Next month leading days — fill to exactly 42
    let remaining = 42 - cells.len();
    for d in 1..=(remaining as u32) {
        cells.push(DayCell {
            year: next_year,
            month: next_month,
            day: d,
            is_outside: true,
            is_today: (next_year, next_month, d) == today,
            is_selected: props.selected == Some((next_year, next_month, d)),
            is_disabled: is_disabled(next_year, next_month, d, &props.min_date, &props.max_date),
        });
    }

    cells
}

// ── Render ──────────────────────────────────────────────────────────────

pub fn render(props: Props) -> Markup {
    let cells = build_grid(&props);

    let selected_str = match props.selected {
        Some((y, m, d)) => fmt_date(y, m, d),
        None => String::new(),
    };

    let min_str = match props.min_date {
        Some((y, m, d)) => fmt_date(y, m, d),
        None => String::new(),
    };

    let max_str = match props.max_date {
        Some((y, m, d)) => fmt_date(y, m, d),
        None => String::new(),
    };

    let title = format!("{} {}", month_name(props.month), props.year);

    html! {
        div.mui-calendar
            data-mui="calendar"
            id=(props.id)
            data-year=(props.year)
            data-month=(props.month)
            data-selected=(selected_str)
            data-min=(min_str)
            data-max=(max_str)
            data-show-outside=(if props.show_outside_days { "true" } else { "false" })
        {
            // Header: prev / title / next
            div.mui-calendar__header {
                button.mui-calendar__nav.mui-calendar__nav--prev
                    type="button"
                    aria-label="Previous month"
                { (PreEscaped("&#8249;")) }

                div.mui-calendar__title aria-live="polite" { (title) }

                button.mui-calendar__nav.mui-calendar__nav--next
                    type="button"
                    aria-label="Next month"
                { (PreEscaped("&#8250;")) }
            }

            // Grid
            div.mui-calendar__grid role="grid" aria-label="Calendar" {
                // Weekday headers
                div.mui-calendar__weekdays role="row" {
                    div.mui-calendar__weekday role="columnheader" { "Su" }
                    div.mui-calendar__weekday role="columnheader" { "Mo" }
                    div.mui-calendar__weekday role="columnheader" { "Tu" }
                    div.mui-calendar__weekday role="columnheader" { "We" }
                    div.mui-calendar__weekday role="columnheader" { "Th" }
                    div.mui-calendar__weekday role="columnheader" { "Fr" }
                    div.mui-calendar__weekday role="columnheader" { "Sa" }
                }

                // 6 week rows
                @for week in 0..6 {
                    div.mui-calendar__week role="row" {
                        @for dow in 0..7 {
                            @let idx = week * 7 + dow;
                            @let cell = &cells[idx];
                            @let date_str = fmt_date(cell.year, cell.month, cell.day);
                            @let mut cls = String::from("mui-calendar__day");
                            @if cell.is_outside {
                                @let _ = cls.push_str(" mui-calendar__day--outside");
                            }
                            @if cell.is_today {
                                @let _ = cls.push_str(" mui-calendar__day--today");
                            }
                            @if cell.is_selected {
                                @let _ = cls.push_str(" mui-calendar__day--selected");
                            }

                            @let show = !cell.is_outside || props.show_outside_days;
                            @let aria_sel = if cell.is_selected { "true" } else { "false" };
                            @let tab_idx = if cell.is_selected || (!props.selected.is_some() && cell.day == 1 && !cell.is_outside) { "0" } else { "-1" };

                            @if cell.is_disabled {
                                button.{(cls)}
                                    type="button" role="gridcell"
                                    data-date=(date_str)
                                    tabindex=(tab_idx)
                                    aria-selected=(aria_sel)
                                    disabled
                                {
                                    @if show { (cell.day) }
                                }
                            } @else {
                                button.{(cls)}
                                    type="button" role="gridcell"
                                    data-date=(date_str)
                                    tabindex=(tab_idx)
                                    aria-selected=(aria_sel)
                                {
                                    @if show { (cell.day) }
                                }
                            }
                        }
                    }
                }
            }

            // Hidden form value
            input type="hidden" name=(props.id) value=(selected_str)
                class="mui-calendar__value";
        }
    }
}

// ── Showcase ────────────────────────────────────────────────────────────

pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Check-in" }
                p.mui-showcase__caption { "Pick a check-in date — earliest arrival is today (Apr 16)." }
                (render(Props {
                    id: "demo-cal-checkin".into(),
                    year: 2026,
                    month: 4,
                    selected: Some((2026, 4, 20)),
                    min_date: Some((2026, 4, 16)),
                    ..Default::default()
                }))
            }

            section {
                h2 { "Departure" }
                p.mui-showcase__caption { "Departure must be after check-in — min Apr 21, max 60 days out." }
                (render(Props {
                    id: "demo-cal-departure".into(),
                    year: 2026,
                    month: 4,
                    selected: Some((2026, 4, 27)),
                    min_date: Some((2026, 4, 21)),
                    max_date: Some((2026, 6, 20)),
                    ..Default::default()
                }))
            }

            section {
                h2 { "Disabled weekends" }
                p.mui-showcase__caption { "Business-day picking — window locked to the Mon–Fri work week of Apr 20–24." }
                (render(Props {
                    id: "demo-cal-weekdays".into(),
                    year: 2026,
                    month: 4,
                    selected: Some((2026, 4, 22)),
                    min_date: Some((2026, 4, 20)),
                    max_date: Some((2026, 4, 24)),
                    ..Default::default()
                }))
            }
        }
    }
}
