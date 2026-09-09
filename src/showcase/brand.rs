use maud::{html, Markup};

pub(super) fn stylesheet(key: &str) -> String {
    let brand = crate::brand::BRANDS
        .iter()
        .find(|b| b.key == key)
        .unwrap_or(&crate::brand::BRANDS[0]);
    format!(
        "/* Radius: sm = brand * 0.5; md = brand; lg = min(brand * 1.5, 12px). Controls = min(sm, 8px). */\n:root {{\n{}}}\n",
        crate::brand::TOKENS
            .iter()
            .zip(brand.values)
            .map(|(token, value)| format!("  {}: {};\n", token.name, value))
            .collect::<String>()
    )
}

pub(super) fn controls() -> Markup {
    html! {
        section id="brand" class="mui-brand-controls" data-mui="brand-customizer" aria-labelledby="brand-title" {
            h2 id="brand-title" { "Make it your brand" }
            p class="mui-caption" { "Nine tokens connect identity and density across the whole kit. Choose a starting point, edit it, then download brand.css below." }
            div class="mui-brand-controls__toolbar" {
                label { "Starting brand" select class="mui-native-select" data-brand-select {
                    option value="" { "Custom brand" }
                    @for brand in crate::brand::BRANDS { option value=(brand.key) { (brand.label) } }
                } }
                a class="mui-btn mui-btn--outline" href="/blocks/shell-brand-mark" { "Brand mark API" }
            }
            div class="mui-brand-controls__fields" {
                @for token in crate::brand::TOKENS {
                    label { (token.label)
                        @if token.kind == "density" {
                            select class="mui-native-select" data-brand-token=(token.name) {
                                option value="0" { "Compact" } option value="1" { "Comfortable" } option value="2" { "Spacious" }
                            }
                        } @else {
                            input class="mui-input" type=(if token.kind == "color" { "color" } else { "text" }) data-brand-token=(token.name) value=(token.default) spellcheck="false";
                        }
                    }
                }
            }
            dl class="mui-brand-radius" aria-label="Resolved radius scale" {
                @for (key, label, formula) in [("sm", "Small", "Brand × 0.5"), ("md", "Medium", "Brand"), ("lg", "Large", "Brand × 1.5, at most 12px")] {
                    div {
                        dt { span class="mui-brand-radius__sample" data-brand-radius-sample=(key) aria-hidden="true" {} (label) }
                        dd { output data-brand-radius-value=(key) { (formula) } }
                    }
                }
            }
            p class="mui-caption" { "Cards, tables and banners share Large. Controls use Small, capped at 8px; compact indicators cap at 4px. Density changes content spacing while the shell frame stays still." }
            div class="mui-brand-controls__preview" data-brand-live {
                (crate::blocks::shell::brand_mark::render(crate::blocks::shell::brand_mark::preset("lodge").unwrap()))
                button type="button" class="mui-btn mui-btn--primary" data-brand-example-action { "Try an action" }
            }
            p class="mui-caption" data-brand-status role="status" {}
            noscript { p { "The brand editor needs JavaScript. The three brand examples and generated CSS remain available." } }
        }
    }
}
pub(super) fn density_control() -> Markup {
    html! { label class="mui-density-controls" { span class="mui-caption" { "Density" }
        select class="mui-native-select" data-mui="density-control" aria-label="Interface density" {
            option value="compact" { "Compact" } option value="comfortable" selected { "Comfortable" } option value="spacious" { "Spacious" }
        }
    } }
}
