//! Native radio cards: keyboard selection and form submission need no JavaScript.
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Choice {
    pub value: String,
    pub title: String,
    pub facts: Vec<String>,
    /// Already formatted, including currency and unit (e.g. CAD $240 / night).
    pub price: Option<String>,
    /// Some(reason) disables the radio and displays the explanation.
    pub disabled_reason: Option<String>,
}
#[derive(Clone, Debug)]
pub struct Props {
    pub id: String,
    pub name: String,
    pub legend: String,
    pub choices: Vec<Choice>,
    pub selected: Option<String>,
    pub required: bool,
    pub disabled: bool,
    /// Explicit form owner when the picker sits outside its form.
    pub form: Option<String>,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            id: "mui-choice-card".into(),
            name: "choice".into(),
            legend: "Choose an option".into(),
            choices: vec![],
            selected: None,
            required: false,
            disabled: false,
            form: None,
        }
    }
}
pub fn render(props: Props) -> Markup {
    let selected = props
        .choices
        .iter()
        .position(|c| Some(&c.value) == props.selected.as_ref() && c.disabled_reason.is_none());
    html! {
        fieldset class="mui-choice-card" id=(&props.id) disabled[props.disabled] {
            legend class="mui-choice-card__legend" { (props.legend) }
            div class="mui-choice-card__options" {
                @for (index, choice) in props.choices.into_iter().enumerate() {
                    @let input_id = format!("{}-{index}", props.id);
                    @let reason_id = format!("{input_id}-reason");
                    label class="mui-choice-card__option" for=(&input_id) {
                        input class="mui-choice-card__input" type="radio" id=(&input_id) name=(&props.name) value=(choice.value)
                            form=[props.form.as_deref()] checked[Some(index) == selected]
                            disabled[props.disabled || choice.disabled_reason.is_some()] required[props.required]
                            aria-describedby=[choice.disabled_reason.as_ref().map(|_| reason_id.as_str())];
                        span class="mui-choice-card__content" {
                            strong class="mui-choice-card__title" { (choice.title) }
                            @for fact in choice.facts { span class="mui-choice-card__fact" { (fact) } }
                            @if let Some(price) = choice.price { span class="mui-choice-card__price" { (price) } }
                            @if let Some(reason) = choice.disabled_reason { span class="mui-choice-card__reason" id=(reason_id) { (reason) } }
                        }
                    }
                }
            }
        }
    }
}
pub fn showcase() -> Markup {
    crate::blocks::kit_preview(|theme| {
        html! {
            form method="get" action="/choice_card" {
                (render(Props { id: format!("kit-rooms-{theme}"), name: "room".into(), legend: "Choose a room".into(), selected: Some("juniper".into()), required: true, choices: vec![
                    Choice { value: "juniper".into(), title: "Juniper 04".into(), facts: vec!["Queen bed · Garden terrace".into(), "Current room · 2 adults".into()], price: Some("CAD $240 / night".into()), ..Default::default() },
                    Choice { value: "birch".into(), title: "Birch 07".into(), facts: vec!["King bed · Forest balcony".into(), "Upper floor · 2 adults".into()], price: Some("CAD $280 / night".into()), ..Default::default() },
                    Choice { value: "cedar".into(), title: "Cedar 12".into(), facts: vec!["Family room · Garden view".into()], disabled_reason: Some("Unavailable · Booked 8–10 September".into()), ..Default::default() },
                ], ..Default::default() }))
                (crate::blocks::action_row::render(crate::blocks::action_row::Props { primary: Some(crate::blocks::action::Action::submit("Review selection")), ..Default::default() }))
            }
            p class="mui-showcase__caption" { "Tab to the selected room; arrows select another available room. Submitting reloads this example with a room parameter. No room is reserved." }
        }
    })
}
