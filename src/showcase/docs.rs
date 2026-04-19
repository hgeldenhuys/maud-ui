//! Component API-docs rendering — reads `docs/components/<slug>.md` at
//! compile time and renders it as styled HTML alongside each primitive's
//! showcase on the gallery page. The docs markdown ships with the crate
//! via the Cargo.toml `include` list, so external consumers who build the
//! showcase as a library also get the rendered docs.

use maud::{html, Markup, PreEscaped};
use pulldown_cmark::{html::push_html, Options, Parser};

/// Returns the rendered API-docs HTML for a component slug (the
/// `src/primitives/<name>.rs` filename without extension), or `None`
/// if no doc file exists.
///
/// Wrapped in `.mui-docs` so component pages can style the rendered
/// markdown consistently.
pub fn render_component_docs(slug: &str) -> Option<Markup> {
    let raw = component_docs_source(slug)?;
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(raw, opts);
    let mut html_str = String::with_capacity(raw.len() * 2);
    push_html(&mut html_str, parser);
    Some(html! {
        section class="mui-docs" {
            (PreEscaped(html_str))
        }
    })
}

/// Match slug → `include_str!(...)`. Keep the match arms in lockstep
/// with `COMPONENT_NAMES` in `src/showcase/mod.rs` and with
/// `docs/components/*.md` on disk.
fn component_docs_source(slug: &str) -> Option<&'static str> {
    // Path is relative to THIS file (src/showcase/docs.rs):
    // ../../docs/components/<name>.md
    match slug {
        "accordion" => Some(include_str!("../../docs/components/accordion.md")),
        "alert" => Some(include_str!("../../docs/components/alert.md")),
        "alert_dialog" => Some(include_str!("../../docs/components/alert_dialog.md")),
        "aspect_ratio" => Some(include_str!("../../docs/components/aspect_ratio.md")),
        "avatar" => Some(include_str!("../../docs/components/avatar.md")),
        "badge" => Some(include_str!("../../docs/components/badge.md")),
        "breadcrumb" => Some(include_str!("../../docs/components/breadcrumb.md")),
        "button" => Some(include_str!("../../docs/components/button.md")),
        "button_group" => Some(include_str!("../../docs/components/button_group.md")),
        "calendar" => Some(include_str!("../../docs/components/calendar.md")),
        "card" => Some(include_str!("../../docs/components/card.md")),
        "carousel" => Some(include_str!("../../docs/components/carousel.md")),
        "chart" => Some(include_str!("../../docs/components/chart.md")),
        "checkbox" => Some(include_str!("../../docs/components/checkbox.md")),
        "collapsible" => Some(include_str!("../../docs/components/collapsible.md")),
        "combobox" => Some(include_str!("../../docs/components/combobox.md")),
        "command" => Some(include_str!("../../docs/components/command.md")),
        "context_menu" => Some(include_str!("../../docs/components/context_menu.md")),
        "data_table" => Some(include_str!("../../docs/components/data_table.md")),
        "date_picker" => Some(include_str!("../../docs/components/date_picker.md")),
        "dialog" => Some(include_str!("../../docs/components/dialog.md")),
        "direction" => Some(include_str!("../../docs/components/direction.md")),
        "drawer" => Some(include_str!("../../docs/components/drawer.md")),
        "empty_state" => Some(include_str!("../../docs/components/empty_state.md")),
        "field" => Some(include_str!("../../docs/components/field.md")),
        "fieldset" => Some(include_str!("../../docs/components/fieldset.md")),
        "hover_card" => Some(include_str!("../../docs/components/hover_card.md")),
        "input" => Some(include_str!("../../docs/components/input.md")),
        "input_group" => Some(include_str!("../../docs/components/input_group.md")),
        "input_otp" => Some(include_str!("../../docs/components/input_otp.md")),
        "item" => Some(include_str!("../../docs/components/item.md")),
        "kbd" => Some(include_str!("../../docs/components/kbd.md")),
        "label" => Some(include_str!("../../docs/components/label.md")),
        "menu" => Some(include_str!("../../docs/components/menu.md")),
        "menubar" => Some(include_str!("../../docs/components/menubar.md")),
        "meter" => Some(include_str!("../../docs/components/meter.md")),
        "native_select" => Some(include_str!("../../docs/components/native_select.md")),
        "navigation_menu" => Some(include_str!("../../docs/components/navigation_menu.md")),
        "number_field" => Some(include_str!("../../docs/components/number_field.md")),
        "pagination" => Some(include_str!("../../docs/components/pagination.md")),
        "popover" => Some(include_str!("../../docs/components/popover.md")),
        "progress" => Some(include_str!("../../docs/components/progress.md")),
        "radio" => Some(include_str!("../../docs/components/radio.md")),
        "radio_group" => Some(include_str!("../../docs/components/radio_group.md")),
        "resizable" => Some(include_str!("../../docs/components/resizable.md")),
        "scroll_area" => Some(include_str!("../../docs/components/scroll_area.md")),
        "select" => Some(include_str!("../../docs/components/select.md")),
        "separator" => Some(include_str!("../../docs/components/separator.md")),
        "sheet" => Some(include_str!("../../docs/components/sheet.md")),
        "sidebar" => Some(include_str!("../../docs/components/sidebar.md")),
        "skeleton" => Some(include_str!("../../docs/components/skeleton.md")),
        "slider" => Some(include_str!("../../docs/components/slider.md")),
        "sonner" => Some(include_str!("../../docs/components/sonner.md")),
        "spinner" => Some(include_str!("../../docs/components/spinner.md")),
        "swatch" => Some(include_str!("../../docs/components/swatch.md")),
        "switch" => Some(include_str!("../../docs/components/switch.md")),
        "table" => Some(include_str!("../../docs/components/table.md")),
        "tabs" => Some(include_str!("../../docs/components/tabs.md")),
        "textarea" => Some(include_str!("../../docs/components/textarea.md")),
        "toast" => Some(include_str!("../../docs/components/toast.md")),
        "toggle" => Some(include_str!("../../docs/components/toggle.md")),
        "toggle_group" => Some(include_str!("../../docs/components/toggle_group.md")),
        "tooltip" => Some(include_str!("../../docs/components/tooltip.md")),
        "typography" => Some(include_str!("../../docs/components/typography.md")),
        _ => None,
    }
}
