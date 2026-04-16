(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["menubar"] = function (root) {
    var triggers = [];
    var contents = [];
    var menuWrappers = root.querySelectorAll(".mui-menubar__menu");
    var activeIndex = -1;

    // Collect triggers and content panels
    for (var i = 0; i < menuWrappers.length; i++) {
      var trigger = menuWrappers[i].querySelector(".mui-menubar__trigger");
      var content = menuWrappers[i].querySelector(".mui-menubar__content");
      if (trigger && content) {
        triggers.push(trigger);
        contents.push(content);
      }
    }

    if (triggers.length === 0) return;

    function isAnyOpen() {
      return activeIndex >= 0;
    }

    function openMenu(index) {
      // Close current if different
      if (activeIndex >= 0 && activeIndex !== index) {
        closeMenu(activeIndex);
      }
      activeIndex = index;
      triggers[index].setAttribute("aria-expanded", "true");
      contents[index].removeAttribute("hidden");

      // Set roving tabindex
      for (var i = 0; i < triggers.length; i++) {
        triggers[i].tabIndex = i === index ? 0 : -1;
      }

      // Focus first item in the dropdown
      var firstItem = getMenuItems(index);
      if (firstItem.length > 0) {
        firstItem[0].tabIndex = 0;
        firstItem[0].focus();
      }

      // Attach global listeners
      document.addEventListener("click", handleClickOutside, true);
      document.addEventListener("keydown", handleGlobalKeydown, true);
    }

    function closeMenu(index) {
      if (index < 0 || index >= triggers.length) return;
      triggers[index].setAttribute("aria-expanded", "false");
      contents[index].setAttribute("hidden", "");

      // Reset item tabindices
      var items = getMenuItems(index);
      for (var i = 0; i < items.length; i++) {
        items[i].tabIndex = -1;
      }
    }

    function closeAll() {
      for (var i = 0; i < triggers.length; i++) {
        closeMenu(i);
      }
      activeIndex = -1;
      document.removeEventListener("click", handleClickOutside, true);
      document.removeEventListener("keydown", handleGlobalKeydown, true);
    }

    function getMenuItems(index) {
      var items = [];
      var children = contents[index].children;
      for (var j = 0; j < children.length; j++) {
        if (children[j].getAttribute("role") === "menuitem") {
          items.push(children[j]);
        }
      }
      return items;
    }

    function getFocusedItemIndex(menuIndex) {
      var items = getMenuItems(menuIndex);
      var focused = document.activeElement;
      for (var i = 0; i < items.length; i++) {
        if (items[i] === focused) return i;
      }
      return -1;
    }

    function focusItem(menuIndex, itemIndex) {
      var items = getMenuItems(menuIndex);
      if (itemIndex < 0 || itemIndex >= items.length) return;
      for (var i = 0; i < items.length; i++) {
        items[i].tabIndex = i === itemIndex ? 0 : -1;
      }
      items[itemIndex].focus();
    }

    // --- Event Handlers ---

    function handleClickOutside(e) {
      if (!root.contains(e.target)) {
        closeAll();
      }
    }

    function handleGlobalKeydown(e) {
      if (!isAnyOpen()) return;

      var items = getMenuItems(activeIndex);
      var focusedIdx = getFocusedItemIndex(activeIndex);

      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          if (items.length > 0) {
            var next = focusedIdx < 0 ? 0 : (focusedIdx + 1) % items.length;
            focusItem(activeIndex, next);
          }
          break;

        case "ArrowUp":
          e.preventDefault();
          if (items.length > 0) {
            var prev = focusedIdx <= 0 ? items.length - 1 : focusedIdx - 1;
            focusItem(activeIndex, prev);
          }
          break;

        case "ArrowRight":
          e.preventDefault();
          var nextMenu = (activeIndex + 1) % triggers.length;
          openMenu(nextMenu);
          break;

        case "ArrowLeft":
          e.preventDefault();
          var prevMenu = (activeIndex - 1 + triggers.length) % triggers.length;
          openMenu(prevMenu);
          break;

        case "Enter":
        case " ":
          e.preventDefault();
          if (focusedIdx >= 0 && items[focusedIdx] && !items[focusedIdx].disabled) {
            items[focusedIdx].click();
            closeAll();
            triggers[activeIndex >= 0 ? activeIndex : 0].focus();
          }
          break;

        case "Escape":
          e.preventDefault();
          var returnTo = activeIndex;
          closeAll();
          if (returnTo >= 0) {
            triggers[returnTo].focus();
          }
          break;

        case "Tab":
          closeAll();
          break;
      }
    }

    // Click on trigger: toggle that menu
    for (var t = 0; t < triggers.length; t++) {
      (function (idx) {
        triggers[idx].addEventListener("click", function (e) {
          e.stopPropagation();
          if (activeIndex === idx) {
            closeAll();
            triggers[idx].focus();
          } else {
            openMenu(idx);
          }
        });

        // Hover while another menu is open: switch to this menu
        triggers[idx].addEventListener("mouseenter", function () {
          if (isAnyOpen() && activeIndex !== idx) {
            openMenu(idx);
          }
        });
      })(t);
    }

    // Item click: close menubar
    for (var c = 0; c < contents.length; c++) {
      (function (menuIdx) {
        contents[menuIdx].addEventListener("click", function (e) {
          var target = e.target;
          while (target && target !== contents[menuIdx]) {
            if (target.getAttribute("role") === "menuitem" && !target.disabled) {
              closeAll();
              triggers[menuIdx].focus();
              return;
            }
            target = target.parentElement;
          }
        });
      })(c);
    }

    // Keyboard on the menubar triggers (when no menu is open)
    root.addEventListener("keydown", function (e) {
      if (isAnyOpen()) return; // Handled by global handler

      var focused = document.activeElement;
      var currentIdx = -1;
      for (var i = 0; i < triggers.length; i++) {
        if (triggers[i] === focused) {
          currentIdx = i;
          break;
        }
      }
      if (currentIdx < 0) return;

      switch (e.key) {
        case "ArrowRight":
          e.preventDefault();
          var nextIdx = (currentIdx + 1) % triggers.length;
          for (var i = 0; i < triggers.length; i++) {
            triggers[i].tabIndex = i === nextIdx ? 0 : -1;
          }
          triggers[nextIdx].focus();
          break;

        case "ArrowLeft":
          e.preventDefault();
          var prevIdx = (currentIdx - 1 + triggers.length) % triggers.length;
          for (var i = 0; i < triggers.length; i++) {
            triggers[i].tabIndex = i === prevIdx ? 0 : -1;
          }
          triggers[prevIdx].focus();
          break;

        case "ArrowDown":
        case "Enter":
        case " ":
          e.preventDefault();
          openMenu(currentIdx);
          break;
      }
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
