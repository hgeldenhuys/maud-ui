//! Time split — one bar showing where a sequence of steps actually spent its time.
//!
//! For a pipeline, a build, a request trace: anything that runs as an ordered list
//! of steps where the reader's question is "what took so long". Segments sit in
//! execution order and are sized by share of measured time, so the answer is the
//! shape of the bar.
//!
//! Design consulted with Kimi K3 (2026-09-14), first shipped on the Kapable delivery
//! dashboard's run page. Three rules came out of that and are baked in here:
//!
//! * **The caption is half the widget.** A bar alone only says "one step was big",
//!   which a duration column already says in numbers. Pass a `caption` that states
//!   the finding; [`dominant_caption`] writes the usual one for you.
//! * **No legend and no per-segment labels.** At phone width any labelling scheme is
//!   the thing that breaks, and the table this usually sits above already carries
//!   every name. Names reach a screen reader through `aria-label`, and a pointer
//!   through `title`.
//! * **Never rescale.** A log scale or equal-width segments are the two usual
//!   "fixes" for one dominant step, and both destroy the only insight the widget
//!   exists to show. A step that took 96% looks like it took 96%.
//!
//! A segment that ran but was never measured is drawn as a marker with no width:
//! inventing a width would put a made-up number into the only picture on the page.
//! When nothing at all was measured, [`render`] emits nothing — an empty track reads
//! as "this did nothing", which is a different and usually false claim.

use maud::{html, Markup, PreEscaped};

/// What happened to one step, which decides its fill AND its border style — status
/// never rests on colour alone here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SegmentState {
    /// Finished, no complaint.
    #[default]
    Done,
    /// Ended the sequence.
    Failed,
    /// Still going.
    Running,
    /// Ran, but its duration is not known.
    Unmeasured,
}

impl SegmentState {
    fn class(self) -> &'static str {
        match self {
            SegmentState::Done => "mui-time-split__seg--done",
            SegmentState::Failed => "mui-time-split__seg--failed",
            SegmentState::Running => "mui-time-split__seg--running",
            SegmentState::Unmeasured => "mui-time-split__seg--unmeasured",
        }
    }
}

/// One step of the sequence.
#[derive(Debug, Clone)]
pub struct Segment {
    /// What this step is called. Reaches the reader through `title` and the
    /// bar's `aria-label`, never as an on-screen label.
    pub name: String,
    /// How long it took. `None` means it ran without being measured: it gets a
    /// marker, not a width.
    pub millis: Option<u64>,
    pub state: SegmentState,
    /// The duration as you want it read, e.g. `"2m 29s"` or `"13 ms"`. Empty is
    /// fine for an unmeasured step.
    pub label: String,
}

impl Segment {
    /// A finished step.
    pub fn done(name: impl Into<String>, millis: u64, label: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            millis: Some(millis),
            state: SegmentState::Done,
            label: label.into(),
        }
    }
}

/// Bar properties.
#[derive(Debug, Clone, Default)]
pub struct Props {
    /// Steps in EXECUTION ORDER. Steps that never ran should be left out entirely
    /// rather than passed with a zero duration: the empty remainder of the track is
    /// what "it ended here" looks like.
    pub segments: Vec<Segment>,
    /// The finding, in words. Do not skip it — see the module note.
    pub caption: String,
    /// Read to a screen reader in place of the bar. [`aria_from`] builds a decent
    /// one from the segments.
    pub aria_label: String,
}

/// The sentence the caption usually wants: which step took the run, and its share.
///
/// Returns `None` when nothing was measured, which is also when [`render`] draws
/// nothing — the two agree by construction.
pub fn dominant_caption(segments: &[Segment], total_label: &str) -> Option<String> {
    let total: u64 = segments.iter().filter_map(|s| s.millis).sum();
    if total == 0 {
        return None;
    }
    let top = segments
        .iter()
        .filter(|s| s.millis.is_some())
        .max_by_key(|s| s.millis.unwrap_or(0))?;
    let share = (top.millis.unwrap_or(0) as f64) * 100.0 / (total as f64);
    // 99.7% rounds to 100, and "took 100% of this run" is false whenever another
    // step also ran — it did not take all of it. Hold the claim at 99 instead.
    let measured = segments.iter().filter(|s| s.millis.is_some()).count();
    let shown = if share >= 99.5 && measured > 1 { 99.0 } else { share };
    Some(format!(
        "{} took {:.0}% of this run ({} of {}).",
        top.name, shown, top.label, total_label
    ))
}

