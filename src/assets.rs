//! Complete, version-matched bundles. Serve these constants or vendor static/.
//! CSS contains no imports. JS includes the base runtime and all behaviors.
pub const CSS: &str = include_str!("../static/maud-ui.css");
pub const CSS_MIN: &str = include_str!("../static/maud-ui.min.css");
pub const JS: &str = include_str!("../static/maud-ui.js");
pub const JS_MIN: &str = include_str!("../static/maud-ui.min.js");

/// Inter Variable (weight 100–900, OFL). The default theme's @font-face points
/// at the RELATIVE url `fonts/InterVariable.woff2`, so serve these bytes at
/// `<css dir>/fonts/InterVariable.woff2` — next to wherever maud-ui.css is
/// served from. If it 404s the font stack falls back to system-ui silently.
pub const INTER_WOFF2: &[u8] = include_bytes!("../static/fonts/InterVariable.woff2");
