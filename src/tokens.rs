//! Design tokens for maud-ui. These constants mirror the CSS custom properties
//! in `css/maud-ui.css` — consumers can reference either side.

pub mod colors {
    pub const BG: &str = "#0a0a0b";
    pub const BG_CARD: &str = "#111113";
    pub const BG_OVERLAY: &str = "#18181b";
    pub const BG_INPUT: &str = "#18181b";
    pub const BORDER: &str = "#27272a";
    pub const BORDER_HOVER: &str = "#3f3f46";
    pub const BORDER_FOCUS: &str = "#a1a1aa";
    pub const TEXT: &str = "#fafafa";
    pub const TEXT_MUTED: &str = "#a1a1aa";
    pub const TEXT_SUBTLE: &str = "#71717a";
    pub const ACCENT: &str = "#3b82f6";
    pub const ACCENT_HOVER: &str = "#60a5fa";
    pub const ACCENT_FG: &str = "#ffffff";
    pub const SUCCESS: &str = "#22c55e";
    pub const WARNING: &str = "#eab308";
    pub const DANGER: &str = "#ef4444";
    pub const DANGER_HOVER: &str = "#dc2626";
}

pub mod spacing {
    pub const XS: &str = "0.25rem";
    pub const SM: &str = "0.5rem";
    pub const MD: &str = "0.75rem";
    pub const LG: &str = "1rem";
    pub const XL: &str = "1.5rem";
    pub const XXL: &str = "2rem";
}

pub mod radius {
    pub const SM: &str = "calc(0.5rem - 2px)";
    pub const MD: &str = "0.5rem";
    pub const LG: &str = "0.75rem";
    pub const FULL: &str = "9999px";
}

/// Viewport breakpoints — the declared scale for **page-level layout**.
///
/// Authored in `rem` on purpose. Inside a media query `rem` resolves against
/// the browser's *initial* font size, not `html { font-size }`, so a reader
/// who raises their default font size in browser settings gets the layout
/// change earlier. A `px` breakpoint ignores that preference entirely.
///
/// # These cannot be CSS custom properties
///
/// `@media (max-width: var(--mui-bp-sm))` is invalid and fails **silently** —
/// the block simply never matches. Custom properties are not permitted in a
/// media *condition*, and the `@custom-media` at-rule is not implemented by
/// any browser (verified against Chrome 150, 2026-07-28). So a breakpoint is
/// a literal in CSS and a constant here; there is no single source both sides
/// can read at runtime. Keep the two in step by hand.
///
/// # Prefer a container query for a component
///
/// These are for the *page shell*. A component does not know the viewport —
/// it knows its slot. A card dropped into a 300px sidebar on a wide monitor
/// should stack, and a viewport query gets that exactly backwards. Reach for
/// `@container` unless the rule really is about the page.
pub mod breakpoints {
    /// 480px — small phones. Below this, side-by-side pairs stop fitting.
    pub const XS: &str = "30rem";
    /// 640px — the phone/tablet line, and the library's dominant breakpoint.
    pub const SM: &str = "40rem";
    /// 768px — tablet portrait; shell layouts collapse to a single column.
    pub const MD: &str = "48rem";
    /// 960px — the width at which a 240px sidebar plus a comfortable content
    /// column both fit, so the gallery shell swaps its hamburger for a
    /// persistent sidebar.
    pub const LG: &str = "60rem";
    /// 1024px — tablet landscape and up; embedded editors and map/calendar
    /// demos get their full height here.
    pub const XL: &str = "64rem";

    /// Upper bound for a "below this breakpoint" query.
    ///
    /// Only needed when a rule is written as a `max-width`/`min-width` **pair
    /// at the same breakpoint**: `max-width: 60rem` and `min-width: 60rem`
    /// both match at exactly 960px, which leaves the boundary pixel decided by
    /// source order.
    ///
    /// ```text
    /// @media (max-width: 59.99rem) { /* below lg */ }
    /// @media (min-width: 60rem)    { /* lg and up */ }
    /// ```
    ///
    /// No such pair exists in the tree today — every responsive rule here is
    /// single-direction, overriding a base that already holds for the other
    /// side. These are for the author who writes the first one.
    pub const BELOW_XS: &str = "29.99rem";
    pub const BELOW_SM: &str = "39.99rem";
    pub const BELOW_MD: &str = "47.99rem";
    pub const BELOW_LG: &str = "59.99rem";
    pub const BELOW_XL: &str = "63.99rem";
}
