# maud-ui

Rust component library (maud + htmx) that every Kapable app renders with. The design bar is Linear.

Read first:
- `docs/product/` for what it is, how it is built, where it is served, and what is left.
- `docs/releasing.md` before any release. Release only with `scripts/release.sh`, never by hand.
- `.claude/maud-ui-improvements-state.md` for the current arc and the owner's rulings.

Checks that must stay green: `cargo test --no-fail-fast` (includes the design linter
`tests/design_lint.rs`), `cargo clippy --all-targets -- -D warnings`, and with the gallery running
(`scripts/gallery.sh`): `node tests/overlay-motion-browser.mjs http://localhost:3456` and
`node tests/palette-browser.mjs http://localhost:3456`.

To show one change: `node scripts/compare-element.mjs <route> <selector>`. To find every page that
changed: `node scripts/visual-check.mjs --candidate http://localhost:3456`.

Never stage `docs/night-6-live-events.jsonl`: another session writes it.
