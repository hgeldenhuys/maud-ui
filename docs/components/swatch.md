# Swatch

Colour chips with labels, click-to-copy, and a design-token mode. Reads live CSS variables so a swatch grid doubles as a live theme preview. Three rendering modes: Raw (literal colours), Token (CSS variables), and Scale (Tailwind-style ramps).

## Import

```rust
use maud_ui::primitives::swatch::{self, Mode, Size, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::swatch;

// Single raw colour
html! {
    (swatch::render(swatch::Props {
        label: "Primary".into(),
        sublabel: Some("#2563eb".into()),
        mode: swatch::Mode::Raw("#2563eb".into()),
        size: swatch::Size::Md,
        copyable: true,
    }))
}

// Design token (live)
html! {
    (swatch::render(swatch::Props {
        label: "Accent".into(),
        sublabel: Some("--mui-accent".into()),
        mode: swatch::Mode::Token("mui-accent".into()),
        size: swatch::Size::Md,
        copyable: true,
    }))
}

// Scale ramp (Tailwind-style)
html! {
    (swatch::render_scale("slate", &swatch::tailwind_ramp("slate")))
}

// Grid of design tokens (live theme preview)
html! {
    (swatch::render_tokens(&[
        ("Background", "mui-bg"),
        ("Text", "mui-text"),
        ("Border", "mui-border"),
        ("Accent", "mui-accent"),
    ]))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| label | String | (empty) | Human-readable name shown under the chip (e.g., "Primary"). |
| sublabel | Option<String> | `None` | Optional second line; typically the raw value or token name. |
| mode | Mode | (required) | Colour source: Raw(String) or Token(String). |
| size | Size | `Md` | Chip size: Sm, Md, Lg. |
| copyable | bool | `true` | Whether clicking the swatch copies the value to clipboard. |

## Mode Enum

| Variant | Format | Example |
|---------|--------|---------|
| Raw(String) | Literal CSS colour | `Raw("#2563eb")` or `Raw("hsl(210 40% 96%)")` or `Raw("rgba(37,99,235,0.35)")` |
| Token(String) | CSS custom-property name (without `--` prefix) | `Token("mui-accent")` renders as `background: var(--mui-accent)`. |

## Size Enum

| Variant | Dimensions | Use Case |
|---------|-----------|----------|
| Sm | 32×32 px | Compact galleries or inline previews. |
| Md | 56×56 px | Standard documentation and customiser pages. |
| Lg | 80×80 px | Large focus cards or hero sections. |

## Helper Functions

### render

```rust
swatch::render(props: Props) -> Markup
```

Renders a single swatch chip with label and optional sublabel.

### render_scale

```rust
swatch::render_scale(name: &str, stops: &[(&str, &str)]) -> Markup
```

Renders a horizontal tone ramp — one row of N chips sharing a name and labelled by tone key (e.g., `50`, `100`, … `950`). Each chip is clickable to copy its hex value.

**Parameters:**
- `name`: Ramp name (e.g., "slate", "blue").
- `stops`: Array of `(key, hex)` tuples. Key is typically `"50"`, `"100"`, etc.; hex is the colour value.

### render_tokens

```rust
swatch::render_tokens(tokens: &[(&str, &str)]) -> Markup
```

Renders a grid of design-token swatches — one chip per maud-ui `--mui-*` token. Pass an array of `(label, token-name)` pairs. Each chip reads `var(--{token-name})` at paint time, so the grid re-renders when `data-theme` flips or tokens are mutated. Perfect for live theme previews.

### tailwind_ramp

```rust
swatch::tailwind_ramp(name: &str) -> Vec<(&'static str, &'static str)>
```

Returns a Tailwind-style tone ramp for a given colour name. Supported names: `slate`, `gray`, `zinc`, `neutral`, `stone`, `red`, `orange`, `amber`, `yellow`, `lime`, `green`, `emerald`, `teal`, `cyan`, `sky`, `blue`, `indigo`, `violet`, `purple`, `fuchsia`, `pink`, `rose`. Returns an empty vec for unknown names.

## Accessibility

- Copyable swatches have `role="button"` and `tabindex="0"` so they're keyboard-accessible.
- Non-copyable swatches have `role="presentation"` and `tabindex="-1"`.
- Title attribute on copyable swatches: `"Click to copy"`.
- Copy value is written to `data-swatch-value` so the behaviour script reads it.

## CSS Classes

| Class | Purpose |
|-------|---------|
| `mui-swatch` | Root swatch container. |
| `mui-swatch--sm` | Small size variant. |
| `mui-swatch--md` | Medium size variant (default). |
| `mui-swatch--lg` | Large size variant. |
| `mui-swatch--copyable` | Applied when `copyable: true`. |
| `mui-swatch__chip` | Colour display box. |
| `mui-swatch__body` | Label and sublabel container. |
| `mui-swatch__label` | Primary label text. |
| `mui-swatch__sub` | Sublabel text (smaller, muted). |
| `mui-swatch__scale` | Applied to scale ramp chips. |
| `mui-swatch__scale-label` | Ramp name heading. |
| `mui-swatch__scale-row` | Horizontal row of ramp chips. |
| `mui-swatch__scale-key` | Tone key label (50, 100, etc.). |
| `mui-swatch__grid` | Grid wrapper for token swatches. |

## Behaviour

The companion JS behaviour (`dist/behaviors/swatch.js`) handles:

- Listening for clicks on `[data-mui="swatch-copy"]` elements.
- Reading the colour value from `data-swatch-value`.
- Copying to clipboard via the Clipboard API.
- Optional success feedback (e.g., toast notification).

## Related

ThemeCustomiser, ColourPicker, ColorInput.

## Shadcn reference

Swatch is a maud-ui original primitive for design-token visualization and colour management.
