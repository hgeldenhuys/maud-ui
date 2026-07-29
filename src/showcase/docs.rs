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
    let html_str = wrap_tables(&html_str);
    Some(html! {
        section class="mui-docs" {
            (PreEscaped(html_str))
        }
    })
}

/// Prepare every rendered markdown table for narrow screens.
///
/// Two transforms, both needed by `docs.css`:
///
/// 1. Wrap the table in `.mui-docs__scroll`, so a table that is still too
///    wide at tablet widths scrolls in its own box instead of widening the
///    document.
/// 2. Stamp each `<td>` with `data-label="<column header>"`. Below 640px the
///    stylesheet drops the header row and restacks each `<tr>` as a labelled
///    card, which needs the header text available per cell.
///
/// The stacking is what makes prop tables readable on a phone. Scrolling a
/// four-column table through a 276px window shows roughly one column at a
/// time with the rest as dead space — the columns pad out to fill the forced
/// width, so most of what you scroll past is empty.
///
/// Scanning the string is sound here: `push_html` emits these tags itself,
/// and a literal `<table>` written in markdown prose or a code fence is
/// escaped to `&lt;table&gt;`, so it can never match.
fn wrap_tables(html_str: &str) -> String {
    if !html_str.contains("<table>") {
        return html_str.to_string();
    }
    let mut out = String::with_capacity(html_str.len() + 512);
    let mut rest = html_str;
    while let Some(start) = rest.find("<table>") {
        out.push_str(&rest[..start]);
        let after = &rest[start..];
        let Some(close) = after.find("</table>") else {
            // Unbalanced markup — emit the remainder untouched rather than
            // dropping it.
            out.push_str(after);
            return out;
        };
        let end = close + "</table>".len();
        out.push_str("<div class=\"mui-docs__scroll\">");
        out.push_str(&label_cells(&after[..end]));
        out.push_str("</div>");
        rest = &after[end..];
    }
    out.push_str(rest);
    out
}

/// Add `data-label="<header>"` to every `<td>` in one table, matching cells to
/// their column by position and resetting at each row. A table with no
/// `<thead>` is returned unchanged.
fn label_cells(table: &str) -> String {
    let headers = header_texts(table);
    if headers.is_empty() {
        return table.to_string();
    }
    let mut out = String::with_capacity(table.len() + headers.len() * 24);
    let mut rest = table;
    let mut col = 0usize;
    loop {
        let next_tr = rest.find("<tr>");
        let next_td = rest.find("<td>");
        let (idx, is_td) = match (next_tr, next_td) {
            (None, None) => break,
            (Some(a), Some(b)) => {
                if a < b {
                    (a, false)
                } else {
                    (b, true)
                }
            }
            (Some(a), None) => (a, false),
            (None, Some(b)) => (b, true),
        };
        out.push_str(&rest[..idx]);
        if is_td {
            match headers.get(col) {
                Some(label) if !label.is_empty() => {
                    out.push_str("<td data-label=\"");
                    out.push_str(label);
                    out.push_str("\">");
                }
                // More cells than headers — leave the extra ones unlabelled.
                _ => out.push_str("<td>"),
            }
            col += 1;
        } else {
            out.push_str("<tr>");
            col = 0;
        }
        rest = &rest[idx + 4..];
    }
    out.push_str(rest);
    out
}

/// Column headers of a table, as attribute-safe text.
fn header_texts(table: &str) -> Vec<String> {
    let Some(head_start) = table.find("<thead>") else {
        return Vec::new();
    };
    let Some(head_end) = table.find("</thead>") else {
        return Vec::new();
    };
    if head_end < head_start {
        return Vec::new();
    }
    let head = &table[head_start..head_end];
    let mut headers = Vec::new();
    let mut rest = head;
    while let Some(open) = rest.find("<th>") {
        let after = &rest[open + "<th>".len()..];
        let Some(close) = after.find("</th>") else {
            break;
        };
        headers.push(escape_attr(&strip_tags(&after[..close])));
        rest = &after[close..];
    }
    headers
}

