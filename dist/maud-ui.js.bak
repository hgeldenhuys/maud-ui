// maud-ui runtime — behaviors register into MaudUI.behaviors and self-initialize
// via `data-mui="<name>"` attributes. Wave 2/3 agents add behaviors here.
(function () {
  "use strict";

  var MaudUI = {
    behaviors: {},
    init: function (root) {
      if (!root) root = document.body;
      var els = root.querySelectorAll("[data-mui]");
      for (var i = 0; i < els.length; i++) {
        var el = els[i];
        if (el.hasAttribute("data-mui-init")) continue;
        var type = el.getAttribute("data-mui");
        if (type && MaudUI.behaviors[type]) {
          MaudUI.behaviors[type](el);
          el.setAttribute("data-mui-init", "");
        }
      }
    },
  };

  window.MaudUI = MaudUI;

  document.addEventListener("DOMContentLoaded", function () {
    MaudUI.init();
  });
  document.addEventListener("htmx:afterSwap", function (e) {
    if (e.target instanceof Element) MaudUI.init(e.target);
  });
})();
