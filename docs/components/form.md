# Form

The `<form>` element itself. No primitive emitted one before this — every form in `src/blocks/**` hand-wrote the tag, so the submission contract (method, encoding, validation) was retyped at each site and could drift between them.

`form` is deliberately **semantic only**: it applies no layout. A form is almost always a column of fields, and [stack](stack.md) already is that column — making `form` a second flex container would mean two ways to express one thing. Compose them: `form` carries the submission contract, `stack` carries the rhythm. `form::stacked` does exactly that pairing in one call.

## Import

```rust
use maud_ui::primitives::form::{self, Enctype, Method, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::{button, form, input};

html! {
    (form::stacked("/account/profile", html! {
        (input::render(input::Props {
            name: "display_name".into(),
            id: "display-name".into(),
            ..Default::default()
        }))
        (button::render(button::Props {
            label: "Save".into(),
            variant: button::Variant::Primary,
            button_type: "submit",
            ..Default::default()
        }))
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `action` | `Option<String>` | `None` | Submission target; `None` omits the attribute, submitting to the current URL |
| `method` | `Method` | `Post` | HTTP method — **diverges from HTML's `GET`**, see Accessibility & safety |
| `enctype` | `Enctype` | `UrlEncoded` | Body encoding; the attribute is omitted for the default |
| `id` | `Option<String>` | `None` | For `aria-labelledby`, and for submit buttons placed outside the form via `form="<id>"` |
| `aria_label` | `Option<String>` | `None` | Accessible name — required for the form to be a landmark |
| `novalidate` | `bool` | `false` | Skip the browser's built-in constraint validation |
| `autocomplete` | `bool` | `true` | `false` emits `autocomplete="off"` |
| `feedback` | `bool` | `false` | Enhance native validation with inline messages, focus and pending submit controls |
| `pending_label` | `String` | `"Saving…"` | Submit button text while a feedback-enabled request is pending |
| `children` | `Markup` | `html! {}` | Fields and the submit control |

## Variants / Enums

### Method
- `Get`: values are appended to the URL as a query string — correct for searches and filters, whose results should be linkable and bookmarkable
- `Post`: values travel in the request body — **the default here**
- `Dialog`: closes the surrounding `<dialog>` and submits nothing over the network; only meaningful inside [dialog](dialog.md)

### Enctype
Only meaningful for `Method::Post`.

- `UrlEncoded`: `application/x-www-form-urlencoded` — the HTML default; the attribute is omitted
- `Multipart`: `multipart/form-data` — **required** for any form containing a file input. Without it the browser submits the filename and not the file
- `TextPlain`: `text/plain` — debugging only; it does not escape reliably

## Helper Functions

| Function | Signature | Purpose |
|----------|-----------|---------|
| `stacked` | `fn(impl Into<String>, Markup) -> Markup` | A `POST` form wrapping its children in a vertical stack — the shape almost every form takes |
| `Method::as_attr` | `fn(self) -> &'static str` | The `method` attribute value |
| `Enctype::as_attr` | `fn(self) -> Option<&'static str>` | The `enctype` attribute value, or `None` for the HTML default |

## Accessibility & safety

**`method` defaults to `Post`, not HTML's `GET`.** This is the one place the primitive knowingly diverges from the platform. `GET` serialises every field into the URL, where it lands in browser history, server access logs, and the `Referer` header sent to third parties. A login form whose author forgot to set `method` should not leak the password that way. Set `Method::Get` explicitly for searches and filters, where a linkable result URL is the point.

A `<form>` is exposed as a `form` landmark **only** when it has an accessible name, so pass `aria_label` on any page carrying more than one form. A single unnamed form is fine — it simply is not a landmark, which is the correct outcome when there is nothing to distinguish it from.

`novalidate` turns off the browser's constraint validation, including the focus management and error announcement that come with it. Use it only when the server owns the validation rules and reports them back into the markup; do not use it to suppress inconvenient built-in messages.

## Related

