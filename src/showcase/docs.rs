//! Pre-rendered API docs, generated with `cargo run --example build_docs`.
//! Markdown and HTML ship in the crate; no Markdown parser is linked into
//! consumer applications. Tables receive responsive labels at render time.

use maud::{html, Markup, PreEscaped};

/// Returns the rendered API-docs HTML for a component slug (the
/// `src/primitives/<name>.rs` filename without extension), or `None`
/// if no doc file exists.
///
/// Wrapped in `.mui-docs` so component pages can style the rendered
/// markdown consistently.
pub fn render_component_docs(slug: &str) -> Option<Markup> {
    let raw = component_docs_source(slug)?;
    let html_str = wrap_tables(raw);
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
/// These are trusted, pre-rendered repository docs. Code fences are escaped
/// by the development-time Markdown renderer before reaching this function.
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
/// `docs/components/rendered/*.md` on disk.
fn component_docs_source(slug: &str) -> Option<&'static str> {
    // Path is relative to THIS file (src/showcase/docs.rs):
    // ../../docs/components/rendered/<name>.md
    match slug {
        "accordion" => Some(include_str!("../../docs/components/rendered/accordion.html")),
        "alert" => Some(include_str!("../../docs/components/rendered/alert.html")),
        "alert_dialog" => Some(include_str!("../../docs/components/rendered/alert_dialog.html")),
        "aspect_ratio" => Some(include_str!("../../docs/components/rendered/aspect_ratio.html")),
        "attention_pill" => Some(include_str!("../../docs/components/rendered/attention_pill.html")),
        "avatar" => Some(include_str!("../../docs/components/rendered/avatar.html")),
        "badge" => Some(include_str!("../../docs/components/rendered/badge.html")),
        "bottom_tab_bar" => Some(include_str!("../../docs/components/rendered/bottom_tab_bar.html")),
        "breadcrumb" => Some(include_str!("../../docs/components/rendered/breadcrumb.html")),
        "button" => Some(include_str!("../../docs/components/rendered/button.html")),
        "button_group" => Some(include_str!("../../docs/components/rendered/button_group.html")),
        "calendar" => Some(include_str!("../../docs/components/rendered/calendar.html")),
        "card" => Some(include_str!("../../docs/components/rendered/card.html")),
        "carousel" => Some(include_str!("../../docs/components/rendered/carousel.html")),
        "chart" => Some(include_str!("../../docs/components/rendered/chart.html")),
        "checkbox" => Some(include_str!("../../docs/components/rendered/checkbox.html")),
        "code_block" => Some(include_str!("../../docs/components/rendered/code_block.html")),
        "collapsible" => Some(include_str!("../../docs/components/rendered/collapsible.html")),
        "combobox" => Some(include_str!("../../docs/components/rendered/combobox.html")),
        "command" => Some(include_str!("../../docs/components/rendered/command.html")),
        "composer" => Some(include_str!("../../docs/components/rendered/composer.html")),
        "context_menu" => Some(include_str!("../../docs/components/rendered/context_menu.html")),
        "data_table" => Some(include_str!("../../docs/components/rendered/data_table.html")),
        "date_picker" => Some(include_str!("../../docs/components/rendered/date_picker.html")),
        "dialog" => Some(include_str!("../../docs/components/rendered/dialog.html")),
        "diff" => Some(include_str!("../../docs/components/rendered/diff.html")),
        "direction" => Some(include_str!("../../docs/components/rendered/direction.html")),
        "drawer" => Some(include_str!("../../docs/components/rendered/drawer.html")),
        "empty_state" => Some(include_str!("../../docs/components/rendered/empty_state.html")),
        "facts_list" => Some(include_str!("../../docs/components/rendered/facts_list.html")),
        "field" => Some(include_str!("../../docs/components/rendered/field.html")),
        "fieldset" => Some(include_str!("../../docs/components/rendered/fieldset.html")),
        "form" => Some(include_str!("../../docs/components/rendered/form.html")),
        "grid" => Some(include_str!("../../docs/components/rendered/grid.html")),
        "gutter_section" => Some(include_str!("../../docs/components/rendered/gutter_section.html")),
        "hover_card" => Some(include_str!("../../docs/components/rendered/hover_card.html")),
        "input" => Some(include_str!("../../docs/components/rendered/input.html")),
        "input_group" => Some(include_str!("../../docs/components/rendered/input_group.html")),
        "input_otp" => Some(include_str!("../../docs/components/rendered/input_otp.html")),
        "item" => Some(include_str!("../../docs/components/rendered/item.html")),
        "kbd" => Some(include_str!("../../docs/components/rendered/kbd.html")),
        "label" => Some(include_str!("../../docs/components/rendered/label.html")),
        "menu" => Some(include_str!("../../docs/components/rendered/menu.html")),
        "menubar" => Some(include_str!("../../docs/components/rendered/menubar.html")),
        "message" => Some(include_str!("../../docs/components/rendered/message.html")),
        "meter" => Some(include_str!("../../docs/components/rendered/meter.html")),
        "native_select" => Some(include_str!("../../docs/components/rendered/native_select.html")),
        "navigation_menu" => Some(include_str!("../../docs/components/rendered/navigation_menu.html")),
        "number_field" => Some(include_str!("../../docs/components/rendered/number_field.html")),
        "pagination" => Some(include_str!("../../docs/components/rendered/pagination.html")),
        "popover" => Some(include_str!("../../docs/components/rendered/popover.html")),
        "progress" => Some(include_str!("../../docs/components/rendered/progress.html")),
        "radio" => Some(include_str!("../../docs/components/rendered/radio.html")),
        "radio_group" => Some(include_str!("../../docs/components/rendered/radio_group.html")),
        "resizable" => Some(include_str!("../../docs/components/rendered/resizable.html")),
        "scroll_area" => Some(include_str!("../../docs/components/rendered/scroll_area.html")),
        "segmented_control" => Some(include_str!("../../docs/components/rendered/segmented_control.html")),
        "select" => Some(include_str!("../../docs/components/rendered/select.html")),
        "separator" => Some(include_str!("../../docs/components/rendered/separator.html")),
        "sheet" => Some(include_str!("../../docs/components/rendered/sheet.html")),
        "sidebar" => Some(include_str!("../../docs/components/rendered/sidebar.html")),
        "skeleton" => Some(include_str!("../../docs/components/rendered/skeleton.html")),
        "slider" => Some(include_str!("../../docs/components/rendered/slider.html")),
        "sonner" => Some(include_str!("../../docs/components/rendered/sonner.html")),
        "spinner" => Some(include_str!("../../docs/components/rendered/spinner.html")),
        "stack" => Some(include_str!("../../docs/components/rendered/stack.html")),
        "status_chip_group" => Some(include_str!("../../docs/components/rendered/status_chip_group.html")),
        "status_dot" => Some(include_str!("../../docs/components/rendered/status_dot.html")),
        "streaming_cursor" => Some(include_str!("../../docs/components/rendered/streaming_cursor.html")),
        "swatch" => Some(include_str!("../../docs/components/rendered/swatch.html")),
        "switch" => Some(include_str!("../../docs/components/rendered/switch.html")),
        "table" => Some(include_str!("../../docs/components/rendered/table.html")),
        "tabs" => Some(include_str!("../../docs/components/rendered/tabs.html")),
        "textarea" => Some(include_str!("../../docs/components/rendered/textarea.html")),
        "toast" => Some(include_str!("../../docs/components/rendered/toast.html")),
        "toggle" => Some(include_str!("../../docs/components/rendered/toggle.html")),
        "toggle_group" => Some(include_str!("../../docs/components/rendered/toggle_group.html")),
        "tool_call" => Some(include_str!("../../docs/components/rendered/tool_call.html")),
        "tooltip" => Some(include_str!("../../docs/components/rendered/tooltip.html")),
        "turn_progress" => Some(include_str!("../../docs/components/rendered/turn_progress.html")),
        "typography" => Some(include_str!("../../docs/components/rendered/typography.html")),
        _ => None,
    }
}

/// Render the API contract for operational blocks.
pub fn render_block_docs(slug: &str) -> Option<Markup> {
    let raw = match slug {
        "worklist-header" => include_str!("../../docs/blocks/rendered/worklist-header.html"),
        "record-header" => include_str!("../../docs/blocks/rendered/record-header.html"),
        "task-grid" => include_str!("../../docs/blocks/rendered/task-grid.html"),
        "shell-sidebar" => include_str!("../../docs/blocks/rendered/shell-sidebar.html"),
        "shell-page-header" => include_str!("../../docs/blocks/rendered/shell-page-header.html"),
        "shell-app-header" => include_str!("../../docs/blocks/rendered/shell-app-header.html"),
        "shell-app-footer" => include_str!("../../docs/blocks/rendered/shell-app-footer.html"),

        _ => return None,
    };
    Some(html! { section class="mui-docs" { (PreEscaped(wrap_tables(raw))) } })
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
