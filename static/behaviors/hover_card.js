(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["hover-card"] = function (root) {
    // root is .mui-hover-card with trigger and content inside
    var trigger = root.querySelector(".mui-hover-card__trigger");
    var content = root.querySelector(".mui-hover-card__content");
    if (!trigger || !content) return;

    var openDelay = parseInt(root.getAttribute("data-open-delay") || "300", 10);
    var closeDelay = parseInt(root.getAttribute("data-close-delay") || "200", 10);
    var openTimer = null;
    var closeTimer = null;

    function show() {
      clearTimeout(closeTimer);
      window.MaudUI.cancelOverlayExit(content);
      openTimer = setTimeout(function () {
        // Remove hidden first so CSS transition can play
        content.removeAttribute("hidden");
        // Force reflow before adding visible state
        void content.offsetHeight;
        content.setAttribute("data-visible", "true");
      }, openDelay);
    }

    function hide() {
      clearTimeout(openTimer);
      function close() {
        content.setAttribute("data-visible", "false");
        window.MaudUI.closeOverlay(content, function () { content.setAttribute("hidden", ""); });
      }
      if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) close();
      else closeTimer = setTimeout(close, closeDelay);
    }

    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    // Content stays open while hovering the card itself
    content.addEventListener("mouseenter", function () {
      clearTimeout(closeTimer);
      window.MaudUI.cancelOverlayExit(content);
    });
    content.addEventListener("mouseleave", hide);
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
