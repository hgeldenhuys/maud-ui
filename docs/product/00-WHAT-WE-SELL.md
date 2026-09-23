---
verified_on: 2026-09-23
covers:
  - "Cargo.toml"
  - "README.md"
  - "src/primitives/**"
  - "src/blocks/**"
verify_with: "curl -fsS https://crates.io/api/v1/crates/maud-ui | jq .crate.max_version"
---
# What maud-ui is

maud-ui is a free, MIT-licensed Rust crate: 84 accessible UI components and 31 application
blocks for server-rendered web apps built with maud and htmx. Its default look follows
Linear, in light and dark.

Who uses it today: the four Kapable apps (kaps, backlog, pulse and the conductor) and anyone
on crates.io. It is not sold. It is the look every Kapable app renders with, so a defect here
shows up in all of them.

What a user gets: components as Rust functions that return markup, one CSS bundle and one JS
bundle (`maud_ui::assets`), and a live gallery with docs for every component.
