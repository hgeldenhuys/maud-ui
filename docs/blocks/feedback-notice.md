# Result notice

`blocks::feedback::notice` keeps the outcome visible. Lead with a sentence such as “Guest saved.” or “The payment could not be recorded.” Add recovery guidance in `description`; reserve `details` for diagnostic text behind a native disclosure. Errors use `role="alert"`; other results use `status`. Nothing auto-dismisses or steals focus on success.

```rust
use maud_ui::blocks::feedback::notice::{self, Props, Tone};
let saved = notice::render(Props {
    message: "Guest saved.".into(),
    description: Some("Leila Morgan is ready for a booking.".into()),
    tone: Tone::Success,
    ..Default::default()
});
```

`notice::region("save-result")` creates an empty region for dynamic outcomes. `MaudUI.notice(element, {status: 'error', message, description, details})` safely inserts text, including diagnostic characters. A blank error message falls back to “Your changes could not be saved.” The application maps transport codes and business failures into appropriate human wording.

See [Form](../components/form.md) for pending, validation and the response contract; [Confirm](feedback-confirm.md) for a deliberate action. Run the interactive [workflow fixtures](/fixtures/workflow-lodge-comfortable.html) for success, validation failure, server failure and retry.
