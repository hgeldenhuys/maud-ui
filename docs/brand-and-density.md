# Brand and density — 0.12.0

The same kit now serves hospitality and banking. The landing's Banking tab contains account balances, exact transaction sorting and a KYC queue, with compact density by default. The theme customiser switches Lodge, Bank and Clinic live and exports one file containing exactly nine public brand tokens.

## The brand contract

Serve maud-ui.css, then your brand.css. Keep identity copy in Rust props and visual identity in these tokens. Both light and dark surfaces remain semantic; a brand does not replace success, warning or error colors.

| Token | Purpose | Default |
|---|---|---|
| `--mui-brand-accent` | Primary control fill; derives active text, focus and selected tint | Light `#285bc5`, dark `#84aaff` |
| `--mui-brand-accent-ink` | Text on the accent fill | Light `#ffffff`, dark `#101b32` |
| `--mui-brand-font-heading` | Heading and wordmark family | Existing Avenir Next / Nunito Sans / platform / system sans stack |
| `--mui-brand-font-body` | Body and control family | Existing Avenir Next / Nunito Sans / platform / system sans stack |
| `--mui-brand-radius` | Medium radius; small is ×0.5; large is min(×1.5, 12px) | `0.375rem` |
| `--mui-brand-density` | Default density: 0 compact, 1 comfortable, 2 spacious | `1` |
| `--mui-brand-logo-mask` | Monochrome logo, as `url(...)`; embedded SVG survives export | Default bank-style glyph |
| `--mui-brand-logo-size` | Logo width and height | `2rem` |
| `--mui-brand-logo-radius` | Logo clipping radius | `0` |

```css
:root {
  --mui-brand-accent: #24428c;
  --mui-brand-accent-ink: #ffffff;
  --mui-brand-font-heading: "Avenir Next", system-ui, sans-serif;
  --mui-brand-font-body: "Avenir Next", system-ui, sans-serif;
  --mui-brand-radius: 0.25rem;
  --mui-brand-density: 0;
  --mui-brand-logo-mask: url("/identity/northline.svg");
  --mui-brand-logo-size: 1.75rem;
  --mui-brand-logo-radius: 0;
}
```

The export contains this set of names and no theme internals. A comment explains the radius formula and includes the live resolved small/medium/large values. Controls use Small capped at 8px; compact indicators use Small capped at 4px. The corner input accepts one scalar length. Copy CSS and Download brand.css use the same builder. Built-in logos are embedded data URLs; custom remote/relative URLs remain caller-managed assets. Font names do not download fonts: supply an installed fallback or your own `@font-face`. Logo masks take their ink from the derived accent text; use the logo markup slot for a full-color image.

The three examples come from `static/brands/brands.json`. Lodge uses green, a serif heading and comfortable spacing; Bank uses blue, sans-serif headings and compact spacing; Clinic uses teal, softer corners and spacious spacing. `python3 examples/generate-brands.py` regenerates Rust metadata, CSS presets and runtime data. `--check` verifies all three outputs. Choosing a brand replaces advanced preview overrides; advanced theme controls remain available, but their independent adjustments are excluded from brand.css.

## Shared brand mark

```rust
use maud::html;
use maud_ui::blocks::shell::{app_header, brand_mark, sidebar};

let identity = brand_mark::Props {
    wordmark: "Northline".into(),
    href: Some("/accounts".into()),
    tagline: Some(html! { "Business banking" }),
    ..Default::default()
};
let header = app_header::render(app_header::Props {
    brand_mark: Some(identity.clone()), ..Default::default()
});
let shell = sidebar::render(sidebar::Props {
    brand_mark: Some(identity), app_header: header, ..Default::default()
});
```

`brand_mark` accepts wordmark, href, optional logo markup and a tagline markup slot. Empty wordmark emits nothing. The tagline stays outside the wordmark link, preserving nested-link safety. The image is decorative; the wordmark names the identity. `brand_mark::preset("lodge" | "bank" | "clinic")` returns optional example props; pair with the matching `data-brand` scope for example visuals.

App-header and sidebar Props gain `brand_mark: Option<brand_mark::Props>`. It overrides the existing raw brand slot when supplied. Existing raw markup remains supported. Exhaustive Props literals need the new field. The desktop icon rail hides the text and its link together; the full wordmark returns in the phone drawer.

## Document density

