(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["select"] = function (root) {
    var trigger = root.querySelector(".mui-select__trigger");
    var dropdown = root.querySelector("[role='listbox']");
    var hidden = root.querySelector(".mui-select__hidden");
    var valueEl = root.querySelector(".mui-select__value");
    var options = dropdown.querySelectorAll("[role='option']");
    var activeIndex = -1;
    var typeAheadTimeout = null;
    var typeAheadStr = "";

    function indexOf(nodeList, element) {
      for (var i = 0; i < nodeList.length; i++) {
        if (nodeList[i] === element) return i;
      }
      return -1;
    }

    function open() {
      dropdown.removeAttribute("hidden");
      trigger.setAttribute("aria-expanded", "true");
      var sel = dropdown.querySelector(".mui-select__option--selected");
      activeIndex = sel ? indexOf(options, sel) : 0;
      highlight(activeIndex);
      document.addEventListener("click", clickOutside, true);
    }

    function close() {
      dropdown.setAttribute("hidden", "");
      trigger.setAttribute("aria-expanded", "false");
      trigger.removeAttribute("aria-activedescendant");
      unhighlightAll();
      document.removeEventListener("click", clickOutside, true);
      trigger.focus();
    }

    function highlight(idx) {
      unhighlightAll();
      if (idx >= 0 && idx < options.length) {
        activeIndex = idx;
        options[idx].classList.add("mui-select__option--highlighted");
        trigger.setAttribute("aria-activedescendant", options[idx].id);
      } else {
        activeIndex = -1;
        trigger.removeAttribute("aria-activedescendant");
      }
    }

    function unhighlightAll() {
      for (var i = 0; i < options.length; i++) {
        options[i].classList.remove("mui-select__option--highlighted");
      }
    }

    function selectOption(idx) {
      var opt = options[idx];
      if (!opt || opt.getAttribute("aria-disabled") === "true") return;
      for (var i = 0; i < options.length; i++) {
        options[i].classList.remove("mui-select__option--selected");
        options[i].setAttribute("aria-selected", "false");
      }
      opt.classList.add("mui-select__option--selected");
      opt.setAttribute("aria-selected", "true");
      valueEl.textContent = opt.textContent;
      hidden.value = opt.getAttribute("data-value");
      close();
    }

    function clickOutside(e) {
      if (!root.contains(e.target)) {
        close();
      }
    }

    trigger.addEventListener("click", function () {
      if (trigger.getAttribute("aria-expanded") === "true") {
        close();
      } else {
        open();
      }
    });

    trigger.addEventListener("keydown", function (e) {
      var isOpen = trigger.getAttribute("aria-expanded") === "true";
      var nextIdx;

      if (e.key === "ArrowDown" || e.key === "ArrowUp") {
        e.preventDefault();
        if (!isOpen) {
          open();
        } else {
          nextIdx = activeIndex + (e.key === "ArrowDown" ? 1 : -1);
          while (nextIdx >= 0 && nextIdx < options.length) {
            if (options[nextIdx].getAttribute("aria-disabled") !== "true") break;
            nextIdx += e.key === "ArrowDown" ? 1 : -1;
          }
          if (nextIdx >= 0 && nextIdx < options.length) {
            highlight(nextIdx);
          }
        }
      } else if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        if (isOpen && activeIndex >= 0) {
          selectOption(activeIndex);
        } else {
          open();
        }
      } else if (e.key === "Escape") {
        e.preventDefault();
        if (isOpen) {
          close();
        }
      } else if (e.key.length === 1 && isOpen) {
        e.preventDefault();
        typeAheadStr += e.key.toLowerCase();
        clearTimeout(typeAheadTimeout);
        typeAheadTimeout = setTimeout(function () {
          typeAheadStr = "";
        }, 500);

        for (var i = 0; i < options.length; i++) {
          if (options[i].getAttribute("aria-disabled") !== "true") {
            var optText = options[i].textContent.toLowerCase();
            if (optText.indexOf(typeAheadStr) === 0) {
              highlight(i);
              break;
            }
          }
        }
      }
    });

    // Event delegation on dropdown instead of per-option listeners
    dropdown.addEventListener("click", function (e) {
      var option = e.target.closest("[role='option']");
      if (option) {
        var idx = indexOf(options, option);
        if (idx >= 0) {
          selectOption(idx);
        }
      }
    });
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();