/// A screen-reader description listing every step and its duration.
pub fn aria_from(segments: &[Segment]) -> String {
    let parts: Vec<String> = segments
        .iter()
        .map(|s| match s.millis {
            Some(_) => format!("{} {}", s.name, s.label),
            None => format!("{} timing unavailable", s.name),
        })
        .collect();
    format!("Time split across {} steps: {}.", segments.len(), parts.join(", "))
}

/// Render the bar. Emits nothing when no segment carries a duration.
pub fn render(props: Props) -> Markup {
    let total: u64 = props.segments.iter().filter_map(|s| s.millis).sum();
    if total == 0 {
        return html! {};
    }
    // The step the caption names takes the full accent, so the bar and the sentence
    // point at the same thing. Without this the ramp can hand the dominant step its
    // palest tone, which is exactly backwards (seen in QA on the live page).
    let top: Option<u64> = props.segments.iter().filter_map(|s| s.millis).max();

    html! {
        section.mui-time-split aria-label=(props.aria_label.clone()) {
            div.mui-time-split__track role="img" aria-label=(props.aria_label) {
                @for (i, seg) in props.segments.iter().enumerate() {
                    @let pct = seg.millis.unwrap_or(0) as f64 * 100.0 / total as f64;
                    @let title = if seg.label.is_empty() {
                        seg.name.clone()
                    } else {
                        format!("{} · {}", seg.name, seg.label)
                    };
                    @let tone = match i % 5 {
                        0 => "mui-time-split__seg--t0",
                        1 => "mui-time-split__seg--t1",
                        2 => "mui-time-split__seg--t2",
                        3 => "mui-time-split__seg--t3",
                        _ => "mui-time-split__seg--t4",
                    };
                    @let lead = seg.millis.is_some() && seg.millis == top;
                    span
                        .mui-time-split__seg
                        .(seg.state.class())
                        .(tone)
                        .mui-time-split__seg--lead[lead]
                        style=(PreEscaped(format!("width:{pct:.4}%")))
                        tabindex="0"
                        title=(title.clone())
                        data-tip=(title) {}
                }
            }
            @if !props.caption.is_empty() {
                p.mui-time-split__caption { (props.caption) }
            }
        }
    }
}

