(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // Individual toggle behavior
  window.MaudUI.behaviors["toggle"] = function (el) {
    // el is the toggle button
    function toggle() {
      if (el.hasAttribute("disabled")) return;

      var isPressed = el.getAttribute("aria-pressed") === "true";
      el.setAttribute("aria-pressed", !isPressed ? "true" : "false");
    }

    el.addEventListener("click", toggle);

    el.addEventListener("keydown", function (e) {
      if (e.code === "Space" || e.code === "Enter") {
        e.preventDefault();
        toggle();
      }
    });
  };

  // Toggle group behavior
  window.MaudUI.behaviors["toggle-group"] = function (el) {
    // el is the toggle-group container
    var items = el.querySelectorAll(".mui-toggle-group__item");
    var isMultiple = el.getAttribute("data-multiple") === "true";
    var isDisabled = el.hasAttribute("data-disabled");

    function updateTabindex() {
      var focusable = null;
      for (var i = 0; i < items.length; i++) {
        if (items[i].getAttribute("aria-pressed") === "true") {
          focusable = items[i];
          break;
        }
      }
      if (!focusable && items.length > 0) {
        focusable = items[0];
      }

      for (var i = 0; i < items.length; i++) {
        items[i].tabIndex = items[i] === focusable ? 0 : -1;
      }
    }

    function pressItem(item) {
      if (isDisabled || item.hasAttribute("disabled")) return;

      var isPressed = item.getAttribute("aria-pressed") === "true";
      var newPressed = !isPressed;

      if (isMultiple) {
        item.setAttribute("aria-pressed", newPressed ? "true" : "false");
      } else {
        for (var i = 0; i < items.length; i++) {
          items[i].setAttribute("aria-pressed", "false");
        }
        item.setAttribute("aria-pressed", "true");
      }

      updateTabindex();
    }

    function handleArrowKey(direction) {
      var focusedIndex = -1;
      for (var i = 0; i < items.length; i++) {
        if (items[i] === document.activeElement) {
          focusedIndex = i;
          break;
        }
      }

      if (focusedIndex === -1) return;

      var nextIndex = focusedIndex;
      if (direction === "left" || direction === "up") {
        nextIndex = focusedIndex === 0 ? items.length - 1 : focusedIndex - 1;
      } else if (direction === "right" || direction === "down") {
        nextIndex = focusedIndex === items.length - 1 ? 0 : focusedIndex + 1;
      }

      items[nextIndex].focus();
    }

    for (var i = 0; i < items.length; i++) {
      (function (item) {
        item.addEventListener("click", function () {
          pressItem(item);
        });

        item.addEventListener("keydown", function (e) {
          if (e.code === "Space" || e.code === "Enter") {
            e.preventDefault();
            pressItem(item);
          } else if (e.code === "ArrowLeft" || e.code === "ArrowUp") {
            e.preventDefault();
            handleArrowKey("left");
          } else if (e.code === "ArrowRight" || e.code === "ArrowDown") {
            e.preventDefault();
            handleArrowKey("right");
          }
        });
      })(items[i]);
    }

    updateTabindex();
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
