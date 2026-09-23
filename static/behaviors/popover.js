(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["popover"] = function (root) {
    var trigger = root.querySelector(".mui-popover__trigger");
    var content = root.querySelector(".mui-popover__content");
    if (!trigger || !content) return;

    // Find the actual interactive element (consumer's button/link). If none,
    // fall back to the trigger span — but avoid setting aria-expanded on a
    // bare span (invalid on a generic role).
    var interactive = trigger.querySelector('button, a, [role="button"]') || trigger;
    interactive.setAttribute("aria-haspopup", "dialog");
    interactive.setAttribute("aria-expanded", content.hidden ? "false" : "true");
    interactive.setAttribute("aria-controls", content.id || "");

    function toggle() {
      var expanded = interactive.getAttribute("aria-expanded") === "true";
      if (expanded) close();
      else open();
    }

    function open() {
      interactive.setAttribute("aria-expanded", "true");
      // Remove hidden first so CSS transition can play
      window.MaudUI.cancelOverlayExit(content);
      content.removeAttribute("hidden");
      // Force reflow before adding visible state
      void content.offsetHeight;
      content.setAttribute("data-state", "open");
      content.setAttribute("data-visible", "true");
      content.focus();
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("keydown", escClose, true);
    }

    function close() {
      interactive.setAttribute("aria-expanded", "false");
      content.setAttribute("data-state", "closed");
      content.setAttribute("data-visible", "false");
      window.MaudUI.closeOverlay(content, function () { content.setAttribute("hidden", ""); });
      document.removeEventListener("click", clickOutside, true);
      document.removeEventListener("keydown", escClose, true);
    }

    function clickOutside(e) {
      if (!root.contains(e.target)) close();
    }

    function escClose(e) {
      // Escape hands focus back to the button that opened it (a click outside does not:
      // the reader has already moved on). Focus used to stay on the hidden panel.
      if (e.key === "Escape") { close(); interactive.focus(); }
    }

    trigger.addEventListener("click", toggle);
    if (!content.hidden) {
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("keydown", escClose, true);
    }
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
