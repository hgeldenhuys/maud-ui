# Autoagent State — maud-ui ↔ shadcn Base UI API alignment

**Started:** 2026-04-18
**Timebox:** 180 minutes
**Base version:** maud-ui v0.2.0 (crates.io 2026-04-18)
**Next target:** v0.2.1 (patch) or v0.3.0 (if breaking changes land)

## Input

- Gap list: `./gap-list.md` (81 actionable rows, priority P0 → P1 → P2)
- Shadcn Base UI docs root: https://ui.shadcn.com/docs/components/base/
- maud-ui source: `/Users/hgeldenhuys/WebstormProjects/maud-ui/src/primitives/`

## Scope & guardrails

**ADDITIVE ONLY.** Do not rename Props fields, do not restructure existing Props. Everything is a v0.2.1-compatible patch.

**Allowed changes:**
- ADD new primitive files (`src/primitives/<name>.rs`) + register in `mod.rs` + register in `src/showcase/mod.rs` sidebar constants
- ADD new helper fns (`pub fn action`, `pub fn media`, etc.) on existing primitive modules
- ADD new variants to existing enums (`Variant::Ghost`, `Size::Xs`) — new variants are NOT breaking because consumers match on `_ =>` or construct with field-name spread
- ADD new Props fields — MUST be optional with `Default` impl so existing callers still compile
- ADD new ARIA attributes to render output
- ADD new CSS rules in the corresponding `css/primitives/<name>.css` file
- ADD new JS behaviours under `dist/behaviors/<name>.js` (+ bundle into `maud-ui.min.js` via `js/build.mjs`)

**Forbidden changes (log to `breaking-proposals.md` if encountered):**
- Renaming existing variants or fields
- Removing variants
- Changing default values in a way that changes rendered output for existing callers
- Anything that breaks `cargo check --workspace` for downstream users

## Order of execution

Work items in this order:

1. **P0 primitives** (Sheet, Sidebar, Sonner) — each is a new file, may be ~150-300 LOC. Commit one per primitive.
2. **P1 primitives** (Item, Direction) — smaller.
3. **P1 subcomponent/variant adds** — one commit per row in gap-list.md, in the order they appear.
4. **P2 a11y / polish** — same cadence; skip any P2 that requires browser verification (we're --skip-chrome).

If a row looks wrong on re-read, skip it and note in `skipped.md` — do not fabricate APIs.

## Commit convention

Every commit:
- Subject: `feat(<area>): <what was added>` (e.g. `feat(sheet): add new slide-out panel primitive`)
- Body: reference shadcn URL that motivated the change, 1-line diff summary, the gap-list.md row
- Trailer: `Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>`
- Branch: stay on `main`
- Push cadence: push after every 3 commits (or at landing)
- Remote: `forgejo` (NOT origin — refer to parent kapable CLAUDE.md remotes section)

## Verify step (between diagnose → fix → commit)

**Baseline note (captured 2026-04-19T02:10Z):** repo has pre-existing fmt drift and 36 clippy errors under `-D warnings`. Published v0.2.0 carried these because the publish path doesn't enforce `-D warnings`. Do NOT block on them. Ceiling is 36 — if a change raises it, fix the new warnings inside the same commit.

1. `cargo check --all-targets` — MUST pass (hard gate).
2. `cargo test` — MUST pass if the change introduces new tests or touches a tested path.
3. `cargo clippy --all-targets -- -D warnings 2>&1 | grep '^error' | wc -l` — MUST be ≤ 34 (baseline). If it rises, the change introduced new warnings — fix them inside the same commit.
4. `cargo fmt --all` — on the modified files only. Don't reformat the whole tree in an unrelated commit.

Pre-existing hygiene issues (34 clippy errors, fmt drift) are out-of-scope for this loop.

## State updates — NEXT_ACTION pattern

After every phase transition, append a `## NEXT_ACTION` block at the bottom of THIS file with the exact next tool call. Read this file FIRST on wake-up after any compaction or context loss.

## Landing conditions

1. 180 minutes elapsed since start timestamp (see top of file) → write final summary, commit WIP if any, land.
2. All P0 + P1 rows in gap-list.md marked `[x]` → graceful land, write v0.2.1 release summary to `release-0.2.1.md`.
3. Context >45% used → commit WIP, update this file with resume position, land.
4. Stuck — same commit failed 3x → mark row `[blocked]`, move to next row. If all remaining rows blocked, land.
5. `cargo check` has persistent failures from a prior commit we can't revert → land and file issue.

## Progress log

_append one line per commit here; scrub on landing to produce a summary_

| time (mm:ss) | row | commit | sha |
|--------------|-----|--------|-----|

## NEXT_ACTION

On autoagent wake-up:
1. Read `./gap-list.md` and find the first unchecked `[ ]` row (starting from P0).
2. Delegate implementation to a subagent (general-purpose or Explore with write access) with a tight brief: the gap-list row, the target file path, the scope rules above, and "return when cargo check + clippy + fmt pass".
3. On return, run verify step locally (supervisor's cargo).
4. Commit with the convention above.
5. Append a row to the progress log.
6. Update gap-list.md — change `[ ]` to `[x]` for the row just completed.
7. Repeat.
