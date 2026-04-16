(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["tooltip"] = function (wrapper) {
    // wrapper is .mui-tooltip with a trigger inside and a .mui-tooltip__content sibling
    const trigger = wrapper.querySelector(".mui-tooltip__trigger");
    const content = wrapper.querySelector(".mui-tooltip__content");
    if (!trigger || !content) return;

    const delay = parseInt(wrapper.getAttribute("data-delay") || "500", 10);
    let showTimer = null;
    let hideTimer = null;

    function show() {
      clearTimeout(hideTimer);
      clearTimeout(showTimer);
      showTimer = setTimeout(() => {
        content.setAttribute("data-visible", "true");
        content.removeAttribute("hidden");
      }, delay);
    }

    function hide() {
      clearTimeout(showTimer);
      clearTimeout(hideTimer);
      hideTimer = setTimeout(() => {
        content.setAttribute("data-visible", "false");
        content.setAttribute("hidden", "");
      }, 100);
    }

    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    trigger.addEventListener("focus", show);
    trigger.addEventListener("blur", hide);
    // Escape dismisses
    trigger.addEventListener("keydown", (e) => {
      if (e.key === "Escape") hide();
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