Set `data-density="compact|comfortable|spacious"` on `<html>`. This explicit choice wins over a brand's default. Density never moves the frame. The landing density selector changes content throughout the document and saves the choice; selecting the Banking tab uses compact only when no density was explicitly chosen. Without JavaScript, the Banking example has a compact preview scope. `data-mui-density-scope` is for independently framed gallery examples; application content can inherit from the root or set the same `data-density` attribute on `main` / a container. The existing action-row attribute remains a separate component variant.

| Property | Compact | Comfortable | Spacious |
|---|---:|---:|---:|
| Standard single-line control | 32px | 36px | 40px |
| Row button / overflow / status chip minimum | 32px | 32px | 36px |
| Small text | 13px | 14px | 15px |
| Body text | 14px | 15px | 16px |
| Caption text | 12px | 12px | 13px |
| Card block / inline inset | 16 / 20px | 20 / 24px | 24 / 28px |
| Table cell block inset | 6px | 8px | 10px |

Controls retain a 44px coarse-pointer floor and touch text inputs retain 16px text. Badges remain 22px and chip count bubbles 18px. The page header remains a 56px row at ≥64rem in every density. Multiline content determines its own height. The existing action-row density prop still chooses its action style; root density scales its geometry. Cards, record kit, task grid, grouped worklists, banners and state wrappers use the derived control/type/inset roles. Shell chrome uses independent `--mui-shell-*` geometry and type; its 64px masthead, 56px page bar, 240px column, 24px desktop gutters and minimum 64px footer do not follow density. See [Shell frame](shell-frame.md).

Existing `--mui-accent`, `--mui-accent-fg`, font and radius roles now resolve through the brand. `--mui-accent-hover` uses the accent fill with shared hover feedback. Accent text/focus mixes 40% accent with 60% theme text; selected tint mixes 8% accent with 92% card surface. The latter keeps muted labels readable even under the bank's darker accent. Surface, status, spacing, motion and shadow palette values otherwise remain available as before. New internal density roles are `--mui-density-level`, `--mui-density-choice`, `--mui-row-height`, `--mui-card-inset-block` and `--mui-card-inset-inline`; applications override the nine brand tokens or the root attribute instead.

## Feedback and input absence

At ≥64rem the page header uses one fixed 56px grid row with a visible native search field capped at 20rem. At 64–80rem switchers use a Settings disclosure; at ≥80rem they are inline. Below 64rem search collapses to an icon while breadcrumbs and switchers move into the drawer. Escape closes phone search, retains its query and restores summary focus; desktop search remains visible. A custom `search_markup` slot stays caller-owned and is capped at 20rem; use an icon trigger for a command palette.

Task grids size to their count: zero emits nothing, one is a compact full-width row, and multiple tasks use available columns. Descriptions retain complete text with a title hint and CSS ellipsis. `related_card::Variant::Compact` retains supplied facts in a dense card; Inline also retains facts when explicitly requested. Use admitted guest contact and room facts once, instead of repeating Guest/Room in an overview below them.

`State::Absent` means no rule or input was declared and emits nothing, including no heading or status wrapper. All 13 operational blocks support it. A blank attention title also emits nothing. Keep an optional section heading inside the same caller-side conditional. Error remains for a declared operation that actually failed; an unconfigured rule is never user-facing error copy.

The application lane still owns the manager-home exception rule, duplicate Booking Overview rows, and the Today count string. Omit the entire absent section; format a supplied count as “2 arriving · 1 departing · of 10 visible” or omit the scope caption. The library cannot infer whether a financial value or rule was admitted. The banking example has no developer-facing missing-input messages.

## Exact numbers and verification

The banking fixture formats integer cents, includes full USD precision and aligns account balances, holds and transaction amounts in tabular columns. Debit amounts use a visible minus sign. KYC review opens the original native details panel and returns focus on close; it does not send a request or change a real account.

The shared data-table sorter now compares signed decimals with BigInt and aligns their decimal scales. It preserves cent differences beyond JavaScript's safe integer range, grouping commas, optional currency symbols and Unicode minus. Malformed grouping, empty values, scientific notation and nonnumeric text use text comparison. Columns must represent comparable units/currencies; this does not perform currency conversion or locale-specific parsing. Original row nodes remain intact through sorting/filtering.

Gates cover rendering, generated metadata, both emitted bundles, density arithmetic, exact comparison, brand persistence/export, semantic contrast, HTTP identities and script parsing. These are source/DOM fixtures, not visual measurements. Supervisor browser review covers the 56px app bar with real toolbar content, keyboard behavior, phone numeric-table scrolling, and all three brands in both themes.
