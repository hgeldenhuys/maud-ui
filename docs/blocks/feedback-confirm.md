# Confirm an action

`blocks::feedback::confirm` composes a native modal dialog and a real POST form. Name the consequence and put the guest, room, dates or amount in the message. The first focus is Cancel; Escape closes and returns focus to the trigger. Tab reaches the confirming verb, and Enter activates that focused button. No `window.confirm` is used. Background content is inert while the modal is open.

```rust
use maud_ui::blocks::feedback::confirm::{self, Props};
let trigger = confirm::trigger("check-in", "Review check-in");
let dialog = confirm::render(Props {
    id: "check-in".into(), title: "Check in Leila Morgan?".into(),
    message: "Garden suite · 10–11 September. Mark this booking as checked in.".into(),
    confirm_label: "Check in guest".into(), cancel_label: "Keep booking".into(),
    pending_label: "Checking in…".into(), action: "/bookings/B-410/check-in".into(),
    hidden_fields: vec![("booking".into(), "B-410".into())],
    ..Default::default()
});
```

Render both outside any existing form. Include CSRF values in `hidden_fields`. The generated form ID is `{id}-form`; use the [Form response API](../components/form.md) for asynchronous results. Keep failures in the dialog. On confirmed success, close the dialog and publish the human result in a notice outside it; the [workflow fixture](/fixtures/workflow-lodge-comfortable.html) shows this adapter. Do not put transaction handlers on a plain dialog-close button.

The gallery preview is a local simulation. Real applications own authorization, request encoding, execution and response handling. Cancellation does not retract an already submitted request; disable or relabel cancellation during irreversible work in the application if required by its semantics.
