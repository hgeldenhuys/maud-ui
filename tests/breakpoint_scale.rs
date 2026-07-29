//! Breakpoint-scale integrity.
//!
//! Every width in a `@media` condition must be one of the five declared
//! breakpoints in `tokens::breakpoints` (or its `BELOW_*` twin). Nothing
//! enforces this in CSS itself: an off-scale width is perfectly valid CSS,
//! renders fine, and reviews cleanly — it just quietly adds one more place the
//! layout can change, which nobody documented and nobody tests.
//!
//! That is how the scale drifted before this test existed (audited
//! 2026-07-28). Eight distinct widths were live across the tree for what is
//! really five decisions:
//!
//!   * `640px` × 21 and `40rem` × 6 — the *same* breakpoint, written two ways
//!   * `45rem` in `table.css`, gating an attribute literally named
//!     `data-hide-sm` — so "sm" fired at 720px while every other "sm" in the
//!     library fired at 640px
//!   * one-offs at `760px`, `768px`, `62rem`
//!
//! Two widths that *looked* like drift turned out to be real, separate
//! decisions and were declared rather than folded away: `960px` is where a
//! 240px sidebar plus a readable content column start to fit (LG), and
//! `1024px` is where embedded editor demos get their full height (XL).
//! Collapsing 960 into 1024 pushed 961–1023px onto the mobile nav — caught by
//! diffing against the deployed build before it shipped.
//!
//! Note what that episode proves about this file: the test can only tell you a
//! width is off-scale. It cannot tell you whether folding it onto the scale
//! preserves behaviour. That judgement stays with the author.

use std::fs;
use std::path::{Path, PathBuf};

use maud_ui::tokens::breakpoints as bp;

/// Widths a `@media` condition is allowed to name.
fn allowed() -> Vec<&'static str> {
    vec![
        bp::XS,
        bp::SM,
        bp::MD,
        bp::LG,
        bp::XL,
        bp::BELOW_XS,
        bp::BELOW_SM,
        bp::BELOW_MD,
        bp::BELOW_LG,
        bp::BELOW_XL,
    ]
}

fn collect(dir: &Path, exts: &[&str], out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, exts, out);
        } else if path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| exts.contains(&e))
        {
            out.push(path);
        }
    }
}

/// Every `<...>width: <value>` named inside a `@media` condition, with the
/// 1-based line number it sits on.
fn media_widths(src: &str) -> Vec<(usize, String)> {
    let mut found = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let Some(at) = line.find("@media") else { continue };
        // The condition runs to the opening brace (all our queries are one line).
        let cond_end = line[at..].find('{').map(|b| at + b).unwrap_or(line.len());
        let cond = &line[at..cond_end];
        let mut rest = cond;
        while let Some(w) = rest.find("width:") {
            let after = &rest[w + "width:".len()..];
            let value: String = after
                .chars()
                .take_while(|c| *c != ')' && *c != ',')
                .collect::<String>()
                .trim()
                .to_string();
            if !value.is_empty() {
                found.push((i + 1, value));
            }
            rest = after;
        }
    }
    found
}

#[test]
fn every_media_width_is_a_declared_breakpoint() {
    let mut files = Vec::new();
    collect(Path::new("css"), &["css"], &mut files);
    collect(Path::new("src/showcase"), &["css", "rs"], &mut files);
    assert!(!files.is_empty(), "found no files to scan");

    let allowed = allowed();
    let mut offences = Vec::new();

    for path in &files {
        let src = fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
        for (line, value) in media_widths(&src) {
            if !allowed.contains(&value.as_str()) {
                offences.push(format!("  {}:{line} — `{value}`", path.display()));
            }
        }
    }

    assert!(
        offences.is_empty(),
        "off-scale breakpoint(s) — {} site(s) name a width that is not in the declared scale:\n{}\n\n\
         The scale is XS {} · SM {} · MD {} · LG {} · XL {} (src/tokens.rs → `breakpoints`).\n\
         Fix by moving the rule to the nearest scale value. If the rule sits opposite a\n\
         `min-width` rule at the same breakpoint, use the BELOW_* twin instead\n\
         (e.g. `max-width: {}` against `min-width: {}`) so the two halves stay disjoint\n\
         and leave no unstyled band between them.\n\
         Adding a new breakpoint is a design-system decision: declare it in\n\
         `tokens::breakpoints` and document it, don't inline it here.",
        offences.len(),
        offences.join("\n"),
        bp::XS,
        bp::SM,
        bp::MD,
        bp::LG,
        bp::XL,
        bp::BELOW_LG,
        bp::LG,
    );
}

#[test]
fn below_twins_sit_just_under_their_breakpoint() {
    // A BELOW_* that drifted away from its partner would reopen the unstyled
    // band this scale exists to close.
    for (below, at) in [
        (bp::BELOW_XS, bp::XS),
        (bp::BELOW_SM, bp::SM),
        (bp::BELOW_MD, bp::MD),
        (bp::BELOW_LG, bp::LG),
        (bp::BELOW_XL, bp::XL),
    ] {
        let parse = |s: &str| {
            s.trim_end_matches("rem")
                .parse::<f64>()
                .unwrap_or_else(|e| panic!("breakpoint `{s}` is not a rem value: {e}"))
        };
        let (lo, hi) = (parse(below), parse(at));
        assert!(
            hi - lo > 0.0 && hi - lo <= 0.02,
            "`{below}` must sit just below `{at}` (gap {:.4}rem) — a wider gap leaves \
             viewport widths matched by neither half of a max/min pair",
            hi - lo
        );
    }
}

#[test]
fn the_scale_is_ordered_and_distinct() {
    let scale = [bp::XS, bp::SM, bp::MD, bp::LG, bp::XL];
    let mut prev = 0.0_f64;
    for value in scale {
        let v: f64 = value.trim_end_matches("rem").parse().expect("rem value");
        assert!(
            v > prev,
            "breakpoint scale must ascend and hold no duplicates; `{value}` does not exceed {prev}rem"
        );
        prev = v;
    }
}
