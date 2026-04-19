(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // TODO: implement multi-select chip interaction (data-multiple), clear-button
  // click handler (.mui-combobox__clear), and auto-highlight commit-on-enter
  // behaviour. Current JS handles only single-select.
  window.MaudUI.behaviors["combobox"] = function (root) {
    var trigger = root.querySelector(".mui-combobox__trigger");
    var dropdown = root.querySelector(".mui-combobox__dropdown");
    var searchInput = root.querySelector(".mui-combobox__search");
    var list = root.querySelector(".mui-combobox__list");
    var emptyEl = root.querySelector(".mui-combobox__empty");
    var valueEl = root.querySelector(".mui-combobox__value");
    var hidden = root.querySelector(".mui-combobox__hidden");
    var options = list.querySelectorAll(".mui-combobox__option");
    var activeIndex = -1;

    function visibleOptions() {
      var result = [];
      for (var i = 0; i < options.length; i++) {
        if (options[i].style.display !== "none") {
          result.push(options[i]);
        }
      }
      return result;
    }

    function open() {
      dropdown.removeAttribute("hidden");
      trigger.setAttribute("aria-expanded", "true");
      searchInput.value = "";
      filterOptions("");
      activeIndex = -1;
      searchInput.focus();
      document.addEventListener("click", clickOutside, true);
    }

    function close() {
      dropdown.setAttribute("hidden", "");
      trigger.setAttribute("aria-expanded", "false");
      unhighlightAll();
      activeIndex = -1;
      document.removeEventListener("click", clickOutside, true);
      trigger.focus();
    }

    function filterOptions(query) {
      var q = query.toLowerCase();
      var visibleCount = 0;
      for (var i = 0; i < options.length; i++) {
        var label = options[i].querySelector(".mui-combobox__option-label").textContent.toLowerCase();
        var visible = !q || label.indexOf(q) !== -1;
        options[i].style.display = visible ? "" : "none";
        if (visible) visibleCount++;
      }
      if (visibleCount === 0) {
        emptyEl.removeAttribute("hidden");
        emptyEl.style.display = "";
      } else {
        emptyEl.setAttribute("hidden", "");
        emptyEl.style.display = "none";
      }
      unhighlightAll();
      activeIndex = -1;
    }

    function highlight(idx) {
      var vis = visibleOptions();
      unhighlightAll();
      if (idx >= 0 && idx < vis.length) {
        activeIndex = idx;
        vis[idx].classList.add("mui-combobox__option--highlighted");
        vis[idx].scrollIntoView({ block: "nearest" });
      } else {
        activeIndex = -1;
      }
    }

    function unhighlightAll() {
      for (var i = 0; i < options.length; i++) {
        options[i].classList.remove("mui-combobox__option--highlighted");
      }
    }

    function selectOption(opt) {
      for (var i = 0; i < options.length; i++) {
        options[i].setAttribute("aria-selected", "false");
        options[i].classList.remove("mui-combobox__option--selected");
      }
      opt.setAttribute("aria-selected", "true");
      opt.classList.add("mui-combobox__option--selected");
      var value = opt.getAttribute("data-value");
      var label = opt.querySelector(".mui-combobox__option-label").textContent;
      valueEl.textContent = label;
      hidden.value = value;
      close();
    }

    function clickOutside(e) {
      if (!root.contains(e.target)) {
        close();
      }
    }

    // Toggle on trigger click
    trigger.addEventListener("click", function () {
      if (trigger.getAttribute("aria-expanded") === "true") {
        close();
      } else {
        open();
      }
    });

    // Search input events
    searchInput.addEventListener("input", function () {
      filterOptions(searchInput.value);
    });

    searchInput.addEventListener("keydown", function (e) {
      var vis = visibleOptions();

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
          selectOption(vis[activeIndex]);
        }
      } else if (e.key === "Escape") {
        e.preventDefault();
        close();
      }
    });

    // Trigger keyboard (open on arrow/enter/space)
    trigger.addEventListener("keydown", function (e) {
      var isOpen = trigger.getAttribute("aria-expanded") === "true";
      if (!isOpen && (e.key === "ArrowDown" || e.key === "ArrowUp" || e.key === "Enter" || e.key === " ")) {
        e.preventDefault();
        open();
      } else if (isOpen && e.key === "Escape") {
        e.preventDefault();
        close();
      }
    });

    // Event delegation for option clicks
    list.addEventListener("click", function (e) {
      var option = e.target.closest(".mui-combobox__option");
      if (option) {
        selectOption(option);
      }
    });
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();
