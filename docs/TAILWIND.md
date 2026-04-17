# Using maud-ui with Tailwind CSS

maud-ui pairs cleanly with Tailwind. All component classes are prefixed with `mui-`, so nothing collides with Tailwind utilities or your app's classes.

## TL;DR

```html
<!-- maud-ui handles the component styling -->
<link rel="stylesheet" href="/dist/maud-ui.min.css">

<!-- Tailwind handles layout, spacing, typography around it -->
<link rel="stylesheet" href="/dist/tailwind.css">

<!-- Use Tailwind for layout, maud-ui for the widget -->
<div class="grid grid-cols-3 gap-4 p-8">
    <!-- maud-ui renders its own classes here -->
    (button::render(...))
</div>
```

No special config is needed. Tailwind won't touch `mui-*` classes, maud-ui won't touch Tailwind utilities.

---

## 1. Why this works

maud-ui follows BEM naming with a `mui-` prefix — every class shipped by the library starts with `mui-`:

```
mui-btn, mui-btn--primary, mui-dialog__content,
mui-card__header, mui-table__row, ...
```

Tailwind v3 and v4 generate only their own utility classes (`flex`, `p-4`, `text-sm`, etc.). There is zero overlap, so both can live in the same HTML without fighting.

## 2. Tailwind preflight (reset)

Tailwind injects a CSS reset called **Preflight** that resets margins, button styles, list styles, etc. maud-ui's component CSS is already specific enough to override most of it, but there are two edge cases worth knowing:

### Button reset
Preflight sets `button { background: transparent; ... }`. maud-ui's `.mui-btn` selector is more specific (it includes variant + size), so this is fine out of the box.

### Heading margins
Preflight zeroes `h1`–`h6` margins. maud-ui relies on this — it deliberately does not add heading margins so you can layout headings with Tailwind spacing. No conflict.

### If you want to opt out of Preflight

If you only want Tailwind utilities without the reset, disable Preflight in your config:

```js
// tailwind.config.js (v3)
module.exports = {
  corePlugins: {
    preflight: false,
  },
};
```

```css
/* tailwind.css (v4) */
@import "tailwindcss/utilities";  /* skip preflight + components layer */
```

This is useful if you want maud-ui to be the sole source of base-element styling.

## 3. Tailwind as layout-only

A common pattern: use maud-ui for **components** (buttons, dialogs, tables, forms) and Tailwind for **layout and spacing** between them.

```rust
use maud::{html, Markup};
use maud_ui::primitives::{button, card};

fn dashboard() -> Markup {
    html! {
        div class="grid grid-cols-1 md:grid-cols-3 gap-6 p-8" {
            (card::render(card::Props {
                title: Some("Revenue".into()),
                children: html! { p class="text-2xl font-bold" { "$12,345" } },
                ..Default::default()
            }))
            (card::render(card::Props {
                title: Some("Users".into()),
                children: html! { p class="text-2xl font-bold" { "1,234" } },
                ..Default::default()
            }))
            (card::render(card::Props {
                title: Some("Sessions".into()),
                children: html! { p class="text-2xl font-bold" { "8,901" } },
                ..Default::default()
            }))
        }

        div class="mt-8 flex justify-end" {
            (button::render(button::Props {
                label: "Export".into(),
                ..Default::default()
            }))
        }
    }
}
```

maud-ui renders the card + button classes. Tailwind handles the grid, spacing, text size, and alignment. Each tool does what it's best at.

## 4. Theming — two design systems side-by-side

maud-ui uses CSS custom properties on `[data-theme="dark"]` / `[data-theme="light"]`. Tailwind uses its own color tokens from your config.

You can either:

### Keep them separate (default)

maud-ui components use `--mui-accent`, `--mui-bg`, etc. Tailwind utilities use Tailwind's palette (`bg-blue-500`, `text-zinc-100`). They coexist without interaction.

### Share tokens between them

Point Tailwind's theme at maud-ui's CSS variables so `bg-accent` and `.mui-btn--primary` use the same color:

