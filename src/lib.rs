//! maud-ui — Headless accessible component library for maud + htmx.
//!
//! Components return `maud::Markup` and follow Base UI / WAI-ARIA patterns.
//! Ships with a paired light/dark themes in `static/styles/tokens.css` that consumers
//! can override via CSS custom properties.

pub mod assets;
pub mod blocks;
pub mod brand;
pub mod primitives;
pub mod showcase;
pub mod time;
pub mod tokens;
