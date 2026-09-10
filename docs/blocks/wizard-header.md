# Wizard header

`blocks::wizard::header` keeps a journey's name, human context and current step together. It renders no application shell and no business controls. Put the step content directly after it. Carry guest, room and dates in `context`; do not expose schema or workflow identifiers.

```rust
use maud_ui::blocks::wizard::header::{self, Props};
let header = header::render(Props {
    title: "Check in guest".into(),
    context: Some("Leila Morgan · Garden suite · 10–11 September".into()),
    steps: vec!["Select booking".into(), "Verify guest".into(), "Check in".into()],
    current: 2,
    ..Default::default()
});
```

`current` is one-based and clamped to the supplied list. Previous steps receive visible check marks and accessible completion text. Exactly one step has `aria-current="step"`; `complete: true` explicitly marks all complete and removes the current state. Empty steps render only the title/context. The compact list becomes vertical on a phone, keeping every label readable.

The application decides eligibility, completion and backward navigation. Focus the next step's heading or first field after replacing its content; do not focus this reusable header on unrelated swaps. See the interactive [workflow fixture](/fixtures/workflow-lodge-comfortable.html).
