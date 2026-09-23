---
verified_on: 2026-09-23
covers:
  - "static/**"
  - "examples/build-assets.mjs"
  - "js/export-static.mjs"
  - "scripts/**"
  - "server.ts"
verify_with: "node examples/build-assets.mjs --check"
---
# How maud-ui is built and released

Styles live in `static/styles/` and behaviours in `static/behaviors/`. The command
`node examples/build-assets.mjs` bundles them into `static/maud-ui.{css,js}` and minified
copies.

`dist/` and `public/` hold copies that tests compare against `static/`.

The gallery is `examples/showcase.rs`. The website is a static export: `bun run build:static`
starts the showcase, fetches every route into `public/`, and `server.ts` serves that folder.

A release is one command, `scripts/release.sh`. It refuses a dirty tree or an unbumped
version.

It rebuilds, then screenshots 21 pages against the live site (`scripts/visual-check.mjs`)
and runs the overlay and palette browser tests. 

Then it runs `cargo test` (which includes the
design linter, `tests/design_lint.rs`), clippy and a publish dry run. Only then does it
commit, publish, push four remotes and wait for the live site to serve a marker twice.
See `docs/releasing.md`.
