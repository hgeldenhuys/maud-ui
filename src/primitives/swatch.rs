//! Swatch component — colour chips with labels, click-to-copy, and a
//! design-token mode that reads live `--mui-*` CSS variables so a
//! swatch grid doubles as a live theme preview.
//!
//! Three modes are supported:
//!
//!   * `Mode::Raw`   — a single colour chip with a literal value such as
//!                      `#2563eb` or `hsl(210 40% 96%)`. Label + value
//!                      are rendered beneath the chip.
//!   * `Mode::Token` — reads `var(--mui-accent)` (or any other token)
//!                      so the chip re-renders when `data-theme` flips
//!                      or a `/theme` page mutates the tokens.
//!   * `Mode::Scale` — a horizontal Tailwind-style tone ramp (50..950).
//!
//! All three use the same `mui-swatch` block for the chip, so gallery
//! / customiser / documentation share one set of pixels.

use maud::{html, Markup, PreEscaped};

/// Swatch rendering mode.
#[derive(Debug, Clone)]
pub enum Mode {
    /// A literal colour (any valid CSS colour string).
    Raw(String),
    /// A maud-ui design token — a CSS custom-property name without the
    /// leading `--`. Rendered with `background: var(--{name})`.
    Token(String),
}

/// Swatch size.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Size { Sm, Md, Lg }

impl Size {
    fn class(&self) -> &'static str {
        match self {
            Self::Sm => "mui-swatch--sm",
            Self::Md => "mui-swatch--md",
            Self::Lg => "mui-swatch--lg",
        }
    }
}

/// Swatch rendering properties.
#[derive(Debug, Clone)]
pub struct Props {
    /// Human-readable name shown under the chip (e.g. "Primary").
    pub label: String,
    /// Optional second line — typically the raw value or token name.
    pub sublabel: Option<String>,
    /// Colour source.
    pub mode: Mode,
    /// Size of the chip.
    pub size: Size,
    /// Whether clicking the swatch copies the underlying value to the
    /// clipboard. Handled by a tiny vanilla-JS behaviour registered
    /// by `mui-swatch.js`.
    pub copyable: bool,
}

impl Default for Props {
    fn default() -> Self {
        Self {
            label: String::new(),
            sublabel: None,
            mode: Mode::Raw("#000000".into()),
            size: Size::Md,
            copyable: true,
        }
    }
}

/// Render a single swatch.
pub fn render(props: Props) -> Markup {
    let (chip_style, copy_value) = match &props.mode {
        Mode::Raw(v) => (format!("background: {};", v), v.clone()),
        Mode::Token(name) => (
            format!("background: var(--{});", name),
            format!("var(--{})", name),
        ),
    };
    let role_attr: &'static str = if props.copyable { "button" } else { "presentation" };
    let tabindex_attr: &'static str = if props.copyable { "0" } else { "-1" };
    html! {
        div class={"mui-swatch " (props.size.class()) @if props.copyable { " mui-swatch--copyable" }}
            role=(role_attr)
            tabindex=(tabindex_attr)
            data-mui=(if props.copyable { "swatch-copy" } else { "" })
            data-swatch-value=(copy_value)
            title=(if props.copyable { "Click to copy" } else { "" }) {
            div class="mui-swatch__chip" style=(PreEscaped(chip_style)) {}
            div class="mui-swatch__body" {
                div class="mui-swatch__label" { (props.label) }
                @if let Some(sub) = &props.sublabel {
                    div class="mui-swatch__sub" { (sub) }
                }
            }
        }
    }
}

/// A horizontal tone ramp — one row of N chips sharing a name and
/// labelled by their tone key (e.g. `50`, `100`, … `950`).
pub fn render_scale(name: &str, stops: &[(&str, &str)]) -> Markup {
    html! {
        div class="mui-swatch__scale" {
            div class="mui-swatch__scale-label" { (name) }
            div class="mui-swatch__scale-row" {
                @for (key, hex) in stops {
                    div class="mui-swatch mui-swatch--scale mui-swatch--copyable"
                        role="button" tabindex="0"
                        data-mui="swatch-copy"
                        data-swatch-value=(*hex)
                        title=(format!("{} · {} — click to copy", name, key)) {
                        div class="mui-swatch__chip" style=(PreEscaped(format!("background: {};", hex))) {}
                        div class="mui-swatch__scale-key" { (*key) }
                    }
                }
            }
        }
    }
}

