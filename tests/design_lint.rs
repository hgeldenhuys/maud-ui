//! Design-system linter for maud-ui, run as a cargo test.
//!
//! Scans maud markup in `src/primitives/` and `src/blocks/` and plain CSS under
//! `static/styles/`, enforcing six rules. Existing findings are ratcheted via
//! `tests/design-lint-baseline.txt`: counts may only go down, and only through
//! `DESIGN_LINT_UPDATE=1 cargo test --test design_lint`.
//!
//! Modelled on github.com/shadcn-ui/lint, but reading maud markup in Rust
//! source rather than Tailwind classes in TSX.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const MARKUP_ROOTS: [&str; 2] = ["src/primitives", "src/blocks"];
const CSS_SCAN_ROOTS: [&str; 2] = ["static/styles/components", "static/styles/blocks"];
const CSS_DEFS_ROOT: &str = "static/styles";
const BASELINE_FILE: &str = "tests/design-lint-baseline.txt";

/// Classes that exist only to be read by JS and so have no CSS rule.
/// One line of reason each; keep empty unless genuinely needed.
const JS_ONLY_CLASSES: &[&str] = &[];

const FIX_TOKEN: &str =
    "add a rule for .mui-foo in static/styles/components/<component>.css, or fix the class name";
const FIX_INLINE_STYLE: &str = "move this to a class in static/styles/components/<component>.css; if the value is computed per render (a width from data), add // design-lint: allow no-inline-styles — <reason> on the line above";
const FIX_RAW_COLOR: &str = "use a colour token such as var(--mui-…) from static/styles/tokens.css";
const FIX_ARBITRARY_PX: &str = "use a spacing, size or radius token from static/styles/tokens.css";
const FIX_STATIC_CLASS: &str =
    "return the class from a match on an enum as &'static str so the linter can check it";
const FIX_ALLOW_REASON: &str = "state why this finding is allowed after the rule name: // design-lint: allow <rule> — <reason>";
const FIX_RESTYLE: &str = "restyle .mui-other in <other>.css, or add a variant to it";

#[derive(Debug, Clone)]
struct Finding {
    rule: &'static str,
    line: usize,
    what: String,
    fix: &'static str,
}

fn crate_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rel_path(path: &Path) -> String {
    path.strip_prefix(crate_root())
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn walk(dir: &Path, ext: &str, out: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let p = entry.path();
        if p.is_dir() {
            walk(&p, ext, out);
        } else if p.extension().map(|e| e == ext).unwrap_or(false) {
            out.push(p);
        }
    }
}

