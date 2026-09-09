use maud::html;
use maud_ui::{assets, primitives::status_chip_group as chips};
use std::collections::BTreeMap;

fn chip(count: Option<u64>) -> chips::Chip {
    chips::Chip {
        label: "Awaiting review".into(),
        href: "/reservations?status=review".into(),
        count,
        tone: chips::Tone::Warning,
    }
}

#[test]
fn unknown_counts_are_label_only_and_empty_collections_emit_no_group() {
    let unknown = chips::render(chips::Props {
        items: vec![chip(None)],
        ..Default::default()
    })
    .0;
    assert!(unknown.contains("Awaiting review") && unknown.contains("aria-current=\"page\""));
    assert!(!unknown.contains("__count") && !unknown.contains("mui-sr-only"));
    assert!(!unknown.contains('—'));
    let known = chips::render(chips::Props {
        items: vec![chip(Some(0)), chip(Some(10482))],
        ..Default::default()
    })
    .0;
    assert!(known.contains("aria-hidden=\"true\">0</span>"));
    assert!(known.contains(" 0 items</span>") && known.contains(" 10482 items</span>"));
    assert!(chips::render(Default::default()).0.is_empty());
}

// Read declarations from the actual shipped bundle, including later overrides.
// This is a render/style contract test, not a claim of browser layout measurement.
fn declarations(selector: &str) -> BTreeMap<String, String> {
    let mut result = BTreeMap::new();
    for part in assets::CSS.split('}') {
        let Some((selectors, body)) = part.rsplit_once('{') else {
            continue;
        };
        let selectors = selectors.rsplit("*/").next().unwrap();
        if selectors.split(',').any(|s| s.trim() == selector) {
            for declaration in body.split(';') {
                if let Some((property, value)) = declaration.split_once(':') {
                    result.insert(property.trim().into(), value.trim().into());
                }
            }
        }
    }
    result
}

fn resolve(value: &str, tokens: &BTreeMap<String, String>) -> String {
    if let Some(name) = value.strip_prefix("var(").and_then(|v| v.strip_suffix(')')) {
        resolve(&tokens[name], tokens)
    } else {
        value.into()
    }
}

fn pixels(value: &str, tokens: &BTreeMap<String, String>) -> f64 {
    let value = resolve(value, tokens);
    value.strip_suffix("rem").unwrap().parse::<f64>().unwrap() * 16.0
}

#[test]
fn emitted_chip_and_badge_classes_resolve_to_compact_border_boxes() {
    let markup = chips::render(chips::Props {
        items: vec![chip(Some(48))],
        ..Default::default()
    })
    .0;
    assert!(markup.contains("class=\"mui-status-chip-group__chip\""));
    assert!(markup.contains("class=\"mui-status-chip-group__count\""));
    let tokens = declarations(":root");
    let style = declarations(".mui-status-chip-group__chip");
    let count = declarations(".mui-status-chip-group__count");
    let badge = declarations(".mui-badge");
    assert_eq!(style["box-sizing"], "border-box");
    let label = pixels(&style["font-size"], &tokens);
    let height = pixels(&style["min-height"], &tokens);
    let padding = pixels(style["padding"].split_whitespace().next().unwrap(), &tokens);
    let line = resolve(&style["line-height"], &tokens)
        .parse::<f64>()
        .unwrap()
        * label;
    let bubble = pixels(&count["height"], &tokens);
    let border_box = height.max(line.max(bubble) + padding * 2.0 + 2.0);
    assert_eq!(border_box, 30.0);
    assert_eq!(label, 13.0);
    assert_eq!(bubble, 18.0);
    assert_eq!(pixels(&count["font-size"], &tokens), 11.0);
    assert_eq!(count["font-variant-numeric"], "tabular-nums");
    assert_eq!(badge["box-sizing"], "border-box");
    assert_eq!(pixels(&badge["height"], &tokens), 22.0);
    assert_eq!(pixels(&badge["font-size"], &tokens), 12.0);
    assert_eq!(declarations(".mui-status-chip-group")["flex-wrap"], "wrap");
}

#[test]
fn landing_has_one_document_theme_and_one_workspace_search_input() {
    let page = maud_ui::showcase::landing_page().0;
    // Includes markup only: theme selectors and documentation are not elements.
    let body = page
        .split("<body")
        .nth(1)
        .unwrap()
        .split("<script")
        .next()
        .unwrap();
    assert!(!body.contains("data-theme="));
    let hero = page
        .split("class=\"lp__hero\"")
        .nth(1)
        .unwrap()
        .split("class=\"lp__claims\"")
        .next()
        .unwrap();
    assert!(!hero.contains("data-theme="));
    assert_eq!(hero.matches("type=\"search\"").count(), 1);
    assert!(
        !hero.contains("Find a destination") && !hero.contains("Reservations and guest arrivals")
    );
    let background = page
        .split(".mui-showcase__header {")
        .skip(1)
        .flat_map(|rule| rule.split('}').next().unwrap().split(';'))
        .map(str::trim)
        .filter(|declaration| declaration.starts_with("background:"))
        .last()
        .unwrap();
    assert_eq!(background, "background: var(--mui-bg-card)");
    assert!(!page.contains("var(--mui-bg) 82%"));
}

#[test]
fn shell_keeps_unique_control_ids_and_full_navigation_titles() {
    use maud_ui::blocks::shell::{page_header, sidebar};
    let markup = sidebar::render(sidebar::Props {
        page_header: Some(page_header::Props {
            search_markup: Some(html! { input id="one-search" type="search"; }),
            switchers: html! { select id="one-language" { option { "EN" } } },
            ..Default::default()
        }),
        nav_groups: vec![sidebar::NavGroup {
            label: None,
            items: vec![sidebar::NavItem {
                label: "Reservations and guest arrivals".into(),
                href: "/reservations".into(),
                badge: Some("48".into()),
                ..Default::default()
            }],
        }],
        ..Default::default()
    })
    .0;
    assert_eq!(markup.matches("id=\"one-search\"").count(), 1);
    assert_eq!(markup.matches("id=\"one-language\"").count(), 1);
    assert!(markup.contains("mui-block--shell__mobile-controls"));
    assert!(markup.contains("title=\"Reservations and guest arrivals\""));
}
