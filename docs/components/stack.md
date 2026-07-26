# Stack

The general-purpose layout container: a flex box along one axis with a gap drawn from the shared spacing scale. Where every other primitive renders *content*, `stack` renders *structure* — it is what lets a page be composed as a tree of containers holding leaves, instead of each block hand-writing `style="display:flex;…"`.

Every appearance prop is a closed enum, so the legal values can be enumerated by tooling (a manifest, a builder UI's dropdown) rather than guessed from a string.

## Import

```rust
use maud_ui::primitives::stack::{self, Align, Direction, Justify, Props, Space, Tag};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::{badge, stack};

html! {
    (stack::render(stack::Props {
        direction: stack::Direction::Horizontal,
        gap: stack::Space::Sm,
        align: stack::Align::Center,
        justify: stack::Justify::Between,
        children: html! {
            span { "Status" }
            (badge::render(badge::Props {
                label: "Healthy".into(),
                variant: badge::Variant::Success,
                ..Default::default()
            }))
        },
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `direction` | `Direction` | `Vertical` | Main-axis direction — column or row |
| `gap` | `Space` | `Md` (`0.75rem`) | Space between children |
| `padding` | `Space` | `None` | Inset applied by the container itself; `None` keeps a stack a drop-in wrapper |
| `align` | `Align` | `Stretch` | Cross-axis alignment (`align-items`) |
| `justify` | `Justify` | `Start` | Main-axis distribution (`justify-content`) |
| `wrap` | `bool` | `false` | Allow children to wrap onto additional lines |
| `tag` | `Tag` | `Div` | The HTML element to render as |
| `id` | `Option<String>` | `None` | Optional `id`, for anchor targets and `aria-labelledby` references |
| `aria_label` | `Option<String>` | `None` | Accessible name — **required** when `tag` is `Section` |
| `children` | `Markup` | `html! {}` | The stacked content |

## Variants / Enums

### Direction
- `Vertical`: children flow top-to-bottom (`flex-direction: column`) — the default
- `Horizontal`: children flow left-to-right (`flex-direction: row`)

Reversed directions are deliberately absent: `row-reverse` / `column-reverse` decouple visual order from DOM order, which sends keyboard focus and screen-reader output somewhere other than what a sighted user sees. Reorder the children instead.

### Space
Shared by `gap` and `padding`. Mirrors `tokens::spacing`, resolving through the `--mui-space-*` custom properties, so a consumer theme retunes the whole scale in one place.

- `None`: `0` — the `padding` default
- `Xs`: `0.25rem`
- `Sm`: `0.5rem`
- `Md`: `0.75rem` — the `gap` default
- `Lg`: `1rem`
- `Xl`: `1.5rem`
- `Xxl`: `2rem`

### Align
Cross-axis alignment (`align-items`).

- `Stretch`: children fill the cross axis — the default
- `Start`: pack to the cross-axis start (`flex-start`)
- `Center`: centre on the cross axis
- `End`: pack to the cross-axis end (`flex-end`)
- `Baseline`: align children's text baselines

`Stretch` is the default because it matches flexbox's own, and it is what you want for a column of inputs or cards. It is **not** what you want for a column containing an intrinsically-sized child — a badge in a `Vertical` stack stretches into a full-width bar rather than staying a chip. Use `Align::Start` there.

### Justify
Main-axis distribution (`justify-content`).

- `Start`: pack to the main-axis start (`flex-start`) — the default
- `Center`: centre on the main axis
- `End`: pack to the main-axis end (`flex-end`)
- `Between`: equal space between children, none at the edges (`space-between`)
- `Around`: equal space around each child (`space-around`)
- `Evenly`: equal space between children and at the edges (`space-evenly`)

### Tag
The element the stack renders as. A page assembled entirely from `div`s has no landmarks, so assistive tech gets one undifferentiated blob.

- `Div`: no semantics — the default
- `Section`: a thematic grouping. **Requires `aria_label`**
- `Article`: a self-contained composition
- `Aside`: tangentially related content (`complementary` landmark)
- `Nav`: navigation links (`navigation` landmark)
- `Header`: introductory content (`banner` landmark at page scope)
- `Footer`: closing content (`contentinfo` landmark at page scope)
- `Main`: the document's primary content — at most one per page

## Helper Functions

| Function | Signature | Purpose |
|----------|-----------|---------|
| `vertical` | `fn(Markup) -> Markup` | Vertical stack with the default gap — the common case, without the `Props` ceremony |
| `horizontal` | `fn(Markup) -> Markup` | Horizontal stack, vertically centred, default gap — a toolbar row, a button pair |
| `Direction::as_class` | `fn(self) -> &'static str` | Modifier class for a direction, empty for the default |
| `Space::gap_class` | `fn(self) -> &'static str` | Modifier class for a step used as `gap`, empty for `Md` |
| `Space::padding_class` | `fn(self) -> &'static str` | Modifier class for a step used as `padding`, empty for `None` |
| `Space::as_length` | `fn(self) -> &'static str` | The CSS length a step resolves to — mirrors `tokens::spacing` |
| `Align::as_class` | `fn(self) -> &'static str` | Modifier class for an alignment, empty for the default |
| `Justify::as_class` | `fn(self) -> &'static str` | Modifier class for a distribution, empty for the default |
| `Tag::as_element` | `fn(self) -> &'static str` | The element name a tag renders as |

## Accessibility

`Tag::Section` MUST be paired with a non-`None` `aria_label`. A `<section>` with no accessible name is stripped of its `region` role by the HTML-AAM mapping, so the grouping the tag was chosen for silently does not exist for screen-reader users. This is enforced at runtime by a `debug_assert!` in debug builds — release builds skip the check, matching `button`'s icon-only `aria_label` rule.

`Tag::Nav` and `Tag::Aside` are landmarks with or without a name, but a page carrying more than one of either should label each so they can be told apart. `Tag::Main` should appear at most once per document.

Reversed flex directions are not offered, so DOM order always matches visual order — keyboard focus and screen-reader reading order follow what is on screen.

## Related

Card (a styled container with header/body/footer, often filled with a stack), Separator (a rule between stacked items), Item (a row with leading/trailing slots), AspectRatio (fixed-ratio box)

## Shadcn reference

No direct shadcn/Base UI equivalent — shadcn leans on Tailwind utility classes (`flex flex-col gap-4`) for this role. `stack` fills the same need in a utility-free, enumerable form.
