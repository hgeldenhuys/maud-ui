# maud-ui ↔ shadcn Base UI Gap List
_generated 2026-04-18 from shadcn Base UI docs at https://ui.shadcn.com/docs/components/base/*_
_source of truth: our v0.2.0 primitives under `src/primitives/` vs the 58-component shadcn Base UI sidebar_

## Scope Rules
- ADDITIVE changes only (new subcomponents, new variants, new ARIA wiring). No renames.
- New primitives are fresh files under `src/primitives/` + a line in `src/primitives/mod.rs`.
- Breaking API changes → log in `breaking-proposals.md` next to this file instead of implementing.
- Each `[ ]` row is ONE commit for the fix loop. Keep diffs under ~200 lines.
- If shadcn's doc is ambiguous on a point, say so in the row; don't fabricate API surface.

## High priority (missing primitives — NEW FILES)
- [x] [P0] Add `src/primitives/sheet.rs` — slide-out panel, extends Dialog. Props: `id: String`, `title: String`, `description: Option<String>`, `children: Markup`, `footer: Option<Markup>`, `side: Side { Top, Right, Bottom, Left }`, `show_close_button: bool` (default true), `open: bool`. Render `<dialog class="mui-sheet mui-sheet--{side}" role="dialog" aria-labelledby aria-describedby>`. Reuse drawer JS behaviour if appropriate — but keep it a separate module (shadcn treats Sheet as first-class). Ref: https://ui.shadcn.com/docs/components/base/sheet
- [x] [P0] Add `src/primitives/sidebar.rs` — collapsible app-shell sidebar. Subcomponents needed as helper fns: `provider`, `render` (root), `header`, `content`, `footer`, `group`, `group_label`, `group_action`, `group_content`, `menu`, `menu_item`, `menu_button`, `menu_action`, `menu_sub`, `menu_sub_item`, `menu_badge`, `menu_skeleton`, `trigger`, `rail`, `inset`. Props on root: `side: Side { Left, Right }` (default Left), `variant: SidebarVariant { Sidebar, Floating, Inset }`, `collapsible: Collapsible { Offcanvas, Icon, None }`, `default_open: bool`. Expose a small JS that toggles `data-state="expanded"|"collapsed"` via `Cmd/Ctrl+B`. Ref: https://ui.shadcn.com/docs/components/base/sidebar
- [x] [P0] Add `src/primitives/sonner.rs` — thin wrapper/alias around our existing `toast.rs` API. shadcn deprecated `toast` in favor of `sonner`, so we publish a parallel module that re-exports `toast::render` and adds `position: Position { TopLeft, TopCenter, TopRight, BottomLeft, BottomCenter, BottomRight }` on the viewport. Keep `toast.rs` around (no rename). Ref: https://ui.shadcn.com/docs/components/base/sonner
- [x] [P1] Add `src/primitives/item.rs` — media + content + actions list row (sibling to Empty). Helper fns: `render(Props)`, `group`, `media`, `content`, `title`, `description`, `actions`, `header`, `footer`, `separator`. Props: `variant: Variant { Default, Outline, Muted }`, `size: Size { Default, Sm, Xs }`. `Media` sub-variant: `Default | Icon | Image`. Ref: https://ui.shadcn.com/docs/components/base/item
- [x] [P1] Add `src/primitives/direction.rs` — `dir` context provider helper. Props: `dir: Dir { Ltr, Rtl }`, `children: Markup`. Renders `<div dir="ltr|rtl">` (no JS). Ref: https://ui.shadcn.com/docs/components/base/direction

## Medium priority (missing subcomponents / variants / helper fns)

### Alert (alert.rs)
- [x] [P1] alert.rs — add `pub fn action(children: Markup) -> Markup` rendering `<div class="mui-alert__action">{children}</div>` for shadcn's `AlertAction` slot (top-right action area). Update `render()` to accept optional `action: Option<Markup>` prop and place it in the grid so the action hugs the top-right. CSS rule: `.mui-alert { display: grid; grid-template-columns: auto 1fr auto; } .mui-alert__action { grid-column: 3; grid-row: 1 / span 2; align-self: start; }`. Ref: https://ui.shadcn.com/docs/components/base/alert
- [x] [BREAKING] alert.rs — shadcn ships only `default | destructive` variants; we ship `Default, Info, Success, Warning, Danger`. Our superset is fine. No action — but log in `breaking-proposals.md` that we deliberately go wider. Already noted as alignment.

### Avatar (avatar.rs)
- [x] [P1] avatar.rs — add `pub fn badge(children: Markup) -> Markup` for shadcn `AvatarBadge` (bottom-right status pip). Today the showcase hand-rolls inline styles for a status dot; we need a first-class helper. CSS class: `.mui-avatar__badge { position: absolute; bottom: 0; right: 0; }`. Ref: https://ui.shadcn.com/docs/components/base/avatar
- [x] [P1] avatar.rs — add `pub fn group(children: Markup) -> Markup` helper returning `<div class="mui-avatar-group">` + `pub fn group_count(n: usize) -> Markup` for the `+N` tail marker. Today the showcase uses `div.mui-avatar-group` implicitly; formalize the API. Ref: shadcn `AvatarGroup` / `AvatarGroupCount`.