/// Count braces on a line, ignoring braces inside string literals.
fn brace_delta(line: &str) -> i32 {
    let mut delta = 0i32;
    let mut in_string = false;
    let mut chars = line.chars().peekable();
    while let Some(c) = chars.next() {
        if in_string {
            if c == '\\' {
                chars.next();
            } else if c == '"' {
                in_string = false;
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '/' if chars.peek() == Some(&'/') => break,
            '{' => delta += 1,
            '}' => delta -= 1,
            _ => {}
        }
    }
    delta
}

/// Split a line into (code, had_line_comment): the comment part is blanked so
/// scanners do not see it. A `//` preceded by `:` (as in https://) is not a comment.
fn strip_line_comment(line: &str) -> String {
    let bytes = line.as_bytes();
    for i in 0..bytes.len() {
        if bytes[i] == b'/' && i + 1 < bytes.len() && bytes[i + 1] == b'/' {
            if i > 0 && bytes[i - 1] == b':' {
                continue;
            }
            return line[..i].to_string();
        }
    }
    line.to_string()
}

/// True when the line is a design-lint allow comment. Returns (rule, has_reason).
fn parse_allow_comment(line: &str) -> Option<(&'static str, bool)> {
    const RULES: [&str; 7] = [
        "no-unknown-classes",
        "no-inline-styles",
        "no-raw-colors",
        "no-arbitrary-values",
        "require-static-classes",
        "no-restyle",
        "allow-no-reason",
    ];
    let idx = line.find("design-lint: allow")?;
    let rest = line[idx + "design-lint: allow".len()..].trim_start();
    let mut rule = None;
    for r in RULES {
        if rest.starts_with(r) {
            rule = Some(match r {
                "no-unknown-classes" => "no-unknown-classes",
                "no-inline-styles" => "no-inline-styles",
                "no-raw-colors" => "no-raw-colors",
                "no-arbitrary-values" => "no-arbitrary-values",
                "require-static-classes" => "require-static-classes",
                "no-restyle" => "no-restyle",
                _ => "allow-no-reason",
            });
            break;
        }
    }
    let rule = rule?;
    let after = &rest[rule.len()..];
    let has_reason = match after.find('—') {
        Some(pos) => !after[pos + '—'.len_utf8()..].trim().is_empty(),
        None => false,
    };
    Some((rule, has_reason))
}

fn rule_allowed_on(rules: &[(usize, &'static str)], line: usize, rule: &str) -> bool {
    for (l, r) in rules {
        if *l + 1 == line && *r == rule {
            return true;
        }
    }
    false
}

// ---------------------------------------------------------------------------
// Markup (Rust) scanning
// ---------------------------------------------------------------------------

/// Extract the contents of every string literal on a code line.
fn string_literals(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut chars = line.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if c == '"' {
            let mut s = String::new();
            while let Some((_, c2)) = chars.next() {
                if c2 == '\\' {
                    chars.next();
                    continue;
                }
                if c2 == '"' {
                    break;
                }
                s.push(c2);
            }
            out.push(s);
        }
        let _ = i;
    }
    out
}

/// Is `pos` the start of an identifier character (so a token would continue)?
fn ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
}

/// Collect `mui-…` class tokens from a code line: `.mui-x` shorthand outside
/// strings and tokens inside string literals.
fn collect_class_tokens(code: &str, out: &mut BTreeSet<String>) {
    let b = code.as_bytes();
    // Shorthand: `.mui-…` outside strings.
    let mut in_string = false;
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'"' {
            in_string = !in_string;
        }
        if !in_string && b[i] == b'.' && i + 5 <= b.len() && &b[i + 1..i + 5] == b"mui-" {
            let mut j = i + 1;
            while j < b.len() && ident_byte(b[j]) {
                j += 1;
            }
            out.insert(code[i + 1..j].to_string());
            i = j;
            continue;
        }
        i += 1;
    }
    // String literals.
    for s in string_literals(code) {
        let sb = s.as_bytes();
        let mut k = 0;
        while k + 4 <= sb.len() {
            if &sb[k..k + 4] == b"mui-" && (k == 0 || !ident_byte(sb[k - 1])) {
                let mut j = k;
                while j < sb.len() && ident_byte(sb[j]) {
                    j += 1;
                }
                out.insert(s[k..j].to_string());
                k = j;
            } else {
                k += 1;
            }
        }
    }
}

/// Hand scan for colour literals: `#abc`, `#aabbcc`, `#aabbccdd`, and
/// rgb(/rgba(/hsl(/hsla(/oklch( calls. Comments must already be stripped.
fn scan_colors(code: &str, line: usize, findings: &mut Vec<Finding>) {
    let b = code.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'#' {
            let mut j = i + 1;
            while j < b.len() && b[j].is_ascii_hexdigit() {
                j += 1;
            }
            let len = j - (i + 1);
            let boundary = j >= b.len() || !b[j].is_ascii_hexdigit();
            if matches!(len, 3 | 4 | 6 | 8) && boundary {
                findings.push(Finding {
                    rule: "no-raw-colors",
                    line,
                    what: format!("raw hex colour `{}`", &code[i..j]),
                    fix: FIX_RAW_COLOR,
                });
                i = j;
                continue;
            }
            i = j.max(i + 1);
            continue;
        }
        if b[i].is_ascii() {
            for func in ["rgba(", "rgb(", "hsla(", "hsl(", "oklch("] {
                if code[i..].starts_with(func) {
                    findings.push(Finding {
                        rule: "no-raw-colors",
                        line,
                        what: format!("raw colour function `{}`…", &func[..func.len() - 1]),
                        fix: FIX_RAW_COLOR,
                    });
                    i += func.len();
                    break;
                }
            }
        }
        i += 1;
    }
}

