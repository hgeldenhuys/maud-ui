//! One live operational screen is the front door, built from the public blocks.
use maud::{html, Markup, DOCTYPE};
use crate::blocks;
use super::{page_head, page_header, sidebar_nav, showcase_js, COMPONENT_NAMES};

pub fn landing_page() -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" data-theme="light" {
            head {
                (page_head("maud-ui · Thoughtful defaults for Rust interfaces"))
                meta name="description" content="Typed Maud components, composed application blocks, and considered light and dark defaults. Try a live workspace, then build your own.";
                style { (maud::PreEscaped(include_str!("landing.css"))) }
            }
            body class="lp-page" {
                (page_header())
                (sidebar_nav())
                main class="lp mui-gallery__main" {
                    section class="lp__intro" aria-labelledby="lp-title" {
                        div {
                            p class="mui-eyebrow" { "maud-ui / " (env!("CARGO_PKG_VERSION")) }
                            h1 id="lp-title" { "Good defaults. " span { "Room to make them yours." } }
                            p class="lp__lede" { "Components and application blocks for Rust. Built with Maud, for the work people do every day." }
                        }
                        a class="mui-btn mui-btn--outline" href="/getting-started" { "Build your first screen" span aria-hidden="true" { " ↗" } }
                    }
                    section class="lp__hero" aria-label="Live application example" {
                        div class="lp__example-caption" {
                            span { "A workspace, composed." }
                            span { "Explore hospitality and banking, then choose the density that fits your work." }
                        }
                        (super::brand::density_control())
                        div class="lp__workspace-pages" data-mui="workspace-pages" {
                            nav class="lp__workspace-tabs" aria-label="Workspace pages" {
                                a id="lp-worklist-tab" href="#lp-worklist-panel" data-workspace-tab { "Worklist" }
                                a id="lp-record-tab" href="#lp-record-panel" data-workspace-tab { "Record" }
                                a id="lp-bank-tab" href="#lp-bank-panel" data-workspace-tab { "Banking" }
                            }
                            section class="lp__screen" id="lp-worklist-panel" aria-labelledby="lp-worklist-tab" data-workspace-panel data-default-density="comfortable" data-mui-density-scope="comfortable" {
                                (blocks::shell::sidebar::example("landing-workspace"))
                            }
                            section class="lp__screen" id="lp-record-panel" aria-labelledby="lp-record-tab" data-workspace-panel data-default-density="comfortable" data-mui-density-scope="comfortable" {
                                (blocks::shell::record_example::render("landing-record"))
                            }
                            section class="lp__screen" id="lp-bank-panel" aria-labelledby="lp-bank-tab" data-workspace-panel data-default-density="compact" data-mui-density-scope="compact" {
                                (blocks::shell::banking_example::render("landing-bank"))
                            }
                        }
                        noscript { p class="lp__noscript" { "All workspace pages are available below. JavaScript adds tabs, filters, the drawer and the local form." } }
                    }
                    section class="lp__claims" aria-label="Why maud-ui" {
                        article { h2 { "Start with a whole screen." } p { "A shell, a worklist, a record. " (blocks::BLOCK_NAMES.len()) " composed blocks give your first route a clear structure." } a href="/blocks" { "Explore blocks →" } }
                        article { h2 { "Keep the detail consistent." } p { "A shared palette, type scale and spacing system connect " (COMPONENT_NAMES.len()) " primitives across light and dark." } a href="/theme" { "Explore the defaults →" } }
                        article { h2 { "Stay close to the web." } p { "Typed Rust props produce Maud markup. Native links and forms work with your routes; the behavior bundle adds interaction." } a href="/getting-started" { "Read the guide →" } }
                    }
                    section class="lp__library" aria-labelledby="lp-library-title" {
                        div class="lp__library-heading" {
                            div { p class="mui-eyebrow" { "The parts, considered together" } h2 id="lp-library-title" { "A library for the everyday." } }
                            a href="/gallery" { "Open the full gallery →" }
                        }
                        ul class="lp__wall" {
                            @for name in COMPONENT_NAMES {
                                li { a href=(format!("/{name}")) { span { (super::display_name(name)) } span class="lp__wall-arrow" aria-hidden="true" { "↗" } } }
                            }
                        }
                    }
                    (blocks::shell::app_footer::render(blocks::shell::app_footer::Props {
                        line: Some(format!("maud-ui {} · MIT · Built with itself", env!("CARGO_PKG_VERSION"))),
                        links: vec![
                            blocks::action::Link { label: "Rust API".into(), href: "https://docs.rs/maud-ui".into() },
                            blocks::action::Link { label: "Source".into(), href: "https://github.com/hgeldenhuys/maud-ui".into() },
                            blocks::action::Link { label: "Getting started".into(), href: "/getting-started".into() },
                        ], ..Default::default()
                    }))
                }
                script src=(format!("/js/maud-ui.js?v={}", super::JS_VER)) defer {}
                script { (maud::PreEscaped(showcase_js())) }
            }
        }
    }
}
