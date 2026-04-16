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

    el.addEventListener("focus", function () {
      el.style.outline = "";
    });

    el.addEventListener("blur", function () {
      el.style.outline = "";
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
