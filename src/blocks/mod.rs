//! Pre-composed block templates built from primitives.
//!
//! Blocks are opinionated, production-ready compositions you drop into
//! real apps — authentication screens, dashboards, settings pages,
//! marketing sections. Each block is a single `render(Props) -> Markup`
//! function that delegates to `primitives::*`.
//!
//! Blocks are meant to be **copied and customized**, not treated as
//! black boxes. Read the source of the one closest to what you need,
//! paste it into your own module, and edit. The Props are a convenient
//! API for the common case, but the real value is the structure shown
//! in the render body.
//!
//! Block naming convention:
//!   - Slug: `category-name` (e.g., `auth-login`, `dashboard-stats`)
//!   - Module: `category::name` (e.g., `blocks::auth::login`)
//!
//! See `BLOCK_NAMES` below for the canonical registry — anything listed
//! there has a showcase page at `/blocks/{slug}`.

pub mod auth;

/// Canonical slug registry. Mirrors the showcase routes.
///
/// A block name here MUST have:
///   1. A module at `blocks::{category}::{name}`
///   2. A `render(Props) -> Markup` consumer API
///   3. A `preview() -> Markup` for the showcase page
///   4. A matching arm in `showcase::block_content()`
pub const BLOCK_NAMES: &[&str] = &["auth-login"];

/// Convert a block slug into a human-readable name.
/// "auth-login" → "Auth — Login"
pub fn display_name(slug: &str) -> String {
    let parts: Vec<&str> = slug.split('-').collect();
    if parts.len() < 2 {
        return title_case(slug);
    }
    let category = title_case(parts[0]);
    let name = title_case(&parts[1..].join(" "));
    format!("{category} \u{2014} {name}")
}

fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}
