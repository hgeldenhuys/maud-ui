(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["theme-toggle"] = function (el) {
    el.addEventListener("click", function () {
      var html = document.documentElement;
      var current = html.getAttribute("data-theme") || "dark";
      var next = current === "dark" ? "light" : "dark";
      html.setAttribute("data-theme", next);
      el.textContent = next === "dark" ? "Light mode" : "Dark mode";
      try { localStorage.setItem("mui-theme", next); } catch(e) {}
    });

    // Restore saved theme
    try {
      var saved = localStorage.getItem("mui-theme");
      if (saved && saved !== (document.documentElement.getAttribute("data-theme") || "dark")) {
        document.documentElement.setAttribute("data-theme", saved);
        el.textContent = saved === "dark" ? "Light mode" : "Dark mode";
      }
    } catch(e) {}
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();