### Badge (badge.rs)
- [x] [P1] badge.rs — add `Variant::Ghost` (shadcn lists `default | secondary | destructive | outline | ghost | link`). Our current enum is missing `Ghost` and `Link`. Add `Ghost` and `Link` variants with corresponding `.mui-badge--ghost` / `.mui-badge--link` classes.
- [x] [P1] badge.rs — add optional `href: Option<String>` prop so `Variant::Link` actually renders as `<a>` instead of `<span>` (shadcn's badge uses `render` prop for this). When `href` is `Some`, emit an `<a class="mui-badge mui-badge--link" href=(…)>`; otherwise keep `<span>`.
- [x] [P2] badge.rs — support leading icon slot analogous to shadcn's `data-icon="inline-start"`. Add `leading_icon: Option<Markup>` and emit `data-icon="inline-start"` on the root when present.

### Button (button.rs)
- [x] [P1] button.rs — add `Size::Xs`, `Size::IconXs`, `Size::IconSm`, `Size::IconLg` to match shadcn's `xs | sm | default | lg | icon | icon-xs | icon-sm | icon-lg`. Today we have `Sm, Md, Lg, Icon`. Keep existing names; add the missing four as new variants with `.mui-btn--xs`, `.mui-btn--icon-xs`, `.mui-btn--icon-sm`, `.mui-btn--icon-lg`.
- [x] [P2] button.rs — add `trailing_icon: Option<Markup>` prop to match shadcn's `data-icon="inline-end"` slot. Currently we ship `leading_icon` only. Emit a trailing span with `data-icon="inline-end"` inside the button.
- [x] [P2] button.rs — when `Size::Icon*` is used AND `aria_label` is `None`, panic in debug builds (or emit a `tracing::warn!` if `tracing` is already a dep). Icon-only buttons with no label are a shadcn ARIA gotcha the docs call out.

### ButtonGroup (button_group.rs)
- [x] [P1] button_group.rs — add `pub fn separator(orientation: Orientation) -> Markup` helper to match shadcn's `ButtonGroupSeparator` (renders `<div class="mui-button-group__separator" role="separator" aria-orientation=(…)>`). Default orientation is opposite of the group's orientation.
- [x] [P1] button_group.rs — add `pub fn text(children: Markup) -> Markup` helper for `ButtonGroupText` — a non-interactive text cell inside the group.

### Card (card.rs)
- [x] [P1] card.rs — add `size: Size { Default, Sm }` prop (shadcn docs list `default | sm`). Wire a `.mui-card--sm` class that reduces header/body padding.
- [x] [P1] card.rs — add `pub fn action(children: Markup) -> Markup` helper for shadcn's `CardAction` slot (top-right corner of header). Today we render title + description in the header only. Restructure the header to a 2-col grid: `(title/description) | (action)`. Add `action: Option<Markup>` to Props.
- [x] [P2] card.rs — add standalone `pub fn content(children: Markup) -> Markup` and `pub fn footer(children: Markup) -> Markup` helper fns that emit the `.mui-card__body` / `.mui-card__footer` wrappers, letting callers compose a Card by subcomponents without the bundled `render(Props)`.

### Checkbox (checkbox.rs)
- [x] [P2] checkbox.rs — add `aria_invalid: bool` prop (shadcn explicitly lists `aria-invalid` on their Checkbox Props table). Currently only `indeterminate` is wired; add `aria-invalid="true"` when set. Corresponding `.mui-checkbox--invalid` CSS hook.

### Combobox (combobox.rs)
- [x] [P1] combobox.rs — add `multiple: bool` prop to match shadcn's multi-select / chip mode. When true, selected values render as `.mui-combobox__chip` pills with an X to remove. Requires minor JS. Start with the struct field and `TODO` comment for JS if over the 200-line budget.
- [x] [P1] combobox.rs — add `auto_highlight: bool` prop (shadcn's `autoHighlight`). When true, the first filtered option gets `aria-selected="true"` and `data-highlighted`.
- [x] [P2] combobox.rs — add `show_clear: bool` prop that renders a small X button inside the trigger to clear the selection.
- [x] [P2] combobox.rs — add `aria_invalid: bool` prop and render `aria-invalid="true"` on the trigger when set. Shadcn docs list this on the Combobox prop table.
- [x] [P1] combobox.rs — add grouping: introduce `ComboboxGroup { label: String, options: Vec<ComboboxOption> }` and accept `groups: Vec<ComboboxGroup>` OR `options: Vec<ComboboxOption>` (keep both; groups supersedes flat list when present). Render group labels as `.mui-combobox__group-label`.

### Context Menu (context_menu.rs)
- [x] [P2] context_menu.rs — accept `variant: Variant { Default, Destructive }` on individual items, aligning with shadcn's per-item `variant="destructive"`. Currently `MenuItem.destructive: bool` already covers this — action: confirm in a comment that `destructive=true` maps to shadcn's `variant="destructive"` and add an explicit helper `pub fn destructive_item(...)` for symmetry with dropdown-menu API. No breaking change.
- [x] [P2] context_menu.rs — emit `data-side="inline-end"` on the submenu content for RTL support (shadcn calls this out).

### Command (command.rs)
- [x] [P1] command.rs — add `pub fn shortcut(children: Markup) -> Markup` helper for `CommandShortcut` slot. Currently shortcuts live on `CommandItem.shortcut: Option<String>` — OK — but expose the helper for custom rendering.
- [x] [P2] command.rs — add `pub fn separator() -> Markup` helper emitting `<div class="mui-command__separator" role="separator">` (shadcn's `CommandSeparator`). Currently we only support separators implicitly via groups.
- [x] [P2] command.rs — add a `CommandEmpty` helper `pub fn empty(text: &str) -> Markup` and render it when `items.is_empty()` automatically. Currently the "no results" state is JS-driven.

### Dialog (dialog.rs)
- [x] [P1] dialog.rs — add `show_close_button: bool` prop (default true). Shadcn documents this on `DialogContent`. Today we unconditionally render `close_button("Close")`. Wire a `@if props.show_close_button` gate.
- [x] [P2] dialog.rs — emit `aria-modal="true"` on the `<dialog>` element. Shadcn's ARIA table lists it explicitly. Our current markup relies on the native `<dialog>` to provide modal semantics; add the attribute for redundancy + screen-reader consistency.
- [x] [P1] dialog.rs — add `size: Size { Default, Sm }` prop mirroring shadcn's `AlertDialogContent.size` (and matching the Card size pattern). Wire `.mui-dialog--sm` for reduced padding.

### Alert Dialog (alert_dialog.rs)
- [x] [P1] alert_dialog.rs — add `pub fn media(children: Markup) -> Markup` helper for shadcn's `AlertDialogMedia` slot (icon/image above the title). Update `render()` to accept `media: Option<Markup>` and place it before the header.
- [x] [P1] alert_dialog.rs — add `size: Size { Default, Sm }` prop matching shadcn's documented `AlertDialogContent.size`.
- [x] [P2] alert_dialog.rs — add `pub fn action(label, variant) -> Markup` and `pub fn cancel(label) -> Markup` helpers so callers don't hand-craft `<button class="mui-btn mui-btn--danger" data-mui-close>` in the footer. Mirror shadcn's `AlertDialogAction` / `AlertDialogCancel`.

### Drawer (drawer.rs)
- [x] [P2] drawer.rs — add optional `should_scale_background: bool` prop mirroring Vaul's `shouldScaleBackground` — render `data-scale-background="true"` on the root. JS hook-up can be deferred; attribute is enough so CSS can target `body[data-scale-background] main { transform: scale(0.95) }`.
- [x] [P2] drawer.rs — add `show_close_button: bool` (default true) — today `close_button` is hardcoded in `render()`.

### Dropdown Menu (menu.rs — our `menu.rs` maps to shadcn `dropdown-menu`)
- [x] [P1] menu.rs — add `MenuEntry::CheckboxItem { label, checked, disabled, shortcut }` variant — shadcn's `DropdownMenuCheckboxItem`. Render with `role="menuitemcheckbox" aria-checked=(…)`. Currently we only have `Item`, `Separator`, `Label`.
- [x] [P1] menu.rs — add `MenuEntry::RadioGroup { name, items: Vec<RadioItem> }` and `MenuEntry::RadioItem { label, value, checked, disabled, shortcut }` — shadcn's `DropdownMenuRadioGroup` / `DropdownMenuRadioItem`. Render with `role="menuitemradio" aria-checked`.
- [x] [P1] menu.rs — add `MenuEntry::Sub { trigger_label, items: Vec<MenuEntry> }` — shadcn's `DropdownMenuSub` / `DropdownMenuSubTrigger` / `DropdownMenuSubContent`. Render as nested `div.mui-menu__sub` with its own `role="menu"` child. Requires small JS hover-open; ship the markup first.
- [x] [P1] menu.rs — add `MenuEntry::Group { items: Vec<MenuEntry> }` — shadcn's `DropdownMenuGroup`. Render as `<div role="group">`. Mostly a wrapper but improves a11y grouping semantics.
- [x] [P2] menu.rs — on destructive items, render `data-variant="destructive"` to match shadcn's `variant="destructive"` data hook (we currently only add a `--danger` class).

### Empty State (empty_state.rs — our file maps to shadcn `empty`)
- [x] [P1] empty_state.rs — add subcomponent helpers matching shadcn's composition: `pub fn header(children: Markup) -> Markup`, `pub fn media(children: Markup, variant: MediaVariant) -> Markup`, `pub fn title(text: &str) -> Markup`, `pub fn description(text: &str) -> Markup`, `pub fn content(children: Markup) -> Markup`. Keep existing `render(Props)` as the convenience path; add a `compose(children: Markup) -> Markup` wrapper that emits `<div class="mui-empty-state">` for callers that want full composition.
- [x] [P1] empty_state.rs — add `MediaVariant { Default, Icon }` to shadcn's `EmptyMedia.variant`. Apply `.mui-empty-state__icon--default` vs `.mui-empty-state__icon--icon` classes.
- [ ] [BREAKING] empty_state.rs — rename is out of scope. Document in `breaking-proposals.md` that shadcn's primitive is named `Empty` / filename `empty.rs`, and for 0.3.0 we may rename `empty_state` → `empty`. Don't implement; log only.

### Field (field.rs)
- [x] [P1] field.rs — add `orientation: Orientation { Vertical, Horizontal, Responsive }` prop (shadcn default `vertical`). Today we always emit a vertical stack. Add `.mui-field--horizontal` CSS hook.
- [x] [P1] field.rs — split into subcomponents matching shadcn's `FieldLabel`, `FieldDescription`, `FieldError`, `FieldGroup`, `FieldLegend`, `FieldSet`, `FieldContent`, `FieldSeparator`, `FieldTitle`. Add helper fns `pub fn label`, `pub fn description`, `pub fn error`, `pub fn group`, `pub fn legend`, `pub fn fieldset`, `pub fn content`, `pub fn separator`, `pub fn title`. Keep the all-in-one `render(Props)` as the canonical path.
- [x] [P1] field.rs — add `errors: Vec<String>` prop alongside `error: Option<String>` (shadcn docs `FieldError.errors: Array<{message?: string}>` — multiple errors supported). Our current single-error is a strict subset — add the plural without breaking.
- [x] [P2] field.rs — add `data-invalid` attribute on the root when `error` is present (shadcn's `Field.data-invalid` hook) so CSS can target it from outside. Today we use `mui-field--invalid` class; keep the class AND add the data attr.
- [x] [P2] field.rs — emit `role="group"` on `<fieldset>` wrappers (shadcn says Field outputs `role="group"`). Today we render a plain `<div>`; when rendered inside a FieldSet helper, use `<fieldset role="group">`.

### Fieldset (fieldset.rs)
- [x] [P1] fieldset.rs — (NEW subcomponent, if not already covered by field.rs split) — shadcn's `FieldSet` + `FieldLegend` are distinct primitives. Verify `src/primitives/fieldset.rs` exposes a `render(Props { legend, children })` that emits `<fieldset class="mui-fieldset" role="group"><legend>{legend}</legend>{children}</fieldset>`. If missing, add it. Ref: shadcn Field docs.

### Hover Card (hover_card.rs)
- [x] [P2] hover_card.rs — add `side: Placement { Top, Right, Bottom, Left }` prop (today we don't expose placement; shadcn has both `side` and `align`). Render `.mui-hover-card__content--{side}` class.
- [x] [P2] hover_card.rs — add `align: Align { Start, Center, End }` prop matching shadcn's Popover/HoverCard align semantics.

### Input (input.rs)
- [x] [P2] input.rs — add `aria_describedby: Option<String>` prop (today callers must reach into `field` to wire it, or inline). Shadcn documents this implicitly via Field integration. Make it first-class on `Props`.
- [x] [P2] input.rs — add `input_type: InputType::File` variant so `InputType` is a superset of shadcn's `type="file"` example. Emit `type="file"` and default `class` should not apply padding that breaks the file picker.
- [x] [P2] input.rs — switch the implementation away from `format!`-built `PreEscaped` string (lines 72–99). The current approach doesn't escape attribute values (`name`, `placeholder`, `value`) and is a XSS vector if any caller passes user-controlled text. Use maud's html! macro with conditional attrs. Treat as a quiet hardening, not API breaking.

### Input Group (input_group.rs)
- [x] [P1] input_group.rs — introduce `align: Align { InlineStart, InlineEnd, BlockStart, BlockEnd }` on addons (shadcn `InputGroupAddon.align`). Today our API is `prefix` (InlineStart) + `suffix` (InlineEnd) with no way to express Block placement. Add `pub fn addon(children: Markup, align: Align) -> Markup` and accept a `Vec<Markup>` of pre-wrapped addons alongside the existing prefix/suffix shortcut.
- [x] [P1] input_group.rs — add `pub fn button(label: &str, size: ButtonSize, variant: ButtonVariant) -> Markup` helper for `InputGroupButton` with shadcn's documented size enum (`xs | icon-xs | sm | icon-sm`) and variant enum. Currently callers hand-roll `<button>` inside `children`.
- [x] [P1] input_group.rs — add `pub fn text(children: Markup) -> Markup` for `InputGroupText`.
- [x] [P1] input_group.rs — add `pub fn input_el(props: input::Props) -> Markup` and `pub fn textarea(props: textarea::Props) -> Markup` helpers emitting the input with `data-slot="input-group-control"` set, so focus/invalid state propagates to the group wrapper (shadcn calls this out explicitly).

### Input OTP (input_otp.rs)
- [x] [P2] input_otp.rs — add `pattern: OtpPattern { Digits, DigitsAndChars, Custom(String) }` prop (shadcn's `REGEXP_ONLY_DIGITS` / `REGEXP_ONLY_DIGITS_AND_CHARS`). Currently hardcoded to `pattern="[0-9]"`. Default stays Digits.
- [x] [P2] input_otp.rs — add `aria_invalid: bool` prop and propagate to each slot when set.

### Label (label.rs)
- [x] [P2] label.rs — change shadcn-documented required indicator from `aria-hidden` to a separate `aria-label="required"` span (today we render `aria-hidden="true"` on the "*" span, which means screen readers never announce "required"). Field.rs already does this correctly. Align label.rs to match.

### Menubar (menubar.rs)
- [x] [P1] menubar.rs — add support for `MenuEntry::CheckboxItem`, `MenuEntry::RadioGroup`, `MenuEntry::Sub` at the nested menu level — same delta as the `menu.rs` Dropdown Menu gap. Since `menubar.rs` already uses `MenuEntry` from `menu.rs`, fixing menu.rs earns this one for free once the shared enum grows.

### Navigation Menu (navigation_menu.rs)
- [x] [P2] navigation_menu.rs — add `orientation: Orientation { Horizontal, Vertical }` prop (shadcn docs list orientation). Today horizontal is the only layout. Add `.mui-nav-menu--vertical` CSS hook.
- [x] [P2] navigation_menu.rs — add `viewport: bool` prop (default true) controlling whether to render a shared viewport panel vs. per-item popovers. Shadcn explicitly documents `NavigationMenuViewport` as a distinct subcomponent.
- [x] [P2] navigation_menu.rs — add `pub fn indicator() -> Markup` helper for `NavigationMenuIndicator` (the small chevron/arrow that slides between active triggers). Currently we hand-draw `" ▾"` inside the trigger label; separate it so CSS can animate it.

### Pagination (pagination.rs)
- [x] [P1] pagination.rs — add `href_builder: Option<Box<dyn Fn(usize) -> String>>` (or simpler: `href_pattern: Option<String>` where `{page}` is substituted) so the page buttons can be rendered as `<a>` tags. Today they are `<button>` without any way to navigate — shadcn's `PaginationLink` is an anchor by default.
- [x] [P2] pagination.rs — add explicit `aria-label="Go to previous page"` on the prev button and `aria-label="Go to next page"` on the next button (shadcn documents these verbatim). Currently neither button carries a `aria-label`, so screen readers only hear "Previous" / "Next" which is OK but the shadcn explicit-label pattern adds clarity for icon-only variants.
- [x] [P2] pagination.rs — add `icons_only: bool` prop — hides the "Previous"/"Next" text, keeping only the chevron (shadcn documents this as an explicit variant).

### Popover (popover.rs)
- [x] [P1] popover.rs — add `side: Side { Top, Right, Bottom, Left }` prop to match shadcn's 4-way `side`. Today we only expose `Placement { Top, Bottom }` — add `Left`, `Right`. Update `.mui-popover__content--{side}-{align}` class matrix.
- [x] [P2] popover.rs — add `side_offset: Option<u32>` prop (shadcn documents it) and emit it as `data-side-offset=(…)` for the JS to read.
- [x] [P2] popover.rs — add subcomponent helpers `pub fn header`, `pub fn title`, `pub fn description` for shadcn's `PopoverHeader`, `PopoverTitle`, `PopoverDescription` — today callers compose these by hand in showcase.
- [x] [P2] popover.rs — add `open: Option<bool>` prop for controlled-state rendering (shadcn `open` + `onOpenChange`). Open would render `data-state="open"` and `hidden` would be omitted. JS still handles transitions.

### Progress (progress.rs)
- [x] [P2] progress.rs — expose subcomponent helpers `pub fn label(text: &str) -> Markup` and `pub fn value(val: u32) -> Markup` to match shadcn's composition structure `Progress > ProgressLabel / ProgressValue / ProgressTrack / ProgressIndicator`. Keep `render(Props)` as the simple one-call path.

### Radio Group (radio_group.rs)
- [x] [P2] radio_group.rs — add `required: bool` prop (shadcn lists `required`). Currently missing; propagate `required` to the first (or selected) input.
- [x] [P2] radio_group.rs — add `variant: Variant { Default, Comfortable, Compact }` to match shadcn's documented variants. Hook `.mui-radio-group--comfortable` / `.mui-radio-group--compact` CSS classes.

### Select (select.rs)
- [x] [P1] select.rs — add `size: Size { Default, Sm }` prop matching shadcn's implied size variants. Wire `.mui-select--sm` on the trigger.
- [x] [P2] select.rs — add `aria_invalid: bool` prop (shadcn docs call this out on `SelectTrigger`). Emit `aria-invalid="true"` on the trigger button.
- [x] [P1] select.rs — add `SelectGroup { label: String, options: Vec<SelectOption> }` and accept `groups: Vec<SelectGroup>` alongside the existing flat `options`. Shadcn documents `SelectGroup` + `SelectLabel` as first-class. Render `<div role="group">` with `<div class="mui-select__group-label" role="presentation">`.
- [x] [P2] select.rs — add `pub fn scroll_up_button() -> Markup` / `pub fn scroll_down_button() -> Markup` helpers (shadcn's `SelectScrollUpButton` / `SelectScrollDownButton`). Today our dropdown scrolls but has no visual cue at the edges.
- [x] [P2] select.rs — add `pub fn separator() -> Markup` helper returning `<div class="mui-select__separator" role="separator">`.

### Slider (slider.rs)
- [x] [P1] slider.rs — support multiple thumbs / range via `values: Vec<f64>` (shadcn's `defaultValue={[33]}` is always an array — multi-thumb is a first-class case). Today `value: f64` is a single scalar. Add `values: Vec<f64>` alongside `value` (keep `value` for back-compat; when `values.len() > 1`, render one thumb per value and a fill spanning min→max of the set).
- [x] [P2] slider.rs — add `orientation: Orientation { Horizontal, Vertical }` prop. Today horizontal-only. Wire `.mui-slider--vertical` and `aria-orientation="vertical"` on the thumb.

### Spinner (spinner.rs)
- [x] [P2] spinner.rs — align ARIA with shadcn's documented `role="status"` + `aria-label="Loading"` pattern. We already emit both — spot-check that the label defaults to `"Loading"` (it does: line 43). No action; keep row here for the fix loop to verify + mark DONE on inspection.

### Switch (switch.rs)
- [x] [P2] switch.rs — add `size: Size { Default, Sm }` prop matching shadcn's "Size (small, default)" variant line. Wire `.mui-switch--sm` for reduced height/width.
- [x] [P2] switch.rs — add `aria_invalid: bool` prop and `required: bool` prop (shadcn lists both). Propagate `aria-invalid="true"` and `required` on the hidden `<input>`.

### Tabs (tabs.rs)
- [x] [P1] tabs.rs — add `orientation: Orientation { Horizontal, Vertical }` (shadcn documents `vertical`). Wire `.mui-tabs--vertical` and `aria-orientation="vertical"` on the tablist.
- [x] [P1] tabs.rs — add `variant: Variant { Default, Line }` prop matching shadcn's `variant="line"` on TabsList. Wire `.mui-tabs__list--line`.
- [x] [P2] tabs.rs — add `activation_mode: ActivationMode { Automatic, Manual }` prop. Shadcn documents it. Emit `data-activation-mode=(…)` so our JS knows whether arrow-key focus auto-activates or requires Enter/Space.
- [x] [P2] tabs.rs — support a `disabled: bool` per-`Tab`. Today all tabs are enabled.

### Textarea (textarea.rs)
- [x] [P2] textarea.rs — add `aria_invalid: bool` prop (shadcn lists `aria-invalid` as a documented prop). If not already present, add it.
- [x] [P2] textarea.rs — verify `rows: Option<u32>` exists and defaults to sensible (e.g. 3). Shadcn treats `rows` as a native HTMLTextareaElement pass-through. If absent, add.

### Toast (toast.rs)
- [x] [P2] toast.rs — add subcomponent helper `pub fn action(label: &str, onclick: &str) -> Markup` returning `<button class="mui-toast__action">` — shadcn's `ToastAction` slot (undo / retry button next to the close X). Currently we only have a close button.
- [x] [P2] toast.rs — add note in module doc that shadcn deprecated `toast` in favor of `sonner` — we keep both. Point users to `sonner::*` once that module lands (see P0 above).

### Toggle Group (toggle_group.rs)
- [x] [P2] toggle_group.rs — add `variant: Variant { Default, Outline }` prop matching shadcn's `ToggleGroup.variant`. Today we inherit from the inner toggle's variant via CSS; expose on the group for consistency.
- [x] [P2] toggle_group.rs — add `orientation: Orientation { Horizontal, Vertical }` prop (shadcn documents it). Wire arrow-key navigation axis in the existing JS.
- [x] [P2] toggle_group.rs — add `spacing: Spacing { None, Sm, Md }` prop (shadcn's `spacing`). Wire `.mui-toggle-group--spacing-sm`/`-md` for inter-item gaps.

### Toggle (toggle.rs)
- [x] [P2] toggle.rs — add `Size::Lg` (shadcn ships `default, sm, lg`). Today we only have `Md, Sm`. Add `Lg` with `.mui-toggle--lg`.

### Tooltip (tooltip.rs)
- [x] [P2] tooltip.rs — add `align: Align { Start, Center, End }` prop. Shadcn documents `align` on TooltipContent; today we only expose `placement`.
- [x] [P2] tooltip.rs — add `side_offset: Option<u32>` prop — emit `data-side-offset` for the JS to consume.

### Typography (typography.rs)
- [x] [P1] typography.rs — add the missing shadcn variants: `blockquote` (we have), `list` (`pub fn list_ul(items: Vec<Markup>)` + `pub fn list_ol(items: Vec<Markup>)`), `large` (`pub fn large(text: &str)`), `small` (`pub fn small(text: &str)`), and `table` wrapper (`pub fn table(children: Markup)`). Today we ship `h1..h4`, `p`, `lead`, `muted`, `code_inline`, `blockquote` — missing `large`, `small`, `list`, `table`.

### Chart (chart.rs)
- [x] [P2] chart.rs — inspect current API. If it does not already expose a `config` prop analogous to shadcn's `ChartContainer.config: ChartConfig`, add one. Shadcn treats config (nameKey, labelKey, colors) as first-class. Note: chart is fundamentally a JS-heavy primitive; a Rust-side gap row is acceptable to log and skip if deep-equivalence is out of scope. Mark "[needs-investigation]" if unclear.
- [x] [P2] chart.rs — add an `accessibility_layer: bool` prop — shadcn documents this for keyboard access + SR support. Emit `data-accessibility-layer="true"` on the container.

### Data Table (data_table.rs)
- [x] [P2] data_table.rs — add `pub fn column_header(label: &str, sortable: bool) -> Markup` and `pub fn view_options(columns: &[Column]) -> Markup` helpers matching shadcn's `DataTableColumnHeader` and `DataTableViewOptions`. Today we hand-roll these inside `render()`.
- [x] [P2] data_table.rs — add `selectable: bool` prop that prepends a `<th>` with "Select all" checkbox + per-row checkboxes (shadcn calls out aria-label="Select all" / "Select row" for this).

### Date Picker (date_picker.rs)
- [x] [P2] date_picker.rs — add `mode: Mode { Single, Range }` prop matching shadcn's `mode`. Today assumed single; add the enum and wire two-thumb range state.
- [x] [P2] date_picker.rs — add `format: Option<String>` (e.g. `"yyyy-MM-dd"`) so callers can control the display format. Shadcn delegates to date-fns `format`.

## Low priority (a11y / polish)

- [x] [P2] accordion.rs — add `role="region"` label via `aria-label` on the `<div class="mui-accordion">` root. Currently only items have a `role="region"` — a top-level label aids SR navigation. Shadcn relies on underlying Radix here.
- [x] [P2] avatar.rs — the fallback's `role="img" aria-label=alt` is set on the outer `<span>` regardless of whether an `<img>` is rendered; this double-announces ("image: Sofia Davis, image: Sofia Davis"). Move `role="img"` and `aria-label` to emit ONLY when `src` is None, so the nested `<img alt="">` (which we set to empty) takes over when present. Shadcn behaviour matches.
- [x] [P2] breadcrumb.rs — the current-page item is a `<span>` inside a `<li>`; shadcn wraps it in a `<span role="link" aria-disabled="true" aria-current="page">` for some consistency with link semantics. Consider `role="link" aria-disabled="true"` on the current span.
- [x] [P2] button_group.rs — add `aria-orientation` attribute to the root when `orientation=Vertical`. Today we only wire the CSS class; ARIA orientation is missing.
- [x] [P2] carousel.rs — `aria-live="off"` is correct for manual carousels but should flip to `"polite"` when `auto_play=true` so SR users hear slide changes. Today we always emit `"off"`.
- [x] [P2] command.rs — the dialog element does not set `aria-modal="true"`; add it.
- [x] [P2] context_menu.rs — the content div has `role="menu"` but no `aria-orientation="vertical"`. Add it (shadcn's Radix menu emits `aria-orientation`).
- [x] [P2] dialog.rs — when `description` is None, we still generate `aria-describedby=[None]` — verify maud actually omits the attr in that case (looks correct at line 71 but worth asserting). No change unless test fails.
- [x] [P2] drawer.rs — emit `aria-modal="true"` on the `<dialog>` root (same rationale as Dialog).
- [x] [P2] menu.rs — on the trigger button, `aria-haspopup="menu"` is set but `aria-controls` points at `{id}-items` — the content div is `role="menu"` but doesn't have `aria-labelledby` back to the trigger. Add `aria-labelledby=(trigger_id)` on the content.
- [x] [P2] menubar.rs — the `role="menubar"` root does not set `aria-orientation="horizontal"`; add it.
- [x] [P2] navigation_menu.rs — `data-has-content` is a non-semantic attribute; add `aria-haspopup="true"` on items that have submenus.
- [x] [P2] pagination.rs — the ellipsis `<span>` should be `role="presentation"` (shadcn's `PaginationEllipsis` is a span with `aria-hidden="true"`); currently plain `<span>`. Add `aria-hidden="true"`.
- [x] [P2] popover.rs — the content `<div role="dialog">` lacks `aria-labelledby` / `aria-describedby`. If callers pass a title/description via children, there's no hook. Add optional `title_id: Option<String>` + `description_id: Option<String>` props so callers can wire these.
- [x] [P2] radio_group.rs — `fieldset role="radiogroup"` is a role-on-fieldset anti-pattern. Either use `<div role="radiogroup">` OR rely on the native `<fieldset>`+`<legend>` semantics without the explicit role. Prefer keeping native + dropping `role="radiogroup"`.
- [x] [P2] scroll_area.rs — the `<div role="region">` has `aria-label="Scrollable region"` — fine — but the scrollbar thumb should have `role="scrollbar" aria-controls=(viewport_id) aria-orientation=(…) aria-valuenow=(…) aria-valuemin="0" aria-valuemax="100"` for non-native scrollbars. Today thumb is aria-hidden. Either keep aria-hidden (rely on the native scroll) OR promote the thumb to a real scrollbar widget.
- [x] [P2] select.rs — the option row has `aria-disabled="false"` set even when the option isn't disabled. Shadcn convention is to omit the attr entirely when not disabled. Gate it behind `@if opt.disabled`.
- [x] [P2] slider.rs — when vertical orientation lands (above), emit `aria-orientation="vertical"` on the thumb.
- [x] [P2] switch.rs — the hidden `<input type="hidden">` is used to carry form value, but screen readers will not announce switch changes because we're rendering a `<button role="switch">`. This is actually correct (SRs read `aria-checked`), but verify form submission works — the hidden input's value only updates via JS. Document in module comment.
- [x] [P2] tabs.rs — `role="tabpanel"` panels have `tabindex="0"` even when hidden, which means tab order includes hidden panels. Change to `tabindex="0"` only when NOT hidden.
- [x] [P2] toast.rs — viewport `<div>` has `aria-live="polite"` — good — but children toasts ALSO carry `aria-live`. Nested live regions are discouraged. Remove `aria-live` from individual toasts when rendered inside the viewport; keep it only for standalone inline toasts.
- [x] [P2] tooltip.rs — `delay_ms` default is 500, shadcn's `delayDuration` default is 700. Not a bug; log alignment consideration in `breaking-proposals.md`. No change.

## Out-of-scope / already aligned (do NOT queue)
The following primitives match shadcn's documented API closely enough that no action is needed this pass:
- **aspect_ratio.rs** — shadcn's Props = `{ ratio, className }`; we expose `ratio` (inferred — verify) and pass-through styling. Spot-check only.
- **calendar.rs** — our API is a deliberate subset (static month view, no react-day-picker dependency). Shadcn uses React DayPicker; matching their full API (Persian/Hijri calendars, timezone-aware selection) is a breaking restructure — log in `breaking-proposals.md` for 0.3.0, do not queue here.
- **checkbox.rs** — covers shadcn's `checked | defaultChecked | onCheckedChange | disabled | required | name | value` plus our `indeterminate` + `description`. Only `aria_invalid` gap (queued P2 above).
- **kbd.rs** — our single-prop `keys: Vec<String>` covers shadcn's use case. `KbdGroup` maps to any outer wrapper. No action.
- **label.rs** — minimal 1:1 match with shadcn.
- **native_select.rs** — matches shadcn's `NativeSelect` + `NativeSelectOption` + `NativeSelectOptGroup` structure (confirm `NativeOption` has `disabled` and groups exist). Spot-check only.
- **resizable.rs** — matches `ResizablePanelGroup` + `ResizablePanel` + `ResizableHandle` naming pattern. `direction` / `default_size` / `min_size` all present.
- **separator.rs** — matches shadcn's `orientation | decorative` exactly. One of our cleanest.
- **skeleton.rs** — shadcn's only prop is `className`; we likely pass through class styling. No action.
- **table.rs** — shadcn ships plain composition (`Table / TableHeader / TableBody / …`); our table is similar. Spot-check subcomponent names but no functional gap.
- **meter.rs** / **number_field.rs** / **radio.rs** / **swatch.rs** — these exist in maud-ui but have NO shadcn Base UI counterpart at `/docs/components/base/*`. Keep as maud-ui extensions. Note in module docstrings that they're deliberate supersets.

## Notes for the autoagent fix loop
- When adding variants to an `enum Variant`, prefer `#[non_exhaustive]` ONLY if we are committed to 0.3.0 breaking — it's not breaking today, but `match` sites currently enumerate all variants and Rust's exhaustiveness check is our friend (see kapable WoW `feedback_enum-exhaustiveness-is-the-drift-scanner.md`). **Do not add `#[non_exhaustive]` in this additive pass.**
- CSS changes must go in whatever `src/css/` / `styles.scss` / theme file the existing variants live in — grep for an existing `.mui-btn--*` rule to find the right file.
- When adding a subcomponent helper `pub fn foo(children: Markup) -> Markup`, keep the existing all-in-one `render(Props)` untouched so callers aren't forced to refactor.
- Skip any row where `cargo check` reveals the feature already exists — the audit was derived from reading the top ~30 primitives in detail; the remaining ~29 were inferred from shadcn docs. A "[already aligned — verified by cargo check]" close-out is a legitimate resolution.
- No row here should require more than ~200 lines of diff. If the autoagent estimates more, split into two rows and log the split in a `split-notes.md`.
