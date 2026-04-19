//! Sonner component — positioned toast viewport, shadcn's successor to `toast`.
//!
//! shadcn deprecated the original `toast` primitive in favour of `sonner`. We
//! keep [`super::toast`] intact for backwards compatibility and expose this
//! parallel module that:
//!
//!  * re-exports the toast [`Props`] / [`Variant`] / [`render`] API as-is, so
//!    call sites that think in "sonner" terms can use `sonner::toast(props)`;
//!  * adds a [`Position`] enum (six placements, default `BottomRight`) that
//!    drives the viewport anchoring via a `data-position` attribute;
//!  * ships a new [`viewport`] fn that takes a `Position` — the original
//!    `toast::viewport()` is still valid for the legacy bottom-right default.
//!
//! The browser behaviour is shared with the existing `dist/behaviors/toast.js`
//! (via a thin dispatcher also wired up in `toast.js` for the
//! `mui:sonner-toast` custom event — see that file for details). We do not
//! duplicate the injection logic.
//!
//! Ref: <https://ui.shadcn.com/docs/components/base/sonner>

use maud::{html, Markup};

pub use super::toast::{render, Props, Variant};

/// Viewport placement. Default is `BottomRight`, matching shadcn's sonner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    #[default]
    BottomRight,
}

impl Position {
    /// Value written to the viewport's `data-position` attribute.
    pub fn data_value(&self) -> &'static str {
        match self {
            Self::TopLeft => "top-left",
            Self::TopCenter => "top-center",
            Self::TopRight => "top-right",
            Self::BottomLeft => "bottom-left",
            Self::BottomCenter => "bottom-center",
            Self::BottomRight => "bottom-right",
        }
    }
}

/// Render the sonner viewport container. Toasts dispatched via
/// `MaudUI.sonner(...)` or the `mui:sonner-toast` custom event land here.
///
/// The element carries `id="mui-sonner-viewport"` so the behaviour file can
/// find it without needing a query selector walk. Callers that need multiple
/// viewports on one page should use [`viewport_with_id`] instead.
pub fn viewport(position: Position) -> Markup {
    viewport_with_id("mui-sonner-viewport", position)
}

/// Render a sonner viewport with an explicit DOM id (for pages that need
/// more than one independent viewport — rare, but shadcn allows it).
pub fn viewport_with_id(id: &str, position: Position) -> Markup {
    html! {
        div id=(id)
            class="mui-sonner"
            data-mui="sonner-viewport"
            data-position=(position.data_value())
            aria-live="polite" {}
    }
}

/// Thin alias for [`render`] that mirrors shadcn's JS `sonner.toast(...)`
/// spelling. Returns the markup for a single toast node — use this for
/// server-rendered static toasts; for imperative dispatch fire a
/// `mui:sonner-toast` CustomEvent on `window`.
pub fn toast(props: Props) -> Markup {
    render(props)
}

/// Showcase — position picker + "Show toast" button that dispatches a demo
/// toast into the currently-selected viewport.
pub fn showcase() -> Markup {
    // The picker buttons carry `data-position` values; the inline script
    // (kept inline so the showcase file is self-contained) flips the
    // viewport's `data-position` and `aria-pressed` on click, then fires
    // a demo toast.
    let script = r#"
(function(){
  var root = document.currentScript && document.currentScript.parentElement;
  if (!root) return;
  var vp = root.querySelector('.mui-sonner');
  var btns = root.querySelectorAll('[data-sonner-pos]');
  for (var i = 0; i < btns.length; i++) {
    btns[i].addEventListener('click', function(e){
      var pos = e.currentTarget.getAttribute('data-sonner-pos');
      if (vp) vp.setAttribute('data-position', pos);
      for (var j = 0; j < btns.length; j++) {
        btns[j].setAttribute('aria-pressed', btns[j] === e.currentTarget ? 'true' : 'false');
      }
    });
  }
  var fire = root.querySelector('[data-sonner-fire]');
  if (fire) {
    fire.addEventListener('click', function(){
      window.dispatchEvent(new CustomEvent('mui:sonner-toast', {
        detail: {
          variant: 'success',
          title: 'Saved',
          description: 'Your changes have been stored.',
          duration_ms: 4000
        }
      }));
    });
  }
})();
"#;

    html! {
        div.mui-showcase__grid {
            div {
                p.mui-showcase__caption { "Position (pick one, then fire a toast)" }
                (viewport(Position::BottomRight))

                div style="display:grid;grid-template-columns:repeat(3,max-content);gap:0.5rem;margin-bottom:0.75rem" {
                    button type="button" class="mui-btn mui-btn--outline mui-btn--sm" data-sonner-pos="top-left" aria-pressed="false" { "top-left" }
                    button type="button" class="mui-btn mui-btn--outline mui-btn--sm" data-sonner-pos="top-center" aria-pressed="false" { "top-center" }
                    button type="button" class="mui-btn mui-btn--outline mui-btn--sm" data-sonner-pos="top-right" aria-pressed="false" { "top-right" }
                    button type="button" class="mui-btn mui-btn--outline mui-btn--sm" data-sonner-pos="bottom-left" aria-pressed="false" { "bottom-left" }
                    button type="button" class="mui-btn mui-btn--outline mui-btn--sm" data-sonner-pos="bottom-center" aria-pressed="false" { "bottom-center" }
                    button type="button" class="mui-btn mui-btn--outline mui-btn--sm" data-sonner-pos="bottom-right" aria-pressed="true" { "bottom-right" }
                }

                div.mui-showcase__row {
                    button type="button" class="mui-btn mui-btn--primary mui-btn--md" data-sonner-fire {
                        "Show toast"
                    }
                }

                script { (maud::PreEscaped(script)) }
            }
        }
    }
}