/// Does the code line build a class at runtime for maud?
fn scan_runtime_class(code: &str, line: usize, findings: &mut Vec<Finding>) {
    if code.contains("class=(format!(") || code.contains(".(format!(") {
        findings.push(Finding {
            rule: "require-static-classes",
            line,
            what: "class built at runtime with format! — the linter cannot check it".to_string(),
            fix: FIX_STATIC_CLASS,
        });
    }
}

/// Scan one Rust markup file. Returns findings and the set of class tokens used.
fn scan_markup_file(path: &Path) -> (Vec<Finding>, BTreeSet<String>) {
    let mut findings = Vec::new();
    let mut used_classes = BTreeSet::new();
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return (findings, used_classes),
    };

    let raw_lines: Vec<&str> = text.lines().collect();
    let mut allows: Vec<(usize, &'static str)> = Vec::new();

    // Determine skip ranges (showcase fns, #[cfg(test)] modules) by brace depth.
    let mut skipped = vec![false; raw_lines.len()];
    let mut skipping = false;
    let mut depth = 0i32;
    // The item after `#[cfg(test)]` is test code whatever it is called: matching only
    // `mod tests` linted badge.rs's `mod dot_color_tests` fixtures as markup (2026-09-23).
    let mut cfg_test_pending = false;
    for (idx, raw) in raw_lines.iter().enumerate() {
        if !skipping && raw.trim() == "#[cfg(test)]" {
            cfg_test_pending = true;
            skipped[idx] = true;
            continue;
        }
        if !skipping && cfg_test_pending && !raw.contains('{') {
            // A one-line item such as `use …;` or `mod x;`.
            skipped[idx] = true;
            cfg_test_pending = !raw.trim_end().ends_with(';');
            continue;
        }
        if !skipping && (cfg_test_pending || raw.contains("fn showcase(") || raw.contains("mod tests")) {
            cfg_test_pending = false;
            skipping = true;
            depth = 0;
            skipped[idx] = true;
            depth += brace_delta(raw);
            if depth <= 0 {
                skipping = false;
            }
            continue;
        }
        if skipping {
            skipped[idx] = true;
            depth += brace_delta(raw);
            if depth <= 0 {
                skipping = false;
            }
            continue;
        }
    }

    for (idx, raw) in raw_lines.iter().enumerate() {
        if skipped[idx] {
            continue;
        }
        let line_no = idx + 1;
        // Allow comments live in comments, so inspect the raw line first.
        if let Some((rule, has_reason)) = parse_allow_comment(raw) {
            if has_reason {
                allows.push((idx, rule));
            } else {
                findings.push(Finding {
                    rule: "allow-no-reason",
                    line: line_no,
                    what: format!(
                        "design-lint allow comment for `{rule}` states no reason"
                    ),
                    fix: FIX_ALLOW_REASON,
                });
            }
            continue;
        }
        let code = strip_line_comment(raw);
        if code.trim().is_empty() {
            continue;
        }

        collect_class_tokens(&code, &mut used_classes);

        if code.contains("style=") {
            findings.push(Finding {
                rule: "no-inline-styles",
                line: line_no,
                what: "inline `style=` attribute in markup".to_string(),
                fix: FIX_INLINE_STYLE,
            });
        }
        if !rule_allowed_on(&allows, idx, "no-raw-colors") {
            scan_colors(&code, line_no, &mut findings);
        }
        if !rule_allowed_on(&allows, idx, "require-static-classes") {
            scan_runtime_class(&code, line_no, &mut findings);
        }
    }
    (findings, used_classes)
}

// ---------------------------------------------------------------------------
// CSS scanning
// ---------------------------------------------------------------------------

/// Strip /* … */ comments from CSS while preserving line structure.
fn strip_css_comments(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut in_comment = false;
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if in_comment {
            if c == '*' && chars.peek() == Some(&'/') {
                chars.next();
                in_comment = false;
            } else if c == '\n' {
                out.push('\n');
            }
            continue;
        }
        if c == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_comment = true;
            continue;
        }
        out.push(c);
    }
    out
}

