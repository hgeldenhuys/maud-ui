---
verified_on: 2026-09-23
covers:
  - "Cargo.toml"
  - "src/showcase/**"
  - "public/**"
verify_with: "curl -fsS https://maudui.herman.engineer/css/maud-ui.min.css | head -c 200"
---
# Where maud-ui is served

- crates.io: https://crates.io/crates/maud-ui (0.20.2 on 2026-09-23).

- Gallery and docs: https://maudui.herman.engineer. It deploys from `customer/main` on
  git.kapable.dev about four minutes after a push. It only changes when a release regenerates `public/`.

- Mirrors: github.com/hgeldenhuys/maud-ui `master` and git.kapable.dev/kapable/maud-ui `master`.

Each app that depends on it pins a version, so a release does not move it:

- kapable-kaps and kapable-backlog: crates.io `=0.20.0`. Backlog also keeps a copy of the
  minified CSS in its own `src/assets/`.

- kv2-pulse: crates.io `=0.20.0`. Its build runs from `master` but its deploy fetches `main`,
  so push both (board card R-071).

- claude-conductor: crates.io `=0.20.0` plus copied bundles in `src/htmx_ui/assets/`. Its trunk
  is the `forgejo` remote (the clone at ~/WebstormProjects/claude-code-sdk).
