---
verified_on: 2026-09-23
covers:
  - ".claude/maud-ui-improvements-state.md"
  - "CHANGELOG.md"
verify_with: "sed -n '1,40p' .claude/maud-ui-improvements-state.md"
---
# What is left

Herman decided on 2026-09-23 that light mode keeps the grey `#f6f6f7` card surface. The app shell's nav
rows are square. The sidebar component and the gallery keep rounded rows.

Known and not yet fixed, from the 2026-09-23 screenshot review:

- The swatch component has no stylesheet, and 78 class names have no CSS rule (the design
  linter's baseline lists them).

- Props tables break code mid-word at 1440px, show empty description cells, and show raw
  rustdoc markdown.

- The button size order is wrong: `xs` is as tall as the default size.

- The data table's Columns button and row checkboxes are unstyled.

- Component pages have text lines over 80 characters and skip from h2 to h4.

Not started: moving the four apps to 0.20.2, which would bring them the square shell rows and
the navigation fixes.