/// Collect every class token appearing in (comment-stripped) CSS text —
/// a class counts as defined when `.name` appears in any selector there.
fn collect_css_defined_classes(text: &str, out: &mut BTreeSet<String>) {
    let b = text.as_bytes();
    let mut i = 0;
    while i + 5 <= b.len() {
        if b[i] == b'.' && &b[i + 1..i + 5] == b"mui-" {
            let mut j = i + 1;
            while j < b.len() && ident_byte(b[j]) {
                j += 1;
            }
            out.insert(text[i + 1..j].to_string());
            i = j;
        } else {
            i += 1;
        }
    }
}

/// The block name a class belongs to: `mui-b`, `mui-b__…` or `mui-b--…`.
fn class_block(class: &str) -> Option<&str> {
    let rest = class.strip_prefix("mui-")?;
    let cut = rest.find("__").or_else(|| rest.find("--")).unwrap_or(rest.len());
    Some(&rest[..cut])
}

/// no-restyle: in components/<name>.css, a selector whose FIRST class belongs
/// to a different block. Descendant selectors starting with the file's own
/// block are fine.
fn scan_restyle(block: &str, stripped: &str, path_str: &str, findings: &mut Vec<Finding>) {
    let mut selector = String::new();
    let mut depth = 0i32;
    for (line_no, raw) in stripped.lines().enumerate() {
        let line_no = line_no + 1;
        if depth == 0 {
            selector.push_str(raw);
            selector.push(' ');
        }
        for c in raw.chars() {
            if c == '{' {
                depth += 1;
            } else if c == '}' {
                depth -= 1;
                if depth == 0 {
                    selector.clear();
                }
            }
        }
        if depth == 0 {
            if let Some(brace) = selector.find('{') {
                let sel_text = selector[..brace].trim();
                if !sel_text.is_empty() && !sel_text.starts_with('@') {
                    for part in sel_text.split(',') {
                        let first = first_mui_class(part.trim());
                        if let Some(class) = first {
                            if let Some(owner) = class_block(class.as_str()) {
                                if owner != block {
                                    findings.push(Finding {
                                        rule: "no-restyle",
                                        line: line_no,
                                        what: format!(
                                            "`{path_str}` restyles another component's block: \
                                             first class `.{class}` belongs to `{owner}`"
                                        ),
                                        fix: FIX_RESTYLE,
                                    });
                                }
                            }
                        }
                    }
                }
                selector.clear();
            }
            if raw.trim_end().ends_with(';') {
                selector.clear();
            }
        }
    }
}

/// First `.mui-…` class token in a selector.
fn first_mui_class(selector: &str) -> Option<String> {
    let b = selector.as_bytes();
    let mut i = 0;
    while i + 5 <= b.len() {
        if b[i] == b'.' && &b[i + 1..i + 5] == b"mui-" {
            let mut j = i + 1;
            while j < b.len() && ident_byte(b[j]) {
                j += 1;
            }
            return Some(selector[i + 1..j].to_string());
        }
        i += 1;
    }
    None
}

/// Lengths in px other than 0px/1px/2px (hairlines and focus rings), outside
/// @media query conditions.
fn scan_px(stripped: &str, path_str: &str, findings: &mut Vec<Finding>) {
    for (idx, raw) in stripped.lines().enumerate() {
        let trimmed = raw.trim_start();
        if trimmed.starts_with("@media") {
            continue;
        }
        let b = raw.as_bytes();
        let mut i = 0;
        while i + 2 <= b.len() {
            if b[i] == b'p' && b[i + 1] == b'x' && i > 0 && (b[i - 1].is_ascii_digit()) {
                let mut start = i;
                while start > 0
                    && (b[start - 1].is_ascii_digit() || b[start - 1] == b'.' || b[start - 1] == b'-')
                {
                    start -= 1;
                }
                let num = &raw[start..i];
                if num != "0" && num != "1" && num != "2" && num != "0." && num != "-0" {
                    findings.push(Finding {
                        rule: "no-arbitrary-values",
                        line: idx + 1,
                        what: format!("arbitrary length `{num}px` in `{path_str}`"),
                        fix: FIX_ARBITRARY_PX,
                    });
                }
                i += 2;
                continue;
            }
            i += 1;
        }
    }
}