/// Grid of design-token swatches — one chip per maud-ui `--mui-*`
/// token. Pass a slice of `(label, token-name)` pairs. The chip
/// renders live against the current theme, so placing this on any
/// page gives a visual preview of the active tokens.
pub fn render_tokens(tokens: &[(&str, &str)]) -> Markup {
    html! {
        div class="mui-swatch__grid" {
            @for (label, token) in tokens {
                (render(Props {
                    label: (*label).into(),
                    sublabel: Some(format!("--{}", token)),
                    mode: Mode::Token((*token).into()),
                    size: Size::Md,
                    copyable: true,
                }))
            }
        }
    }
}

/// Showcase all swatch variants and use cases.
pub fn showcase() -> Markup {
    html! {
        div.mui-showcase__grid {
            section {
                h2 { "Sizes" }
                div.mui-showcase__row style="align-items:flex-end;" {
                    (render(Props {
                        label: "Small".into(),
                        sublabel: Some("32\u{00d7}32".into()),
                        mode: Mode::Raw("#2563eb".into()),
                        size: Size::Sm, copyable: true,
                    }))
                    (render(Props {
                        label: "Medium".into(),
                        sublabel: Some("56\u{00d7}56".into()),
                        mode: Mode::Raw("#2563eb".into()),
                        size: Size::Md, copyable: true,
                    }))
                    (render(Props {
                        label: "Large".into(),
                        sublabel: Some("80\u{00d7}80".into()),
                        mode: Mode::Raw("#2563eb".into()),
                        size: Size::Lg, copyable: true,
                    }))
                }
            }

            section {
                h2 { "Raw values" }
                p.mui-text-muted style="font-size:0.875rem;margin:0 0 0.75rem;" {
                    "Any valid CSS colour string. Click a chip to copy the value."
                }
                div.mui-showcase__row {
                    (render(Props { label: "Primary".into(),     sublabel: Some("#2563eb".into()),          mode: Mode::Raw("#2563eb".into()), size: Size::Md, copyable: true }))
                    (render(Props { label: "Success".into(),     sublabel: Some("#16a34a".into()),          mode: Mode::Raw("#16a34a".into()), size: Size::Md, copyable: true }))
                    (render(Props { label: "Warning".into(),     sublabel: Some("#f59e0b".into()),          mode: Mode::Raw("#f59e0b".into()), size: Size::Md, copyable: true }))
                    (render(Props { label: "Danger".into(),      sublabel: Some("#dc2626".into()),          mode: Mode::Raw("#dc2626".into()), size: Size::Md, copyable: true }))
                    (render(Props { label: "Fuchsia".into(),     sublabel: Some("hsl(289 65% 54%)".into()), mode: Mode::Raw("hsl(289 65% 54%)".into()), size: Size::Md, copyable: true }))
                    (render(Props { label: "Translucent".into(), sublabel: Some("rgba(37,99,235,0.35)".into()), mode: Mode::Raw("rgba(37,99,235,0.35)".into()), size: Size::Md, copyable: true }))
                }
            }

            section {
                h2 { "Design tokens (live)" }
                p.mui-text-muted style="font-size:0.875rem;margin:0 0 0.75rem;" {
                    "Each chip reads "
                    code style="font-family:var(--mui-font-mono);font-size:0.8125rem;" { "var(--mui-*)" }
                    " at paint time, so toggling the gallery theme flips the grid."
                }
                (render_tokens(&[
                    ("Background",      "mui-bg"),
                    ("Card surface",    "mui-bg-card"),
                    ("Input surface",   "mui-bg-input"),
                    ("Body text",       "mui-text"),
                    ("Muted text",      "mui-text-muted"),
                    ("Subtle text",     "mui-text-subtle"),
                    ("Border",          "mui-border"),
                    ("Border · hover",  "mui-border-hover"),
                    ("Border · focus",  "mui-border-focus"),
                    ("Accent",          "mui-accent"),
                    ("Accent text",     "mui-accent-text"),
                    ("Danger",          "mui-danger"),
                ]))
            }

            section {
                h2 { "Scale ramps (Tailwind-style)" }
                p.mui-text-muted style="font-size:0.875rem;margin:0 0 0.75rem;" {
                    "Horizontal ramps from 50 to 950 — click any tone to copy its hex."
                }
                (render_scale("slate", &tailwind_ramp("slate")))
                (render_scale("blue",  &tailwind_ramp("blue")))
                (render_scale("emerald", &tailwind_ramp("emerald")))
                (render_scale("rose",  &tailwind_ramp("rose")))
                (render_scale("amber", &tailwind_ramp("amber")))
            }
        }
    }
}

