//! Tabs component — maud-ui Wave 2

use maud::{html, Markup};

#[derive(Clone, Debug)]
pub struct Tab {
    pub id: String,
    pub label: String,
    pub content: Markup,
}

#[derive(Clone, Debug)]
pub struct Props {
    pub tabs: Vec<Tab>,
    pub default_active: usize,
    pub aria_label: String,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            tabs: vec![],
            default_active: 0,
            aria_label: "Tabs".to_string(),
        }
    }
}

pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-tabs" data-mui="tabs" {
            div class="mui-tabs__list" role="tablist" aria-label=(props.aria_label) {
                @for (i, tab) in props.tabs.iter().enumerate() {
                    @let is_active = i == props.default_active;
                    @let tabindex = if is_active { "0" } else { "-1" };
                    button type="button"
                        class="mui-tabs__trigger"
                        role="tab"
                        id=(format!("{}-trigger", tab.id))
                        aria-controls=(format!("{}-panel", tab.id))
                        aria-selected=(if is_active { "true" } else { "false" })
                        tabindex=(tabindex) {
                        (tab.label)
                    }
                }
            }
            @for (i, tab) in props.tabs.iter().enumerate() {
                @let is_active = i == props.default_active;
                div class="mui-tabs__panel"
                    role="tabpanel"
                    id=(format!("{}-panel", tab.id))
                    aria-labelledby=(format!("{}-trigger", tab.id))
                    tabindex="0"
                    hidden[!is_active]
                {
                    (tab.content)
                }
            }
        }
    }
}

pub fn showcase() -> Markup {
    let tabs = vec![
        Tab {
            id: "overview".to_string(),
            label: "Overview".to_string(),
            content: html! { p { "General overview content here." } },
        },
        Tab {
            id: "features".to_string(),
            label: "Features".to_string(),
            content: html! {
                ul {
                    li { "Feature A" }
                    li { "Feature B" }
                    li { "Feature C" }
                }
            },
        },
        Tab {
            id: "pricing".to_string(),
            label: "Pricing".to_string(),
            content: html! { p { "Pricing details." } },
        },
    ];

    let props = Props {
        tabs,
        default_active: 0,
        aria_label: "Content tabs".to_string(),
    };

    render(props)
}
