# Releasing maud-ui

Publishing is **irreversible**. crates.io versions can be yanked but never replaced, so every
mistake below ships forever. Each rule here exists because it was nearly (or actually) shipped.

## Current release workflow (0.19.x, verified 2026-09-22/23 across six releases)

`static/` holds the current bundles. `dist/` and `public/` are copies that parity tests compare
against it, and `public/` is what the WEBSITE serves. The site is a static export: pushing code
rebuilds its container in about four minutes, but the pages only change if `public/` was
regenerated and committed. Until 0.19.1 the export copied a year-old stylesheet from `dist/`.

First bump `Cargo.toml`, write the newest `CHANGELOG.md` release entry, and commit those
changes. Then run `scripts/release.sh`:

```bash
scripts/release.sh --marker '<text new in this release>' [--marker-in <site path>] [--dry-run] [--visual-ok]
scripts/release.sh --help
```

Choose literal text new in this release. By default it must appear in the minified stylesheet; for a release that changes no CSS (a JS or page fix) pass `--marker-in` with the page that carries it, e.g. `--marker-in /` and `--marker 'maud-ui 0.20.3 ·'` (the home page footer shows the version). A path ending in `/` means its `index.html`. The script refuses a marker the live site already serves. The
script requires a clean working tree except for `docs/night-6-live-events.jsonl`, which it
excludes from staging, and a version newer than crates.io with a matching first changelog entry.
It rebuilds `static/`, copies the bundles to `dist/`, checks the marker, regenerates `public/`,
and gates the release on tests, clippy, and `cargo publish --dry-run --allow-dirty`.

After regenerating `public/` it serves that folder locally and screenshots about 20 pages in
light and dark at 1440 and 390 px against https://maudui.herman.engineer (the previous release),
using `scripts/visual-check.mjs` and the routes in `tests/visual-routes.txt`. More than 16 changed
pixels on any page, or a click or focus that changes a page's height, stops the release and prints
a side-by-side report. Open it; if every change is intended, re-run with `--visual-ok`. Captures
are deterministic (80 of 84 pages were pixel-identical to the live site on 2026-09-23), so a lost
breadcrumb "/" (about 90 pixels) is enough to stop it. `tests/design_lint.rs` runs inside
`cargo test` and fails on new raw colours, inline styles, raw pixel values, runtime-built class
names, cross-component restyles, or `mui-*` classes no stylesheet defines; its counts in
`tests/design-lint-baseline.txt` only go down (`DESIGN_LINT_UPDATE=1 cargo test --test design_lint`).

With `--dry-run`, it stops after those checks and shows `git status --short`; generated files
remain for review, but nothing is committed, published or pushed. Commit or remove those
changes before another run to satisfy the clean-tree preflight.

Without `--dry-run`, it commits regenerated assets, publishes the crate, pushes all four
targets, verifies the deploy fired, and polls the website every 20 seconds for up to 15 minutes.
Two consecutive marker matches are required because the site's two serving lanes can answer
from different builds during a switchover. Finally, it confirms crates.io's `max_version`.
Publishing is **irreversible**. If a later step stops, inspect the reported state and finish the
remaining steps manually; a published version cannot pass a fresh run's version preflight.

Apps that depend on maud-ui (kapable-kaps, kapable-backlog, kv2-pulse on its `master` branch,
claude-conductor on the forgejo remote) do not move with a release; each pins a version.

## 0.8 asset contract

