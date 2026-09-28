# css/ is the pre-0.9 style tree: it does not reach the bundle

Edit component styles in `static/styles/components/*.css`. `node examples/build-assets.mjs`
builds `static/maud-ui{,.min}.css` from `static/styles/maud-ui.css` only; nothing here is read
(docs/product/01-ARCHITECTURE.md). A rule added here passes `cargo test` and never ships:
0.21.0's first release attempt stopped at the marker check for exactly that (2026-09-28).
