---
arc: maud-ui-improvements
repo: /Users/hgeldenhuys/WebstormProjects/maud-ui
branch: curation-2026-09-08 (pushed to customer; website deploys from customer/main, mirrors github/master + forgejo/master)
ratified: 2026-09-23 by Herman ("let's do all 5, after we /wrapoff", plus #6 the linter)
seat: claude-opus-5-5
---

# maud-ui improvement arc

## POSITION (2026-09-23 13:35 EDT)

Items 3, 1, 2 and 6 LANDED on the branch (unreleased; 0.19.5 is still the latest on crates.io).
- Item 3: cargo test 321 pass / 0 fail; clippy -D warnings clean.
- Item 1: scripts/release.sh (Astra built; I fixed FORCE_COLOR=3 making node colour the version
  comparison, which refused every release). Dry run PASSED end to end on a throwaway worktree
  bumped to 0.19.6 ($CLAUDE_JOB_DIR/tmp/wt-release; never pushed).
- Item 2: scripts/visual-check.mjs + tests/visual-routes.txt (21 routes x light/dark x 1440/390 =
  84 captures) vs the LIVE site; gate = >16 changed pixels or an action that resizes the page.
  Proved: breadcrumb "/" hidden (90-97 px), select unstyled, dark focus box, tab-panel resize all
  FAIL; clean runs 0 px (tabs 3 runs in a row). Fixes I made: absolute pixel count (a % threshold
  passed the breadcrumb bug at 100.00%), focus emulation (headless never painted :focus), DOM click
  (mouse click scrolled sticky header differently per run), MAUD_UI_PYTHON (Homebrew python3 has
  no Pillow). Wired into release.sh as step 5b; --visual-ok after reading the report.
- Item 6: tests/design_lint.rs (GLM built) + baseline ratchet (457 findings frozen: raw colours 257,
  unknown classes 78, raw px 77, inline styles 39, runtime classes 6). Proved fails on a planted
  unknown class, inline style and raw colour. I fixed its #[cfg(test)] skip (only matched `mod tests`).
- Screenshot of the breadcrumb catch SENT to Herman 13:30.
- Reviews: K3 is OUT of weekly quota (403); Grok reviewing release.sh (in flight 13:12).

## ITEM 4 + REVIEW (2026-09-23 14:00 EDT) — LANDED 6c2413a, 23d7087, pushed to customer branch
- Overlays fade out on close (Astra built): dialog family, menus, context menu, menubar, popover,
  hover card, action-row More; toast already had it. tests/overlay-motion-browser.mjs: 17/17 PASS on
  the gallery, every non-toast case FAILS on the 0.19.5 live site. I fixed: test awaited its own
  setup promise (never pressed Escape); dialogs lost column layout mid-fade (layout lived under
  [open]; seen frame by frame, now asserted: size must stay within 3% while closing).
- Taste call (mine): popover returns focus to its button on Escape; context menu to what had focus
  before it opened. Hover card and toast are exempt from the focus-return check (no trigger contract).
- Grok (cross-family) FAILED release.sh with 9 findings; all 9 fixed in 6c2413a: push dry-runs
  BEFORE publish, "do not bump" message after a post-publish push failure, stage only build output,
  marker must not already be live + uncached fetch, no --max-time 0, trap cleared after the server
  stops, server identity check by stylesheet bytes, new pages need review, deadline edge.
- Full hardened dry run PASSED on $CLAUDE_JOB_DIR/tmp/wt-release2 (local branch
  rehearsal-0.19.6-never-push; never pushed). Motion screenshot SENT to Herman 14:00.
- No CHANGELOG entry yet for the unreleased work: the next release's entry must list items 1-4, 6
  (release.sh requires the first `## [` heading to be the new version, so no "Unreleased" section).

## ITEM 5 + RELEASE (2026-09-23 14:20 EDT)
- Item 5 LANDED 23b0691: 8 fixes (light accent 3.91->4.7:1, combobox nested-button escape, popover
  collision incl. a 390px fallback that first landed 2,126px off-screen and was re-fixed, team +
  data table scroll at 390, light surfaces, switch off-track, native select). All measured in a
  browser. visual-check now compares at tolerance 2 (5db027f): at 24 the accent change was invisible.
- **maud-ui 0.20.0 RELEASED 14:18 by scripts/release.sh in one run** (781d0b9): crates.io
  max_version 0.20.0; live site serves `mui-overlay-exit` (2 uncached samples); 4 pushes; herald
  repo.push seen. The 52 changed captures were reviewed first (visual-run4 + element crops).
- Screenshots SENT to Herman: breadcrumb catch (13:30), dialog close frames (14:00), item 5
  before/after sheet (14:15).

## ITEM 7 DONE (2026-09-23 14:47 EDT) — all four apps LIVE on maud-ui 0.20.0
- kapable-kaps a9d8019 (main): CI build 21745aca + deploy 603ac036 green; live CSS sha = 0.20.0 min.css.
- kapable-backlog b69d01ecf (kv2-platform main): build 0b945985 + deploy 3dce1bfc green; marker x4;
  index list took a few minutes to fill after restart (filled by 14:46; scanned 18:28Z).
- kv2-pulse 7090865: pushed master; ⚠ the DEPLOY recipe (ci-kv2-pulse) fetches `main` and refuses a
  sha main lacks, so the first deploy 5345b5c4 FAILED; main fast-forwarded to 7090865 (no force),
  deploy cac91cab green. Every Pulse bump hits this until the recipe fetches master (NOT fixed).
- claude-conductor 406c31f0 (forgejo main; the forgejo clone is ~/WebstormProjects/claude-code-sdk,
  NOT ~/tapestry-comms/transcript-tui-rs which has only GitHub origin): pin crates.io =0.20.0 +
  vendored min bundles; deployed --all-orgs, md5 gate passed on 3115-3118 + 3120; marker x2 on all.
  No visible change (conductor sets its own theme colours).
- Screenshots: $CLAUDE_JOB_DIR/tmp/{kapable-kaps,kapable-backlog,kv2-pulse}-0.20.0.png, conductor-ui-{before,after}.png.

## 0.20.1 (2026-09-23 16:25 EDT) — RELEASED by scripts/release.sh (56554c6)
- Herman: "spotlight ... doesn't close on esc or click outside, it shows again". Was the GALLERY
  search palette opened from the header box (Kapable app switchers were fine): closing a modal
  dialog hands focus back to the box BEFORE the close event, so the box's focus handler reopened
  it. Fixed 6f90c23; tests/palette-browser.mjs (fails on 0.20.0, passes live on 0.20.1); release.sh
  step 5b now runs it + overlay-motion-browser against the export.
- Herman: "/impeccable audit" of the gallery left nav, "fix all the Ps". 16/20. Fixed 2200dd4:
  P1 left-edge stripe (gallery curved border, shell block + sidebar accent 2px bar); P2 gallery
  drifted from the library's sidebar recipe; P2 text-glyph chevrons -> drawn --mui-icon-chevron;
  P3 37px rows -> 28px; P3 uppercase tracked group labels -> sentence case (library-wide).
  Not in nav scope, left: detector's line-length (content measure) + one skipped heading level.
- Apps still pin =0.20.0: the stripe/chevron/label changes reach them only on a bump (the
  conductor uses the sidebar component; kaps/backlog/pulse use kp-* shell chrome).

## RULED
- 2026-09-23 16:09 EDT, Herman: "go" = KEEP light --mui-bg-card #f6f6f7 (item 5, 23b0691). In light
  mode app pages (kaps, backlog, pulse) are light grey with white header/dock. Do not revert.

## NEXT_ACTION

Arc complete. Remaining: /wrapoff when Herman asks. Follow-ups worth a backlog item: kv2-pulse deploy
recipe fetches main (build triggers on master); the DEFERRED list below.

## FOUND FOR ITEM 5 (light mode + phone QA)
- 14:00 QA reviewer (fresh Opus, screenshots) found 28 defects; px-contrast CONFIRMED light primary
  button #FFF on #6E79D6 = 3.91:1 (dark 4.7:1). In flight: Astra fixing 8 ($CLAUDE_JOB_DIR/tmp/qa-fix-brief.md):
  light accent, combobox parts escaping trigger, controlled popover off-screen, settings-team +
  data_table clipped at 390, light surfaces flat, switch off-state invisible in light, native select.
- DEFERRED (not in this arc's fix pass; each a real defect per the reviewer, unverified by me):
  props tables (code split mid-word at 1440, empty Description cells, raw rustdoc markdown shown);
  button size ladder (xs 32px = default, taller than sm 24px); data_table "Columns" button and row
  checkboxes unstyled (likely among the 78 undefined classes); dialog demo triggers full-width grey;
  gallery GET form + home header search squeezed at 390; toast stack gap 2-3px; pager text 3.25:1;
  pagination at 390 shows only the current page; select chevron ~4px; stray 2px bar left of home's
  active nav item; auth-login double border at 390.
- NOT a defect: the "heavy ring" on the active tab in /tabs captures is the focus ring from the
  capture's own click (focus emulation paints :focus-visible).
- swatch has NO stylesheet: every mui-swatch* class is undefined (linter, 15 findings).
  78 undefined classes total: tests/design-lint-baseline.txt `no-unknown-classes` lines.
- At 390px the "<- Navigation Menu / Command ->" pager sits squeezed beside the breadcrumb
  (seen in the breadcrumb page capture).

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

- (taste, mine, 2026-09-23) Item 2 compares the LOCAL gallery against the LIVE site (= previous
  release) rather than storing baseline PNGs in git. Buys: no binary churn, the baseline can't go
  stale. Costs: needs network; an intentional visual change always shows as a diff, so release.sh
  stops at the report until a human passes `--visual-ok`. Alternative: committed baselines.
- In flight 13:30 EDT: Astra building scripts/release.sh (+ docs/releasing.md) from
  $CLAUDE_JOB_DIR/tmp/release-brief.md; a second Astra run building scripts/visual-check.mjs,
  tests/visual-routes.txt, faster scripts/pixel-diff.py. Reviews go to K3 (another family).
