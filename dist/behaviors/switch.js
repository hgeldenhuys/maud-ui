(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["switch"] = function (el) {
    // el is the switch button
    var hiddenInput = el.nextElementSibling;
    while (hiddenInput && hiddenInput.className.indexOf("mui-switch__value") === -1) {
      hiddenInput = hiddenInput.nextElementSibling;
    }

    function toggle() {
      if (el.hasAttribute("disabled")) return;

      var isChecked = el.getAttribute("aria-checked") === "true";
      var newState = !isChecked;

      el.setAttribute("aria-checked", newState ? "true" : "false");
      if (hiddenInput) {
        hiddenInput.value = newState ? "true" : "false";
      }
    }

    el.addEventListener("click", toggle);

    el.addEventListener("keydown", function (e) {
      if (e.code === "Space" || e.code === "Enter") {
        e.preventDefault();
        toggle();
      }
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
