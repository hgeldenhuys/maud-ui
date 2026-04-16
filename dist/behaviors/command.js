(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // Command trigger — opens the palette
  window.MaudUI.behaviors["command-trigger"] = function (el) {
    el.addEventListener("click", function () {
      var targetId = el.getAttribute("data-target");
      var dialog = targetId ? document.getElementById(targetId) : null;
      if (dialog && dialog.showModal) {
        dialog.showModal();
        var search = dialog.querySelector(".mui-command__search");
        if (search) {
          search.value = "";
          search.focus();
        }
        // Reset filter state on open
        resetFilter(dialog);
      }
    });
  };

  // Command palette behavior
  window.MaudUI.behaviors["command"] = function (el) {
    var searchInput = el.querySelector(".mui-command__search");
    var list = el.querySelector(".mui-command__list");
    var emptyEl = el.querySelector(".mui-command__empty");
    var items = el.querySelectorAll(".mui-command__item");
    var groups = el.querySelectorAll(".mui-command__group");
    var activeIndex = -1;

    function visibleItems() {
      var result = [];
      for (var i = 0; i < items.length; i++) {
        if (items[i].getAttribute("data-hidden") !== "true" &&
            !items[i].classList.contains("mui-command__item--disabled")) {
          result.push(items[i]);
        }
      }
      return result;
    }

    function filterItems(query) {
      var q = query.toLowerCase();
      var visibleCount = 0;

      for (var i = 0; i < items.length; i++) {
        var label = items[i].getAttribute("data-label") || "";
        var match = !q || label.toLowerCase().indexOf(q) !== -1;
        if (match) {
          items[i].removeAttribute("data-hidden");
          visibleCount++;
        } else {
          items[i].setAttribute("data-hidden", "true");
        }
      }

      // Hide groups that have no visible items
      for (var g = 0; g < groups.length; g++) {
        var groupItems = groups[g].querySelectorAll(".mui-command__item");
        var anyVisible = false;
        for (var j = 0; j < groupItems.length; j++) {
          if (groupItems[j].getAttribute("data-hidden") !== "true") {
            anyVisible = true;
            break;
          }
        }
        if (anyVisible) {
          groups[g].removeAttribute("data-hidden");
        } else {
          groups[g].setAttribute("data-hidden", "true");
        }
      }

      // Show/hide empty state
      if (visibleCount === 0) {
        emptyEl.removeAttribute("hidden");
      } else {
        emptyEl.setAttribute("hidden", "");
      }

      unhighlightAll();
      activeIndex = -1;
    }

    function highlight(idx) {
      var vis = visibleItems();
      unhighlightAll();
      if (idx >= 0 && idx < vis.length) {
        activeIndex = idx;
        vis[idx].classList.add("mui-command__item--highlighted");
        vis[idx].scrollIntoView({ block: "nearest" });
      } else {
        activeIndex = -1;
      }
    }

    function unhighlightAll() {
      for (var i = 0; i < items.length; i++) {
        items[i].classList.remove("mui-command__item--highlighted");
      }
    }

    function selectItem(item) {
      var label = item.getAttribute("data-label") || "";
      el.close();
      // Dispatch custom event for consumers
      el.dispatchEvent(new CustomEvent("mui-command-select", {
        detail: { label: label },
        bubbles: true
      }));
    }

    // Search input filtering
    searchInput.addEventListener("input", function () {
      filterItems(searchInput.value);
    });

    // Keyboard navigation
    searchInput.addEventListener("keydown", function (e) {
      var vis = visibleItems();

      if (e.key === "ArrowDown") {
        e.preventDefault();
        var next = activeIndex + 1;
        if (next < vis.length) {
          highlight(next);
        }
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        var prev = activeIndex - 1;
        if (prev >= 0) {
          highlight(prev);
        }
      } else if (e.key === "Enter") {
        e.preventDefault();
        if (activeIndex >= 0 && activeIndex < vis.length) {
          selectItem(vis[activeIndex]);
        }
      }
      // Escape is handled natively by <dialog>
    });

    // Click on items
    list.addEventListener("click", function (e) {
      var item = e.target.closest(".mui-command__item");
      if (item && !item.classList.contains("mui-command__item--disabled")) {
        selectItem(item);
      }
    });

    // Backdrop click closes
    el.addEventListener("click", function (e) {
      if (e.target === el) {
        el.close();
      }
    });

    // Reset on close
    el.addEventListener("close", function () {
      searchInput.value = "";
      resetFilter(el);
      unhighlightAll();
      activeIndex = -1;
    });
  };

  // Helper: reset filter state
  function resetFilter(dialog) {
    var items = dialog.querySelectorAll(".mui-command__item");
    var groups = dialog.querySelectorAll(".mui-command__group");
    var emptyEl = dialog.querySelector(".mui-command__empty");

    for (var i = 0; i < items.length; i++) {
      items[i].removeAttribute("data-hidden");
    }
    for (var g = 0; g < groups.length; g++) {
      groups[g].removeAttribute("data-hidden");
    }
    if (emptyEl) {
      emptyEl.setAttribute("hidden", "");
    }
  }

  // Global Cmd+K / Ctrl+K keyboard shortcut
  document.addEventListener("keydown", function (e) {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault();
      // Find the first command palette on the page
      var command = document.querySelector("[data-mui='command']");
      if (command && command.showModal && !command.open) {
        command.showModal();
        var search = command.querySelector(".mui-command__search");
        if (search) {
          search.value = "";
          search.focus();
        }
        resetFilter(command);
      }
    }
  });

  if (window.MaudUI.init) window.MaudUI.init();
})();
