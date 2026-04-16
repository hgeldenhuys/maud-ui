(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // Menu behavior — dropdown triggered by button click
  window.MaudUI.behaviors["menu"] = function (root) {
    var trigger = root.querySelector(".mui-menu__trigger");
    var content = root.querySelector("[role='menu']");
    if (!trigger || !content) return;

    var focusedIndex = -1;

    function open() {
      trigger.setAttribute("aria-expanded", "true");
      content.removeAttribute("hidden");
      focusedIndex = 0;
      focusItem(0);
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("keydown", handleKeydown, true);
    }

    function close() {
      trigger.setAttribute("aria-expanded", "false");
      content.setAttribute("hidden", "");
      focusedIndex = -1;
      document.removeEventListener("click", clickOutside, true);
      document.removeEventListener("keydown", handleKeydown, true);
    }

    function toggle() {
      if (trigger.getAttribute("aria-expanded") === "true") {
        close();
      } else {
        open();
      }
    }

    function getMenuItems() {
      var items = [];
      for (var i = 0; i < content.children.length; i++) {
        var child = content.children[i];
        if (child.getAttribute("role") === "menuitem") {
          items.push(child);
        }
      }
      return items;
    }

    function focusItem(index) {
      var items = getMenuItems();
      if (index < 0 || index >= items.length) return;
      focusedIndex = index;
      for (var i = 0; i < items.length; i++) {
        items[i].tabIndex = i === index ? 0 : -1;
      }
      items[index].focus();
    }

    function clickOutside(e) {
      if (!root.contains(e.target)) {
        close();
      }
    }

    function handleKeydown(e) {
      if (trigger.getAttribute("aria-expanded") !== "true") return;

      var items = getMenuItems();
      if (items.length === 0) return;

      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          focusItem((focusedIndex + 1) % items.length);
          break;
        case "ArrowUp":
          e.preventDefault();
          focusItem((focusedIndex - 1 + items.length) % items.length);
          break;
        case "Enter":
        case " ":
          e.preventDefault();
          if (focusedIndex >= 0 && items[focusedIndex]) {
            items[focusedIndex].click();
            close();
          }
          break;
        case "Escape":
          e.preventDefault();
          close();
          trigger.focus();
          break;
        case "Tab":
          close();
          break;
      }
    }

    trigger.addEventListener("click", toggle);
  };

  // Context menu behavior — right-click menu with fixed positioning
  window.MaudUI.behaviors["context-menu"] = function (root) {
    var region = root.querySelector(".mui-context-menu__region");
    var content = root.querySelector("[role='menu']");
    if (!region || !content) return;

    var focusedIndex = -1;

    function open(x, y) {
      content.style.left = x + "px";
      content.style.top = y + "px";
      content.removeAttribute("hidden");
      focusedIndex = 0;
      focusItem(0);
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("contextmenu", closeOnContext, true);
      document.addEventListener("keydown", handleKeydown, true);
    }

    function close() {
      content.setAttribute("hidden", "");
      focusedIndex = -1;
      document.removeEventListener("click", clickOutside, true);
      document.removeEventListener("contextmenu", closeOnContext, true);
      document.removeEventListener("keydown", handleKeydown, true);
    }

    function getMenuItems() {
      var items = [];
      for (var i = 0; i < content.children.length; i++) {
        var child = content.children[i];
        if (child.getAttribute("role") === "menuitem") {
          items.push(child);
        }
      }
      return items;
    }

    function focusItem(index) {
      var items = getMenuItems();
      if (index < 0 || index >= items.length) return;
      focusedIndex = index;
      for (var i = 0; i < items.length; i++) {
        items[i].tabIndex = i === index ? 0 : -1;
      }
      items[index].focus();
    }

    function clickOutside(e) {
      if (!root.contains(e.target) && !content.contains(e.target)) {
        close();
      }
    }

    function closeOnContext(e) {
      if (!root.contains(e.target)) {
        close();
      }
    }

    function handleKeydown(e) {
      if (content.getAttribute("hidden") !== null) return;

      var items = getMenuItems();
      if (items.length === 0) return;

      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          focusItem((focusedIndex + 1) % items.length);
          break;
        case "ArrowUp":
          e.preventDefault();
          focusItem((focusedIndex - 1 + items.length) % items.length);
          break;
        case "Enter":
        case " ":
          e.preventDefault();
          if (focusedIndex >= 0 && items[focusedIndex]) {
            items[focusedIndex].click();
            close();
          }
          break;
        case "Escape":
          e.preventDefault();
          close();
          break;
        case "Tab":
          close();
          break;
      }
    }

    region.addEventListener("contextmenu", function (e) {
      e.preventDefault();
      open(e.clientX, e.clientY);
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
