(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["dir-toggle"] = function (el) {
    el.addEventListener("click", function () {
      var html = document.documentElement;
      var current = html.getAttribute("dir") || "ltr";
      var next = current === "ltr" ? "rtl" : "ltr";
      html.setAttribute("dir", next);
      el.textContent = next === "ltr" ? "RTL" : "LTR";
      try { localStorage.setItem("mui-dir", next); } catch (e) {}
    });

    // Restore saved direction
    try {
      var saved = localStorage.getItem("mui-dir");
      if (saved && saved !== (document.documentElement.getAttribute("dir") || "ltr")) {
        document.documentElement.setAttribute("dir", saved);
        el.textContent = saved === "ltr" ? "RTL" : "LTR";
      }
    } catch (e) {}
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();