/// Scan one CSS file for the CSS-side rules. Also appends its class tokens to
/// `defined` when requested (the definitions pass reads every file anyway).
fn scan_css_file(path: &Path, scan_rules: bool) -> (Vec<Finding>, BTreeSet<String>) {
    let mut findings = Vec::new();
    let mut defined = BTreeSet::new();
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return (findings, defined),
    };
    let path_str = rel_path(path);
    let stripped = strip_css_comments(&text);

    collect_css_defined_classes(&stripped, &mut defined);
    if !scan_rules {
        return (findings, defined);
    }

    // Allow comments (raw text, before stripping).
    let mut allows: Vec<(usize, &'static str)> = Vec::new();
    for (idx, raw) in text.lines().enumerate() {
        if let Some((rule, has_reason)) = parse_allow_comment(raw) {
            if has_reason {
                allows.push((idx, rule));
            } else {
                findings.push(Finding {
                    rule: "allow-no-reason",
                    line: idx + 1,
                    what: format!("design-lint allow comment for `{rule}` states no reason"),
                    fix: FIX_ALLOW_REASON,
                });
            }
        }
    }

    for (idx, raw) in stripped.lines().enumerate() {
        let line_no = idx + 1;
        if rule_allowed_on(&allows, idx, "no-raw-colors") {
            continue;
        }
        scan_colors(raw, line_no, &mut findings);
    }

    if !rule_allowed_lines_exist(&allows, "no-arbitrary-values") {
        scan_px(&stripped, &path_str, &mut findings);
    }

    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        let block = stem.replace('_', "-");
        scan_restyle(&block, &stripped, &path_str, &mut findings);
    }

    (findings, defined)
}