/// Showcase for the component gallery.
pub fn showcase() -> Markup {
    // The real run this component was built for: one step is 96% of it.
    let pipeline = vec![
        Segment::done("clone-or-pull", 447, "447 ms"),
        Segment::done("gate-clippy-test", 574, "574 ms"),
        Segment::done("vendor-stage", 3_964, "4.0s"),
        Segment::done("build-in-guest", 149_101, "2m 29s"),
        Segment::done("upload-harbor", 1_630, "1.6s"),
        Segment::done("report", 13, "13 ms"),
    ];
    let failed = vec![
        Segment::done("clone-or-pull", 900, "900 ms"),
        Segment {
            name: "gate-clippy-test".into(),
            millis: Some(6_200),
            state: SegmentState::Failed,
            label: "6.2s".into(),
        },
    ];
    let running = vec![
        Segment::done("clone-or-pull", 512, "512 ms"),
        Segment {
            name: "build-in-guest".into(),
            millis: Some(71_000),
            state: SegmentState::Running,
            label: "1m 11s so far".into(),
        },
    ];
    let partial = vec![
        Segment::done("clone-or-pull", 1_000, "1.0s"),
        Segment {
            name: "gate-clippy-test".into(),
            millis: None,
            state: SegmentState::Unmeasured,
            label: String::new(),
        },
    ];

    html! {
        div.mui-showcase__stack {
            div {
                p.mui-showcase__caption { "One dominant step — the common case, drawn honestly" }
                (render(Props {
                    caption: dominant_caption(&pipeline, "2m 35s").unwrap_or_default(),
                    aria_label: aria_from(&pipeline),
                    segments: pipeline,
                }))
            }
            div {
                p.mui-showcase__caption { "Failed part-way — the empty track is where it stopped" }
                (render(Props {
                    caption: "Failed at gate-clippy-test after 7.1s. 2 of 6 steps ran.".into(),
                    aria_label: aria_from(&failed),
                    segments: failed,
                }))
            }
            div {
                p.mui-showcase__caption { "Still running" }
                (render(Props {
                    caption: "build-in-guest running · 1m 11s elapsed.".into(),
                    aria_label: aria_from(&running),
                    segments: running,
                }))
            }
            div {
                p.mui-showcase__caption { "A step that ran without a measurement — a mark, never a width" }
                (render(Props {
                    caption: "Timing unavailable for 1 step.".into(),
                    aria_label: aria_from(&partial),
                    segments: partial,
                }))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Vec<Segment> {
        vec![
            Segment::done("clone", 447, "447 ms"),
            Segment::done("build", 149_101, "2m 29s"),
            Segment::done("report", 13, "13 ms"),
        ]
    }

    #[test]
    fn a_dominant_step_is_drawn_dominant_and_named_in_the_caption() {
        let segs = sample();
        let caption = dominant_caption(&segs, "2m 30s").expect("something was measured");
        assert!(caption.starts_with("build took 99%"), "{caption}");
        // Not "100%": two other steps ran, so the run was not all one step.
        assert!(!caption.contains("100%"), "{caption}");

        let html = render(Props {
            caption,
            aria_label: aria_from(&segs),
            segments: segs,
        })
        .into_string();
        assert!(html.contains("width:99."), "the widest segment is the slow step:\n{html}");
        // Never rescaled: the 13 ms step keeps its real, tiny share.
        assert!(html.contains("width:0.0087"), "{html}");
    }

    #[test]
    fn nothing_measured_renders_nothing_at_all() {
        // An empty track would read as "this did nothing", which is a different claim.
        let segs = vec![Segment {
            name: "clone".into(),
            millis: None,
            state: SegmentState::Unmeasured,
            label: String::new(),
        }];
        assert!(dominant_caption(&segs, "—").is_none());
        let html = render(Props {
            caption: "ignored".into(),
            aria_label: aria_from(&segs),
            segments: segs,
        })
        .into_string();
        assert!(html.is_empty(), "expected no markup, got:\n{html}");
    }

    #[test]
    fn an_unmeasured_step_gets_a_marker_and_no_width() {
        let segs = vec![
            Segment::done("clone", 1_000, "1.0s"),
            Segment {
                name: "build".into(),
                millis: None,
                state: SegmentState::Unmeasured,
                label: String::new(),
            },
        ];
        let html = render(Props {
            caption: "Timing unavailable for 1 step.".into(),
            aria_label: aria_from(&segs),
            segments: segs,
        })
        .into_string();
        assert!(html.contains("width:0.0000%"), "no invented width:\n{html}");
        assert!(html.contains("mui-time-split__seg--unmeasured"), "{html}");
    }

    #[test]
    fn segments_are_tellable_apart_and_the_caption_s_step_leads() {
        // Six segments in one fill read as a single block on the live page; the ramp
        // is what fixes that, and the dominant step takes the accent so the picture
        // agrees with the sentence.
        let segs = sample();
        let html = render(Props {
            caption: dominant_caption(&segs, "2m 30s").unwrap_or_default(),
            aria_label: aria_from(&segs),
            segments: segs,
        })
        .into_string();
        for tone in ["--t0", "--t1", "--t2"] {
            assert!(html.contains(tone), "adjacent steps must differ: {tone}\n{html}");
        }
        assert_eq!(html.matches("--lead").count(), 1, "exactly one lead:\n{html}");
    }

    #[test]
    fn every_segment_carries_a_tooltip_and_can_take_focus() {
        // The tooltip is the only place a step is named on screen, so it must reach
        // a keyboard too — hover-only would be dead on touch and to a screen reader.
        let segs = sample();
        let html = render(Props {
            caption: String::new(),
            aria_label: aria_from(&segs),
            segments: segs,
        })
        .into_string();
        assert_eq!(html.matches("data-tip=").count(), 3, "{html}");
        assert_eq!(html.matches("tabindex=\"0\"").count(), 3, "{html}");
        assert!(html.contains("data-tip=\"build · 2m 29s\""), "{html}");
    }

    #[test]
    fn status_never_rests_on_colour_alone() {
        // Each state carries its own class, and the stylesheet gives each a border
        // treatment as well as a fill.
        for (state, class) in [
            (SegmentState::Done, "done"),
            (SegmentState::Failed, "failed"),
            (SegmentState::Running, "running"),
            (SegmentState::Unmeasured, "unmeasured"),
        ] {
            assert!(state.class().ends_with(class), "{class}");
        }
    }

    #[test]
    fn every_step_reaches_a_screen_reader() {
        let segs = sample();
        let aria = aria_from(&segs);
        for name in ["clone", "build", "report"] {
            assert!(aria.contains(name), "{aria}");
        }
        let html = render(Props {
            caption: String::new(),
            aria_label: aria,
            segments: segs,
        })
        .into_string();
        assert!(html.contains("role=\"img\""), "{html}");
    }
}
