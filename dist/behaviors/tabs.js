(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["tabs"] = function (root) {
    // root is the tabs container (.mui-tabs)
    // Find all [role="tab"] children and [role="tabpanel"] children
    var tabs = root.querySelectorAll('[role="tab"]');
    var panels = root.querySelectorAll('[role="tabpanel"]');

    function activateTab(index) {
      // Deactivate all tabs and hide all panels
      for (var i = 0; i < tabs.length; i++) {
        tabs[i].setAttribute("aria-selected", "false");
        tabs[i].setAttribute("tabindex", "-1");
        if (panels[i]) {
          panels[i].setAttribute("hidden", "");
        }
      }

      // Activate the selected tab and show its panel
      if (tabs[index]) {
        tabs[index].setAttribute("aria-selected", "true");
        tabs[index].setAttribute("tabindex", "0");
        tabs[index].focus();
      }
      if (panels[index]) {
        panels[index].removeAttribute("hidden");
      }
    }

    // Click handler
    for (var i = 0; i < tabs.length; i++) {
      (function (tabIndex) {
        tabs[tabIndex].addEventListener("click", function () {
          activateTab(tabIndex);
        });

        // Keyboard navigation
        tabs[tabIndex].addEventListener("keydown", function (e) {
          var nextIndex;
          switch (e.code) {
            case "ArrowRight":
              e.preventDefault();
              nextIndex = (tabIndex + 1) % tabs.length;
              activateTab(nextIndex);
              break;
            case "ArrowLeft":
              e.preventDefault();
              nextIndex = (tabIndex - 1 + tabs.length) % tabs.length;
              activateTab(nextIndex);
              break;
            case "Home":
              e.preventDefault();
              activateTab(0);
              break;
            case "End":
              e.preventDefault();
              activateTab(tabs.length - 1);
              break;
            case "Enter":
            case "Space":
              e.preventDefault();
              activateTab(tabIndex);
              break;
          }
        });
      })(i);
    }
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
