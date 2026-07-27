# Message

A chat-style message bubble for a single participant turn (user, assistant, or system). Conductor-flavored: compact density, a colored avatar, an optional live pulse during streaming, and a footer slot for inline tool chips or actions.

## Import

```rust
use maud_ui::primitives::message::{self, Role, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::message;

html! {
    (message::render(message::Props {
        role: message::Role::Assistant,
        author: "Claude".into(),
        avatar_initials: Some("C".into()),
        timestamp: Some("14:32".into()),
        body: html! { p { "I'll add the toggle now." } },
        is_live: true,
        ..Default::default()
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `role` | `Role` | `Role::Assistant` | Who authored the message. |
| `author` | `String` | `""` | Display name (e.g. "Claude", "Sofia"). |
| `avatar_initials` | `Option<String>` | `None` | Avatar initials (e.g. "C"). Falls back to first letter of `author`. |
| `avatar_color` | `Option<String>` | `None` | Optional override for avatar background colour (CSS colour string). |
| `timestamp` | `Option<String>` | `None` | Human-readable timestamp ("2 min ago", "14:32"). |
| `body` | `Markup` | `html! {}` | The message body — may contain markdown-rendered HTML, code blocks, etc. |
| `is_live` | `bool` | `false` | When true, the avatar pulses to indicate an in-progress streaming response. |
| `footer` | `Option<Markup>` | `None` | Footer slot — tool chips, action buttons, attachments. |

## Variants / Enums

### Role

Who the message is from — drives colour and alignment.

- `Assistant` (default): renders with the `mui-message--assistant` class.
- `User`: renders with the `mui-message--user` class.
- `System`: renders with the `mui-message--system` class.

## Helper Functions

This module exposes no public helper functions besides `render` and `showcase`. `initials_of` (computes avatar initials, falling back to the first letter of `author`) and `Role::class` (maps a role to its CSS modifier class) both exist in the module but are private — not part of the public API.

## Accessibility

- The avatar element carries `aria-hidden="true"` — it's treated as decorative since the author name is already exposed as visible text alongside it.
- The `is_live` flag only toggles a CSS class (`mui-message__avatar--live`) for a pulsing animation; the source does not emit any `aria-live` attribute or live region. If streaming updates need to be announced to assistive tech, the consuming app must add its own live region (e.g. around the message list or body content).
- The root element is a plain `div` with `data-mui="message"` and role-derived modifier classes — no ARIA `role` (e.g. `listitem`, `log`) is emitted on the container or its children.
- Author name and timestamp render as plain `span` text with no additional ARIA labeling.

## Related

`code_block`, `diff`, `tool_call`, `streaming_cursor` — sibling primitives in the AI-chat/agent family, typically composed into `message`'s `body` or `footer` slots.

## Shadcn reference

message has no shadcn equivalent — this is a Conductor-specific chat primitive.
