# Search results

`blocks::search::results` renders the query, count and native linked rows. Each row has one human identity, a quiet kind, optional reference and useful description. Exact duplicate descriptions are omitted; the caller should supply distinct snippets. Native links preserve Tab, Enter, open-in-new-tab and browser history.

```rust
use maud_ui::blocks::search::results::{self, Item, Props};
let results = results::render(Props {
    query: "Leila".into(),
    items: vec![Item { title: "Leila Morgan".into(), kind: "Guest".into(),
        description: Some("leila.morgan@example.com".into()),
        href: "/guests/G-204".into(), ..Default::default() }],
    ..Default::default()
});
```

`total: None` means the supplied list is complete. Supply the authoritative total for a page of results; a separate showing-count then appears. Loading, empty, error and disabled states use `blocks::state::State`. An empty Ready list keeps the query and offers `clear_action`, if supplied. Do not offer “create the first record” for an unsuccessful search; use [First record](worklist-empty.md) only for a genuinely empty collection.

Search execution, debouncing, query preservation, filtering and authorization stay with the application. The block contains no synthetic combobox/listbox keyboard model.
