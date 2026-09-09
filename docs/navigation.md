# Operating navigation

The shell owns one sidebar node. With script it moves into the native modal drawer; without script, the initially expanded details element exposes the same links and can be collapsed. A native close form is the default; an embedding app may supply its own navigation termination.

Drawer mode renders a 44px hamburger. Tabs mode omits it; More controls the same drawer. `mobile_tab_bar` accepts already admitted schema tabs without flattening or copying them.

The existing XL breakpoint (64rem) separates drawer and desktop rail. The new XXL breakpoint (80rem) expands the desktop toolbar; below it Settings holds the same controls. This leaves room for a real search field at 1024px. The phone title is independent of the one breadcrumb node, which moves to the drawer's Where you are header.

Native GET search remains available without script. With script it stays open on desktop, collapses to one icon on phones, and supports Command/Ctrl+K and phone Escape/focus return. Group and rail preferences are local and scoped to shell IDs; no record access or count is inferred by the library.

API migration: `sidebar::Props::mobile_tab_bar` is an optional opaque `Markup` slot used with `MobileNavigation::Tabs`. Omit it to use the kit's flat tabs. `page_header::Props::title` supplies the phone title; `single_crumb` also exposes it on desktop while hiding the redundant breadcrumb. Existing exhaustive struct literals must supply these fields or use `..Default::default()`.

Use `sidebar::item_icon(ItemKind::{Workflow,Entity,Reference,Report})` for a consistent 16px glyph. `named_icon` recognizes calendar, building, wallet and trending-up names; callers retain their declared label and choose an appropriate fallback when it returns `None`. Nav items keep accessible labels in rail mode.

Validation fixtures: `examples/nav_fixture.rs` accepts `--tabs`; `tests/navigation-browser.mjs` accepts `--html` or `--url`, optional `--role`/`--tabs`, and an existing Puppeteer installation through `MUI_BROWSER_MODULE`. `tests/audit-geometry.mjs` checks search hit targets and rejects a deliberately overlapping submit button.
