# Composer

The multi-state prompt dock for an agent session view. Server-rendered as a plain `<form>` wrapping a `<textarea rows="2">`, so it submits with JavaScript disabled — auto-grow, `⌘↵`-to-send and voice capture are a JS layer the consumer adds on top.

Four states share one anchor line (the dock never moves): **Ready** (the 84px resting dock), **Growing** (focused, ringed, grows then scrolls), **Executing** (a turn is running; the field stays *writable*, and an `Interrupt` hollow-destructive appears), and **Asleep** (a 46px dashed bar whose "type to wake" field is a real single-row textarea, plus a `Wake` submit).

## Import

```rust
use maud_ui::primitives::composer::{self, Props, State, Chip};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::composer::{self, Props, State, Chip};

composer::render(Props {
    state: State::Ready,
    action: "/session/42/turn".into(),
    placeholder: "Message refactor-auth-middleware…".into(),
    chips: vec![Chip::new("sonnet-4.6"), Chip::new("auto-accept: off"), Chip::new("+ attach")],
    show_voice: true,
    primary_label: "Send".into(),
    primary_kbd: Some("⌘↵".into()),
    status: Some(html! { span { "12 turns · 41.2k ctx · $1.84" } span { "⇧↵ newline" } }),
    ..Default::default()
})
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| state | State | `Ready` | Which dock state to render. |
| action | String | `""` | `<form action>`. |
| method | String | `"post"` | `<form method>`. |
| field_name | String | `"message"` | `<textarea name>`. |
| placeholder | String | `""` | Empty-field placeholder (also the sleep-bar hint). |
| value | String | `""` | Pre-filled draft content. |
| chips | Vec\<Chip\> | `[]` | Leading mono chips (model, auto-accept, attachments). |
| show_voice | bool | `false` | Show the voice-capture button before the primary. |
| secondary_label | Option\<String\> | `None` | Renders a hollow-destructive `Interrupt` button (executing state). |
| secondary_action | Option\<String\> | `None` | `<form action>` for the secondary button. When set, Interrupt is a real submit targeting an empty sibling form via the HTML5 `form` attribute — it POSTs with JS disabled. Unset → `type="button"` (consumer JS wires it). |
| primary_label | String | `"Send"` | Primary submit-button label. |
| primary_kbd | Option\<String\> | `None` | Kbd hint inside the primary button (`⌘↵`). |
| status | Option\<Markup\> | `None` | Mono status line below the field — the "beside" data. |

## State enum

| Value | Description |
|-------|-------------|
| Ready | The 84px resting dock. |
| Growing | Focused; field ringed, grows to a cap then scrolls internally. |
| Executing | A turn is running; field writable, `Interrupt` shown. |
| Asleep | 46px dashed bar: mono state tag, a real single-row textarea ("type to wake" is literal — it submits with JS off), and a `Wake` button. |

## Chip

`Chip::new(label)` for a plain mono chip; `Chip::accent(label)` for the accent-tinted "carries a payload" treatment (attachments, e.g. `user.rs ×`).

## Geometry knobs

- `--mui-composer-max` — the field's `max-width` (default `67.5rem`, the 1080px code measure).
- `--mui-composer-max-h` — the height the field grows to before scrolling (default `40vh`).

## Progressive enhancement

The writable states are a real form: with JS off, the textarea submits and the primary button posts. Auto-grow, `⌘↵`, `esc`-to-interrupt and voice are all additive.

## CSS classes

- `mui-composer` (root `<div>`) + one of `mui-composer--ready` / `--growing` / `--executing` / `--asleep`; the form inside is `mui-composer__form`, the secondary target `mui-composer__secondary`.
- `mui-composer__field`, `__input`, `__actions`, `__chip` (+ `--accent`), `__voice`, `__send`, `__kbd`, `__interrupt`, `__status`.
- Asleep: `mui-composer__sleepbar`, `__state-tag`, `__input--sleep`, `__wake`.

## Related

Turn progress (the strip above the executing composer), Segmented control, Textarea, Kbd.
