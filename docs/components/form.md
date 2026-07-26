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
