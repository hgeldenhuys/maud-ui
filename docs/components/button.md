# Button

Clickable action element with variants for style (Default, Primary, Secondary, etc.) and sizes (Xs–Lg plus icon-only variants).

## Import

```rust
use maud_ui::primitives::button::{self, Variant, Size, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::button;

html! {
    (button::render(button::Props {
        label: "Save changes".to_string(),
        variant: button::Variant::Primary,
        size: button::Size::Md,
        disabled: false,
        button_type: "submit",
        leading_icon: None,
        trailing_icon: None,
        aria_label: None,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| label | String | `"Button"` | Button text content. |
| variant | Variant | `Default` | Visual variant: Default, Primary, Secondary, Outline, Ghost, Translucent, Danger, Link. |
| size | Size | `Md` | Button size: Row, Xs, Sm, Md, Lg, Icon, IconXs, IconSm, IconSm28, IconLg. |
| disabled | bool | `false` | Whether the button is disabled (native disabled and aria-disabled set). |
| button_type | &'static str | `"button"` | HTML button type attribute: "button", "submit", "reset". |
| leading_icon | Option<Markup> | `None` | Optional icon markup (SVG) rendered before the label with `data-icon="inline-start"`; use `stroke="currentColor"`. |
| trailing_icon | Option<Markup> | `None` | Optional icon markup rendered after the label with `data-icon="inline-end"`; use `stroke="currentColor"`. |
| aria_label | Option<String> | `None` | Accessible label; **required** for icon-only sizes (Icon, IconXs, IconSm, IconSm28, IconLg). |
| class | Option<String> | `None` | Extra class names appended to the class list — the escape hatch for per-instance overrides a closed enum cannot express: target the class from your own stylesheet instead of wrapping the button. |
| bordered | bool | `false` | Ghost only: give the bare ghost a visible hairline boundary — a quiet bordered pill. Ignored by other variants. |

## Variant Styles

| Variant | Use Case |
|---------|----------|
| Default | Default/neutral button. |
| Primary | Primary call-to-action (Save, Submit, Continue). |
| Secondary | Secondary action (Cancel, Back, etc.). |
| Outline | Outlined button for emphasis without primary color. |
| Ghost | Minimal ghost style, usually for low-priority actions. |
| Ghost + `bordered` | Ghost with a visible hairline (the "New issue" pill). |
| Translucent | Frosted icon pill: white 4% ground, 1px inset highlight ring, muted ink, full radius. |
| Danger | Destructive actions (Delete, Revoke); use with AlertDialog. |
| Link | Styled as a text link. |

## Size Ladder

| Size | Class | Notes |
|------|-------|-------|
| Row | `mui-btn--row` | Fixed 2rem height, nowrap; pair with Outline for quiet table-cell actions. |
| Xs | `mui-btn--xs` | Extra-small text button. |
| Sm | `mui-btn--sm` | Small text button. |
| Md | `mui-btn--md` | Default text button. |
| Lg | `mui-btn--lg` | Large text button. |
| Icon | `mui-btn--icon` | Icon-only button (requires `aria_label`). |
| IconXs | `mui-btn--icon-xs` | Extra-small icon-only (requires `aria_label`). |
| IconSm | `mui-btn--icon-sm` | Small icon-only (requires `aria_label`). |
| IconSm28 | `mui-btn--icon-28` | The 28px square icon-only step (requires `aria_label`). |
| IconLg | `mui-btn--icon-lg` | Large icon-only (requires `aria_label`). |

## Accessibility

- Icon-only buttons (`Size::Icon*`) **must** have `aria_label` set; enforced by `debug_assert!` at render time in debug builds.
- Icons use `stroke="currentColor"` to inherit button text color (emoji characters do NOT inherit and render in OS colors).
- Icons marked `aria-hidden="true"` to prevent double announcement.
- Disabled buttons set `aria-disabled="true"` (not HTML `disabled` attribute for better style control).

## Related

ButtonGroup, AlertDialog, Link.

## Shadcn reference

https://ui.shadcn.com/docs/components/base/button

Navigation actions use an `<a>` with the same `mui-btn`, variant, and size classes as a native button. Every anchor variant stays free of underlines, including hover and focus and when placed inside docs or prose. Use ordinary prose links when an underline is desired.
