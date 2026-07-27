(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // The renderer already emits everything this needs: data-orientation,
  // data-values, and data-thumb-index on every thumb. The previous version of
  // this behaviour read none of it, which produced two bugs the
  // server-rendered markup hid until you actually grabbed a thumb:
  //
  //   1. VERTICAL SLIDERS SLID THE WIDTH. Drag maths was clientX/rect.width
  //      unconditionally, and setValue wrote thumb.style.left and
  //      fill.style.width. On a vertical slider the CSS fills from the bottom
  //      via height, so the script animated the wrong axis entirely — the
  //      track grew sideways instead of the thumb moving up.
  //   2. RANGE SLIDERS HAD ONE LIVE THUMB. querySelector (singular) bound the
  //      first thumb only, so the second was inert, and the fill used
  //      single-thumb maths (0 -> value) instead of spanning lo..hi.
  //
  // Everything below is per-thumb and orientation-aware.
  //
  // KNOWN GAP: horizontal sliders position with physical `left`, matching what
  // the renderer emits, so they do not mirror under the RTL toggle. Fixing it
  // means moving the renderer, this file, and the thumb's centring transform to
  // logical properties together — a half-move leaves the server and the script
  // disagreeing about which edge 0% is.
  window.MaudUI.behaviors["slider"] = function (root) {
    var track = root.querySelector(".mui-slider__track");
    var fill = root.querySelector(".mui-slider__fill");
    var native = root.querySelector(".mui-slider__native");
    var valueLabel = root.querySelector(".mui-slider__value");
    var thumbs = Array.prototype.slice.call(
      root.querySelectorAll(".mui-slider__thumb"),
    );
    if (!track || !fill || thumbs.length === 0) return;
    if (root.getAttribute("data-disabled") === "true") return;

    var vertical = root.getAttribute("data-orientation") === "vertical";
    var min = parseFloat(root.getAttribute("data-min") || "0");
    var max = parseFloat(root.getAttribute("data-max") || "100");
    var step = parseFloat(root.getAttribute("data-step") || "1");
    if (!(step > 0)) step = 1;

    var values = thumbs.map(function (t) {
      return parseFloat(t.getAttribute("aria-valuenow") || "0");
    });

    function clamp(v) {
      return Math.max(min, Math.min(max, v));
    }

    // Snap RELATIVE TO min, not to zero. `Math.round(v / step) * step` puts the
    // stops on multiples of step, so a slider with min=5 step=10 could never
    // reach its own minimum — it snapped to 0 or 10.
    function snap(v) {
      return clamp(Math.round((v - min) / step) * step + min);
    }

    // Mirrors format_value() in src/primitives/slider.rs so the label does not
    // change shape the first time a thumb is touched.
    function fmt(v) {
      return v % 1 === 0 ? String(v) : String(parseFloat(v.toFixed(4)));
    }

    function pct(v) {
      return max === min ? 0 : ((v - min) / (max - min)) * 100;
    }

    function paint() {
      for (var i = 0; i < thumbs.length; i++) {
        var p = pct(values[i]);
        if (vertical) thumbs[i].style.bottom = p + "%";
        else thumbs[i].style.left = p + "%";
        thumbs[i].setAttribute("aria-valuenow", fmt(values[i]));
      }

      // One thumb fills from the track start; several fill the span between
      // the lowest and the highest.
      var lo, hi;
      if (values.length > 1) {
        lo = Math.min.apply(null, values);
        hi = Math.max.apply(null, values);
      } else {
        lo = min;
        hi = values[0];
      }
      var start = pct(lo);
      var extent = Math.max(0, pct(hi) - start);
      if (vertical) {
        fill.style.bottom = start + "%";
        fill.style.height = extent + "%";
      } else {
        fill.style.left = start + "%";
        fill.style.width = extent + "%";
      }

      if (valueLabel) {
        valueLabel.textContent =
          values.length > 1 ? fmt(lo) + " — " + fmt(hi) : fmt(values[0]);
      }
      if (root.hasAttribute("data-values")) {
        root.setAttribute("data-values", values.map(fmt).join(","));
      }
      if (native) {
        native.value = values[0];
        // The native input carries the form value and is aria-hidden, so
        // nothing else would ever notice it changed. Emit input+change so form
        // listeners, htmx bindings and validation see the new value.
        native.dispatchEvent(new Event("input", { bubbles: true }));
        native.dispatchEvent(new Event("change", { bubbles: true }));
      }
    }

    // Thumbs may not cross each other — clamp to the neighbours.
    function setThumb(i, v) {
      v = snap(v);
      if (i > 0) v = Math.max(v, values[i - 1]);
      if (i < values.length - 1) v = Math.min(v, values[i + 1]);
      if (v === values[i]) return;
      values[i] = v;
      paint();
    }

    function valueFromPointer(e) {
      var rect = track.getBoundingClientRect();
      var p;
      if (vertical) {
        // Bottom-anchored: larger values sit higher, matching thumb_style().
        p = rect.height === 0 ? 0 : (rect.bottom - e.clientY) / rect.height;
      } else {
        p = rect.width === 0 ? 0 : (e.clientX - rect.left) / rect.width;
      }
      return snap(min + Math.max(0, Math.min(1, p)) * (max - min));
    }

    function nearestThumb(v) {
      var best = 0;
      var bestDist = Infinity;
      for (var i = 0; i < values.length; i++) {
        var d = Math.abs(values[i] - v);
        if (d < bestDist) {
          bestDist = d;
          best = i;
        }
      }
      return best;
    }

    // Pointer events rather than mouse events: this is what makes the slider
    // work under touch and pen at all, and setPointerCapture keeps the drag
    // alive when the finger leaves the track.
    function bindThumb(thumb, index) {
      thumb.addEventListener("pointerdown", function (e) {
        if (thumb.getAttribute("aria-disabled") === "true") return;
        e.preventDefault();
        thumb.focus();
        // Capture is an optimisation — it keeps the drag alive when the
        // pointer leaves the thumb. It is NOT load-bearing, and it throws
        // NotFoundError for a pointerId the browser is not tracking. Letting
        // that escape would abort the handler before any listener is attached,
        // so the slider would silently not drag at all.
        try {
          thumb.setPointerCapture(e.pointerId);
        } catch (_) {}
        root.setAttribute("data-dragging", "true");

        function onMove(ev) {
          setThumb(index, valueFromPointer(ev));
        }
        function onUp() {
          root.removeAttribute("data-dragging");
          thumb.removeEventListener("pointermove", onMove);
          thumb.removeEventListener("pointerup", onUp);
          thumb.removeEventListener("pointercancel", onUp);
          try {
            if (thumb.hasPointerCapture(e.pointerId)) {
              thumb.releasePointerCapture(e.pointerId);
            }
          } catch (_) {}
        }
        thumb.addEventListener("pointermove", onMove);
        thumb.addEventListener("pointerup", onUp);
        thumb.addEventListener("pointercancel", onUp);
      });

      thumb.addEventListener("keydown", function (e) {
        if (thumb.getAttribute("aria-disabled") === "true") return;
        var v = values[index];
        var big = step * 10;
        var next = null;
        // Up and Right both increase on either orientation: on a vertical
        // slider "up" is toward max, and APG lists Right as increase too.
        if (e.key === "ArrowRight" || e.key === "ArrowUp") next = v + step;
        else if (e.key === "ArrowLeft" || e.key === "ArrowDown") next = v - step;
        else if (e.key === "PageUp") next = v + big;
        else if (e.key === "PageDown") next = v - big;
        else if (e.key === "Home") next = min;
        else if (e.key === "End") next = max;
        if (next === null) return;
        e.preventDefault();
        setThumb(index, next);
      });
    }

    for (var i = 0; i < thumbs.length; i++) bindThumb(thumbs[i], i);

    // Clicking the track jumps the NEAREST thumb, the only sensible behaviour
    // once there is more than one.
    track.addEventListener("pointerdown", function (e) {
      if (e.target !== track && e.target !== fill) return;
      var v = valueFromPointer(e);
      var idx = nearestThumb(v);
      setThumb(idx, v);
      thumbs[idx].focus();
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
