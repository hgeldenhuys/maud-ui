# Create the first record

`blocks::worklist::empty` is for a genuinely empty collection. State what is missing, why the first entry helps, and name the create action. Keep the composition left aligned with the list it will replace.

```rust
use maud_ui::blocks::{action::Link, worklist::empty::{self, Props}};
let empty = empty::render(Props {
    title: "No notes yet".into(),
    description: Some("Keep arrival requests and handover details together.".into()),
    create: Some(Link { label: "Create your first note".into(), href: "/notes/new".into() }),
    ..Default::default()
});
```

Omit `create` when that action is not available to the viewer. Do not infer permission from an empty response, and do not use this block while a request is pending or failed. A search with no matches should use [Search results](search-results.md), retaining its query and clear action. Related-record lists may retain their quieter one-line empty state.
