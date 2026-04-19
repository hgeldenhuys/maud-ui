# Component API Reference

One Markdown doc per primitive in `src/primitives/<name>.rs`. Filenames map
1:1 to module names (underscore_case), so you can jump from code to docs
with `<prefix>/docs/components/<module>.md`.

Each doc follows the same structure:
1. **Import** — `use maud_ui::primitives::<name>::{…}` with any commonly-used enums.
2. **Example** — minimal `html!` + `render(Props { … })` showing a real use.
3. **Props** — every `pub` field with type, default, and intent.
4. **Variants / Enums** — if the primitive ships any.
5. **Helper Functions** — every `pub fn` in the module besides `render` and `showcase`.
6. **Accessibility** — ARIA, roles, keyboard interactions.
7. **Related** — sibling primitives that pair well.
8. **Shadcn reference** — upstream docs link at `https://ui.shadcn.com/docs/components/base/…`.

## Alphabetical index

**Form controls** · [button](button.md) · [input](input.md) · [textarea](textarea.md) · [checkbox](checkbox.md) · [radio](radio.md) · [radio_group](radio_group.md) · [select](select.md) · [switch](switch.md) · [slider](slider.md) · [number_field](number_field.md) · [field](field.md) · [fieldset](fieldset.md) · [label](label.md) · [native_select](native_select.md) · [input_group](input_group.md) · [input_otp](input_otp.md) · [combobox](combobox.md)

**Display** · [badge](badge.md) · [swatch](swatch.md) · [avatar](avatar.md) · [separator](separator.md) · [progress](progress.md) · [meter](meter.md) · [kbd](kbd.md) · [skeleton](skeleton.md) · [spinner](spinner.md) · [typography](typography.md) · [empty_state](empty_state.md) · [item](item.md)

**Layout** · [card](card.md) · [accordion](accordion.md) · [collapsible](collapsible.md) · [aspect_ratio](aspect_ratio.md) · [tabs](tabs.md) · [resizable](resizable.md) · [scroll_area](scroll_area.md) · [direction](direction.md) · [sidebar](sidebar.md) · [table](table.md) · [data_table](data_table.md)

**Overlay** · [dialog](dialog.md) · [alert_dialog](alert_dialog.md) · [sheet](sheet.md) · [drawer](drawer.md) · [popover](popover.md) · [hover_card](hover_card.md) · [tooltip](tooltip.md) · [toast](toast.md) · [sonner](sonner.md)

**Navigation** · [breadcrumb](breadcrumb.md) · [menu](menu.md) · [menubar](menubar.md) · [context_menu](context_menu.md) · [navigation_menu](navigation_menu.md) · [pagination](pagination.md) · [command](command.md)

**Interaction** · [toggle](toggle.md) · [toggle_group](toggle_group.md) · [button_group](button_group.md) · [carousel](carousel.md) · [calendar](calendar.md) · [date_picker](date_picker.md)

**Visualisation** · [alert](alert.md) · [chart](chart.md)

## Contributing

If you change a primitive's `Props`, update the matching `docs/components/<name>.md` in the same commit. The docs are cross-checked against source during every release audit.