```js
// tailwind.config.js (v3)
module.exports = {
  theme: {
    extend: {
      colors: {
        accent: "var(--mui-accent)",
        "accent-hover": "var(--mui-accent-hover)",
        "bg-card": "var(--mui-bg-card)",
        border: "var(--mui-border)",
      },
    },
  },
};
```

```css
/* tailwind.css (v4) */
@theme {
  --color-accent: var(--mui-accent);
  --color-accent-hover: var(--mui-accent-hover);
  --color-bg-card: var(--mui-bg-card);
  --color-border: var(--mui-border);
}
```

Now `bg-accent` and `.mui-btn--primary` switch together when you flip `data-theme` on `<html>`. The theme toggle (dist/behaviors/theme.js) continues to work unchanged.

## 5. Dark mode coordination

maud-ui uses `[data-theme="dark"]` on `<html>`. Tailwind's default dark mode selector is `.dark` or `media`. Align them:

```js
// tailwind.config.js (v3)
module.exports = {
  darkMode: ["class", '[data-theme="dark"]'],
};
```

```css
/* tailwind.css (v4) */
@custom-variant dark (&:where([data-theme="dark"], [data-theme="dark"] *));
```

Now `dark:bg-zinc-900` fires on the same attribute maud-ui uses.

## 6. Safelist (if you render classes dynamically)

If you inject `mui-*` classes via string interpolation and use Tailwind's JIT purge, add the prefix to your content sources. Your Rust files already contain the literal class names, so make sure your Tailwind config includes them:

```js
// tailwind.config.js
module.exports = {
  content: [
    "./src/**/*.rs",         // maud-ui class literals in Rust
    "./templates/**/*.html",
    // Don't need to safelist mui-* — Tailwind doesn't touch them
  ],
};
```

You generally don't need `safelist` because Tailwind only generates classes it finds in your source files, and maud-ui classes are already in its own Rust source — not yours.

## 7. Layer order

If you use Tailwind's `@layer` directives, place maud-ui **before** utilities so Tailwind utilities can override component styles when you want:

```css
/* app.css */
@import "tailwindcss/base";
@import "tailwindcss/components";
@import "maud-ui/dist/maud-ui.css";   /* ← components layer */
@import "tailwindcss/utilities";      /* ← utilities win */
```

With this order:
- `<button class="mui-btn mui-btn--primary w-full">` → w-full overrides any width maud-ui sets.
- `<button class="mui-btn mui-btn--primary">` → maud-ui's styling wins because nothing overrides it.

## 8. Tree-shaking / smaller bundles

If you use a subset of maud-ui components, you can skip the full `maud-ui.css` bundle and import only what you need:

```css
@import "maud-ui/css/maud-ui.css";      /* just tokens */
@import "maud-ui/css/components/button.css";
@import "maud-ui/css/components/dialog.css";
@import "maud-ui/css/components/card.css";
/* skip the 50+ components you're not using */
```

This trims 60-80 KB if you only need 5-10 components.

## 9. Known interactions

| Interaction | Result |
|---|---|
| `hover:` variants on Tailwind | ✅ Works on any element, including maud-ui components |
| `class="mui-btn bg-red-500"` | ✅ Tailwind wins because it's in utilities layer |
| Tailwind arbitrary values `[color:var(--mui-accent)]` | ✅ Works |
| Tailwind `@apply` inside `.mui-btn` override | ⚠️ Possible but not recommended — you lose maud-ui's variant system |
| Tailwind Typography plugin (`prose`) on maud-ui content | ✅ Fine, but scope it away from component internals (`not-prose` on `.mui-card__body`) |

## 10. Full working example

See `examples/showcase.rs` for a maud + axum server that embeds the bundled CSS + JS. To add Tailwind:

```rust
async fn serve_tailwind() -> impl IntoResponse {
    let css = include_str!("../dist/tailwind.css");
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "text/css; charset=utf-8")],
        css,
    )
}

// in main():
.route("/css/tailwind.css", get(serve_tailwind))
```

Then in your `<head>`:

```html
<link rel="stylesheet" href="/css/maud-ui.css">
<link rel="stylesheet" href="/css/tailwind.css">
```

Both stylesheets cooperate. Ship it.
