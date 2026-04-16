//! Live component gallery. Serves `showcase_page()` as the `showcase` axum example.

use maud::{html, Markup, DOCTYPE};

use crate::primitives;

pub fn showcase_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="dark" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { "maud-ui · Component Gallery" }
                link rel="stylesheet" href="/dist/maud-ui.css";
            }
            body {
                header.mui-showcase__header {
                    div style="display:flex;justify-content:space-between;align-items:center;" {
                        div {
                            h1 { "maud-ui" }
                            p.mui-text-muted { "Headless accessible components for maud + htmx" }
                        }
                        button type="button" class="mui-btn mui-btn--outline mui-btn--md" data-mui="theme-toggle" {
                            "Toggle theme"
                        }
                    }
                }
                main.mui-showcase {
                    section id="button" { h2 { "Button" } (primitives::button::showcase()) }
                    section id="badge" { h2 { "Badge" } (primitives::badge::showcase()) }
                    section id="avatar" { h2 { "Avatar" } (primitives::avatar::showcase()) }
                    section id="separator" { h2 { "Separator" } (primitives::separator::showcase()) }
                    section id="progress" { h2 { "Progress" } (primitives::progress::showcase()) }
                    section id="meter" { h2 { "Meter" } (primitives::meter::showcase()) }
                    section id="input" { h2 { "Input" } (primitives::input::showcase()) }
                    section id="textarea" { h2 { "Textarea" } (primitives::textarea::showcase()) }
                    section id="checkbox" { h2 { "Checkbox" } (primitives::checkbox::showcase()) }
                    section id="radio" { h2 { "Radio" } (primitives::radio::showcase()) }
                    section id="field" { h2 { "Field" } (primitives::field::showcase()) }
                    section id="fieldset" { h2 { "Fieldset" } (primitives::fieldset::showcase()) }
                    section id="number_field" { h2 { "Number Field" } (primitives::number_field::showcase()) }
                    section id="switch" { h2 { "Switch" } (primitives::switch::showcase()) }
                    section id="toggle" { h2 { "Toggle" } (primitives::toggle::showcase()) }
                    section id="toggle_group" { h2 { "Toggle Group" } (primitives::toggle_group::showcase()) }
                    section id="tabs" { h2 { "Tabs" } (primitives::tabs::showcase()) }
                    section id="collapsible" { h2 { "Collapsible" } (primitives::collapsible::showcase()) }
                    section id="accordion" { h2 { "Accordion" } (primitives::accordion::showcase()) }
                    section id="toast" { h2 { "Toast" } (primitives::toast::showcase()) }
                    section id="tooltip" { h2 { "Tooltip" } (primitives::tooltip::showcase()) }
                    section id="dialog" { h2 { "Dialog" } (primitives::dialog::showcase()) }
                    section id="alert_dialog" { h2 { "Alert Dialog" } (primitives::alert_dialog::showcase()) }
                    section id="popover" { h2 { "Popover" } (primitives::popover::showcase()) }
                    section id="select" { h2 { "Select" } (primitives::select::showcase()) }
                    section id="menu" { h2 { "Menu" } (primitives::menu::showcase()) }
                    section id="context_menu" { h2 { "Context Menu" } (primitives::context_menu::showcase()) }
                    section id="slider" { h2 { "Slider" } (primitives::slider::showcase()) }
                    section id="scroll_area" { h2 { "Scroll Area" } (primitives::scroll_area::showcase()) }
                }
                script src="/dist/maud-ui.js" defer {}
            }
        }
    }
}
