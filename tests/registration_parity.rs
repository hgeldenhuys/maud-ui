//! Registration parity — the registration lockstep, enforced.
//!
//! Adding a primitive means touching EIGHT places by hand, and nothing used to
//! check that you did:
//!
//!   1. `src/primitives/mod.rs`            — `pub mod <slug>;`
//!   2. `src/showcase/mod.rs`              — `COMPONENT_NAMES`
//!   3. `src/showcase/mod.rs`              — `TIERS`
//!   4. `src/showcase/docs.rs`             — the `include_str!` arm
//!   5. `tests/render_tests.rs`            — `assert_showcase_renders!`
//!
//! plus `component_content()`'s dispatch arm, `docs/components/<slug>.md`, the
//! `@import` in `css/maud-ui.css`, and the static-export list in
//! `js/export-static.mjs` (now derived, not restated).
//!
//! That invariant had drifted three separate ways by the time these tests were
//! written, each silent:
//!
//!   - `TIERS` listed a `toolbar` slug with no module, no `COMPONENT_NAMES`
//!     entry and no dispatch arm. It rendered as nothing at all, because both
//!     `TIERS` render sites skip what they cannot resolve.
//!   - Five finished primitives — `code_block`, `diff`, `message`,
//!     `streaming_cursor`, `tool_call`, ~1,470 lines with their own CSS — were
//!     registered NOWHERE. They shipped inside the crate and no consumer could
//!     discover them.
//!   - Six primitives were missing from the render-test macro.
//!
//! None of that is catchable by `cargo build`: every one of these is a
//! *cross-file agreement*, not a type error. Hence these tests.
//!
//! Several checks read source files rather than calling APIs. That is
//! deliberate: Rust has no module reflection, so the only way to notice a
//! module that was never registered anywhere is to look at the source.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use maud_ui::showcase::{component_page_by_name, docs, tier_slugs, COMPONENT_NAMES};