Stack (the column a form's fields live in — `form::stacked` pairs them), Field (label + control + description + errors), Fieldset (grouping related controls under a legend), Button (`button_type: "submit"`), Input, Textarea, Select, Checkbox

## Shadcn reference

https://ui.shadcn.com/docs/components/base/form — note that shadcn's Form is a React Hook Form binding, a different concern. This primitive is the HTML element and its submission contract; validation state is [field](field.md)'s job.

## Shared form actions
Use [action_row](../blocks/action-row.md) for one Save/Cancel density. `Action::submit("Save")` submits the surrounding form without nesting a form; `Action::link("Cancel", href)` preserves native navigation. The form primitive remains semantic-only.

## Real-use feedback (0.14.0)

Set `feedback: true` to keep native constraints while adding persistent inline errors, first-invalid-field focus, pending button text and duplicate-submit protection. Set `pending_label` to the action in progress, such as “Saving guest…”. The default stays semantic-only for existing consumers. `novalidate` and submitter `formnovalidate` still opt out of native constraints.

```rust
use maud::{html, Markup};
use maud_ui::{primitives::form, blocks::feedback::notice};
let page_form: Markup = form::render(form::Props {
    id: Some("guest-form".into()), action: Some("/guests".into()),
    feedback: true, pending_label: "Saving guest…".into(),
    children: html! {
        // Compose field::render(...) here, with native required/email/etc. constraints.
        (notice::region("guest-result"))
        button type="submit" { "Save guest" }
    }, ..Default::default()
});
```

The library does not choose a transport or serialize business values. A normal POST navigates normally. An async adapter prevents the submit event and **must settle every outcome**, including network/JSON failures:

```javascript
form.addEventListener('submit', async event => {
  event.preventDefault();
  try {
    const response = await fetch(form.action, {
      method: 'POST', body: new FormData(form)
    });
    const result = await response.json();
    MaudUI.formFeedback(form, result);
  } catch (error) {
    MaudUI.formFeedback(form, {
      status: 'error', message: 'The guest could not be saved.',
      description: 'Your entries are still here. Try again.', details: error.message
    });
  }
});
```

Result contract: `{status: 'success'|'error', message, description?, details?, errors?: {fieldName: 'Human correction'}}`. Error keys may also be control IDs. Return accurate human copy and map backend field paths to actual form control names. The library uses text nodes, retains values, shows a persistent [notice](../blocks/feedback-notice.md), focuses the first editable invalid field in document order, and otherwise focuses the failure notice. Success announces without moving focus. Field errors append their IDs to `aria-describedby`, preserving existing hints. Required radio groups share one error that clears when a choice is selected. For initial server rendering, provide `field::Props.error/errors`; with the runtime, the association is automatic. Without JavaScript, set native `aria-invalid` and `aria-describedby` on custom control markup using the documented `{id}-err` / `{id}-desc` IDs.

`MaudUI.formPending(form, true, submitter)` and `MaudUI.formPending(form, false)` expose the pending contract for other transports, including HTMX. Submit/reset controls are disabled while pending; text fields stay successful controls. A named submitter gets one temporary hidden mirror so `new FormData(form)` retains its name/value. Do not append that submitter again. Original disabled states, button children and ARIA attributes return after settlement. Image-submit coordinate payloads are not supported by this enhancement; use ordinary named submit buttons. Calling `form.submit()` bypasses browser events and validation; use `requestSubmit()`.

Applications using their own async validation must settle a prevented submit or explicitly clear pending. `reset` clears feedback; application cancellation must handle any in-flight request before resetting. History restoration releases stale pending state. After replacing form/error fragments call `MaudUI.init(fragment)`; HTMX swap/settle hooks already integrate with field wiring. Use [Confirm](../blocks/feedback-confirm.md) for modal approval; full-page Cancel is ordinary navigation supplied by the application.

Run `cargo run --example workflow_fixture` and `node tests/workflow-browser.mjs` for 24 real Chrome cases. The adapter and local HTTP server are test/example code, not part of the shipped runtime.