/// A small subset of Tailwind's palette ramps — enough to demo the
/// scale component without pulling in the entire library. Used both
/// here and by the theme customiser page as preset bases.
pub fn tailwind_ramp(name: &str) -> Vec<(&'static str, &'static str)> {
    let stops = [
        "50", "100", "200", "300", "400", "500", "600", "700", "800", "900", "950",
    ];
    let hexes: &[&str] = match name {
        "slate"   => &["#f8fafc","#f1f5f9","#e2e8f0","#cbd5e1","#94a3b8","#64748b","#475569","#334155","#1e293b","#0f172a","#020617"],
        "gray"    => &["#f9fafb","#f3f4f6","#e5e7eb","#d1d5db","#9ca3af","#6b7280","#4b5563","#374151","#1f2937","#111827","#030712"],
        "zinc"    => &["#fafafa","#f4f4f5","#e4e4e7","#d4d4d8","#a1a1aa","#71717a","#52525b","#3f3f46","#27272a","#18181b","#09090b"],
        "neutral" => &["#fafafa","#f5f5f5","#e5e5e5","#d4d4d4","#a3a3a3","#737373","#525252","#404040","#262626","#171717","#0a0a0a"],
        "stone"   => &["#fafaf9","#f5f5f4","#e7e5e4","#d6d3d1","#a8a29e","#78716c","#57534e","#44403c","#292524","#1c1917","#0c0a09"],
        "red"     => &["#fef2f2","#fee2e2","#fecaca","#fca5a5","#f87171","#ef4444","#dc2626","#b91c1c","#991b1b","#7f1d1d","#450a0a"],
        "orange"  => &["#fff7ed","#ffedd5","#fed7aa","#fdba74","#fb923c","#f97316","#ea580c","#c2410c","#9a3412","#7c2d12","#431407"],
        "amber"   => &["#fffbeb","#fef3c7","#fde68a","#fcd34d","#fbbf24","#f59e0b","#d97706","#b45309","#92400e","#78350f","#451a03"],
        "yellow"  => &["#fefce8","#fef9c3","#fef08a","#fde047","#facc15","#eab308","#ca8a04","#a16207","#854d0e","#713f12","#422006"],
        "lime"    => &["#f7fee7","#ecfccb","#d9f99d","#bef264","#a3e635","#84cc16","#65a30d","#4d7c0f","#3f6212","#365314","#1a2e05"],
        "green"   => &["#f0fdf4","#dcfce7","#bbf7d0","#86efac","#4ade80","#22c55e","#16a34a","#15803d","#166534","#14532d","#052e16"],
        "emerald" => &["#ecfdf5","#d1fae5","#a7f3d0","#6ee7b7","#34d399","#10b981","#059669","#047857","#065f46","#064e3b","#022c22"],
        "teal"    => &["#f0fdfa","#ccfbf1","#99f6e4","#5eead4","#2dd4bf","#14b8a6","#0d9488","#0f766e","#115e59","#134e4a","#042f2e"],
        "cyan"    => &["#ecfeff","#cffafe","#a5f3fc","#67e8f9","#22d3ee","#06b6d4","#0891b2","#0e7490","#155e75","#164e63","#083344"],
        "sky"     => &["#f0f9ff","#e0f2fe","#bae6fd","#7dd3fc","#38bdf8","#0ea5e9","#0284c7","#0369a1","#075985","#0c4a6e","#082f49"],
        "blue"    => &["#eff6ff","#dbeafe","#bfdbfe","#93c5fd","#60a5fa","#3b82f6","#2563eb","#1d4ed8","#1e40af","#1e3a8a","#172554"],
        "indigo"  => &["#eef2ff","#e0e7ff","#c7d2fe","#a5b4fc","#818cf8","#6366f1","#4f46e5","#4338ca","#3730a3","#312e81","#1e1b4b"],
        "violet"  => &["#f5f3ff","#ede9fe","#ddd6fe","#c4b5fd","#a78bfa","#8b5cf6","#7c3aed","#6d28d9","#5b21b6","#4c1d95","#2e1065"],
        "purple"  => &["#faf5ff","#f3e8ff","#e9d5ff","#d8b4fe","#c084fc","#a855f7","#9333ea","#7e22ce","#6b21a8","#581c87","#3b0764"],
        "fuchsia" => &["#fdf4ff","#fae8ff","#f5d0fe","#f0abfc","#e879f9","#d946ef","#c026d3","#a21caf","#86198f","#701a75","#4a044e"],
        "pink"    => &["#fdf2f8","#fce7f3","#fbcfe8","#f9a8d4","#f472b6","#ec4899","#db2777","#be185d","#9d174d","#831843","#500724"],
        "rose"    => &["#fff1f2","#ffe4e6","#fecdd3","#fda4af","#fb7185","#f43f5e","#e11d48","#be123c","#9f1239","#881337","#4c0519"],
        _ => return Vec::new(),
    };
    stops.iter().copied().zip(hexes.iter().copied()).collect()
}
