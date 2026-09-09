//! Two native date fields, one range. Duration uses calendar dates, never timezone arithmetic.
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct DateInput {
    pub name: String,
    pub label: String,
    /// ISO YYYY-MM-DD, as required by a native date input.
    pub value: String,
    pub min: Option<String>,
    pub max: Option<String>,
}
#[derive(Clone, Debug)]
pub struct Props {
    pub id: String,
    pub legend: String,
    pub start: DateInput,
    pub end: DateInput,
    pub required: bool,
    pub disabled: bool,
    pub read_only: bool,
    pub form: Option<String>,
    pub night_label: String,
    pub nights_label: String,
    pub incomplete_label: String,
    pub invalid_label: String,
    pub no_script_hint: String,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            id: "mui-date-range".into(),
            legend: "Stay dates".into(),
            start: DateInput {
                name: "start".into(),
                label: "Arrival".into(),
                ..Default::default()
            },
            end: DateInput {
                name: "end".into(),
                label: "Departure".into(),
                ..Default::default()
            },
            required: false,
            disabled: false,
            read_only: false,
            form: None,
            night_label: "night".into(),
            nights_label: "nights".into(),
            incomplete_label: "Choose both dates".into(),
            invalid_label: "Departure must be after arrival, within the allowed dates.".into(),
            no_script_hint: "Duration reflects the initial dates. Submit the form to update it."
                .into(),
        }
    }
}
fn ordinal(date: &str) -> Option<u32> {
    if date.len() != 10 || date.as_bytes()[4] != b'-' || date.as_bytes()[7] != b'-' {
        return None;
    }
    if !date
        .bytes()
        .enumerate()
        .all(|(i, b)| i == 4 || i == 7 || b.is_ascii_digit())
    {
        return None;
    }
    let year: u32 = date[..4].parse().ok()?;
    let month: usize = date[5..7].parse().ok()?;
    let day: u32 = date[8..].parse().ok()?;
    if year == 0 || !(1..=12).contains(&month) {
        return None;
    }
    let leap = year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400));
    let months = [
        31,
        if leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    if day == 0 || day > months[month - 1] {
        return None;
    }
    let previous = year - 1;
    Some(
        previous * 365 + previous / 4 - previous / 100
            + previous / 400
            + months[..month - 1].iter().sum::<u32>()
            + day,
    )
}
/// Positive nights between valid Gregorian dates (0001–9999). Same-day, reversed or invalid = None.
pub fn nights_between(start: &str, end: &str) -> Option<u32> {
    ordinal(end)?
        .checked_sub(ordinal(start)?)
        .filter(|n| *n > 0)
}
fn within_bounds(input: &DateInput) -> bool {
    let Some(value) = ordinal(&input.value) else {
        return false;
    };
    input
        .min
        .as_deref()
        .and_then(ordinal)
        .is_none_or(|min| value >= min)
        && input
            .max
            .as_deref()
            .and_then(ordinal)
            .is_none_or(|max| value <= max)
}
pub fn render(props: Props) -> Markup {
    let nights = nights_between(&props.start.value, &props.end.value);
    let incomplete = props.start.value.is_empty() || props.end.value.is_empty();
    let invalid = !incomplete
        && (nights.is_none() || !within_bounds(&props.start) || !within_bounds(&props.end));
    let summary = if incomplete {
        props.incomplete_label.clone()
    } else if invalid {
        props.invalid_label.clone()
    } else {
        let n = nights.unwrap();
        format!(
            "{n} {}",
            if n == 1 {
                &props.night_label
            } else {
                &props.nights_label
            }
        )
    };
    let start_id = format!("{}-start", props.id);
    let end_id = format!("{}-end", props.id);
    let summary_id = format!("{}-duration", props.id);
    html! {
        fieldset class="mui-date-range" id=(&props.id) data-mui="date-range" disabled[props.disabled]
            data-night=(props.night_label) data-nights=(props.nights_label) data-incomplete=(props.incomplete_label) data-invalid=(props.invalid_label) {
            legend class="mui-date-range__legend" { (props.legend) }
            div class="mui-date-range__fields" {
                @for (index, (input, id)) in [(&props.start, &start_id), (&props.end, &end_id)].into_iter().enumerate() {
                    @if index == 1 { span class="mui-date-range__separator" aria-hidden="true" { "→" } }
                    label class="mui-date-range__field" for=(id) {
                        span { (&input.label) }
                        input class="mui-input" type="date" id=(id) name=(&input.name) value=(&input.value) min=[input.min.as_deref()] max=[input.max.as_deref()]
                            form=[props.form.as_deref()] data-range-field=(if index == 0 { "start" } else { "end" })
                            required[props.required] disabled[props.disabled] readonly[props.read_only]
                            aria-describedby=(&summary_id) aria-invalid=[invalid.then_some("true")];
                    }
                }
            }
            output class="mui-date-range__duration" id=(summary_id) for=(format!("{start_id} {end_id}")) aria-live="polite" data-range-invalid=(invalid.to_string()) { (summary) }
            noscript { p class="mui-date-range__hint" { (props.no_script_hint) } }
        }
    }
}
pub fn showcase() -> Markup {
    crate::blocks::kit_preview(|theme| {
        html! {
            form method="get" action="/date_range" {
                (render(Props { id: format!("kit-dates-{theme}"), start: DateInput { name: "arrival".into(), label: "Arrival".into(), value: "2026-09-08".into(), ..Default::default() }, end: DateInput { name: "departure".into(), label: "Departure".into(), value: "2026-09-11".into(), ..Default::default() }, required: true, ..Default::default() }))
                (crate::blocks::action_row::render(crate::blocks::action_row::Props { primary: Some(crate::blocks::action::Action::submit("Review dates")), secondary_markup: Some(html! { button type="reset" class="mui-btn mui-btn--outline" { "Reset" } }), ..Default::default() }))
            }
            p class="mui-showcase__caption" { "Native date controls submit both values without JavaScript. Enhancement updates the calendar-night count and validates the interval; the server must validate dates on every submission. This preview keeps the original dates when reloaded." }
        }
    })
}
