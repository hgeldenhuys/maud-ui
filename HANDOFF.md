VERIFIED 2026-09-23 12:30 EDT by crates.io API + `git rev-parse`: maud-ui 0.19.5 is the published version and HEAD of branch curation-2026-09-08 matches customer/curation-2026-09-08.
VERIFIED 2026-09-23 by `cargo test --no-fail-fast`: 304 tests pass, exactly 6 fail, all six are the `time_split` component never being fully registered.
VERIFIED 2026-09-22/23 by live probes: the four Kapable apps (kaps, backlog, pulse, conductor) serve 0.19.3; none of the six arc items is started.

````
Start with the arc Herman ratified on 2026-09-23: six maud-ui improvements, in the order below.

## Intent
maud-ui is the component library every Kapable app renders with. Herman's bar is Linear:
professional, calm, nothing janky. Tonight's bugs (a missing breadcrumb "/", harsh focus boxes,
unstyled selects, tab jank) were all found by Herman LOOKING. "Good" for this arc means the
library catches its own regressions before he sees them: a green suite, a one-command release
that refuses a bad build, screenshot diffs, and a linter that tells an agent the fix.

## Project
Rust crate `maud-ui` (maud + htmx components, shadcn-style) at /Users/hgeldenhuys/WebstormProjects/maud-ui,
branch curation-2026-09-08. Gallery/website: https://maudui.herman.engineer (a static export).

## Read first (each checked to exist this ceremony)
- /Users/hgeldenhuys/WebstormProjects/maud-ui/.claude/maud-ui-improvements-state.md — NEXT_ACTION, the ranked plan with reasons.
- /Users/hgeldenhuys/WebstormProjects/maud-ui/docs/releasing.md — section "Current release workflow (0.19.x)": the exact release steps, run six times.
- /Users/hgeldenhuys/WebstormProjects/maud-ui/CHANGELOG.md — entries 0.18.0 to 0.19.5 say what changed and why.

## Where things stand
Everything is committed and pushed (maud-ui 6cfa83d on customer, github/master, forgejo/master).
The only dirty file, docs/night-6-live-events.jsonl, belongs to another session: never stage it.

## Mission: item 3, make `cargo test` fully green
Done when `cargo test --no-fail-fast` prints no FAILED line. Fix in this order, because 1–3 are
the component's registration and 4–6 are counts that only pass once it is registered:
1. Create docs/components/time_split.md (new file; copy a sibling doc's shape).
2. Add the time_split `include_str!` arm in src/showcase/docs.rs, then `cargo run --example build_docs`.
3. Add time_split to `assert_showcase_renders!` in tests/render_tests.rs.
4. Move "time_split" into alphabetical place in COMPONENT_NAMES (src/showcase/mod.rs; it sits after "meter").
5. Cargo.toml description "83 headless" → "84 headless"; README.md lines 3 ("83 headless") and 25 ("83 primitives") → 84.
6. static/og.svg must state 84; then `node examples/build-social-card.mjs` re-renders static/og.png.

Then take the next items without waiting, in this rank (reasons in the state file):
item 1 a new scripts/release.sh → item 2 screenshot regression (build on tests/chrome-cdp.mjs and
scripts/pixel-diff.py) → item 6 design-system linter (modelled on github.com/shadcn-ui/lint's six
rules, but reading maud html! markup and CSS, since shadcn/lint only reads Tailwind in TSX) →
item 4 motion for dialog close, menus, popovers, the More dropdown and toasts (copy
static/behaviors/rail_motion.js's approach) → item 5 light-mode and phone-width QA → bump the
four apps. Show Herman a screenshot early in items 2, 4 and 5.

## Settled; do not redo
- The Linear theme is the default (0.18). The sidebar collapse motion (0.19.5) and brand fonts (0.19.4) are done.
- Apps pin exact versions; a release does not move them. kv2-pulse deploys from its `master` branch; claude-conductor's trunk is the `forgejo` remote (its GitHub `origin` is 424 commits behind).

## Traps, each with its tell
- A release looks live but the site shows old CSS: the site is a static export, so `bun run build:static` must run and public/ must be committed. Tell: the container reports the new commit while https://maudui.herman.engineer/css/maud-ui.min.css lacks this release's new rule.
- Mid-deploy the page and its stylesheet come from different builds. Tell: the page's `maud-ui.css?v=` number differs from the stylesheet's byte size. Sample twice.
- A test that checks a string can pass on a comment. Tell: prove-red says "still PASSES" when you break the code. Run `prove-red` on every new test.
- New breakpoints must come from `tokens::breakpoints`. Tell: `every_media_width_is_a_declared_breakpoint` fails naming the width.

## Not true / not proven
- Light-mode values of the Linear recipe were inferred, not measured.
- Nothing was checked at phone width in this session.
- The linter does not exist; nothing like it is in the repo (checked for a lint script, none).

## Operator mechanics
Run `/start-sprint resume maud-ui-improvements` from the repo, then:

```bash
cd /Users/hgeldenhuys/WebstormProjects/maud-ui && git fetch -q customer && test "$(git rev-parse HEAD)" = "$(git rev-parse customer/curation-2026-09-08)" && test "$(curl -s -A maud-ui-handoff https://crates.io/api/v1/crates/maud-ui | jq -r .crate.max_version)" = 0.19.5 && echo GROUND-OK
opctl init maud-ui-improvements --state "/Users/hgeldenhuys/WebstormProjects/maud-ui/.claude/maud-ui-improvements-state.md"
opctl done-when "cargo test --no-fail-fast in maud-ui prints no FAILED line"
```

The ground line exits non-zero if HEAD moved or a newer version was published; then re-read the
state file before trusting anything here. Read `/staff` before the first dispatch: grunt work
(docs, the release script, screenshot sweeps) goes to builders, and review goes to another model
family. You are the co-founder seat: decide what is worth doing and whether it really happened.
Builders' reports arrive via `opctl inbox`. Release with docs/releasing.md until item 1 lands.
````
