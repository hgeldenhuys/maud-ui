//! A record's fact groups share one surface; a lone fact stays an inline row.
use crate::blocks::{action::Heading, state::State};
use maud::{html, Markup};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Columns {
    Two,
    Three,
    #[default]
    Four,
}
impl Columns {
    pub fn count(self) -> usize {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
        }
    }
}

/// A formatted, already-authorized value. Never pass a secret to be masked by CSS.
#[derive(Clone, Debug, Default)]
pub struct Fact {
    pub label: String,
    pub value: String,
    pub mono: bool,
    /// `value` must already be redacted. Adds the mono face and a visible masking hint.
    pub masked: bool,
    /// Canonical field fragment. Overrides `value`; an empty fragment omits the fact.
    pub value_markup: Option<Markup>,
}
impl Fact {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            ..Default::default()
        }
    }
    pub fn mono(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            mono: true,
            ..Self::new(label, value)
        }
    }
    pub fn masked(label: impl Into<String>, redacted_value: impl Into<String>) -> Self {
        Self {
            masked: true,
            ..Self::new(label, redacted_value)
        }
    }
    fn admitted(&self) -> bool {
        self.value_markup
            .as_ref()
            .is_none_or(|value| !value.0.trim().is_empty())
    }
}

#[derive(Clone, Debug, Default)]
pub struct Group {
    /// Human-facing section title, e.g. Identity, Contact or Loyalty.
    pub title: String,
    pub facts: Vec<Fact>,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub state: State,
    /// Pass every group together, rather than rendering one card per group.
    pub groups: Vec<Group>,
    /// Maximum columns; narrow containers reduce to two, then one.
    pub columns: Columns,
    /// Heading level for group titles; use H2 below the page's H1.
    pub heading: Heading,
    pub aria_label: String,
    pub masked_hint: String,
}
impl Default for Props {
    fn default() -> Self {
        Self {
            state: State::Ready,
            groups: vec![],
            columns: Columns::Four,
            heading: Heading::H2,
            aria_label: "Record facts".into(),
            masked_hint: "masked".into(),
        }
    }
}

pub fn render(mut props: Props) -> Markup {
    let state = std::mem::take(&mut props.state);
    crate::blocks::state::render(state, || {
        for group in &mut props.groups {
            group.facts.retain(Fact::admitted);
        }
        props.groups.retain(|group| !group.facts.is_empty());
        if props.groups.is_empty() {
            return html! {};
        }
        let inline = props.groups.len() == 1 && props.groups[0].facts.len() == 1;
        html! {
            section class=(if inline { "mui-record-facts mui-record-facts--inline mui-stack" } else { "mui-record-facts mui-stack" })
                aria-label=(props.aria_label) data-columns=(props.columns.count()) {
                @for group in props.groups {
                    div class="mui-record-facts__group" data-single=(group.facts.len() == 1) {
                        @if !group.title.trim().is_empty() { (props.heading.render(group.title.trim(), "mui-record-facts__title")) }
                        dl class="mui-record-facts__grid" {
                            @for fact in group.facts {
                                div class="mui-record-facts__fact" {
                                    dt class="mui-record-facts__label" { (fact.label) }
                                    dd class="mui-record-facts__value" data-mono=(fact.mono || fact.masked) {
                                        @if let Some(value) = fact.value_markup { (value) }
                                        @else { (fact.value) }
                                        @if fact.masked { span class="mui-record-facts__masked" { (props.masked_hint) } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

pub fn preview() -> Markup {
    crate::blocks::kit_preview(|_| {
        html! {
            (render(Props { groups: vec![
                Group { title: "Identity".into(), facts: vec![Fact::mono("Guest ID", "UI-G5"), Fact::new("Full name", "Sofia Patel"), Fact::new("Email", "sofia.patel@example.com"), Fact::new("Nationality", "South African")] },
                Group { title: "Sensitive details".into(), facts: vec![Fact::masked("Phone", "••••••0005"), Fact::masked("ID number", "••••••0005")] },
                Group { title: "Loyalty".into(), facts: vec![Fact::mono("Loyalty points", "0")] },
            ], ..Default::default() }))
            (render(Props { groups: vec![Group { title: "Language".into(), facts: vec![Fact::new("Preferred language", "English")] }], ..Default::default() }))
        }
    })
}
