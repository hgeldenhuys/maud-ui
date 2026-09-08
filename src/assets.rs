//! Complete, version-matched bundles. Serve these constants or vendor static/.
//! CSS contains no imports. JS includes the base runtime and all behaviors.
pub const CSS: &str = include_str!("../static/maud-ui.css");
pub const CSS_MIN: &str = include_str!("../static/maud-ui.min.css");
pub const JS: &str = include_str!("../static/maud-ui.js");
pub const JS_MIN: &str = include_str!("../static/maud-ui.min.js");
