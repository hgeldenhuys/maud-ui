(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // Segmented-control / toggle-bar behaviour. Attach with
  // `data-mui="button-group"` on the wrapper. Two modes:
  //
  //   * `data-mode="exclusive"` (default) — radio-like. Exactly one
  //     child button carries `aria-pressed="true"` at a time. Clicking
  //     a pressed button re-affirms (no toggle-off) so the group
  //     always has a selection, matching native segmented controls.
  //
  //   * `data-mode="multiple"` — toggle-like. Each button flips its
  //     own `aria-pressed` independently.
  //
  // The wrapper stays the source of truth via the DOM — host apps
  // don't need to wire state; they can listen for the custom
  // `mui:button-group-change` event that bubbles off the group on
  // every activation, carrying `{ value, pressed }` in `detail` where
  // `value` is `button.value` or `button.dataset.value` or the button
  // label as a fallback.
  window.MaudUI.behaviors["button-group"] = function (group) {
    var mode = (group.getAttribute("data-mode") || "exclusive").toLowerCase();
    var buttons = Array.prototype.slice.call(group.querySelectorAll("button"));
    if (buttons.length === 0) return;

    function valueOf(btn) {
      return btn.value || btn.getAttribute("data-value") || (btn.textContent || "").trim();
    }
    function emit(btn, pressed) {
      group.dispatchEvent(new CustomEvent("mui:button-group-change", {
        bubbles: true,
        detail: { value: valueOf(btn), pressed: pressed, target: btn },
      }));
    }

    group.addEventListener("click", function (e) {
      var btn = e.target && e.target.closest && e.target.closest("button");
      if (!btn || !group.contains(btn) || btn.disabled) return;
      if (mode === "multiple") {
        var pressed = btn.getAttribute("aria-pressed") !== "true";
        btn.setAttribute("aria-pressed", pressed ? "true" : "false");
        emit(btn, pressed);
      } else {
        if (btn.getAttribute("aria-pressed") === "true") {
          // Re-affirm selection — still notify (consumers may want
          // to re-run a nav action).
          emit(btn, true);
          return;
        }
        for (var i = 0; i < buttons.length; i++) {
          var b = buttons[i];
          if (b === btn) b.setAttribute("aria-pressed", "true");
          else           b.setAttribute("aria-pressed", "false");
        }
        emit(btn, true);
      }
    });

    // Keyboard: in exclusive mode, support Left/Right arrows to move
    // the selection (matches native segmented control patterns). In
    // multiple mode, Space / Enter on a button just flips it — the
    // browser's default button semantics cover that.
    if (mode !== "multiple") {
      group.addEventListener("keydown", function (e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var current = document.activeElement;
        if (!buttons.indexOf || buttons.indexOf(current) === -1) return;
        e.preventDefault();
        var idx = buttons.indexOf(current);
        var dir = e.key === "ArrowRight" ? 1 : -1;
        var next = buttons[(idx + dir + buttons.length) % buttons.length];
        if (next.disabled) return;
        next.focus();
        next.click();
      });
    }
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();
