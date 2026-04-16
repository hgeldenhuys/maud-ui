//! DatePicker component — date input with inline calendar dropdown.
//! Self-contained mini calendar (no dependency on calendar.rs which may not be compiled yet).
use maud::{html, Markup};

/// DatePicker rendering properties
#[derive(Clone, Debug)]
pub struct Props {
    /// Unique identifier
    pub id: String,
    /// HTML form name
    pub name: String,
    /// Selected date as (year, month, day) — month is 1-based
    pub selected: Option<(u32, u32, u32)>,
    /// Placeholder text
    pub placeholder: String,
    /// Whether the picker is disabled
    pub disabled: bool,
    /// Minimum selectable date (year, month, day)
    pub min_date: Option<(u32, u32, u32)>,
    /// Maximum selectable date (year, month, day)
    pub max_date: Option<(u32, u32, u32)>,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            id: "date-picker".to_string(),
            name: "date".to_string(),
            selected: None,
            placeholder: "Pick a date".to_string(),
            disabled: false,
            min_date: None,
            max_date: None,
        }
    }
}

/// Format a date tuple as human-readable string (e.g., "April 20, 2026")
fn format_display(year: u32, month: u32, day: u32) -> String {
    let month_name = month_name(month);
    format!("{} {}, {}", month_name, day, year)
}

/// Format a date tuple as ISO string (e.g., "2026-04-20")
fn format_iso(year: u32, month: u32, day: u32) -> String {
    format!("{:04}-{:02}-{:02}", year, month, day)
}

/// Get month name from 1-based month number
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

/// Number of days in a given month/year
fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 => 31,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        3 => 31,
        4 => 30,
        5 => 31,
        6 => 30,
        7 => 31,
        8 => 31,
        9 => 30,
        10 => 31,
        11 => 30,
        12 => 31,
        _ => 30,
    }
}

/// Day of week for a date (0=Sunday, 6=Saturday) using Zeller-like formula
fn day_of_week(year: u32, month: u32, day: u32) -> u32 {
    // Tomohiko Sakamoto's algorithm
    let t: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let y = if month < 3 { year - 1 } else { year };
    let m = month as usize;
    ((y + y / 4 - y / 100 + y / 400 + t[m - 1] + day) % 7) as u32
}

/// Render the inline mini calendar grid for a given month
fn render_calendar_grid(
    year: u32,
    month: u32,
    selected: Option<(u32, u32, u32)>,
    min_date: Option<(u32, u32, u32)>,
    max_date: Option<(u32, u32, u32)>,
) -> Markup {
    let total_days = days_in_month(year, month);
    let first_dow = day_of_week(year, month, 1);
    let day_headers = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

    html! {
        div class="mui-date-picker__calendar" data-year=(year) data-month=(month) {
            // Month/year header with navigation
            div class="mui-date-picker__cal-header" {
                button type="button" class="mui-date-picker__nav-btn" data-action="prev-month"
                    aria-label="Previous month" { "\u{2039}" }
                span class="mui-date-picker__cal-title" {
                    (month_name(month)) " " (year)
                }
                button type="button" class="mui-date-picker__nav-btn" data-action="next-month"
                    aria-label="Next month" { "\u{203a}" }
            }
            // Day-of-week headers
            div class="mui-date-picker__day-headers" {
                @for dh in &day_headers {
                    span class="mui-date-picker__day-header" { (*dh) }
                }
            }
            // Day grid
            div class="mui-date-picker__days" {
                // Empty cells before first day
                @for _ in 0..first_dow {
                    span class="mui-date-picker__day mui-date-picker__day--empty" {}
                }
                // Day cells
                @for d in 1..=total_days {
                    @let is_selected = selected.map_or(false, |(sy, sm, sd)| sy == year && sm == month && sd == d);
                    @let is_disabled = {
                        let before_min = min_date.map_or(false, |(my, mm, md)| {
                            year < my || (year == my && month < mm) || (year == my && month == mm && d < md)
                        });
                        let after_max = max_date.map_or(false, |(xy, xm, xd)| {
                            year > xy || (year == xy && month > xm) || (year == xy && month == xm && d > xd)
                        });
                        before_min || after_max
                    };
                    @let mut cls = String::from("mui-date-picker__day");
                    @if is_selected {
                        @let _ = cls.push_str(" mui-date-picker__day--selected");
                    }
                    @if is_disabled {
                        @let _ = cls.push_str(" mui-date-picker__day--disabled");
                    }
                    button type="button" class=(cls)
                        data-day=(d) data-month=(month) data-year=(year)
                        disabled[is_disabled]
                    {
                        (d)
                    }
                }
            }
        }
    }
}

/// Render the date picker
pub fn render(props: Props) -> Markup {
    let display_text = match props.selected {
        Some((y, m, d)) => format_display(y, m, d),
        None => props.placeholder.clone(),
    };

    let iso_value = match props.selected {
        Some((y, m, d)) => format_iso(y, m, d),
        None => String::new(),
    };

    let has_value = props.selected.is_some();

    // Calendar shows selected month or current April 2026 as fallback
    let (cal_year, cal_month) = match props.selected {
        Some((y, m, _)) => (y, m),
        None => (2026, 4), // Default display month
    };

    html! {
        div class="mui-date-picker" data-mui="date-picker" {
            @if props.disabled {
                button type="button" class="mui-date-picker__trigger mui-input"
                    id=(props.id)
                    aria-expanded="false"
                    aria-haspopup="dialog"
                    disabled
                {
                    span class=(if has_value { "mui-date-picker__value" } else { "mui-date-picker__value mui-date-picker__value--placeholder" }) {
                        (display_text)
                    }
                    span class="mui-date-picker__icon" aria-hidden="true" { "\u{1f4c5}" }
                }
            } @else {
                button type="button" class="mui-date-picker__trigger mui-input"
                    id=(props.id)
                    aria-expanded="false"
                    aria-haspopup="dialog"
                {
                    span class=(if has_value { "mui-date-picker__value" } else { "mui-date-picker__value mui-date-picker__value--placeholder" }) {
                        (display_text)
                    }
                    span class="mui-date-picker__icon" aria-hidden="true" { "\u{1f4c5}" }
                }
            }
            div class="mui-date-picker__dropdown" hidden {
                (render_calendar_grid(cal_year, cal_month, props.selected, props.min_date, props.max_date))
            }
            input type="hidden" name=(props.name) value=(iso_value) class="mui-date-picker__hidden";
        }
    }
}

/// Showcase date picker variants
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "No selection (placeholder)" }
                div.mui-showcase__row {
                    (render(Props {
                        id: "dp-1".to_string(),
                        name: "date-1".to_string(),
                        selected: None,
                        placeholder: "Pick a date".to_string(),
                        disabled: false,
                        min_date: None,
                        max_date: None,
                    }))
                }
            }
            div {
                p.mui-showcase__caption { "With selected date" }
                div.mui-showcase__row {
                    (render(Props {
                        id: "dp-2".to_string(),
                        name: "date-2".to_string(),
                        selected: Some((2026, 4, 20)),
                        placeholder: "Pick a date".to_string(),
                        disabled: false,
                        min_date: None,
                        max_date: None,
                    }))
                }
            }
            div {
                p.mui-showcase__caption { "Disabled" }
                div.mui-showcase__row {
                    (render(Props {
                        id: "dp-3".to_string(),
                        name: "date-3".to_string(),
                        selected: Some((2026, 4, 15)),
                        placeholder: "Pick a date".to_string(),
                        disabled: true,
                        min_date: None,
                        max_date: None,
                    }))
                }
            }
        }
    }
}
