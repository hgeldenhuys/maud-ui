VERIFIED 2026-09-23 12:30 EDT by crates.io API + `git rev-parse`: maud-ui 0.19.5 is the published version; branch curation-2026-09-08 HEAD equals customer/curation-2026-09-08 (nothing unpushed).
VERIFIED 2026-09-23 by `cargo test --no-fail-fast`: 304 tests pass and exactly 6 fail; each of the six failure messages names `time_split` or the 83-vs-84 component count.
VERIFIED 2026-09-22/23 by live stylesheet probes: the four Kapable apps (kaps, backlog, pulse, conductor) serve maud-ui 0.19.3. VERIFIED 2026-09-23 by `ls`: scripts/release.sh and docs/components/time_split.md do not exist, so items 1 and 3 are not started.

````
First action: run this verbatim. It checks the branch, that HEAD equals the fetched customer ref, crates.io's latest version and that the state file exists, then prints the working-tree status. It does not re-run the tests or re-probe the apps. On failure it stops at the failing check: read the state file before continuing.

```bash
(
  set -eu
  cd /Users/hgeldenhuys/WebstormProjects/maud-ui
  test "$(git branch --show-current)" = curation-2026-09-08
  git fetch -q customer
  test "$(git rev-parse HEAD)" = "$(git rev-parse customer/curation-2026-09-08)"
  test "$(curl -fsS -A maud-ui-handoff https://crates.io/api/v1/crates/maud-ui | jq -er .crate.max_version)" = 0.19.5
  test -f .claude/maud-ui-improvements-state.md
  git status --short
  echo GROUND-OK
)
```

Then, in the Claude Code prompt (not the shell): `/start-sprint resume maud-ui-improvements`, and in the shell:

```bash
opctl init maud-ui-improvements --state "/Users/hgeldenhuys/WebstormProjects/maud-ui/.claude/maud-ui-improvements-state.md"
opctl done-when "cargo test --no-fail-fast in maud-ui prints no FAILED line"
```

## Intent
maud-ui is the component library every Kapable app renders with; Herman's bar is Linear. The arc's
test of success: the library catches its own visual and structural regressions before Herman sees
them. Judge every item by whether it would have caught one of these real regressions from
2026-09-22: the breadcrumb lost its "/", text fields focused as a hard dark box, six selects were
unstyled, a tab switch resized the page.

## Project
Rust crate `maud-ui` at /Users/hgeldenhuys/WebstormProjects/maud-ui, branch curation-2026-09-08.
Website https://maudui.herman.engineer is a static export of the gallery.

## Read first (each confirmed to exist with `ls` on 2026-09-23)
- /Users/hgeldenhuys/WebstormProjects/maud-ui/.claude/maud-ui-improvements-state.md — NEXT_ACTION and the plan.
- /Users/hgeldenhuys/WebstormProjects/maud-ui/docs/releasing.md — "Current release workflow (0.19.x)", the steps run six times.
- /Users/hgeldenhuys/WebstormProjects/maud-ui/CHANGELOG.md — entries 0.18.0 to 0.19.5.

## Mission: item 3, make `cargo test` fully green
Done when `cargo test --no-fail-fast` prints no FAILED line. Every path below was checked with `ls`
on 2026-09-23 and exists, except the one marked NEW. Fix in this order, because 1–3 register the
component and 4–6 are counts that only pass once it is registered:
1. Create docs/components/time_split.md (NEW; copy a sibling doc's shape).
2. Add a time_split `include_str!` arm in src/showcase/docs.rs, then `cargo run --example build_docs`.
3. Add time_split to `assert_showcase_renders!` in tests/render_tests.rs.
4. Move "time_split" to its alphabetical place in COMPONENT_NAMES in src/showcase/mod.rs (it sits after "meter").
5. Cargo.toml description and README.md lines 3 and 25 say 83; make them 84.
6. static/og.svg must state 84; then `node examples/build-social-card.mjs` re-renders static/og.png.

## Then, without waiting, in this order (the reason is the rank)
1. Item 1: create scripts/release.sh (NEW), the workflow in docs/releasing.md as one command that stops on the first failure and refuses a dirty tree. First, because every later item ships through it and it makes the green suite a mandatory gate.
2. Item 2: screenshot regression, about 20 gallery routes in light and dark at 1440px and 390px, diffed against the previous release with scripts/pixel-diff.py on top of the CDP harness in tests/chrome-cdp.mjs; wire it into release.sh. Second, because items 4 and 5 change visuals and need it to prove they broke nothing.
3. Item 6: a design-system linter for maud `html!` markup and CSS, modelled on github.com/shadcn-ui/lint's six rules (no-restyle, no-raw-colors, no-arbitrary-values, no-inline-styles, no-unknown-classes, require-static-classes), with errors that name the fix. shadcn/lint itself only reads Tailwind in TSX. Third, because it prevents defects during items 4 and 5.
4. Item 4: motion for dialog close, menus, popovers, the More dropdown and toasts, the way static/behaviors/rail_motion.js does the sidebar; reduced motion stays instant. Fourth, because it is the first item that changes what people see, so it comes after the screenshot check and the linter can catch its side effects.
5. Item 5: light-mode and phone-width QA using item 2's screenshots. Last of the six, because it judges the combined result.
6. Bump the four apps to the new release. Last, because apps should only take a release that passed items 1 to 5. INFERRED from the apps' code: only the conductor uses the sidebar collapse, so it is the one likely to change visibly; screenshot all four after the bump.
Send Herman a screenshot early in items 2, 4 and 5.

## Settled; do not redo
- VERIFIED 2026-09-23 by CHANGELOG.md and crates.io: the Linear theme is the default (0.18.0); brand fonts (0.19.4) and sidebar collapse motion (0.19.5) are published.
- VERIFIED 2026-09-22 from the live foreman recipe: kv2-pulse deploys from its `master` branch. VERIFIED 2026-09-22 by the conductor deploy's lineage gate: claude-conductor's trunk is the `forgejo` remote, not GitHub `origin`.

## Traps, each with its tell
- The site is a static export. Tell: the container reports the new commit while https://maudui.herman.engineer/css/maud-ui.min.css lacks this release's new rule; run `bun run build:static` and commit public/.
- Mid-deploy the page and its stylesheet come from different builds. Tell: the page's `maud-ui.css?v=` number differs from the stylesheet's byte size. Sample twice.
- A new test can pass on a comment. Tell: `prove-red` says the test "still PASSES" under a mutation. Run it on every new test.
- Breakpoints must come from `tokens::breakpoints`. Tell: `every_media_width_is_a_declared_breakpoint` fails naming the width.
- VERIFIED 2026-09-23 by `git status --short`: docs/night-6-live-events.jsonl is modified and was never touched by this arc (inferred: another session's log). Tell: it appears in every status; never stage it.

## Not true / not proven
- Light-mode values of the Linear recipe were inferred, not measured.
- Nothing was checked at phone width.
- No linter exists (checked: no lint file under scripts/, examples/ or js/).

## How to work
Read `/staff` before the first dispatch: writing docs, the release script and screenshot sweeps go
to builders; review goes to another model family. Rule scope and taste calls yourself and record
them in the state file's OPEN list; read builders' reports with `opctl inbox`. Release with
docs/releasing.md until item 1 lands.
````
