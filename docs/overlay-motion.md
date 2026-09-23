# Overlay exit motion

> Updated 2026-09-23 after browser verification: popovers now return focus to their button on
> Escape, and context menus to whatever had focus before they opened. Dialogs keep their
> column layout while fading (it lived only under `[open]`, so content fell into a row
> mid-fade). `node tests/overlay-motion-browser.mjs http://localhost:3456` passes all 17 cases
> against the gallery and fails every non-toast case against the 0.19.5 live site.

## Changes and existing behavior

| Overlay | Before | Change |
| --- | --- | --- |
| Dialog / alert dialog | `showModal()` and surface/backdrop entry keyframes; native Escape or `close()` removed immediately. Existing dialog behavior returns focus. | Supported browsers use discrete display/overlay transitions, opacity and the same translation, enter token on opening and fast token on closing. Backdrop fades too. Unsupported browsers keep old behavior. |
| Sheet | Dialog trigger calls `showModal()`; native Escape closes. Uses directional entry variables. | Same native transitions, retaining direction and flex layout through exit. Existing sheet close-control wiring is not changed. |
| Drawer | Drawer behavior calls `showModal()`; Escape, close control and backdrop call native close. Native focus restoration. | Same native transitions with the original direction and flex layout. |
| Navigation dialog | Navigation trigger calls `showModal()`; native Escape, with existing focus and aria restoration. | Same native surface and backdrop transitions. |
| Menu / dropdown menu | `menu` behavior removes/sets `hidden`; Escape returns focus; outside click and Tab have their existing policies. There is no separately registered dropdown-menu primitive. | Same-named static behavior override delays hiding through the shared helper. Opening and aria updates are unchanged. |
| Context menu | `menu.js` opens on contextmenu, positions the panel, removes `hidden`; Escape/outside click sets `hidden`. No trigger focus restoration. | Shared exit helper delays hiding. Existing positioning statements are retained, with no new inline styling. |
| Menubar | Removes/sets `hidden`; switches active menu and restores focus on Escape. | Shared helper, with cancellation when a panel reopens. |
| Popover | Hidden-attribute implementation, not native `[popover]`. Opens via forced reflow and `data-visible`; closes with a hard-coded 150ms timeout. No focus restoration. | Token-timed exit with event completion, bounded fallback, cancellation, and synchronous reduced-motion hiding. |
| Hover card | Separate behavior using the same hidden/data-visible pattern, with pointer intent delays plus a hard-coded 150ms hide timer. | Shared exit helper replaces the hide timer; existing normal pointer intent delay remains. Reduced motion bypasses close delay. |
| More | `src/blocks/action_row.rs` renders `details.mui-action-row__more`, summary text “More”, and `.mui-action-row__overflow`. Native summary toggle; disclosure behavior closes on outside pointerdown, Escape and another opening. | Retains details open while panel exits, intercepts summary close, cancels on reopening. Escape focus policy is retained. The bottom-tab-bar More is separately covered as a navigation dialog. |
| Toast | Existing enter/exit animation, token-computed timeout, own animationend filtering, immediate reduced-motion removal. No trigger association or focus return. | No source changes. Existing runtime checks passed for both bundles. |

All new exit durations/easing use existing motion tokens. The shared helper sets `data-state="closing"`, temporarily makes the panel inert, and restores its prior state on completion/cancellation. Dialog opening retains its previous motion trajectory and duration. No new native popover implementation was introduced because none of these components uses that mechanism.

## Selector evidence

Evidence is the actual markup in `public/<page>/index.html` (the exports are single-line HTML). Browser test selectors:

| Page | Trigger | Panel |
| --- | --- | --- |
| dialog | `[data-target="demo-dialog-edit-profile"]` | `#demo-dialog-edit-profile` |
| alert_dialog | `[data-target="demo-alert-delete"]` | `#demo-alert-delete` |
| sheet | `[data-target="demo-sheet-{right,left,top,bottom}"]` | matching ID |
| drawer | `[data-target="demo-drawer-{1,2,3}"]` | matching ID (right, left, bottom) |
| bottom_tab_bar | `[data-mui="navigation-trigger"][aria-controls="tab-bar-more"]` | `#tab-bar-more` |
| menu | `#demo-menu-file-trigger` | `#demo-menu-file-items` |
| context_menu | `.mui-context-menu__region` | `#demo-ctx-1-menu` |
| menubar | `[aria-controls="demo-menubar-1-menu-0"]` | `#demo-menubar-1-menu-0` |
| popover | `.mui-popover__trigger button` | `#demo-pop-1-content` |
| hover_card | `.mui-hover-card__trigger` | `#demo-hover-1-card` |
| blocks/action-row | `.mui-action-row__more summary` | `.mui-action-row__overflow` |
| toast | `button[onclick*="Profile updated"]` | `#mui-toast-viewport .mui-toast` (created by the real trigger; viewport and trigger are in the export) |

## Validation and limitations

- `node examples/build-assets.mjs` and `--check`: passed. Copied the four generated assets to dist and public CSS/JS directories. Public synchronization was necessary for registration parity and for the static gallery to exercise the new code.
- `cargo test --no-fail-fast`: final run exit 0, no FAILED lines; includes design_lint. Initial run found only two stale-public-bundle parity failures, corrected by synchronization.
- `cargo clippy --all-targets -- -D warnings`: exit 0.
- `node --test tests/states-motion-runtime.mjs`: 10 passed, including toast event/fallback/reduced-motion tests against both bundles.
- `node --test tests/overlay-motion-runtime.mjs`: 3 passed (completion filtering/inertness, timeout/repeated-close/cancellation, reduced motion).
- Browser test syntax and `git diff --check`: passed.
- Browser execution and visual acceptance were not possible under the requested no-sockets/no-Chrome restriction. Run `node tests/overlay-motion-browser.mjs http://localhost:3456` against the updated gallery.

The browser test prints one PASS/FAIL per case, tests normal and emulated reduced motion, measures the first sample on the close mutation's clock, checks opacity and native backdrop opacity, checks removal by 400ms, attempts focus on the closed panel/descendants, and asserts focus on the original trigger. Hover timing starts after its existing pointer intent delay. Hover/context activation dispatches their existing mouse events; Escape uses CDP key input and toast dismissal uses its real control.

**Requirement conflict:** preserving existing focus policies cannot also make every strict trigger-focus assertion pass. Context menus and popovers do not restore focus; hover cards have no focusable trigger contract; toasts remove their focused dismiss button without returning to a creator. Those assertions are deliberately retained and expected to expose these existing gaps. Fixing them would change the requested existing focus behavior (and toast verification-only scope). No browser PASS is claimed.

## Files

Authored: `static/styles/states-motion.css`; `static/behaviors/disclosure_menu.js`; new static overrides `menu.js`, `menubar.js`, `popover.js`, `hover_card.js`; new shared `overlay_motion.js`; `tests/overlay-motion-browser.mjs`; `tests/overlay-motion-runtime.mjs`; this report.

Generated: `static/maud-ui{,.min}.{css,js}`, `dist/maud-ui{,.min}.{css,js}`, `public/css/maud-ui{,.min}.css`, `public/js/maud-ui{,.min}.js`.

The pre-existing `docs/night-6-live-events.jsonl` modification was not edited. No commit, stash, reset, push, server, browser, or subagent was used.
