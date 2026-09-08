# Task grid

Compact launcher for common tasks: three equal columns on larger screens, one below 40rem. Each card carries a title, one-line description and one action.

## Import and example
```rust
use maud_ui::blocks::{action::Action, task::grid::{self, Props, Task}};
grid::render(Props {
    tasks: vec![Task {
        title: "Check in a guest".into(),
        description: "Confirm a reservation and assign a room.".into(),
        action: Action::link("Start check-in", "/check-in"),
    }],
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| tasks | Vec<Task> | empty | Task has title: String, description: String, action: Action. |
| aria_label | String | Available tasks | Names the list. |
| heading | action::Heading | H3 | Heading level of each card. |

## Composition and accessibility
Use action labels that make sense on their own, such as Start check-in. Actions support the shared [link/POST contract](worklist-header.md). Titles wrap; descriptions visually ellipsize but remain complete in the DOM and title attribute. Footers align after varying title lengths. An empty task vector renders an empty list; compose `empty_state` when you need recovery guidance. No JavaScript is required.
