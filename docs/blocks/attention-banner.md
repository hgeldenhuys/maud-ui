# Attention banner

A supplied operational message with one contextual action. Warning, danger and information tones share the same compact geometry.

## Import and example
```rust
use maud_ui::blocks::{action::Action, attention_banner::{self, Props, Tone}};
attention_banner::render(Props {
    title: "Maya Chen arrives at 11:30 with CAD $560 due".into(),
    subline: Some("Review payment before completing check-in.".into()),
    action: Some(Action::link("Review payment", "/bookings/14/payment")),
    tone: Tone::Warning,
    ..Default::default()
});
```

## Props
| Field | Type | Default | Description |
|---|---|---|---|
| title | String | empty | Visible title and accessible name; caller supplies the reason. |
| subline | Option<String> | None | Supporting context. |
| action | Option<action::Action> | None | One contextual link, submit or POST action. |
| action_markup | Option<Markup> | None | Overrides action, including empty; supply one canonical action. |
| tone | Tone | Warning | Warning, Danger or Info. |
| dismiss | Option<Dismiss> | None | Opt into local dismissal; omit for a required notice. |

`Dismiss { label: String, focus_target: Option<String> }` defaults to "Dismiss notice" and None. `focus_target` is the ID of a visible focusable element outside the banner. Without it, focus moves to the next available control, then the previous one, then the parent region. `Tone::as_str()` returns warning/danger/info.

## Behavior and accessibility
The message is a named aside, with plain text conveying meaning alongside tone. Static notices are not assertive alerts. The action remains usable without JavaScript. A dismiss button is hidden until enhancement is initialized; without JS the notice stays visible. Dismissal hides only this DOM instance, does not write storage or acknowledge business state, and moves focus out of the hidden notice. HTMX-inserted instances use the shared initializer. Required disclosures should never opt into dismissal.

The body and action wrap on phones; light/dark semantic tokens provide surfaces, text and borders. Touch dismissal and actions have a 44px minimum target. The application supplies all balances, readiness and arrival claims; the component derives none of them.

## Related
[Money](record-money.md), [grouped worklist](worklist-grouped.md), [alert](../components/alert.md).

## Presentation states (0.10.1)

`Props::state: maud_ui::blocks::state::State` defaults to `Ready`. `Loading { message }` shows skeletons; `Empty { message, action }` and `Error { message, retry }` provide distinct recovery paths; `Disabled { reason }` retains admitted content in an inert subtree with an external reason. Loading, empty and error omit ready content. Inert disables interaction, not server authorization or submission of values by an enclosing form. Each live API page shows all four states together. Add `state: Default::default()` to exhaustive Props literals.

`State::Absent` (0.11.0) means no input/rule was declared and emits nothing. Keep any caller-owned section heading inside the same conditional. Use Error only for a declared operation that failed; never show an unconfigured-rule message to the user.
