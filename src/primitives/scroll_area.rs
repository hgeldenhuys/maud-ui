//! ScrollArea component — custom-styled scrollbar with auto-hide behavior.
use maud::{html, Markup};

/// ScrollArea rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// CSS value for max-height (e.g., "12rem", "200px")
    pub max_height: String,
    /// Unique identifier for the viewport
    pub id: String,
    /// Content to scroll
    pub children: Markup,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            max_height: "12rem".to_string(),
            id: "scroll-area-default".to_string(),
            children: html! {},
        }
    }
}

/// Render a ScrollArea with custom scrollbar and auto-hide behavior
pub fn render(props: Props) -> Markup {
    html! {
        div class="mui-scroll-area" data-mui="scroll-area" style={"max-height: " (props.max_height)} {
            div class="mui-scroll-area__viewport" id=(props.id) {
                (props.children)
            }
            div class="mui-scroll-area__scrollbar" aria-hidden="true" {
                div class="mui-scroll-area__thumb" {}
            }
        }
    }
}

/// Showcase the ScrollArea component
pub fn showcase() -> Markup {
    let tags: [&str; 18] = [
        "v1.4.0-beta.1", "v1.3.2", "v1.3.1", "v1.3.0",
        "v1.2.5", "v1.2.4", "v1.2.3", "v1.2.2",
        "v1.2.1", "v1.2.0", "v1.1.3", "v1.1.2",
        "v1.1.1", "v1.1.0", "v1.0.2", "v1.0.1",
        "v1.0.0", "v0.9.0-rc.1",
    ];

    let tag_list = html! {
        div style="display:flex;flex-direction:column;" {
            @for tag in tags.iter() {
                div style="padding:0.5rem 0.75rem;font-size:0.8125rem;font-family:var(--mui-font-mono);border-bottom:1px solid var(--mui-border);" {
                    (tag)
                }
            }
        }
    };

    let changelog = html! {
        div style="padding:0.75rem;font-size:0.8125rem;font-family:var(--mui-font-mono);line-height:1.6;white-space:pre;" {
            "commit a1b2c3d\n"
            "Author: Jane Smith\n"
            "Date:   Mon Apr 13 09:14:22 2026 +0000\n\n"
            "    fix: resolve race condition in SSE reconnect\n\n"
            "commit e4f5a6b\n"
            "Author: Alex Chen\n"
            "Date:   Sun Apr 12 17:32:08 2026 +0000\n\n"
            "    feat: add pagination to search results\n\n"
            "commit c7d8e9f\n"
            "Author: Jane Smith\n"
            "Date:   Sat Apr 11 11:05:44 2026 +0000\n\n"
            "    refactor: extract calendar date math\n\n"
            "commit 0a1b2c3\n"
            "Author: Sam Lee\n"
            "Date:   Fri Apr 10 14:22:11 2026 +0000\n\n"
            "    docs: update API reference for v1.3\n\n"
            "commit d4e5f6a\n"
            "Author: Alex Chen\n"
            "Date:   Thu Apr 9 08:47:33 2026 +0000\n\n"
            "    fix: breadcrumb separator a11y\n"
        }
    };

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Release tags" }
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);overflow:hidden;max-width:14rem;" {
                    (render(Props {
                        max_height: "14rem".to_string(),
                        id: "demo-scroll-tags".to_string(),
                        children: tag_list,
                    }))
                }
            }

            div {
                p.mui-showcase__caption { "Commit log" }
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);overflow:hidden;max-width:26rem;" {
                    (render(Props {
                        max_height: "14rem".to_string(),
                        id: "demo-scroll-log".to_string(),
                        children: changelog,
                    }))
                }
            }
        }
    }
}
