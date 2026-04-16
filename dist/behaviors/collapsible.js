(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["collapsible"] = function (root) {
    // Find the trigger button and content div within this collapsible
    var trigger = root.querySelector(".mui-collapsible__trigger");
    var content = root.querySelector(".mui-collapsible__content");

    if (!trigger || !content) return;

    // Handle click on trigger button
    trigger.addEventListener("click", function () {
      toggleCollapsible();
    });

    // Handle keyboard events (Space and Enter)
    trigger.addEventListener("keydown", function (event) {
      if (event.key === " " || event.key === "Enter") {
        event.preventDefault();
        toggleCollapsible();
      }
    });

    function toggleCollapsible() {
      var isExpanded = trigger.getAttribute("aria-expanded") === "true";
      var newState = isExpanded ? "false" : "true";

      trigger.setAttribute("aria-expanded", newState);

      if (newState === "true") {
        content.removeAttribute("hidden");
      } else {
        content.setAttribute("hidden", "");
      }
    }
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();
