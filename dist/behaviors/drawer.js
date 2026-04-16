(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // Drawer Trigger Behavior
  window.MaudUI.behaviors["drawer-trigger"] = function (el) {
    el.addEventListener("click", function () {
      var target_id = el.getAttribute("data-target");
      var drawer = target_id ? document.getElementById(target_id) : null;
      if (drawer && drawer.showModal) {
        drawer.showModal();
      }
    });
  };

  // Drawer Behavior
  window.MaudUI.behaviors["drawer"] = function (el) {
    // Wire close buttons inside the drawer
    var closeBtns = el.querySelectorAll("[data-mui-close]");
    for (var i = 0; i < closeBtns.length; i++) {
      closeBtns[i].addEventListener("click", function () {
        el.close();
      });
    }

    // Click on backdrop closes drawer (when clicking on dialog element outside content)
    el.addEventListener("click", function (e) {
      if (e.target === el) {
        el.close();
      }
    });

    // Native <dialog> with showModal() handles:
    // - ESC closes the drawer (unless prevented by cancel event)
    // - Focus trapping inside the drawer
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();
