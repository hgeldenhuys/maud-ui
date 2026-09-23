//! One predictable reading order for every primitive and block API page.
use super::{docs, generated_props};
use maud::{html, Markup, PreEscaped};

fn split_docs(raw: &str) -> (String, String, String) {
    let first = raw.find("<h2>").unwrap_or(raw.len());
    let intro = &raw[..first];
    let intro = intro
        .find("</h1>")
        .map_or(intro, |end| &intro[end + 5..])
        .to_owned();
    let mut reference = String::new();
    let mut accessibility = String::new();
    let mut rest = &raw[first..];
    while let Some(end) = rest.find("</h2>") {
        let title = &rest[4..end];
        let after = &rest[end + 5..];
        let next = after.find("<h2>").unwrap_or(after.len());
        let body = &after[..next];
        if title.to_lowercase().contains("accessibility") {
            accessibility.push_str(body);
        } else if title != "Props" && title != "Properties" {
            reference.push_str(&format!("<h3>{title}</h3>{body}"));
        } else {
            // Preserve contract prose (e.g. admitted values) while the Rust-generated
            // table replaces the hand-maintained Props table on the live page.
            let mut remainder = body;
            while let Some(start) = remainder.find("<table>") {
                reference.push_str(&remainder[..start]);
                let Some(end) = remainder[start..].find("</table>") else {
                    break;
                };
                remainder = &remainder[start + end + 8..];
            }
            reference.push_str(remainder);
        }
        rest = &after[next..];
    }
    (intro, reference, accessibility)
}

pub(crate) fn article(
    name: &str,
    examples: Markup,
    block: bool,
    fallback: Option<Markup>,
) -> Markup {
    let raw = if block {
        docs::block_docs_source(name)
    } else {
        docs::component_docs_source(name)
    }
    .unwrap_or("");
    let (intro, reference, accessibility) = split_docs(&docs::link_docs(raw, block));
    // These demos supply h4 captions; under the gallery's Examples h2 they
    // belong at h3. Keep typography's intentional heading-scale specimens.
    let examples = if !block && matches!(name, "tabs" | "resizable") {
        PreEscaped(examples.into_string().replace("<h4", "<h3").replace("</h4>", "</h3>"))
    } else {
        examples
    };
    let prefix = format!("api-{name}");
    let module = if block {
        format!(
            "maud_ui::blocks::{}",
            if ["action-row", "attention-banner"].contains(&name) {
                name.replace('-', "_")
            } else {
                name.replacen('-', "::", 1).replace('-', "_")
            }
        )
    } else {
        format!("maud_ui::primitives::{name}")
    };
    html! {
        nav class="mui-api-nav" aria-label="On this page" {
            @for (key, label) in [("anatomy", "Anatomy"), ("props", "Props"), ("examples", "Examples"), ("accessibility", "Accessibility")] {
                a href=(format!("#{prefix}-{key}")) { (label) }
            }
        }
        section class="mui-api-section mui-docs" id=(format!("{prefix}-anatomy")) {
            h2 { "Anatomy" }
            @if intro.trim().is_empty() { p { "A composed application section with typed inputs and native HTML output." } }
            @else { (PreEscaped(intro)) }
            p class="mui-api-module" { code { (module) } }
        }
        section class="mui-api-section mui-docs" id=(format!("{prefix}-props")) {
            h2 { "Props" }
            p class="mui-caption" { "Generated from public Rust fields. Defaults are evaluated from the implementation." }
            @if let Some(props) = generated_props::render(name, block) { (props) }
        }
        section class="mui-api-section" id=(format!("{prefix}-examples")) {
            h2 { "Examples" }
            div class="mui-api-preview" { (examples) }
            @if block {
                @if let Some(states) = crate::blocks::state::preview(name) {
                    h3 class="mui-api-subheading" { "Loading, empty, error and disabled" }
                    p class="mui-caption" { "Each example uses the block’s state prop. Empty and error are separate outcomes." }
                    (states)
                }
            }
            @if !reference.trim().is_empty() {
                details class="mui-api-reference" {
                    summary { "Usage and composition" }
                    div class="mui-docs" { (PreEscaped(docs::wrap_tables(&reference))) }
                }
            } @else if let Some(fallback) = fallback { (fallback) }
        }
        section class="mui-api-section mui-docs" id=(format!("{prefix}-accessibility")) {
            h2 { "Accessibility" }
            @if accessibility.trim().is_empty() {
                p { "Supply meaningful labels, choose heading levels for the surrounding page, and preserve native link and form behavior. Keyboard focus follows document order. Motion honors reduced-motion preferences." }
            } @else { (PreEscaped(docs::wrap_tables(&accessibility))) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rustdoc_descriptions_are_formatted_and_empty_cells_have_a_fallback() {
        let form = generated_props::render("form", false).unwrap().into_string();
        assert!(form.contains("<strong>this diverges from HTML</strong>"));
        let popover = generated_props::render("popover", false).unwrap().into_string();
        assert!(form.contains("<code>render</code>"));
        assert!(!form.contains("[`render`]"));
        assert!(popover.contains("<code>placement</code>"));
        for name in ["select", "native_select", "tabs", "combobox", "data_table", "button"] {
            let props = generated_props::render(name, false).unwrap().into_string();
            assert!(!props.contains("data-label=\"Description\"></td>"), "{name}");
            assert!(props.contains("No description"), "{name}");
        }
    }

    #[test]
    fn gallery_promotes_example_captions_without_changing_library_markup() {
        for name in ["tabs", "resizable"] {
            let examples = super::super::component_content(name).unwrap();
            assert!(examples.0.contains("<h4"));
            let page = article(name, examples, false, None).into_string();
            assert!(!page.contains("<h4"), "{name}");
            assert!(page.contains("<h3"), "{name}");
        }
        let examples = super::super::component_content("typography").unwrap();
        assert!(article("typography", examples, false, None).0.contains("<h4"));
    }
}
