# Design source sweep · 0.9.0

All 81 registered primitives and 16 blocks were checked against the active style imports, shared defaults and rendered routes. The canonical source moved into `static/styles/` so local adjustments no longer fight the readonly 0.8 source tree. This is source and HTTP evidence, not a visual acceptance claim.

The pass replaced local typography, spacing, radii, elevation and interaction timing with tokens, including inline showcase styles. It unified prose/nav link roles, table alignment, semantic surfaces, accessible control boundaries and focus treatment. Responsive widths, SVG/drawing geometry, relative code sizing and functional loading animation geometry remain explicit.

## Primitives

| Primitive | Active style source | HTTP |
|---|---|---|
| `accordion` | `static/styles/components/accordion.css` | 200 |
| `alert` | `static/styles/components/alert.css` | 200 |
| `alert_dialog` | `static/styles/components/alert_dialog.css` | 200 |
| `aspect_ratio` | `static/styles/components/aspect_ratio.css` | 200 |
| `attention_pill` | `static/styles/components/attention_pill.css` | 200 |
| `avatar` | `static/styles/components/avatar.css` | 200 |
| `badge` | `static/styles/components/badge.css` | 200 |
| `bottom_tab_bar` | `static/styles/curation.css + shared defaults` | 200 |
| `breadcrumb` | `static/styles/components/breadcrumb.css` | 200 |
| `button` | `static/styles/components/button.css` | 200 |
| `button_group` | `static/styles/components/button_group.css` | 200 |
| `calendar` | `static/styles/components/calendar.css` | 200 |
| `card` | `static/styles/components/card.css` | 200 |
| `carousel` | `static/styles/components/carousel.css` | 200 |
| `chart` | `static/styles/components/chart.css` | 200 |
| `checkbox` | `static/styles/components/checkbox.css` | 200 |
| `code_block` | `static/styles/components/code_block.css` | 200 |
| `collapsible` | `static/styles/components/collapsible.css` | 200 |
| `combobox` | `static/styles/components/combobox.css` | 200 |
| `command` | `static/styles/components/command.css` | 200 |
| `composer` | `static/styles/components/composer.css` | 200 |
| `context_menu` | `static/styles/components/context_menu.css` | 200 |
| `data_table` | `static/styles/components/data_table.css` | 200 |
| `date_picker` | `static/styles/components/date_picker.css` | 200 |
| `dialog` | `static/styles/components/dialog.css` | 200 |
| `diff` | `static/styles/components/diff.css` | 200 |
| `direction` | `static/styles/curation.css + shared defaults` | 200 |
| `drawer` | `static/styles/components/drawer.css` | 200 |
| `empty_state` | `static/styles/components/empty_state.css` | 200 |
| `facts_list` | `static/styles/components/facts_list.css` | 200 |
| `field` | `static/styles/components/field.css` | 200 |
| `fieldset` | `static/styles/components/fieldset.css` | 200 |
| `form` | `static/styles/components/form.css` | 200 |
| `grid` | `static/styles/components/grid.css` | 200 |
| `gutter_section` | `static/styles/components/gutter_section.css` | 200 |
| `hover_card` | `static/styles/components/hover_card.css` | 200 |
| `input` | `static/styles/components/input.css` | 200 |
| `input_group` | `static/styles/components/input_group.css` | 200 |
| `input_otp` | `static/styles/components/input_otp.css` | 200 |
| `item` | `static/styles/components/item.css` | 200 |
| `kbd` | `static/styles/components/kbd.css` | 200 |
| `label` | `static/styles/components/label.css` | 200 |
| `menu` | `static/styles/components/menu.css` | 200 |
| `menubar` | `static/styles/components/menubar.css` | 200 |
| `message` | `static/styles/components/message.css` | 200 |
| `meter` | `static/styles/components/meter.css` | 200 |
| `native_select` | `static/styles/components/native_select.css` | 200 |
| `navigation_menu` | `static/styles/components/navigation_menu.css` | 200 |
| `number_field` | `static/styles/components/number_field.css` | 200 |
| `pagination` | `static/styles/components/pagination.css` | 200 |
| `popover` | `static/styles/components/popover.css` | 200 |
| `progress` | `static/styles/components/progress.css` | 200 |
| `radio` | `static/styles/components/radio.css` | 200 |
| `radio_group` | `static/styles/components/radio_group.css` | 200 |
| `resizable` | `static/styles/components/resizable.css` | 200 |
| `scroll_area` | `static/styles/components/scroll_area.css` | 200 |
| `segmented_control` | `static/styles/components/segmented_control.css` | 200 |
| `select` | `static/styles/components/select.css` | 200 |
| `separator` | `static/styles/components/separator.css` | 200 |
| `sheet` | `static/styles/components/sheet.css` | 200 |
| `sidebar` | `static/styles/components/sidebar.css` | 200 |
| `skeleton` | `static/styles/components/skeleton.css` | 200 |
| `slider` | `static/styles/components/slider.css` | 200 |
| `sonner` | `static/styles/components/sonner.css` | 200 |
| `spinner` | `static/styles/components/spinner.css` | 200 |
| `stack` | `static/styles/components/stack.css` | 200 |
| `status_chip_group` | `static/styles/curation.css + shared defaults` | 200 |
| `status_dot` | `static/styles/components/status_dot.css` | 200 |
| `streaming_cursor` | `static/styles/components/streaming_cursor.css` | 200 |
| `swatch` | `static/styles/curation.css + shared defaults` | 200 |
| `switch` | `static/styles/components/switch.css` | 200 |
| `table` | `static/styles/components/table.css` | 200 |
| `tabs` | `static/styles/components/tabs.css` | 200 |
| `textarea` | `static/styles/components/textarea.css` | 200 |
| `toast` | `static/styles/components/toast.css` | 200 |
| `toggle` | `static/styles/components/toggle.css` | 200 |
| `toggle_group` | `static/styles/components/toggle_group.css` | 200 |
| `tool_call` | `static/styles/components/tool_call.css` | 200 |
| `tooltip` | `static/styles/components/tooltip.css` | 200 |
| `turn_progress` | `static/styles/components/turn_progress.css` | 200 |
| `typography` | `static/styles/components/typography.css` | 200 |

