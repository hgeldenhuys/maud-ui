//! Compact related-entity context with one action.
use crate::{
    blocks::action::{Action, Heading},
    primitives::facts_list::{self, Fact},
};
use maud::{html, Markup};

#[derive(Clone, Debug, Default)]
pub struct Props {
    pub title: String,
    pub title_markup: Option<Markup>,
    pub subtitle: Option<String>,
    /// Usually 2–4 facts. Values are escaped by facts_list.
    pub facts: Vec<Fact>,
    /// Override facts with canonical, already-authorized field fragments.
    pub facts_markup: Option<Markup>,
    pub action: Option<Action>,
    pub action_markup: Option<Markup>,
    pub heading: Heading,
}
pub fn render(props: Props) -> Markup {
    html! {
        section class="mui-related-card" {
            @if let Some(title) = props.title_markup { div class="mui-kit-title" { (title) } }
            @else { (props.heading.render(&props.title, "mui-kit-title")) }
            @if let Some(subtitle) = props.subtitle { p class="mui-related-card__subtitle" { (subtitle) } }
            @if let Some(facts) = props.facts_markup { (facts) }
            @else if !props.facts.is_empty() { (facts_list::render(facts_list::Props { facts: props.facts })) }
            @if let Some(action) = props.action_markup { div class="mui-related-card__action" { (action) } }
            @else if let Some(action) = props.action { div class="mui-related-card__action" { (action.render(false)) } }
        }
    }
}
pub fn preview() -> Markup {
    crate::blocks::kit_preview(|_| {
        html! {
            div class="mui-kit-related-examples" {
                (render(Props { title: "Maya Chen".into(), subtitle: Some("Returning guest · 2nd stay".into()), facts: vec![Fact::new("Email", "maya.chen@example.com"), Fact::new("Language", "English · Contact by email"), Fact::new("Travelling with", "Alex Chen")], action: Some(Action::link("View guest record", "/blocks/record-header")), ..Default::default() }))
                (render(Props { title: "Juniper 04".into(), subtitle: Some("Garden room · Ready".into()), facts: vec![Fact::new("Room", "Queen bed · Ground floor"), Fact::new("Capacity", "2 adults"), Fact::new("Checked", "Nora · Today at 07:40")], action: Some(Action::link("Choose a room", "/choice_card")), ..Default::default() }))
            }
        }
    })
}
