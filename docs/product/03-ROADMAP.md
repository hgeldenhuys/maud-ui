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

Fixed in 0.20.3: the swatch stylesheet, the 78 undefined classes, the props tables, the button
size order, the data table's controls, the pager, and the skipped heading level.

Still open: component prose runs about 88 characters a line at 1440px (it was 120 to 155).

In progress on 2026-09-23: moving the four apps to 0.20.3.
