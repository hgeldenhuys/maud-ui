# Record facts

Pass all of a record's fact groups to **one** `record::facts` block. Identity, Contact and Loyalty are sections of that surface, not separate cards. Use separate cards only for independent tasks or entities with their own actions. A group with one admitted fact becomes a compact row; a lone single-fact group has no card border, fill or inset.

## Import and example

```rust
use maud_ui::blocks::record::facts::{self, Fact, Group};
facts::render(facts::Props {
    groups: vec![
        Group { title: "Identity".into(), facts: vec![
            Fact::new("Full name", "Sofia Patel"),
            Fact::new("Email", "sofia.patel@example.com"),
        ] },
        Group { title: "Sensitive details".into(), facts: vec![
            Fact::masked("Phone", "••••••0005"),
            Fact::masked("ID number", "••••••0005"),
        ] },
        Group { title: "Loyalty".into(), facts: vec![Fact::mono("Loyalty points", "0")] },
    ],
    ..Default::default()
});
```

## Props and value ownership

`groups: Vec<Group>` owns the sections in caller order. Each `Group` has a human-facing `title` and `facts: Vec<Fact>`. Empty groups are omitted; all groups empty renders nothing. `columns: Columns::{Two, Three, Four}` caps the responsive grid, default Four. Available container width gives one column below 28rem, two at 28rem, up to three at 42rem and up to four at 56rem. A single fact always stays one row.

`Fact::new(label, value)` uses body text. `Fact::mono` uses the mono face. `Fact::masked` expects an **already redacted** value and adds mono text plus the visible `masked_hint` (default `masked`). Masking is never CSS hiding: do not send the underlying secret to this renderer. The caller keeps classification and authorization decisions.

`Fact::value_markup: Option<Markup>` preserves a canonical field emitter, including its links and data attributes; it takes precedence over plain `value`. An explicitly empty fragment omits that fact, including its label. A plain empty string is preserved as supplied. Markup must already be safe and authorized. Plain labels and values are escaped.

`heading: action::Heading` defaults to H2 for section headings below a page H1; `aria_label` defaults to `Record facts`. Set both for nested compositions. `state: State` supports Ready, Absent, Loading, Empty, Error and Disabled using the shared presentation contract. Non-ready replacement states do not render supplied facts.

## Layout and accessibility

Semantic `dl`, `dt` and `dd` keep each label associated with its value. Muted labels and masking hints use the theme's contrast-tested tokens. Masked status has text as well as styling; only its value uses mono. Long admitted values wrap within the grid. Sections share `--mui-radius-lg`, one surface and `--mui-stack-gap`; no nested rounded boxes.

See [Record page](record-page.md), [Related records](record-related-list.md) and [Stack](../components/stack.md). Fixtures: `/fixtures/record-page-{compact,comfortable,spacious}-{guest,long}.html`.
