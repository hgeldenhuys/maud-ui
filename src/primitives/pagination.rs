//! Pagination component — page navigation with prev/next and numbered pages.
use maud::{html, Markup, PreEscaped};

/// Pagination rendering properties
#[derive(Debug, Clone)]
pub struct Props {
    /// Current active page (1-indexed)
    pub current_page: usize,
    /// Total number of pages
    pub total_pages: usize,
    /// Maximum visible page buttons (default 5)
    pub max_visible: usize,
    /// Optional href pattern. When `Some`, page buttons render as `<a>` tags
    /// with `{page}` in the pattern substituted for the page number.
    /// Prev/Next also become anchors. When `None`, `<button>` is used.
    pub href_pattern: Option<String>,
    /// When true, hide the "Previous"/"Next" text (still announced to SRs
    /// via a visually-hidden span). Only chevron icons remain visible.
    pub icons_only: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            current_page: 1,
            total_pages: 1,
            max_visible: 5,
            href_pattern: None,
            icons_only: false,
        }
    }
}

/// SVG chevron-left (lucide icon, 15x15)
const CHEVRON_LEFT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>"#;

/// SVG chevron-right (lucide icon, 15x15)
const CHEVRON_RIGHT: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="15" height="15" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="m9 18 6-6-6-6"/></svg>"#;

impl Props {
    /// Determine which page numbers to display
    fn visible_pages(&self) -> Vec<usize> {
        if self.total_pages <= self.max_visible {
            // Show all pages
            (1..=self.total_pages).collect()
        } else {
            let mut pages = Vec::new();
            pages.push(1); // Always show first page

            let start = (self.current_page.saturating_sub(1)).max(2);
            let end = (self.current_page + 1).min(self.total_pages - 1);

            if start > 2 {
                pages.push(0); // Marker for ellipsis before
            }

            for p in start..=end {
                if p > 1 && p < self.total_pages {
                    pages.push(p);
                }
            }

            if end < self.total_pages - 1 {
                pages.push(0); // Marker for ellipsis after
            }

            pages.push(self.total_pages); // Always show last page
            pages
        }
    }
}

/// Substitute `{page}` in the href pattern with the given page number.
fn render_href(pattern: &str, page: usize) -> String {
    pattern.replace("{page}", &page.to_string())
}

/// Render pagination controls
pub fn render(props: Props) -> Markup {
    let visible = props.visible_pages();
    let prev_disabled = props.current_page == 1;
    let next_disabled = props.current_page == props.total_pages;
    let prev_page = props.current_page.saturating_sub(1).max(1);
    let next_page = (props.current_page + 1).min(props.total_pages);

    html! {
        nav class="mui-pagination" aria-label="Pagination" {
            @match (&props.href_pattern, prev_disabled) {
                (Some(pattern), false) => {
                    a
                        class="mui-pagination__btn mui-pagination__btn--prev"
                        href=(render_href(pattern, prev_page))
                        aria-label="Go to previous page"
                    {
                        span class="mui-pagination__btn-icon" { (PreEscaped(CHEVRON_LEFT)) }
                        @if props.icons_only {
                            span class="mui-visually-hidden" { "Previous" }
                        } @else {
                            "Previous"
                        }
                    }
                }
                _ => {
                    button
                        class="mui-pagination__btn mui-pagination__btn--prev"
                        aria-label="Go to previous page"
                        disabled[prev_disabled]
                    {
                        span class="mui-pagination__btn-icon" { (PreEscaped(CHEVRON_LEFT)) }
                        @if props.icons_only {
                            span class="mui-visually-hidden" { "Previous" }
                        } @else {
                            "Previous"
                        }
                    }
                }
            }

            div class="mui-pagination__pages" {
                @for page in visible.iter() {
                    @if *page == 0 {
                        // Ellipsis marker
                        span class="mui-pagination__ellipsis" aria-hidden="true" { "..." }
                    } @else if *page == props.current_page {
                        @match &props.href_pattern {
                            Some(pattern) => {
                                a
                                    class="mui-pagination__page"
                                    href=(render_href(pattern, *page))
                                    aria-current="page"
                                {
                                    (page)
                                }
                            }
                            None => {
                                button
                                    class="mui-pagination__page"
                                    aria-current="page"
                                {
                                    (page)
                                }
                            }
                        }
                    } @else {
                        @match &props.href_pattern {
                            Some(pattern) => {
                                a
                                    class="mui-pagination__page"
                                    href=(render_href(pattern, *page))
                                {
                                    (page)
                                }
                            }
                            None => {
                                button class="mui-pagination__page" {
                                    (page)
                                }
                            }
                        }
                    }
                }
            }

            @match (&props.href_pattern, next_disabled) {
                (Some(pattern), false) => {
                    a
                        class="mui-pagination__btn mui-pagination__btn--next"
                        href=(render_href(pattern, next_page))
                        aria-label="Go to next page"
                    {
                        @if props.icons_only {
                            span class="mui-visually-hidden" { "Next" }
                        } @else {
                            "Next"
                        }
                        span class="mui-pagination__btn-icon" { (PreEscaped(CHEVRON_RIGHT)) }
                    }
                }
                _ => {
                    button
                        class="mui-pagination__btn mui-pagination__btn--next"
                        aria-label="Go to next page"
                        disabled[next_disabled]
                    {
                        @if props.icons_only {
                            span class="mui-visually-hidden" { "Next" }
                        } @else {
                            "Next"
                        }
                        span class="mui-pagination__btn-icon" { (PreEscaped(CHEVRON_RIGHT)) }
                    }
                }
            }
        }
    }
}

/// Showcase pagination in various states
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Search results" }
                div style="border:1px solid var(--mui-border);border-radius:var(--mui-radius-lg);padding:1rem;background:var(--mui-bg-card);" {
                    p style="font-size:0.875rem;color:var(--mui-fg-muted);margin-bottom:0.75rem;" {
                        "Showing " strong { "21\u{2013}30" } " of " strong { "97" } " results"
                    }
                    (render(Props {
                        current_page: 3,
                        total_pages: 10,
                        max_visible: 5,
                        ..Props::default()
                    }))
                }
            }

            div {
                p.mui-showcase__caption { "First page" }
                (render(Props {
                    current_page: 1,
                    total_pages: 10,
                    max_visible: 5,
                    ..Props::default()
                }))
            }

            div {
                p.mui-showcase__caption { "Last page" }
                (render(Props {
                    current_page: 10,
                    total_pages: 10,
                    max_visible: 5,
                    ..Props::default()
                }))
            }

            div {
                p.mui-showcase__caption { "Few pages (no ellipsis)" }
                (render(Props {
                    current_page: 2,
                    total_pages: 3,
                    max_visible: 5,
                    ..Props::default()
                }))
            }

            div {
                p.mui-showcase__caption { "With href pattern (anchor links)" }
                (render(Props {
                    current_page: 2,
                    total_pages: 5,
                    max_visible: 5,
                    href_pattern: Some("#page-{page}".to_string()),
                    icons_only: false,
                }))
            }

            div {
                p.mui-showcase__caption { "Icons only (prev/next labels hidden)" }
                (render(Props {
                    current_page: 3,
                    total_pages: 10,
                    max_visible: 5,
                    href_pattern: None,
                    icons_only: true,
                }))
            }
        }
    }
}