Every primitive also inherits `tokens.css`, `base.css`, `defaults.css`; navigation uses `navigation.css`. Native form/stack/grid helpers inherit their child components where they introduce no visual surface.

## Blocks

| Block | Active style source | HTTP |
|---|---|---|
| `auth-login` | `static/styles/blocks/auth-login.css` + block common/defaults | 200 |
| `auth-signup` | `static/styles/blocks/auth-login.css` + block common/defaults | 200 |
| `auth-two-factor` | `static/styles/blocks/auth-login.css` + block common/defaults | 200 |
| `dashboard-stats` | `static/styles/blocks/dashboard-stats.css` + block common/defaults | 200 |
| `data-table-full` | `static/styles/blocks/data-table-full.css` + block common/defaults | 200 |
| `pricing-tiers` | `static/styles/blocks/pricing-tiers.css` + block common/defaults | 200 |
| `settings-billing` | `static/styles/blocks/settings.css` + block common/defaults | 200 |
| `settings-profile` | `static/styles/blocks/settings.css` + block common/defaults | 200 |
| `settings-team` | `static/styles/blocks/settings.css` + block common/defaults | 200 |
| `shell-sidebar` | `static/styles/blocks/shell-sidebar.css` + block common/defaults | 200 |
| `shell-page-header` | `static/styles/blocks/page-header.css` + block common/defaults | 200 |
| `shell-app-header` | `static/styles/blocks/app-header.css` + block common/defaults | 200 |
| `shell-app-footer` | `static/styles/blocks/app-footer.css` + block common/defaults | 200 |
| `worklist-header` | `static/styles/curation.css` + block common/defaults | 200 |
| `record-header` | `static/styles/curation.css` + block common/defaults | 200 |
| `task-grid` | `static/styles/curation.css` + block common/defaults | 200 |

## Focused review outcomes

- Both sidebars share the row/icon/group/active language. Nested links indent; the rail preserves full accessible names and icons. Drawer behavior moves the existing navigation node. Group and rail preferences tolerate unavailable storage.
- Optional masthead/footer render no empty landmark. Embedded shell previews use a section inside the gallery main. The 56px page context row owns breadcrumbs, search and caller-defined switchers.
- The landing uses one shared operational composition, with a working local form, search, counted filters and record selection. All sample rows are fictional and page-local.
- Table headers are muted and medium weight. Both table APIs carry numeric alignment; data-table sorting/pagination keep rich nodes and selections.
- Gallery tabs now scope their panel/field IDs per specimen. The combined gallery label and combobox demos use unique IDs, preserving correct labels and navigation.
- Plain links never acquire decoration on hover; prose underlines remain stable. Semantic text and fills are tested in both defaults and all eight gallery presets.
- Palette recents accept only registry routes, sections share small-cap labels, selection uses aria-activedescendant, and native dialog restores focus.
- Motion is reduced to two interaction durations plus an ongoing-progress cadence. prefers-reduced-motion remains globally honored.

## Review boundary

No browser, Chrome or screenshot process was launched. The supervisor should check native dialog/AT, overflow, actual font availability, 56px chrome, pointer target sizes, safe areas and reduced motion on the prioritized pages in `design-lead-report.md`.
