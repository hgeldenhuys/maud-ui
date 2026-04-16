(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["nav-menu"] = function (root) {
    var items = root.querySelectorAll(".mui-nav-menu__item[data-has-content]");
    var triggers = [];
    var contents = [];
    var activeIndex = -1;

    for (var i = 0; i < items.length; i++) {
      var trigger = items[i].querySelector(".mui-nav-menu__trigger");
      var content = items[i].querySelector(".mui-nav-menu__content");
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
      if (activeIndex >= 0 && activeIndex !== index) {
        closeMenu(activeIndex);
      }
      activeIndex = index;
      triggers[index].setAttribute("aria-expanded", "true");
      contents[index].removeAttribute("hidden");

      // Focus first link in the panel
      var firstLink = contents[index].querySelector(".mui-nav-menu__sub-link");
      if (firstLink) {
        firstLink.focus();
      }

      document.addEventListener("click", handleClickOutside, true);
      document.addEventListener("keydown", handleGlobalKeydown, true);
    }

    function closeMenu(index) {
      if (index < 0 || index >= triggers.length) return;
      triggers[index].setAttribute("aria-expanded", "false");
      contents[index].setAttribute("hidden", "");
    }

    function closeAll() {
      for (var i = 0; i < triggers.length; i++) {
        closeMenu(i);
      }
      activeIndex = -1;
      document.removeEventListener("click", handleClickOutside, true);
      document.removeEventListener("keydown", handleGlobalKeydown, true);
    }

    function getLinks(index) {
      return contents[index].querySelectorAll(".mui-nav-menu__sub-link");
    }

    function getFocusedLinkIndex(menuIndex) {
      var links = getLinks(menuIndex);
      var focused = document.activeElement;
      for (var i = 0; i < links.length; i++) {
        if (links[i] === focused) return i;
      }
      return -1;
    }

    function focusLink(menuIndex, linkIndex) {
      var links = getLinks(menuIndex);
      if (linkIndex >= 0 && linkIndex < links.length) {
        links[linkIndex].focus();
      }
    }

    // --- Event Handlers ---

    function handleClickOutside(e) {
      if (!root.contains(e.target)) {
        closeAll();
      }
    }

    function handleGlobalKeydown(e) {
      if (!isAnyOpen()) return;

      var links = getLinks(activeIndex);
      var focusedIdx = getFocusedLinkIndex(activeIndex);

      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          if (links.length > 0) {
            var next = focusedIdx < 0 ? 0 : (focusedIdx + 1) % links.length;
            focusLink(activeIndex, next);
          }
          break;

        case "ArrowUp":
          e.preventDefault();
          if (links.length > 0) {
            var prev = focusedIdx <= 0 ? links.length - 1 : focusedIdx - 1;
            focusLink(activeIndex, prev);
          }
          break;

        case "ArrowRight":
          e.preventDefault();
          if (triggers.length > 1) {
            var nextMenu = (activeIndex + 1) % triggers.length;
            openMenu(nextMenu);
          }
          break;

        case "ArrowLeft":
          e.preventDefault();
          if (triggers.length > 1) {
            var prevMenu =
              (activeIndex - 1 + triggers.length) % triggers.length;
            openMenu(prevMenu);
          }
          break;

        case "Escape":
          e.preventDefault();
          var returnTo = activeIndex;
          closeAll();
          if (returnTo >= 0 && returnTo < triggers.length) {
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

    // Keyboard on triggers when no menu is open
    root.addEventListener("keydown", function (e) {
      if (isAnyOpen()) return;

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
          triggers[nextIdx].focus();
          break;

        case "ArrowLeft":
          e.preventDefault();
          var prevIdx =
            (currentIdx - 1 + triggers.length) % triggers.length;
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
