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
