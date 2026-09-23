# Time Split

One bar showing where a sequence of steps spent its time. Use it for a pipeline, a build or a request trace: any ordered list of steps where the reader asks "what took so long". Segments sit in execution order and are sized by their share of measured time, so the shape of the bar is the answer.

## Import

```rust
use maud_ui::primitives::time_split::{self, Props, Segment, SegmentState};
```

## Example

```rust
use maud::html;
use maud_ui::primitives::time_split::{self, Segment};

let steps = vec![
    Segment::done("clone-or-pull", 447, "447 ms"),
    Segment::done("build-in-guest", 149_101, "2m 29s"),
    Segment::done("report", 13, "13 ms"),
];

html! {
    (time_split::render(time_split::Props {
        caption: time_split::dominant_caption(&steps, "2m 30s").unwrap_or_default(),
        aria_label: time_split::aria_from(&steps),
        segments: steps,
    }))
}
```

## Props

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| segments | Vec<Segment> | `vec![]` | Steps in execution order. Leave out steps that never ran; the empty rest of the track shows where the run stopped. |
| caption | String | `""` | The finding, in words. The bar alone only says "one step was big", so always pass one. |
| aria_label | String | `""` | Read to a screen reader in place of the bar. `aria_from` builds one from the segments. |

### Segment

| Field | Type | Description |
|-------|------|-------------|
| name | String | The step's name. Shown in the tooltip and the `aria-label`, never as an on-screen label. |
| millis | Option<u64> | How long it took. `None` means it ran without a measurement: it gets a marker, not a width. |
| state | SegmentState | `Done`, `Failed`, `Running` or `Unmeasured`. Sets both the fill and the border. |
| label | String | The duration as it should be read, for example `"2m 29s"`. |

`Segment::done(name, millis, label)` builds a finished step.

## Variants

| State | Meaning |
|-------|---------|
| Done | Finished with no complaint. |
| Failed | Ended the sequence. |
| Running | Still going. |
| Unmeasured | Ran, but its duration is not known. Drawn as a marker with no width. |

## Helpers

- `dominant_caption(&segments, total_label) -> Option<String>` writes the usual caption: which step took the run and its share. It returns `None` when nothing was measured.
- `aria_from(&segments) -> String` lists every step and its duration for a screen reader.

## Rules

- **Never rescale.** A log scale or equal-width segments hide the one thing the bar exists to show. A step that took 96% looks like it took 96%.
- **No legend, no segment labels.** Labels break first at phone width, and the table this usually sits above already names every step.
- **Nothing measured, nothing drawn.** `render` emits no markup when no segment has a duration, because an empty track would read as "this did nothing".
- The step the caption names takes the full accent, so the bar and the sentence point at the same thing.

## Accessibility

- **Role:** the track is `role="img"` with an `aria-label` that names every step.
- **Keyboard:** each segment has `tabindex="0"` and a `data-tip` tooltip, so a keyboard reaches the step names too.
- **Colour:** each state has a border treatment as well as a fill, so status never rests on colour alone.

## Related

Progress, Meter, Chart, Table.
