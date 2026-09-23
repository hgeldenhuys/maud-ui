---
arc: maud-ui-improvements
repo: /Users/hgeldenhuys/WebstormProjects/maud-ui
branch: curation-2026-09-08 (pushed to customer; website deploys from customer/main, mirrors github/master + forgejo/master)
ratified: 2026-09-23 by Herman ("let's do all 5, after we /wrapoff", plus #6 the linter)
seat: claude-opus-5-5
---

# maud-ui improvement arc

## POSITION (2026-09-23 13:20 EDT)

maud-ui 0.19.5 is on crates.io and live at https://maudui.herman.engineer. The four Kapable apps
run 0.19.3. Item 3 LANDED (unreleased, on the branch): `cargo test --no-fail-fast` = 311 passed,
0 failed; `cargo clippy --all-targets -D warnings` clean (fixed a needless `..Default::default()`
in tests/record_page.rs:183). time_split is fully registered: doc + rendered page, docs.rs arm,
render test, alphabetical COMPONENT_NAMES, 84 in Cargo.toml / README (7 places) / og.svg + og.png.
Gotcha found: `cargo run --example build_docs` cannot compile while docs.rs include_str!s a page
that does not exist yet; seed an empty `docs/components/rendered/<name>.html` first.

## NEXT_ACTION

Item 1: write scripts/release.sh (the workflow in docs/releasing.md "Current release workflow" as
one command). Must: refuse a dirty tree (ignoring docs/night-6-live-events.jsonl), stop on the first
failure, require Cargo.toml version > crates.io max_version and a CHANGELOG entry for it, run
build-assets + dist sync + build:static + cargo test (any FAILED line stops it) + clippy -D warnings,
commit regenerated output excluding the night-6 log, cargo publish, the four pushes,
kapable-push-verify, then poll the live stylesheet for a marker given as an argument (sampled twice).
Have a --dry-run that stops before publish. Dispatch to a builder, review by another family.

## PLAN (ranked; the reason is the rank)

1. Item 3, green suite. Smallest, and every later item uses the suite as its check; a suite that
   is always red hides new failures (0.19.0's breadcrumb regression had no test).
2. Item 1, `scripts/release.sh`: the workflow in docs/releasing.md as one command that stops on the
   first failure: bump check, build-assets, dist sync, build:static, cargo test (no new FAILED),
   cargo publish, four pushes, kapable-push-verify, then wait for a marker on the live site.
   Must refuse on a dirty tree and never stage docs/night-6-live-events.jsonl.
3. Item 2, screenshot regression: ~20 gallery routes × light/dark × 1440/390, diffed against the
   previous release with `scripts/pixel-diff.py`; build on `tests/chrome-cdp.mjs` (a CDP harness
   already in the repo). Wire into release.sh as a gate that prints the worst pages.
4. Item 6, design-system linter modelled on shadcn/lint (rules: no-restyle, no-raw-colors,
   no-arbitrary-values, no-inline-styles, no-unknown-classes, require-static-classes). shadcn/lint
   reads Tailwind v4 class strings in TSX only, so ours reads maud `html!` markup (Rust) and the CSS.
   Errors must name the fix ("use Size::IconSm28, not a width override"). Wire into cargo test or release.sh.
5. Item 4, motion: dialog close (open already animates), menus, popovers, the More dropdown,
   toasts. Same approach as `static/behaviors/rail_motion.js`; reduced-motion keeps instant.
6. Item 5, light-mode and phone-width QA, using item 2's screenshots. Light values of the Linear
   recipe were inferred, not measured (e.g. `--mui-border-divider` light).
7. Then: bump the four apps to the latest release (only the conductor visibly changes).

## OPEN

- None needing the operator yet.
