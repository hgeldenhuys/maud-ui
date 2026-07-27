# StreamingCursor

The blinking caret shown while an AI response is still streaming — a tiny animated indicator appended to in-progress text (also ships two sibling animations, a thinking-dots pulse and a status-beacon ring pulse, behind the same `render`/`Props`).

## Import

```rust
use maud_ui::primitives::streaming_cursor::{self, Variant, Props};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::streaming_cursor::{self, Variant, Props};

html! {
    p {
        "The answer is 42"
        (streaming_cursor::render(Props { variant: Variant::Cursor, ..Default::default() }))
    }
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `variant` | `Variant` | `Variant::Cursor` | Which animation to render |
| `label` | `Option<String>` | `None` | Optional label rendered next to the animation (e.g. "thinking", "writing…") |

## Variants / Enums

### Variant
- `Cursor` (default): blinking block cursor, appended to streaming text
- `Dots`: three-dot pulse ("Claude is thinking…")
- `Pulse`: ring pulse (status beacon)

## Helper Functions

None. The module exposes only `render` and `showcase` — no other `pub fn`.

## Accessibility

- **`aria-hidden` on the decorative indicator: yes.** Each variant's animated element is explicitly marked `aria-hidden="true"` in the source — `span.mui-streaming__cursor aria-hidden="true"`, `span.mui-streaming__dots aria-hidden="true"`, and `span.mui-streaming__pulse aria-hidden="true"`. The outer wrapper (`span.mui-streaming`) instead carries `role="status" aria-live="polite"`, so a screen reader is not told about the blinking caret itself but will politely announce the optional `label` text when it changes.
- **`prefers-reduced-motion` support: yes.** `css/components/streaming_cursor.css` ends with a `@media (prefers-reduced-motion: reduce)` block setting `animation: none` on `.mui-streaming__cursor`, `.mui-streaming__dots span`, and `.mui-streaming__pulse`. Each one degrades to its **static but still visible** state — a solid caret, three solid dots, a solid status dot — rather than disappearing: none of the base rules set `opacity` or `transform`, so removing the animation leaves the indicator on screen. The indicator keeps indicating; it just stops moving. Added in 0.4.0; before that all three animated unconditionally.

## Related

code_block, message, diff, tool_call (AI-chat/agent family), spinner (for non-streaming waits)

## Shadcn reference

No shadcn equivalent — this is a streaming/AI-chat-specific primitive with no upstream shadcn/ui component.
