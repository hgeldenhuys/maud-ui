// maud-ui runtime — behaviors register into MaudUI.behaviors and self-initialize
// via `data-mui="<name>"` attributes. Wave 2/3 agents add behaviors here.
(function () {
  "use strict";

  var MaudUI = {
    behaviors: {},
    init: function (root) {
      if (!root) root = document.body;
      var els = root.querySelectorAll("[data-mui]");
      for (var i = 0; i < els.length; i++) {
        var el = els[i];
        if (el.hasAttribute("data-mui-init")) continue;
        var type = el.getAttribute("data-mui");
        if (type && MaudUI.behaviors[type]) {
          MaudUI.behaviors[type](el);
          el.setAttribute("data-mui-init", "");
        }
      }
    },
  };

  window.MaudUI = MaudUI;

  document.addEventListener("DOMContentLoaded", function () {
    MaudUI.init();
  });
  document.addEventListener("htmx:afterSwap", function (e) {
    if (e.target instanceof Element) MaudUI.init(e.target);
  });
})();

// ═══ Component behaviors ═══

// --- accordion.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["accordion"] = function (root) {
    var isMultiple = root.getAttribute("data-multiple") === "true";
    var triggers = root.querySelectorAll(".mui-accordion__trigger");

    if (triggers.length === 0) return;

    // Track focus for roving tabindex (all triggers stay tabbable, just track focus)
    for (var i = 0; i < triggers.length; i++) {
      var trigger = triggers[i];

      // Click handler
      trigger.addEventListener("click", function (event) {
        var currentTrigger = event.currentTarget;
        handleTriggerToggle(currentTrigger, isMultiple);
      });

      // Keyboard handlers: Space, Enter, ArrowDown, ArrowUp, Home, End
      trigger.addEventListener("keydown", function (event) {
        var currentTrigger = event.currentTarget;
        var currentIndex = Array.prototype.indexOf.call(triggers, currentTrigger);

        if (event.key === " " || event.key === "Enter") {
          event.preventDefault();
          handleTriggerToggle(currentTrigger, isMultiple);
        } else if (event.key === "ArrowDown") {
          event.preventDefault();
          var nextIndex = (currentIndex + 1) % triggers.length;
          triggers[nextIndex].focus();
        } else if (event.key === "ArrowUp") {
          event.preventDefault();
          var prevIndex = currentIndex === 0 ? triggers.length - 1 : currentIndex - 1;
          triggers[prevIndex].focus();
        } else if (event.key === "Home") {
          event.preventDefault();
          triggers[0].focus();
        } else if (event.key === "End") {
          event.preventDefault();
          triggers[triggers.length - 1].focus();
        }
      });
    }

    function handleTriggerToggle(trigger, isMultiple) {
      var isExpanded = trigger.getAttribute("aria-expanded") === "true";
      var contentId = trigger.getAttribute("aria-controls");
      var content = document.getElementById(contentId);

      if (!content) return;

      // If single-open mode and opening this item, close all others
      if (!isMultiple && !isExpanded) {
        for (var i = 0; i < triggers.length; i++) {
          var otherTrigger = triggers[i];
          if (otherTrigger !== trigger && otherTrigger.getAttribute("aria-expanded") === "true") {
            var otherContentId = otherTrigger.getAttribute("aria-controls");
            var otherContent = document.getElementById(otherContentId);
            if (otherContent) {
              otherTrigger.setAttribute("aria-expanded", "false");
              otherContent.setAttribute("hidden", "");
            }
          }
        }
      }

      // Toggle current item
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

// --- collapsible.js ---
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

// --- dialog.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // Dialog Trigger Behavior
  window.MaudUI.behaviors["dialog-trigger"] = function (el) {
    el.addEventListener("click", function () {
      var target_id = el.getAttribute("data-target");
      var dialog = target_id ? document.getElementById(target_id) : null;
      if (dialog && dialog.showModal) {
        dialog.showModal();
      }
    });
  };

  // Alert Dialog Trigger Behavior
  window.MaudUI.behaviors["alert-dialog-trigger"] = function (el) {
    el.addEventListener("click", function () {
      var target_id = el.getAttribute("data-target");
      var dialog = target_id ? document.getElementById(target_id) : null;
      if (dialog && dialog.showModal) {
        dialog.showModal();
      }
    });
  };

  // Dialog Behavior
  window.MaudUI.behaviors["dialog"] = function (el) {
    // Wire close buttons inside the dialog
    var closeBtns = el.querySelectorAll("[data-mui-close]");
    for (var i = 0; i < closeBtns.length; i++) {
      closeBtns[i].addEventListener("click", function () {
        el.close();
      });
    }

    // Click on backdrop closes dialog (when clicking on dialog element outside content)
    el.addEventListener("click", function (e) {
      if (e.target === el) {
        el.close();
      }
    });

    // Native <dialog> with showModal() handles:
    // - ESC closes the dialog (unless prevented by cancel event)
    // - Focus trapping inside the dialog
  };

  // Alert Dialog Behavior
  window.MaudUI.behaviors["alert-dialog"] = function (el) {
    // Wire close buttons inside the alert dialog only
    var closeBtns = el.querySelectorAll("[data-mui-close]");
    for (var i = 0; i < closeBtns.length; i++) {
      closeBtns[i].addEventListener("click", function () {
        el.close();
      });
    }

    // Prevent ESC key from closing alert dialog by canceling the cancel event
    el.addEventListener("cancel", function (e) {
      e.preventDefault();
    });

    // Prevent backdrop click from closing (don't listen to click outside)
    // The click handler is intentionally omitted for alert dialogs
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();

// --- menu.js ---
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

// --- number_field.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  function updateValue(input, newVal) {
    var min = input.hasAttribute("min") ? parseFloat(input.min) : -Infinity;
    var max = input.hasAttribute("max") ? parseFloat(input.max) : Infinity;
    var step = parseFloat(input.step) || 1;
    // Round to step precision
    newVal = Math.round(newVal / step) * step;
    newVal = Math.max(min, Math.min(max, newVal));
    // Fix floating point
    var decimals = (step.toString().split(".")[1] || "").length;
    input.value = newVal.toFixed(decimals);
    input.dispatchEvent(new Event("change", { bubbles: true }));
  }

  window.MaudUI.behaviors["number-field-dec"] = function (el) {
    el.addEventListener("click", function () {
      if (el.disabled) return;
      var input = el.parentElement.querySelector("input[type='number']");
      if (!input || input.disabled) return;
      var current = parseFloat(input.value) || 0;
      var step = parseFloat(input.step) || 1;
      updateValue(input, current - step);
    });
  };

  window.MaudUI.behaviors["number-field-inc"] = function (el) {
    el.addEventListener("click", function () {
      if (el.disabled) return;
      var input = el.parentElement.querySelector("input[type='number']");
      if (!input || input.disabled) return;
      var current = parseFloat(input.value) || 0;
      var step = parseFloat(input.step) || 1;
      updateValue(input, current + step);
    });
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();

// --- popover.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["popover"] = function (root) {
    var trigger = root.querySelector(".mui-popover__trigger");
    var content = root.querySelector(".mui-popover__content");
    if (!trigger || !content) return;

    function toggle() {
      var expanded = trigger.getAttribute("aria-expanded") === "true";
      if (expanded) close();
      else open();
    }

    function open() {
      trigger.setAttribute("aria-expanded", "true");
      content.removeAttribute("hidden");
      content.setAttribute("data-visible", "true");
      content.focus();
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("keydown", escClose, true);
    }

    function close() {
      trigger.setAttribute("aria-expanded", "false");
      content.setAttribute("hidden", "");
      content.setAttribute("data-visible", "false");
      document.removeEventListener("click", clickOutside, true);
      document.removeEventListener("keydown", escClose, true);
    }

    function clickOutside(e) {
      if (!root.contains(e.target)) close();
    }

    function escClose(e) {
      if (e.key === "Escape") close();
    }

    trigger.addEventListener("click", toggle);
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();

// --- scroll_area.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["scroll-area"] = function (root) {
    var viewport = root.querySelector(".mui-scroll-area__viewport");
    var scrollbar = root.querySelector(".mui-scroll-area__scrollbar");
    var thumb = root.querySelector(".mui-scroll-area__thumb");
    if (!viewport || !scrollbar || !thumb) return;

    var hideTimer = null;

    function updateThumb() {
      var ratio = viewport.clientHeight / viewport.scrollHeight;
      if (ratio >= 1) {
        scrollbar.style.display = "none";
        return;
      }
      scrollbar.style.display = "block";
      var thumbHeight = Math.max(ratio * 100, 10); // min 10% height
      var scrollRatio = viewport.scrollTop / (viewport.scrollHeight - viewport.clientHeight);
      var thumbTop = scrollRatio * (100 - thumbHeight);
      thumb.style.height = thumbHeight + "%";
      thumb.style.top = thumbTop + "%";

      // Show scrollbar, hide after 1s idle
      scrollbar.classList.add("mui-scroll-area__scrollbar--visible");
      clearTimeout(hideTimer);
      hideTimer = setTimeout(function () {
        scrollbar.classList.remove("mui-scroll-area__scrollbar--visible");
      }, 1000);
    }

    viewport.addEventListener("scroll", updateThumb);

    // Thumb drag
    var dragging = false;
    var startY = 0;
    var startScroll = 0;

    thumb.addEventListener("mousedown", function (e) {
      e.preventDefault();
      dragging = true;
      startY = e.clientY;
      startScroll = viewport.scrollTop;
      document.addEventListener("mousemove", onDrag);
      document.addEventListener(
        "mouseup",
        function () {
          dragging = false;
          document.removeEventListener("mousemove", onDrag);
        },
        { once: true }
      );
    });

    function onDrag(e) {
      if (!dragging) return;
      var delta = e.clientY - startY;
      var scrollable = viewport.scrollHeight - viewport.clientHeight;
      var trackHeight = scrollbar.clientHeight;
      viewport.scrollTop = startScroll + (delta / trackHeight) * scrollable;
    }

    // Initial update
    updateThumb();
    // Observe content changes
    if (window.ResizeObserver) {
      new ResizeObserver(updateThumb).observe(viewport);
    }
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();

// --- select.js ---
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

// --- slider.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["slider"] = function (root) {
    var thumb = root.querySelector(".mui-slider__thumb");
    var fill = root.querySelector(".mui-slider__fill");
    var track = root.querySelector(".mui-slider__track");
    var native = root.querySelector(".mui-slider__native");
    var valueLabel = root.querySelector(".mui-slider__value");

    var min = parseFloat(root.getAttribute("data-min") || "0");
    var max = parseFloat(root.getAttribute("data-max") || "100");
    var step = parseFloat(root.getAttribute("data-step") || "1");

    function setValue(val) {
      val = Math.round(val / step) * step;
      val = Math.max(min, Math.min(max, val));
      var pct = ((val - min) / (max - min)) * 100;
      thumb.style.left = pct + "%";
      fill.style.width = pct + "%";
      thumb.setAttribute("aria-valuenow", val);
      native.value = val;
      if (valueLabel) valueLabel.textContent = val;
    }

    // Keyboard: Arrow keys increment/decrement
    thumb.addEventListener("keydown", function (e) {
      if (thumb.getAttribute("aria-disabled") === "true") return;
      var current = parseFloat(thumb.getAttribute("aria-valuenow"));
      if (e.key === "ArrowRight" || e.key === "ArrowUp") {
        e.preventDefault();
        setValue(current + step);
      }
      if (e.key === "ArrowLeft" || e.key === "ArrowDown") {
        e.preventDefault();
        setValue(current - step);
      }
      if (e.key === "Home") {
        e.preventDefault();
        setValue(min);
      }
      if (e.key === "End") {
        e.preventDefault();
        setValue(max);
      }
    });

    // Mouse: drag thumb
    var dragging = false;
    function onDrag(e) {
      if (!dragging) return;
      var rect = track.getBoundingClientRect();
      var pct = (e.clientX - rect.left) / rect.width;
      setValue(min + pct * (max - min));
    }
    thumb.addEventListener("mousedown", function (e) {
      if (thumb.getAttribute("aria-disabled") === "true") return;
      e.preventDefault();
      dragging = true;
      document.addEventListener("mousemove", onDrag);
      document.addEventListener("mouseup", function () {
        dragging = false;
        document.removeEventListener("mousemove", onDrag);
      }, { once: true });
    });

    // Click on track
    track.addEventListener("click", function (e) {
      if (thumb.getAttribute("aria-disabled") === "true") return;
      var rect = track.getBoundingClientRect();
      var pct = (e.clientX - rect.left) / rect.width;
      setValue(min + pct * (max - min));
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();

// --- switch.js ---
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

// --- tabs.js ---
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

// --- theme.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["theme-toggle"] = function (el) {
    el.addEventListener("click", function () {
      var html = document.documentElement;
      var current = html.getAttribute("data-theme") || "dark";
      var next = current === "dark" ? "light" : "dark";
      html.setAttribute("data-theme", next);
      el.textContent = next === "dark" ? "Light mode" : "Dark mode";
      try { localStorage.setItem("mui-theme", next); } catch(e) {}
    });

    // Restore saved theme
    try {
      var saved = localStorage.getItem("mui-theme");
      if (saved && saved !== (document.documentElement.getAttribute("data-theme") || "dark")) {
        document.documentElement.setAttribute("data-theme", saved);
        el.textContent = saved === "dark" ? "Light mode" : "Dark mode";
      }
    } catch(e) {}
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();

// --- toast.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["toast"] = function (el) {
    // el is the toast element — wire close button + auto-dismiss
    const closeBtn = el.querySelector(".mui-toast__close");
    if (closeBtn) {
      closeBtn.addEventListener("click", () => dismissToast(el));
    }

    const duration = parseInt(el.getAttribute("data-duration") || "5000", 10);
    if (duration > 0) {
      let timer = setTimeout(() => dismissToast(el), duration);
      el.addEventListener("mouseenter", () => clearTimeout(timer));
    }
  };

  function dismissToast(el) {
    el.classList.add("mui-toast--exit");
    setTimeout(() => el.remove(), 200);
  }

  // Imperative API: window.MaudUI.toast({ title, description, variant, duration_ms })
  window.MaudUI.toast = function (opts) {
    const viewport = document.getElementById("mui-toast-viewport");
    if (!viewport) {
      console.warn("Toast viewport not found. Call maud_ui::primitives::toast::viewport() in your page.");
      return;
    }

    const variant = opts.variant || "info";
    const title = opts.title || "";
    const description = opts.description || "";
    const duration = opts.duration_ms != null ? opts.duration_ms : 5000;

    const toast = document.createElement("div");
    toast.className = "mui-toast mui-toast--" + variant;
    toast.setAttribute("role", variant === "danger" ? "alert" : "status");
    toast.setAttribute("aria-live", variant === "danger" ? "assertive" : "polite");
    toast.setAttribute("data-mui", "toast");
    toast.setAttribute("data-duration", String(duration));

    const titleEl = document.createElement("div");
    titleEl.className = "mui-toast__title";
    titleEl.textContent = title;
    toast.appendChild(titleEl);

    if (description) {
      const descEl = document.createElement("div");
      descEl.className = "mui-toast__description";
      descEl.textContent = description;
      toast.appendChild(descEl);
    }

    const closeEl = document.createElement("button");
    closeEl.type = "button";
    closeEl.className = "mui-toast__close";
    closeEl.setAttribute("aria-label", "Dismiss");
    closeEl.textContent = "×";
    toast.appendChild(closeEl);

    viewport.appendChild(toast);

    // Trigger behavior attachment if MaudUI.init exists
    if (window.MaudUI.init) window.MaudUI.init(viewport);
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();

// --- toggle.js ---
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

// --- tooltip.js ---
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["tooltip"] = function (wrapper) {
    // wrapper is .mui-tooltip with a trigger inside and a .mui-tooltip__content sibling
    const trigger = wrapper.querySelector(".mui-tooltip__trigger");
    const content = wrapper.querySelector(".mui-tooltip__content");
    if (!trigger || !content) return;

    const delay = parseInt(wrapper.getAttribute("data-delay") || "500", 10);
    let showTimer = null;
    let hideTimer = null;

    function show() {
      clearTimeout(hideTimer);
      clearTimeout(showTimer);
      showTimer = setTimeout(() => {
        content.setAttribute("data-visible", "true");
        content.removeAttribute("hidden");
      }, delay);
    }

    function hide() {
      clearTimeout(showTimer);
      clearTimeout(hideTimer);
      hideTimer = setTimeout(() => {
        content.setAttribute("data-visible", "false");
        content.setAttribute("hidden", "");
      }, 100);
    }

    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    trigger.addEventListener("focus", show);
    trigger.addEventListener("blur", hide);
    // Escape dismisses
    trigger.addEventListener("keydown", (e) => {
      if (e.key === "Escape") hide();
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
