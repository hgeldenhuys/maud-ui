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
}

impl Default for Props {
    fn default() -> Self {
        Self {
            current_page: 1,
            total_pages: 1,
            max_visible: 5,
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

/// Render pagination controls
pub fn render(props: Props) -> Markup {
    let visible = props.visible_pages();

    html! {
        nav class="mui-pagination" aria-label="Pagination" {
            button
                class="mui-pagination__btn mui-pagination__btn--prev"
                disabled[props.current_page == 1]
            {
                span class="mui-pagination__btn-icon" { (PreEscaped(CHEVRON_LEFT)) }
                "Previous"
            }

            div class="mui-pagination__pages" {
                @for page in visible.iter() {
                    @if *page == 0 {
                        // Ellipsis marker
                        span class="mui-pagination__ellipsis" { "..." }
                    } @else if *page == props.current_page {
                        button
                            class="mui-pagination__page"
                            aria-current="page"
                        {
                            (page)
                        }
                    } @else {
                        button class="mui-pagination__page" {
                            (page)
                        }
                    }
                }
            }

            button
                class="mui-pagination__btn mui-pagination__btn--next"
                disabled[props.current_page == props.total_pages]
            {
                "Next"
                span class="mui-pagination__btn-icon" { (PreEscaped(CHEVRON_RIGHT)) }
            }
        }
    }
}

/// Showcase pagination in various states
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Pagination with ellipsis (page 3 of 10)" }
                (render(Props {
                    current_page: 3,
                    total_pages: 10,
                    max_visible: 5,
                }))
            }

            div {
                p.mui-showcase__caption { "Simple pagination (page 1 of 3, no ellipsis)" }
                (render(Props {
                    current_page: 1,
                    total_pages: 3,
                    max_visible: 5,
                }))
            }

            div {
                p.mui-showcase__caption { "Last page (page 10 of 10)" }
                (render(Props {
                    current_page: 10,
                    total_pages: 10,
                    max_visible: 5,
                }))
            }
        }
    }
}