fn rule_allowed_lines_exist(allows: &[(usize, &'static str)], rule: &str) -> bool {
    allows.iter().any(|(_, r)| *r == rule)
}

// ---------------------------------------------------------------------------
// Ratchet
// ---------------------------------------------------------------------------

fn run_lint() -> (Vec<(String, Finding)>, BTreeSet<String>) {
    let root = crate_root();
    let mut findings: Vec<(String, Finding)> = Vec::new();
    let mut defined: BTreeSet<String> = BTreeSet::new();

    // Definitions come from every CSS file under static/styles/.
    let mut all_css = Vec::new();
    walk(&root.join(CSS_DEFS_ROOT), "css", &mut all_css);
    all_css.sort();

    let mut markup_files = Vec::new();
    for r in MARKUP_ROOTS {
        walk(&root.join(r), "rs", &mut markup_files);
    }
    markup_files.sort();

    let mut scan_files = Vec::new();
    for r in CSS_SCAN_ROOTS {
        walk(&root.join(r), "css", &mut scan_files);
    }
    scan_files.sort();

    for path in &all_css {
        let (_, defs) = scan_css_file(path, false);
        defined.extend(defs);
    }
    for path in &markup_files {
        let (f, _used) = scan_markup_file(path);
        let p = rel_path(path);
        for finding in f {
            findings.push((p.clone(), finding));
        }
    }

    // no-unknown-classes
    let defined: BTreeSet<String> = defined;
    for path in &markup_files {
        let p = rel_path(path);
        let (_, used) = scan_markup_file(path);
        for class in &used {
            // A token ending in a separator is a runtime-composed prefix
            // (e.g. "mui-item--" fed to format!), not a whole class name.
            if class.ends_with('-') || class.ends_with('_') || class.ends_with('.') {
                continue;
            }
            if class.starts_with("mui-") && !defined.contains(class) && !JS_ONLY_CLASSES.contains(&class.as_str())
            {
                // Attribute to the first line in the file that uses it.
                if let Ok(text) = fs::read_to_string(path) {
                    for (idx, raw) in text.lines().enumerate() {
                        let code = strip_line_comment(raw);
                        let mut toks = BTreeSet::new();
                        collect_class_tokens(&code, &mut toks);
                        if toks.contains(class) {
                            findings.push((
                                p.clone(),
                                Finding {
                                    rule: "no-unknown-classes",
                                    line: idx + 1,
                                    what: format!("class `.{class}` is used in markup but no CSS defines it"),
                                    fix: FIX_TOKEN,
                                },
                            ));
                            break;
                        }
                    }
                }
            }
        }
    }

    for path in &scan_files {
        let (f, _) = scan_css_file(path, true);
        let p = rel_path(path);
        for finding in f {
            findings.push((p.clone(), finding));
        }
    }
    (findings, defined)
}

fn load_baseline() -> BTreeMap<(String, String), u64> {
    let mut map = BTreeMap::new();
    let path = crate_root().join(BASELINE_FILE);
    if let Ok(text) = fs::read_to_string(path) {
        for line in text.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 3 {
                if let Ok(count) = parts[2].parse::<u64>() {
                    map.insert((parts[0].to_string(), parts[1].to_string()), count);
                }
            }
        }
    }
    map
}

fn write_baseline(counts: &BTreeMap<(String, String), u64>) {
    let mut text = String::from("# design-lint baseline — regenerate with DESIGN_LINT_UPDATE=1 cargo test --test design_lint\n");
    for ((rule, path), count) in counts {
        text.push_str(&format!("{rule} {path} {count}\n"));
    }
    fs::write(crate_root().join(BASELINE_FILE), text).expect("write baseline");
}

fn format_finding(path: &str, f: &Finding) -> String {
    format!("{}  {}:{}  {}  →  {}", f.rule, path, f.line, f.what, f.fix)
}

#[test]
fn design_lint_ratchet() {
    let (findings, _defined) = run_lint();

    let mut current: BTreeMap<(String, String), u64> = BTreeMap::new();
    let mut by_group: BTreeMap<(String, String), Vec<&Finding>> = BTreeMap::new();
    for (path, f) in &findings {
        *current.entry((f.rule.to_string(), path.clone())).or_insert(0) += 1;
        by_group.entry((f.rule.to_string(), path.clone())).or_default().push(f);
    }

    let update = env::var("DESIGN_LINT_UPDATE").map(|v| v == "1").unwrap_or(false);
    let baseline = load_baseline();

    if update {
        write_baseline(&current);
        println!("design-lint baseline rewritten: {} groups, {} findings", current.len(), findings.len());
        return;
    }

    let mut over_budget: Vec<(String, String, u64, u64)> = Vec::new();
    let mut fixed_up: Vec<(String, String, u64, u64)> = Vec::new();
    let mut total_new = 0u64;
    let mut total_fixed = 0u64;

    for (key, count) in &current {
        match baseline.get(key) {
            None => {
                over_budget.push((key.0.clone(), key.1.clone(), 0, *count));
                total_new += count;
            }
            Some(b) if count > b => {
                over_budget.push((key.0.clone(), key.1.clone(), *b, *count));
                total_new += count - b;
            }
            _ => {}
        }
    }
    for (key, b) in &baseline {
        match current.get(key) {
            None => {
                fixed_up.push((key.0.clone(), key.1.clone(), *b, 0));
                total_fixed += b;
            }
            Some(c) if c < b => {
                fixed_up.push((key.0.clone(), key.1.clone(), *b, *c));
                total_fixed += b - c;
            }
            _ => {}
        }
    }

    let mut failure = String::new();
    if !over_budget.is_empty() {
        failure.push_str("design-lint: findings exceed the baseline\n\n");
        for (rule, path, allowed, actual) in &over_budget {
            failure.push_str(&format!(
                "{rule} {path}: baseline {allowed}, now {actual}\n"
            ));
            if let Some(group) = by_group.get(&(rule.clone(), path.clone())) {
                for f in group {
                    failure.push_str(&format!("  {}\n", format_finding(path, f)));
                }
            }
            failure.push('\n');
        }
        failure.push_str(&format!("{total_new} new findings\n"));
    }
    if total_fixed > 0 {
        if !failure.is_empty() {
            failure.push('\n');
        }
        failure.push_str(&format!(
            "{total_fixed} findings were fixed: run DESIGN_LINT_UPDATE=1 cargo test --test design_lint to lower the baseline\n"
        ));
        for (rule, path, allowed, actual) in &fixed_up {
            failure.push_str(&format!("{rule} {path}: baseline {allowed}, now {actual}\n"));
        }
    }

    assert!(
        failure.is_empty(),
        "{}",
        failure.trim_end()
    );
}

// ---------------------------------------------------------------------------
// Unit tests for the scanners
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn classes_of(code: &str) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        collect_class_tokens(code, &mut out);
        out
    }

    #[test]
    fn shorthand_with_multiple_classes_and_condition_yields_both() {
        let got = classes_of("span.mui-a.mui-b[cond] {}");
        assert!(got.contains("mui-a"), "expected mui-a in {got:?}");
        assert!(got.contains("mui-b"), "expected mui-b in {got:?}");
    }

    #[test]
    fn comment_hex_colour_is_ignored() {
        let mut findings = Vec::new();
        scan_colors(&strip_line_comment("div { // fallback #fff here"), 1, &mut findings);
        assert!(findings.is_empty(), "expected no findings, got {findings:?}");
    }

    #[test]
    fn var_fallback_hex_colour_is_reported() {
        let mut findings = Vec::new();
        scan_colors("color: var(--x, #fff);", 1, &mut findings);
        assert_eq!(findings.len(), 1, "expected exactly one finding, got {findings:?}");
        assert_eq!(findings[0].rule, "no-raw-colors");
        assert_eq!(findings[0].line, 1);
        assert!(findings[0].what.contains("#fff"), "what = {}", findings[0].what);
    }

    #[test]
    fn runtime_format_class_is_reported() {
        let mut findings = Vec::new();
        scan_runtime_class("div class=(format!(\"mui-{x}\")) {}", 7, &mut findings);
        assert_eq!(findings.len(), 1, "expected exactly one finding, got {findings:?}");
        assert_eq!(findings[0].rule, "require-static-classes");
        assert_eq!(findings[0].line, 7);
    }

    #[test]
    fn allow_comment_suppresses_matching_rule_on_next_line() {
        let line1 = "// design-lint: allow no-inline-styles — width comes from data";
        let (rule, has_reason) = parse_allow_comment(line1).expect("allow comment should parse");
        assert_eq!(rule, "no-inline-styles");
        assert!(has_reason);
        let allows = [(0usize, "no-inline-styles")];
        assert!(rule_allowed_on(&allows, 1usize, "no-inline-styles"));
        assert!(!rule_allowed_on(&allows, 2usize, "no-inline-styles"));
        assert!(!rule_allowed_on(&allows, 1usize, "no-raw-colors"));
    }

    #[test]
    fn allow_comment_without_reason_is_reported() {
        let findings_line = "// design-lint: allow no-inline-styles";
        match parse_allow_comment(findings_line) {
            Some((rule, has_reason)) => {
                assert_eq!(rule, "no-inline-styles");
                assert!(!has_reason, "an allow with no reason must report has_reason = false");
            }
            None => panic!("allow comment should parse"),
        }
    }

    #[test]
    fn px_outside_hairlines_is_reported_and_media_conditions_skipped() {
        let mut findings = Vec::new();
        scan_px("  width: 8px;\n@media (min-width: 600px) {\n  .x { top: 3px; }\n}\n  border: 1px solid red;", "f.css", &mut findings);
        let widths: Vec<&str> = findings.iter().filter(|f| f.what.contains("8px")).map(|_| "8").collect();
        assert_eq!(widths.len(), 1, "expected the 8px finding once, got {findings:?}");
        assert!(findings.iter().any(|f| f.what.contains("3px")), "3px inside @media rule body should be reported: {findings:?}");
        assert!(!findings.iter().any(|f| f.what.contains("600px")), "@media condition must be skipped: {findings:?}");
        assert!(!findings.iter().any(|f| f.what.contains("1px")), "1px hairline must be allowed: {findings:?}");
    }

    #[test]
    fn class_block_mapping() {
        assert_eq!(class_block("mui-btn"), Some("btn"));
        assert_eq!(class_block("mui-time-split__seg--t0"), Some("time-split"));
        assert_eq!(class_block("mui-alert-dialog__body"), Some("alert-dialog"));
        assert_eq!(class_block("nope"), None);
    }

    #[test]
    fn css_comment_hex_is_ignored_by_stripper() {
        let stripped = strip_css_comments("/* #aabbcc */ .a { color: var(--x); }");
        let mut findings = Vec::new();
        scan_colors(&stripped, 1, &mut findings);
        assert!(findings.is_empty(), "commented hex must not be reported: {findings:?}");
    }
}