/// Drop any inline markup (`<code>`, `<em>`) from header text — a label is
/// rendered through `content: attr(...)`, which takes plain text only.
fn strip_tags(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut depth = 0usize;
    for ch in s.chars() {
        match ch {
            '<' => depth += 1,
            '>' => depth = depth.saturating_sub(1),
            c if depth == 0 => out.push(c),
            _ => {}
        }
    }
    out.trim().to_string()
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
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
        "attention_pill" => Some(include_str!("../../docs/components/attention_pill.md")),
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
        "code_block" => Some(include_str!("../../docs/components/code_block.md")),
        "collapsible" => Some(include_str!("../../docs/components/collapsible.md")),
        "combobox" => Some(include_str!("../../docs/components/combobox.md")),
        "command" => Some(include_str!("../../docs/components/command.md")),
        "composer" => Some(include_str!("../../docs/components/composer.md")),
        "context_menu" => Some(include_str!("../../docs/components/context_menu.md")),
        "data_table" => Some(include_str!("../../docs/components/data_table.md")),
        "date_picker" => Some(include_str!("../../docs/components/date_picker.md")),
        "dialog" => Some(include_str!("../../docs/components/dialog.md")),
        "diff" => Some(include_str!("../../docs/components/diff.md")),
        "direction" => Some(include_str!("../../docs/components/direction.md")),
        "drawer" => Some(include_str!("../../docs/components/drawer.md")),
        "empty_state" => Some(include_str!("../../docs/components/empty_state.md")),
        "facts_list" => Some(include_str!("../../docs/components/facts_list.md")),
        "field" => Some(include_str!("../../docs/components/field.md")),
        "fieldset" => Some(include_str!("../../docs/components/fieldset.md")),
        "form" => Some(include_str!("../../docs/components/form.md")),
        "grid" => Some(include_str!("../../docs/components/grid.md")),
        "gutter_section" => Some(include_str!("../../docs/components/gutter_section.md")),
        "hover_card" => Some(include_str!("../../docs/components/hover_card.md")),
        "input" => Some(include_str!("../../docs/components/input.md")),
        "input_group" => Some(include_str!("../../docs/components/input_group.md")),
        "input_otp" => Some(include_str!("../../docs/components/input_otp.md")),
        "item" => Some(include_str!("../../docs/components/item.md")),
        "kbd" => Some(include_str!("../../docs/components/kbd.md")),
        "label" => Some(include_str!("../../docs/components/label.md")),
        "menu" => Some(include_str!("../../docs/components/menu.md")),
        "menubar" => Some(include_str!("../../docs/components/menubar.md")),
        "message" => Some(include_str!("../../docs/components/message.md")),
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
        "segmented_control" => Some(include_str!("../../docs/components/segmented_control.md")),
        "select" => Some(include_str!("../../docs/components/select.md")),
        "separator" => Some(include_str!("../../docs/components/separator.md")),
        "sheet" => Some(include_str!("../../docs/components/sheet.md")),
        "sidebar" => Some(include_str!("../../docs/components/sidebar.md")),
        "skeleton" => Some(include_str!("../../docs/components/skeleton.md")),
        "slider" => Some(include_str!("../../docs/components/slider.md")),
        "sonner" => Some(include_str!("../../docs/components/sonner.md")),
        "spinner" => Some(include_str!("../../docs/components/spinner.md")),
        "stack" => Some(include_str!("../../docs/components/stack.md")),
        "status_dot" => Some(include_str!("../../docs/components/status_dot.md")),
        "streaming_cursor" => Some(include_str!("../../docs/components/streaming_cursor.md")),
        "swatch" => Some(include_str!("../../docs/components/swatch.md")),
        "switch" => Some(include_str!("../../docs/components/switch.md")),
        "table" => Some(include_str!("../../docs/components/table.md")),
        "tabs" => Some(include_str!("../../docs/components/tabs.md")),
        "textarea" => Some(include_str!("../../docs/components/textarea.md")),
        "toast" => Some(include_str!("../../docs/components/toast.md")),
        "toggle" => Some(include_str!("../../docs/components/toggle.md")),
        "toggle_group" => Some(include_str!("../../docs/components/toggle_group.md")),
        "tool_call" => Some(include_str!("../../docs/components/tool_call.md")),
        "tooltip" => Some(include_str!("../../docs/components/tooltip.md")),
        "turn_progress" => Some(include_str!("../../docs/components/turn_progress.md")),
        "typography" => Some(include_str!("../../docs/components/typography.md")),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TABLE: &str = "<table><thead><tr><th>Field</th><th>Type</th></tr></thead>\
<tbody>\n<tr><td>id</td><td>String</td></tr>\n</tbody></table>";

    #[test]
    fn every_body_cell_carries_its_column_header() {
        let out = wrap_tables(TABLE);
        assert!(out.contains(r#"<td data-label="Field">id</td>"#), "got: {out}");
        assert!(out.contains(r#"<td data-label="Type">String</td>"#), "got: {out}");
    }

    #[test]
    fn table_is_wrapped_in_the_scroll_container() {
        let out = wrap_tables(TABLE);
        assert!(out.starts_with(r#"<div class="mui-docs__scroll"><table>"#));
        assert!(out.ends_with("</table></div>"));
    }

    #[test]
    fn labels_restart_on_each_row() {
        let two_rows = "<table><thead><tr><th>A</th><th>B</th></tr></thead><tbody>\
<tr><td>1</td><td>2</td></tr><tr><td>3</td><td>4</td></tr></tbody></table>";
        let out = wrap_tables(two_rows);
        assert!(out.contains(r#"<td data-label="A">3</td>"#), "got: {out}");
        assert!(out.contains(r#"<td data-label="B">4</td>"#), "got: {out}");
    }

    #[test]
    fn header_markup_is_stripped_and_escaped() {
        let t = "<table><thead><tr><th><code>Size</code></th></tr></thead>\
<tbody><tr><td>x</td></tr></tbody></table>";
        assert!(wrap_tables(t).contains(r#"<td data-label="Size">x</td>"#));
    }

    #[test]
    fn multiple_tables_are_each_transformed() {
        let out = wrap_tables(&format!("{TABLE}<p>between</p>{TABLE}"));
        assert_eq!(out.matches("mui-docs__scroll").count(), 2);
        assert_eq!(out.matches(r#"data-label="Field""#).count(), 2);
        assert!(out.contains("<p>between</p>"));
    }

    #[test]
    fn prose_without_a_table_is_untouched() {
        let prose = "<p>A literal &lt;table&gt; in prose stays escaped.</p>";
        assert_eq!(wrap_tables(prose), prose);
    }

    #[test]
    fn a_table_with_no_header_row_is_left_alone() {
        let t = "<table><tbody><tr><td>x</td></tr></tbody></table>";
        let out = wrap_tables(t);
        assert!(!out.contains("data-label"), "got: {out}");
        assert!(out.contains("<td>x</td>"));
    }

    #[test]
    fn unbalanced_markup_is_not_dropped() {
        let broken = "<table><thead><tr><th>A</th></tr></thead><tbody><tr><td>1</td>";
        assert!(wrap_tables(broken).contains("<td>1</td>"));
    }

    #[test]
    fn real_component_docs_get_labelled_cells() {
        let rendered = render_component_docs("dialog").expect("dialog docs exist");
        let html = rendered.into_string();
        assert!(html.contains(r#"data-label="Field""#), "no Field labels emitted");
        assert!(html.contains(r#"data-label="Description""#), "no Description labels");
    }
}
