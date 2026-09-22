//! Design tokens for maud-ui. These constants mirror the CSS custom properties
//! in `static/styles/tokens.css` — consumers can reference either side.

pub mod colors {
    pub const BG: &str = "#101216";
    pub const BG_CARD: &str = "#181b20";
    pub const BG_OVERLAY: &str = "rgba(0, 0, 0, 0.64)";
    pub const BG_INPUT: &str = "#20242b";
    pub const BORDER: &str = "#303640";
    pub const BORDER_HOVER: &str = "#303640";
    pub const BORDER_FOCUS: &str = "#84aaff";
    pub const TEXT: &str = "#edf0f4";
    pub const TEXT_SECONDARY: &str = "#c1c8d2";
    pub const TEXT_MUTED: &str = "#a1acba";
    pub const TEXT_SUBTLE: &str = "#939eae";
    pub const ACCENT: &str = "#84aaff";
    pub const ACCENT_HOVER: &str = "#9bbaff";
    pub const ACCENT_FG: &str = "#101b32";
    pub const SUCCESS: &str = "#28734f";
    pub const WARNING: &str = "#805815";
    pub const DANGER: &str = "#a1363b";
    pub const DANGER_HOVER: &str = "#892e32";
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
    pub const SM: &str = "0.25rem";
    pub const MD: &str = "0.375rem";
    pub const LG: &str = "0.75rem";
    pub const FULL: &str = "9999px";
}

/// The radius scale, as a closed enum for component props.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Radius {
    /// `var(--mui-radius-sm)`.
    #[default]
    Sm,
    /// `var(--mui-radius-md)`.
    Md,
    /// `var(--mui-radius-lg)` — the card/corner step.
    Lg,
    /// `var(--mui-radius-full)` — the pill.
    Full,
}

impl Radius {
    /// The CSS value this role resolves to.
    pub fn value(self) -> &'static str {
        match self {
            Radius::Sm => "var(--mui-radius-sm)",
            Radius::Md => "var(--mui-radius-md)",
            Radius::Lg => "var(--mui-radius-lg)",
            Radius::Full => "var(--mui-radius-full)",
        }
    }
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
    /// Expanded operating toolbar; below this, Settings contains the controls.
    pub const XXL: &str = "80rem";

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
    /// Immediately below the expanded operating toolbar.
    pub const BELOW_XXL: &str = "79.99rem";
}

/// Compact status filters and table pills, shared by both palettes.
pub mod status {
    pub const CHIP_HEIGHT: &str = "1.875rem";
    pub const CHIP_LABEL_SIZE: &str = "0.8125rem";
    pub const CHIP_COUNT_HEIGHT: &str = "1.125rem";
    pub const CHIP_COUNT_SIZE: &str = "0.6875rem";
    pub const BADGE_HEIGHT: &str = "1.375rem";
}

/// A caller-supplied colour is a colour, never free CSS: a hex colour, a named colour,
/// a `var(--token)` reference, or an `rgb()`/`hsl()`/`oklch()`/`lch()`
/// function over digits, `.`, `,`, `%`, `/` and spaces. Anything else
/// (a `;`, a `url(`, an expression) renders the dot unpainted rather than
/// reaching the style attribute — the value may come from user data.
pub(crate) fn is_css_color(value: &str) -> bool {
    let v = value.trim();
    if v.is_empty() || v.len() > 64 {
        return false;
    }
    if let Some(hex) = v.strip_prefix('#') {
        return matches!(hex.len(), 3 | 4 | 6 | 8) && hex.chars().all(|c| c.is_ascii_hexdigit());
    }
    if let Some(inner) = v.strip_prefix("var(--").and_then(|r| r.strip_suffix(')')) {
        return !inner.is_empty() && inner.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    }
    for func in ["rgb(", "rgba(", "hsl(", "hsla(", "oklch(", "lch("] {
        if let Some(inner) = v.strip_prefix(func).and_then(|r| r.strip_suffix(')')) {
            return !inner.is_empty()
                && inner
                    .chars()
                    .all(|c| c.is_ascii_digit() || matches!(c, '.' | ',' | '%' | '/' | ' ' | '-'));
        }
    }
    v.chars().all(|c| c.is_ascii_alphabetic())
}

