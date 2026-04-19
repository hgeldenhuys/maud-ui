(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  function toggleSidebar(sidebar) {
    if (!sidebar) return;
    var next = sidebar.getAttribute("data-state") === "expanded"
      ? "collapsed"
      : "expanded";
    sidebar.setAttribute("data-state", next);
  }

  function isTypingTarget(el) {
    if (!el) return false;
    var tag = el.tagName;
    if (tag === "INPUT" || tag === "TEXTAREA" || tag === "SELECT") return true;
    if (el.isContentEditable) return true;
    return false;
  }

  // Sidebar Behavior — per-element (registers the Cmd/Ctrl+B listener once).
  window.MaudUI.behaviors["sidebar"] = function (el) {
    if (el.__muiSidebarBound) return;
    el.__muiSidebarBound = true;

    // Rail inside this sidebar toggles it on click
    var rail = el.querySelector('[data-mui="sidebar-rail"]');
    if (rail) {
      rail.addEventListener("click", function () {
        toggleSidebar(el);
      });
    }
  };

  // Sidebar Trigger Behavior — click toggles the matching sidebar by id.
  window.MaudUI.behaviors["sidebar-trigger"] = function (el) {
    el.addEventListener("click", function () {
      var target_id = el.getAttribute("data-target");
      var sidebar = target_id ? document.getElementById(target_id) : null;
      toggleSidebar(sidebar);
    });
  };

  // Sidebar Rail Behavior — hover/click expand of a collapsed sidebar.
  window.MaudUI.behaviors["sidebar-rail"] = function (el) {
    el.addEventListener("click", function () {
      var sidebar = el.closest('[data-mui="sidebar"]');
      toggleSidebar(sidebar);
    });
  };

  // Global Cmd/Ctrl+B shortcut — toggles the first sidebar on the page.
  // Register once per document, guarded by a global flag.
  if (!window.__muiSidebarShortcutBound) {
    window.__muiSidebarShortcutBound = true;
    document.addEventListener("keydown", function (e) {
      var isToggle = (e.metaKey || e.ctrlKey) && (e.key === "b" || e.key === "B");
      if (!isToggle) return;
      if (isTypingTarget(e.target)) return;
      var sidebar = document.querySelector('[data-mui="sidebar"]');
      if (!sidebar) return;
      e.preventDefault();
      toggleSidebar(sidebar);
    });
  }

  if (window.MaudUI.init) window.MaudUI.init();
})();
