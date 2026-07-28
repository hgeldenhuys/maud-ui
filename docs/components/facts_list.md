# Facts list

Mono subtle labels paired with foreground values, on a 4px rhythm. The key/value register used by the empty-session card and the inspector's SESSION section. Renders as a semantic `<dl>`.

## Import

```rust
use maud_ui::primitives::facts_list::{self, Props, Fact};
```

## Example

```rust
use maud_ui::primitives::facts_list::{self, Props, Fact};

facts_list::render(Props {
    facts: vec![
        Fact::new("model", "sonnet-4.6"),
        Fact::mono("context", "18.4k / 200k"),
        Fact::mono("spend", "$1.84"),
        Fact::new("started", "41m ago"),
    ],
})
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| facts | Vec\<Fact\> | `[]` | The rows, top to bottom. |

## Fact

| Constructor | Value face |
|-------------|-----------|
| `Fact::new(label, value)` | Sans value. |
| `Fact::mono(label, value)` | Mono value — for shas, counts, paths. |

Labels are always mono and subtle; the `mono` flag governs only the value.

## CSS classes

- `mui-facts` — the `<dl>`.
- `mui-facts__row`, `mui-facts__label`, `mui-facts__value` (+ `mui-facts__value--mono`).

## Related

Gutter section (wraps this under a header), Table, Item.
