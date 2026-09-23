---
arc: maud-ui-improvements
repo: /Users/hgeldenhuys/WebstormProjects/maud-ui
branch: curation-2026-09-08 (pushed to customer; website deploys from customer/main, mirrors github/master + forgejo/master)
ratified: 2026-09-23 by Herman ("let's do all 5, after we /wrapoff", plus #6 the linter)
seat: claude-opus-5-5
---

# maud-ui improvement arc

## POSITION (2026-09-23 12:30 EDT)

maud-ui 0.19.5 is on crates.io and live at https://maudui.herman.engineer. The four Kapable apps
(kapable-kaps, kapable-backlog, kv2-pulse, claude-conductor) run 0.19.3. Six items were ratified;
none is started. Release procedure as run six times: `docs/releasing.md` → "Current release workflow".

## NEXT_ACTION

Item 3: make `cargo test` fully green. All six failures are `time_split` never being fully
registered (measured 2026-09-23 from `cargo test --no-fail-fast`):
1. `docs/components/time_split.md` missing → write it (copy the shape of a sibling doc).
2. `src/showcase/docs.rs` has no `include_str!` arm for time_split → add one, then
   `cargo run --example build_docs` to render `docs/components/rendered/time_split.html`.
3. `tests/render_tests.rs` `assert_showcase_renders!` lacks time_split → add it.
4. `COMPONENT_NAMES` in `src/showcase/mod.rs` is not alphabetical (time_split sits after meter) → move it.
5. `Cargo.toml` description says "83 headless" → "84 headless"; README.md lines 3 and 25 say 83 → 84.
6. `static/og.svg` must state 84 → edit, then `node examples/build-social-card.mjs` re-renders og.png.
Done when `cargo test --no-fail-fast` prints no `FAILED` line.

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
