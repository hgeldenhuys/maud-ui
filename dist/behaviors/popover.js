(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["popover"] = function (root) {
    var trigger = root.querySelector(".mui-popover__trigger");
    var content = root.querySelector(".mui-popover__content");
    if (!trigger || !content) return;

    function toggle() {
      var expanded = trigger.getAttribute("aria-expanded") === "true";
      if (expanded) close();
      else open();
    }

    function open() {
      trigger.setAttribute("aria-expanded", "true");
      // Remove hidden first so CSS transition can play
      content.removeAttribute("hidden");
      // Force reflow before adding visible state
      void content.offsetHeight;
      content.setAttribute("data-visible", "true");
      content.focus();
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("keydown", escClose, true);
    }

    function close() {
      trigger.setAttribute("aria-expanded", "false");
      content.setAttribute("data-visible", "false");
      // Delay hidden to let CSS transition complete
      setTimeout(function () {
        content.setAttribute("hidden", "");
      }, 150); // Match --mui-transition duration
      document.removeEventListener("click", clickOutside, true);
      document.removeEventListener("keydown", escClose, true);
    }

    function clickOutside(e) {
      if (!root.contains(e.target)) close();
    }

    function escClose(e) {
      if (e.key === "Escape") close();
    }

    trigger.addEventListener("click", toggle);
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
