(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["tooltip"] = function (wrapper) {
    // wrapper is .mui-tooltip with a trigger inside and a .mui-tooltip__content sibling
    var trigger = wrapper.querySelector(".mui-tooltip__trigger");
    var content = wrapper.querySelector(".mui-tooltip__content");
    if (!trigger || !content) return;

    var delay = parseInt(wrapper.getAttribute("data-delay") || "500", 10);
    var showTimer = null;
    var hideTimer = null;

    function show() {
      clearTimeout(hideTimer);
      clearTimeout(showTimer);
      showTimer = setTimeout(function () {
        // Remove hidden first so CSS transition can play
        content.removeAttribute("hidden");
        // Force reflow before adding visible state
        void content.offsetHeight;
        content.setAttribute("data-visible", "true");
      }, delay);
    }

    function hide() {
      clearTimeout(showTimer);
      clearTimeout(hideTimer);
      content.setAttribute("data-visible", "false");
      hideTimer = setTimeout(function () {
        content.setAttribute("hidden", "");
      }, 150); // Match --mui-transition duration
    }

    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    trigger.addEventListener("focus", show);
    trigger.addEventListener("blur", hide);
    // Escape dismisses
    trigger.addEventListener("keydown", function (e) {
      if (e.key === "Escape") hide();
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
