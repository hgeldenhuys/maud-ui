(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["hover-card"] = function (root) {
    // root is .mui-hover-card with trigger and content inside
    const trigger = root.querySelector(".mui-hover-card__trigger");
    const content = root.querySelector(".mui-hover-card__content");
    if (!trigger || !content) return;

    const openDelay = parseInt(root.getAttribute("data-open-delay") || "300", 10);
    const closeDelay = parseInt(root.getAttribute("data-close-delay") || "200", 10);
    let openTimer = null;
    let closeTimer = null;

    function show() {
      clearTimeout(closeTimer);
      openTimer = setTimeout(() => {
        content.removeAttribute("hidden");
        content.setAttribute("data-visible", "true");
      }, openDelay);
    }

    function hide() {
      clearTimeout(openTimer);
      closeTimer = setTimeout(() => {
        content.setAttribute("data-visible", "false");
        content.setAttribute("hidden", "");
      }, closeDelay);
    }

    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    // Content stays open while hovering the card itself
    content.addEventListener("mouseenter", function () {
      clearTimeout(closeTimer);
    });
    content.addEventListener("mouseleave", hide);
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
