//! CSS custom-property integrity.
//!
//! Every `var(--mui-…)` reference with no fallback must resolve to a token
//! that is actually defined somewhere in `css/`. An undefined one does not
//! error — CSS drops the declaration at computed-value time and the element
//! silently falls back to its initial value. That failure mode is invisible
//! to `cargo build`, to the browser console, and to a screenshot unless you
//! already know what the component was supposed to look like.
//!
//! Found by dogfooding maud-ui inside the Conductor daemon UI (2026-07-27),
//! which surfaced five of them at once:
//!   * `--mui-radius`     (4 sites) → sidebar active-item and docs corners
//!                        rendered SQUARE in an otherwise rounded library
//!   * `--mui-bg-hover`   (2 sites) → sheet + drawer close buttons had no
//!                        hover feedback at all (background resolved to
//!                        transparent)
//!   * `--mui-mono`       (1 site)  → wrong name for `--mui-font-mono`
//!   * `--mui-spacing-xxl`(1 site)  → wrong name for `--mui-space-xxl`; only
//!                        worked via its fallback, so it was immune to theming
//!   * `--mui-sidebar-w`            → a real consumer knob that existed only
//!                        as an inline fallback, undiscoverable in the token list
//!
//! A reference WITH a fallback — `var(--mui-x, 1rem)` — is allowed: that is the
//! documented way to expose a consumer-set knob. Only bare references are
//! checked, because only those can silently render wrong.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

fn css_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("read {}: {e}", dir.display()));
    for entry in entries {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            css_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "css") {
            out.push(path);
        }
    }
}

/// Collect `--mui-foo` from `--mui-foo: value;` declarations.
fn defined_tokens(text: &str, out: &mut BTreeSet<String>) {
    for (idx, _) in text.match_indices("--mui-") {
        let rest = &text[idx..];
        let name: String = rest
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        // A definition is `--mui-foo:`; a reference is `var(--mui-foo`.
        let after = rest[name.len()..].trim_start();
        let is_definition = after.starts_with(':');
        let preceded_by_var = text[..idx].trim_end().ends_with("var(");
        if is_definition && !preceded_by_var {
            out.insert(name);
        }
    }
}

/// Collect bare `var(--mui-foo)` references — those with NO fallback.
fn bare_references(text: &str, out: &mut BTreeMap<String, ()>) {
    for (idx, _) in text.match_indices("var(") {
        let rest = &text[idx + 4..];
        let inner = rest.trim_start();
        if !inner.starts_with("--mui-") {
            continue;
        }
        let name: String = inner
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '-')
            .collect();
        // Bare iff the next non-space char closes the var() — a comma means
        // a fallback was supplied, which is legitimate.
        if inner[name.len()..].trim_start().starts_with(')') {
            out.insert(name, ());
        }
    }
}

#[test]
fn every_bare_mui_token_reference_is_defined() {
    let css_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("css");
    let mut files = Vec::new();
    css_files(&css_root, &mut files);
    assert!(!files.is_empty(), "no CSS files found under {}", css_root.display());

    let mut defined = BTreeSet::new();
    for file in &files {
        defined_tokens(&fs::read_to_string(file).expect("read css"), &mut defined);
    }

    let mut offenders: Vec<String> = Vec::new();
    for file in &files {
        let text = fs::read_to_string(file).expect("read css");
        let mut refs = BTreeMap::new();
        bare_references(&text, &mut refs);
        for name in refs.keys() {
            if !defined.contains(name) {
                let rel = file.strip_prefix(&css_root).unwrap_or(file);
                offenders.push(format!("  {} references undefined {}", rel.display(), name));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "bare var() references to undefined --mui-* tokens render silently wrong \
         (the declaration is dropped and the property falls back to its initial value).\n\
         Either define the token in css/maud-ui.css, or give the reference an explicit \
         fallback if it is a consumer-set knob:\n{}",
        offenders.join("\n")
    );
}

#[test]
fn token_checker_detects_a_planted_undefined_reference() {
    // Guards the guard: proves the detector above actually fires, so a future
    // refactor can't neuter it into a test that passes unconditionally.
    let mut defined = BTreeSet::new();
    defined_tokens(":root { --mui-real: 1rem; }", &mut defined);
    assert!(defined.contains("--mui-real"), "definition not detected");

    let mut refs = BTreeMap::new();
    bare_references(
        ".a { margin: var(--mui-real); padding: var(--mui-ghost); gap: var(--mui-knob, 2rem); }",
        &mut refs,
    );
    assert!(refs.contains_key("--mui-real"), "bare reference not detected");
    assert!(refs.contains_key("--mui-ghost"), "undefined bare reference not detected");
    assert!(
        !refs.contains_key("--mui-knob"),
        "a reference WITH a fallback must not be flagged — that is the documented knob pattern"
    );
    assert!(!defined.contains("--mui-ghost"), "ghost token must not count as defined");
}