0.8 consumers serve `maud_ui::assets::{CSS_MIN, JS_MIN}` or vendor **static/**. Rebuild with `node examples/build-assets.mjs`, `cargo run --example build_docs`, and `node examples/build-social-card.mjs` (librsvg). The example server serves those complete assets. The legacy **dist/** and **public/** snapshots stay at 0.7; the older website export command below does not constitute a 0.8 website build. Migrate the exporter to the new bundles and all 13 block routes before publishing the website. No publication or git write was performed by the 0.8 curation task.

Require both `cargo test` and `cargo clippy --all-targets -- -D warnings`, plus the Node curation handler and contrast checks documented in testing.md. Every new Markdown page has committed generated HTML checked for freshness.

## Historical release workflow (through 0.7)


```bash
# 1. Regenerate the artifacts. They are built by DIFFERENT commands.
bun run build          # → dist/    (what the CRATE ships)
bun run build:static   # → public/  (what the WEBSITE serves)
bun run build:og       # → assets/  (og.png + apple-touch-icon.png) — only needed when
                       #            assets/og-source.html or the mark changed. The card
                       #            RASTERISES the component count, so a release that adds
                       #            a component without re-rendering ships a link preview
                       #            that understates the library. cargo test catches it:
                       #            og_card_component_count_matches_reality.

# 2. Gates.
cargo test             # includes the parity guards; see below

# 3. Bump the version + write the CHANGELOG entry. Commit.

# 4. Inspect what will actually ship — BEFORE anything permanent.
cargo package --list | grep -v '^\(src/\|css/\|dist/\|assets/\|docs/components/\|examples/\|README\|LICENSE\|Cargo\)'
#    ^ anything printed is a stowaway. Investigate before continuing.

cargo publish --dry-run

# 5. Publish, THEN tag.
cargo publish
git tag -a v0.4.1 -m "..." && git push github v0.4.1 && git push forgejo v0.4.1
```

## Rules, and the incident behind each

**Bump the version if ANY file inside `Cargo.toml`'s `include` changed since the last tag.**
`v0.4.0` was tagged and left unpublished; 17 included files then changed. Publishing "0.4.0" at
that point would have permanently shipped bytes its own tag disagreed with. It became 0.4.1
instead. Check with:

```bash
git diff --name-only vX.Y.Z..HEAD | \
  grep -E '^(src/|css/|dist/|assets/|docs/components/|examples/showcase.rs|README.md|LICENSE|Cargo.toml)'
```

**Tag AFTER the publish succeeds, never before.** A tag is a claim that a version exists; make it
true first. The tagged-but-unpublished `v0.4.0` is exactly what this rule prevents.

**Entries in `include` are GLOBS, not paths.** A bare `"README.md"` matched *nested* readmes and
pulled `node_modules/esbuild/README.md` into the package. Root-level entries must be anchored:
`"/README.md"`, `"/LICENSE"`, `"/Cargo.toml"`. Caught by reading `cargo package --list` before
publishing; 289 files is the current correct count, and the stowaway check in step 4 re-verifies
it. `registration_parity::cargo_include_entries_are_anchored_or_explicit_globs` now rejects bare
filenames outright — on its first run it found a second one the by-hand fix had missed.

**`dist/` and `public/` are regenerated by different commands.** `js/build.mjs` rebuilds `dist/`;
`js/export-static.mjs` rebuilds `public/`. Running only the first ships a repo that is
simultaneously correct (crate) and stale (website) — the build passes, the tests pass, the diff
looks complete, and the deployed site serves the previous build. `cargo test` now guards this:
`registration_parity::static_export_css_matches_the_built_bundle` fails if they diverge.

**The component count is asserted, not trusted.** `Cargo.toml`'s `description` and `README.md`
both hardcode it, and `registration_parity::published_component_count_matches_reality` checks both
against `COMPONENT_NAMES.len()`. If you added a primitive, that test tells you before crates.io
does.

## Authentication

`cargo publish` needs a valid crates.io token. `403 Forbidden: authentication failed` from
**cargo** means the token is expired or revoked. Confirm it is not an ownership problem by
checking the crate's owners (below) rather than assuming either way.

```bash
# Get a token: https://crates.io/settings/tokens
cargo login          # paste at the prompt; never commit it, never inline it
```

Verify ownership separately — note the User-Agent, see below:

```bash
curl -s -A "maud-ui-release-check" https://crates.io/api/v1/crates/maud-ui/owners \
  | python3 -m json.tool | head -20
```

## After publishing

**The crates.io API rejects requests with no User-Agent** — it answers `403` with an
"API data access policy" body, which is easy to misread as "this crate does not exist". That
misread happened twice in one session: once concluding the name was unclaimed (it was published
in April, with 184 downloads), and once concluding a just-succeeded publish had not landed. Always
send a UA:

```bash
curl -s -A "maud-ui-release-check" https://crates.io/api/v1/crates/maud-ui \
  | python3 -c "import json,sys; print(json.load(sys.stdin)['crate']['max_version'])"
```

A `403` from that endpoint is a *client* problem until proven otherwise. Check the response body
before drawing any conclusion about the crate.

- docs.rs builds on its own; check it a few minutes later.
- The **website** is a separate deploy — see below. Publishing the crate does not update it.

## The website is not the crate

`maudui.herman.engineer` is a Kapable customer app that deploys from a **third** remote:

| Remote | Branch | Purpose |
|---|---|---|
| `github` | `master` | canonical source |
| `forgejo` | `master` | internal mirror |
| `customer` | **`main`** | **what actually deploys the website** |

Note the branch name differs. Pushing `master` to `customer` creates a dead branch and deploys
nothing:

```bash
git push customer HEAD:main   # correct
```

The deploy takes several minutes and replaces the container, so there is a window where the proxy
briefly points at nothing. **Do not diagnose during that window** — sample twice with a gap before
concluding anything is broken. Verify the end state instead:

```bash
curl -s https://maudui.herman.engineer/ | grep -o '[0-9]* components'
```