/// Modules under `src/primitives/` that are deliberately not gallery
/// components. Add here WITH A REASON, never to silence a failure — an entry
/// with no reason is a registration someone gave up on.
const NOT_COMPONENTS: &[(&str, &str)] = &[];

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(rel: &str) -> String {
    let path = repo_root().join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

/// Every `pub mod x;` declared in `src/primitives/mod.rs`.
fn declared_modules() -> BTreeSet<String> {
    read("src/primitives/mod.rs")
        .lines()
        .filter_map(|l| {
            l.trim()
                .strip_prefix("pub mod ")
                .and_then(|r| r.strip_suffix(';'))
                .map(str::to_string)
        })
        .collect()
}

fn registered_names() -> BTreeSet<String> {
    COMPONENT_NAMES.iter().map(|s| s.to_string()).collect()
}

/// THE check the five orphans needed: a finished primitive that no list
/// mentions is invisible to every consumer, and the compiler is happy.
#[test]
fn every_primitive_module_is_a_registered_component() {
    let exempt: BTreeSet<String> = NOT_COMPONENTS.iter().map(|(m, _)| m.to_string()).collect();
    let unregistered: Vec<String> = declared_modules()
        .difference(&registered_names())
        .filter(|m| !exempt.contains(*m))
        .cloned()
        .collect();

    assert!(
        unregistered.is_empty(),
        "these primitives exist in src/primitives/ but are in no list, so no consumer \
         can discover them: {unregistered:?}\n\
         Register each at all five points, or add it to NOT_COMPONENTS with a reason."
    );
}

/// The reverse: a slug in the list with no module behind it.
#[test]
fn every_registered_component_has_a_module() {
    let missing: Vec<String> = registered_names()
        .difference(&declared_modules())
        .cloned()
        .collect();
    assert!(
        missing.is_empty(),
        "COMPONENT_NAMES lists slugs with no src/primitives/<slug>.rs: {missing:?}"
    );
}

#[test]
fn component_names_is_sorted_and_unique() {
    let mut sorted = COMPONENT_NAMES.to_vec();
    sorted.sort_unstable();
    assert_eq!(
        COMPONENT_NAMES.to_vec(),
        sorted,
        "COMPONENT_NAMES must stay alphabetical — it is read as an index"
    );

    let unique: BTreeSet<_> = COMPONENT_NAMES.iter().collect();
    assert_eq!(
        unique.len(),
        COMPONENT_NAMES.len(),
        "COMPONENT_NAMES contains a duplicate"
    );
}

/// The `toolbar` ghost: a TIERS slug that resolves to nothing is silently
/// skipped by both render sites, so a tier can misdescribe its own contents
/// forever without failing.
#[test]
fn tiers_and_component_names_agree_both_ways() {
    let tiered: BTreeSet<String> = tier_slugs().iter().map(|s| s.to_string()).collect();
    let named = registered_names();

    let ghosts: Vec<_> = tiered.difference(&named).cloned().collect();
    assert!(
        ghosts.is_empty(),
        "TIERS lists slugs absent from COMPONENT_NAMES — they render as nothing: {ghosts:?}"
    );

    let untiered: Vec<_> = named.difference(&tiered).cloned().collect();
    assert!(
        untiered.is_empty(),
        "these components are in COMPONENT_NAMES but no tier, so the gallery nav omits them: {untiered:?}"
    );

    assert_eq!(
        tier_slugs().len(),
        COMPONENT_NAMES.len(),
        "a slug appears in more than one tier"
    );
}

/// A missing `component_content()` arm falls through to the 404 page — a live
/// nav link to a "not found" page, with a green build.
#[test]
fn every_component_renders_a_real_page() {
    let broken: Vec<&str> = COMPONENT_NAMES
        .iter()
        .filter(|name| {
            component_page_by_name(name)
                .into_string()
                .contains("Component not found")
        })
        .copied()
        .collect();
    assert!(
        broken.is_empty(),
        "these slugs fall through to the 404 page — add a component_content() arm: {broken:?}"
    );
}

#[test]
fn every_component_has_rendered_api_docs() {
    let missing: Vec<&str> = COMPONENT_NAMES
        .iter()
        .filter(|name| docs::render_component_docs(name).is_none())
        .copied()
        .collect();
    assert!(
        missing.is_empty(),
        "no docs.rs include_str! arm for: {missing:?}"
    );
}

/// The docs ship inside the published crate (Cargo.toml `include`), so a
/// missing file is a hole in what consumers download.
#[test]
fn every_component_has_a_doc_file() {
    let missing: Vec<&str> = COMPONENT_NAMES
        .iter()
        .filter(|name| !repo_root().join(format!("docs/components/{name}.md")).exists())
        .copied()
        .collect();
    assert!(
        missing.is_empty(),
        "missing docs/components/<slug>.md for: {missing:?}"
    );
}

/// A component stylesheet that exists but is never imported produces an
/// unstyled component and no error anywhere.
#[test]
fn every_component_stylesheet_is_imported() {
    let root = repo_root();
    let bundle = read("css/maud-ui.css");
    let unimported: Vec<String> = COMPONENT_NAMES
        .iter()
        .filter(|name| root.join(format!("css/components/{name}.css")).exists())
        .filter(|name| !bundle.contains(&format!("components/{name}.css")))
        .map(|s| s.to_string())
        .collect();
    assert!(
        unimported.is_empty(),
        "css/components/<slug>.css exists but is not @imported in css/maud-ui.css, \
         so the component renders unstyled: {unimported:?}"
    );
}

/// The fifth point, and the one nothing documents as part of the invariant.
#[test]
fn every_component_is_in_the_render_test_macro() {
    let src = read("tests/render_tests.rs");
    let macro_body = src
        .split_once("assert_showcase_renders!(")
        .and_then(|(_, rest)| rest.split_once(");"))
        .map(|(body, _)| body)
        .expect("assert_showcase_renders! invocation not found in tests/render_tests.rs");

    let listed: BTreeSet<&str> = macro_body
        .lines()
        .map(|l| l.trim().trim_end_matches(','))
        .filter(|l| !l.is_empty() && !l.starts_with("//"))
        .collect();

    let missing: Vec<&str> = COMPONENT_NAMES
        .iter()
        .filter(|name| !listed.contains(*name))
        .copied()
        .collect();
    assert!(
        missing.is_empty(),
        "not covered by assert_showcase_renders! in tests/render_tests.rs: {missing:?}"
    );
}

/// The public "N components" claim is derived from `COMPONENT_NAMES`, but
/// Cargo.toml's description and the README still hardcode it. They have been
/// wrong before.
#[test]
fn published_component_count_matches_reality() {
    let n = COMPONENT_NAMES.len();
    let expected = format!("{n} headless");

    let manifest = read("Cargo.toml");
    let description = manifest
        .lines()
        .find(|l| l.starts_with("description ="))
        .expect("Cargo.toml has no description");
    assert!(
        description.contains(&expected),
        "Cargo.toml description must say \"{expected}\" — there are {n} components.\nGot: {description}"
    );

    let readme = read("README.md");
    assert!(
        readme.contains(&expected),
        "README.md must say \"{expected}\" — there are {n} components"
    );
    assert!(
        readme.contains(&format!("{n} primitives")),
        "README.md must say \"{n} primitives\" — there are {n} components"
    );
}

/// The static-site export used to carry its OWN hardcoded copy of the component
/// list, under a "keep in lockstep with COMPONENT_NAMES" comment — a ninth
/// registration surface, and one where the failure is invisible: the component
/// works everywhere except that it never ships a page on the public site.
/// It now parses `COMPONENT_NAMES` instead. This keeps it that way.
#[test]
fn static_export_derives_its_component_list_from_source() {
    let src = read("js/export-static.mjs");
    assert!(
        src.contains("COMPONENT_NAMES"),
        "js/export-static.mjs must parse COMPONENT_NAMES from src/showcase/mod.rs, not restate it"
    );
    // A re-introduced literal list would show up as many quoted slugs in a row.
    let literal_run = COMPONENT_NAMES
        .iter()
        .filter(|n| src.contains(&format!("\"{n}\",")))
        .count();
    assert!(
        literal_run < 5,
        "js/export-static.mjs looks like it hardcodes the component list again \
         ({literal_run} slugs found as string literals) — derive it from COMPONENT_NAMES"
    );
}
