# Autoagent Session — 2026-04-19 — shadcn Base UI Parity

**Started:** 2026-04-19T02:10:02Z
**Ended:** 2026-04-19 ~04:20Z
**Duration:** ~130 minutes (of 180 budget)
**Result:** Landed gracefully with v0.2.1 tagged.

## What shipped

40 commits on top of v0.2.0 → v0.2.1.

- **5 new primitives** (59 → 64): Sheet, Sidebar, Sonner, Item, Direction.
- **35+ primitives enhanced** with shadcn Base UI Props/helper parity.
- **1 SECURITY fix**: `input::render()` XSS hardening via maud `html!` macro + 3 unit tests.
- **10+ a11y refinements**: `aria-modal`, `aria-orientation`, `aria-haspopup`,
  `aria-label`, role corrections, double-announce fixes.

See `CHANGELOG.md` `[0.2.1]` for full detail.

## Process metrics

- Rows from gap-list (`.autoagent/maud-ui-shadcn-audit/gap-list.md`):
  - 81 actionable rows identified up-front.
  - ~60-65 explicitly implemented across commits.
  - Remaining rows are mostly nuanced P2 a11y refinements or already-aligned
    items the audit flagged for verification (e.g. spinner already emits
    correct roles — row was a verify-then-mark candidate).
- Baseline clippy errors with `-D warnings`: 36 pre-existing (not blocking the
  publish path). Ceiling held throughout — every commit audited against the
  ceiling before landing.
- 3 unit tests added (all in `input.rs` for the XSS hardening).
- Verify gate enforced: `cargo check --all-targets` pass + clippy-delta-≤0
  per commit.

## Files NOT in the default Cargo include

`.autoagent/`, `.claude/autoagent-state.md`, `results.tsv`, `evals/*` —
intentionally excluded from the published artifact via the existing
`include` list in `Cargo.toml`.

## Commit composition

Every commit:
- Is additive (no Props renames, no enum variant removals).
- Covers one primitive section OR one cascade-bundle when a Props field
  addition forced consumer updates in block/*.rs or card.rs.
- Carries `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`
  trailer.
- References the shadcn docs URL in the body.
- References the gap-list row number/section.

## Resume / next session

If picking this up again:

1. Read `.autoagent/maud-ui-shadcn-audit/gap-list.md` — rows with `[ ]`
   are genuinely uncovered or deferred. Rows with `[x]` are done.
2. Breaking-proposals list (deferred to v0.3.0+) — not yet written;
   rows marked `[BREAKING]` in the gap list describe them.
3. Consider publishing v0.2.1 to crates.io once docs.rs builds clean.
   Publish command: `cargo publish --allow-dirty` (the `--allow-dirty`
   is only needed because `node_modules/` is untracked locally; the
   `include` list scopes the published artifact cleanly).
4. README has a `58 components` claim inside a `<details>` block
   describing the component tiers — update that count if you add more
   primitives. (Not updated this session because the tier-by-tier
   listing is out-of-date already pre-0.2.1.)

## Hindsight notes

- **Parallel subagents were productive but talky.** Each batch of 3-4
  parallel agents could produce 600+ LOC across 4 primitives in 2-3
  minutes of wall-clock, but each agent independently worried about
  concurrent edits and wasted some context on that. A tighter prompt
  that says "you are one of N parallel workers, your file list is
  disjoint, DO NOT stash/reset" cut that chatter in later batches.
- **Cascade handling was the main friction.** Adding optional Props
  fields to primitives like `badge::Props` or `switch::Props` broke
  every struct-literal call site in blocks/ and showcase/mod.rs that
  didn't use `..Default::default()`. Pre-migration of these call
  sites (in a dedicated "prep" commit) would have let every P1/P2
  commit be single-file.
- **fmt drift at the start wasted 1 commit.** The repo shipped v0.2.0
  with pre-existing fmt drift. The first subagent's `cargo fmt --all`
  auto-cleanup normalized 24 files into a separate `chore: cargo fmt`
  commit. Better: run `cargo fmt --all` as an explicit preflight
  before any feature work.
