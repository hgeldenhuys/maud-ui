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
                    h1 { "maud-ui" }
                    p.mui-text-muted { "Headless accessible components for maud + htmx" }
                }
                main.mui-showcase {
                    section id="button" { h2 { "Button" } (primitives::button::render()) }
                    section id="badge" { h2 { "Badge" } (primitives::badge::render()) }
                    section id="avatar" { h2 { "Avatar" } (primitives::avatar::render()) }
                    section id="separator" { h2 { "Separator" } (primitives::separator::render()) }
                    section id="progress" { h2 { "Progress" } (primitives::progress::render()) }
                    section id="meter" { h2 { "Meter" } (primitives::meter::render()) }
                    section id="input" { h2 { "Input" } (primitives::input::render()) }
                    section id="textarea" { h2 { "Textarea" } (primitives::textarea::render()) }
                    section id="checkbox" { h2 { "Checkbox" } (primitives::checkbox::render()) }
                    section id="radio" { h2 { "Radio" } (primitives::radio::render()) }
                    section id="field" { h2 { "Field" } (primitives::field::render()) }
                    section id="fieldset" { h2 { "Fieldset" } (primitives::fieldset::render()) }
                    section id="number_field" { h2 { "Number Field" } (primitives::number_field::render()) }
                    section id="switch" { h2 { "Switch" } (primitives::switch::render()) }
                    section id="toggle" { h2 { "Toggle" } (primitives::toggle::render()) }
                    section id="toggle_group" { h2 { "Toggle Group" } (primitives::toggle_group::render()) }
                    section id="tabs" { h2 { "Tabs" } (primitives::tabs::render()) }
                    section id="collapsible" { h2 { "Collapsible" } (primitives::collapsible::render()) }
                    section id="accordion" { h2 { "Accordion" } (primitives::accordion::render()) }
                    section id="toast" { h2 { "Toast" } (primitives::toast::render()) }
                    section id="tooltip" { h2 { "Tooltip" } (primitives::tooltip::render()) }
                    section id="dialog" { h2 { "Dialog" } (primitives::dialog::render()) }
                    section id="alert_dialog" { h2 { "Alert Dialog" } (primitives::alert_dialog::render()) }
                    section id="popover" { h2 { "Popover" } (primitives::popover::render()) }
                    section id="select" { h2 { "Select" } (primitives::select::render()) }
                    section id="menu" { h2 { "Menu" } (primitives::menu::render()) }
                    section id="context_menu" { h2 { "Context Menu" } (primitives::context_menu::render()) }
                    section id="slider" { h2 { "Slider" } (primitives::slider::render()) }
                    section id="scroll_area" { h2 { "Scroll Area" } (primitives::scroll_area::render()) }
                }
                script src="/dist/maud-ui.js" defer {}
            }
        }
    }
}
