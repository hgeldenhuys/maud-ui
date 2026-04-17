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


// ============ Component Behaviors ============

// ============ accordion.js ============
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


// ============ calendar.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  var MONTHS = [
    "January","February","March","April","May","June",
    "July","August","September","October","November","December"
  ];

  function isLeapYear(y) { return (y % 4 === 0 && y % 100 !== 0) || y % 400 === 0; }

  function daysInMonth(y, m) {
    if (m === 2) return isLeapYear(y) ? 29 : 28;
    if (m === 4 || m === 6 || m === 9 || m === 11) return 30;
    return 31;
  }

  // Tomohiko Sakamoto — returns 0=Sun..6=Sat
  function dayOfWeek(y, m, d) {
    var t = [0,3,2,5,0,3,5,1,4,6,2,4];
    if (m < 3) y--;
    return (y + Math.floor(y/4) - Math.floor(y/100) + Math.floor(y/400) + t[m-1] + d) % 7;
  }

  function fmtDate(y, m, d) {
    return String(y) + "-" + (m < 10 ? "0" : "") + m + "-" + (d < 10 ? "0" : "") + d;
  }

  function parseDate(s) {
    if (!s) return null;
    var p = s.split("-");
    if (p.length !== 3) return null;
    return { y: parseInt(p[0],10), m: parseInt(p[1],10), d: parseInt(p[2],10) };
  }

  function dateCmp(a, b) {
    if (a.y !== b.y) return a.y < b.y ? -1 : 1;
    if (a.m !== b.m) return a.m < b.m ? -1 : 1;
    if (a.d !== b.d) return a.d < b.d ? -1 : 1;
    return 0;
  }

  function isDisabled(y, m, d, minD, maxD) {
    var cur = {y:y, m:m, d:d};
    if (minD && dateCmp(cur, minD) < 0) return true;
    if (maxD && dateCmp(cur, maxD) > 0) return true;
    return false;
  }

  window.MaudUI.behaviors["calendar"] = function (root) {
    var title = root.querySelector(".mui-calendar__title");
    var grid = root.querySelector(".mui-calendar__grid");
    var hidden = root.querySelector(".mui-calendar__value");
    var prevBtn = root.querySelector(".mui-calendar__nav--prev");
    var nextBtn = root.querySelector(".mui-calendar__nav--next");

    var currentYear = parseInt(root.getAttribute("data-year"), 10);
    var currentMonth = parseInt(root.getAttribute("data-month"), 10);
    var selected = root.getAttribute("data-selected") || "";
    var minDate = parseDate(root.getAttribute("data-min"));
    var maxDate = parseDate(root.getAttribute("data-max"));
    var showOutside = root.getAttribute("data-show-outside") !== "false";

    // Today
    var now = new Date();
    var todayStr = fmtDate(now.getFullYear(), now.getMonth() + 1, now.getDate());

    // Mark today on initial server-rendered grid
    markToday();

    function markToday() {
      var days = grid.querySelectorAll(".mui-calendar__day");
      for (var i = 0; i < days.length; i++) {
        var btn = days[i];
        if (btn.getAttribute("data-date") === todayStr) {
          btn.classList.add("mui-calendar__day--today");
        }
      }
    }

    function renderMonth() {
      title.textContent = MONTHS[currentMonth - 1] + " " + currentYear;
      root.setAttribute("data-year", currentYear);
      root.setAttribute("data-month", currentMonth);

      // Remove existing week rows
      var weekRows = grid.querySelectorAll(".mui-calendar__week");
      for (var i = 0; i < weekRows.length; i++) weekRows[i].remove();

      var firstDow = dayOfWeek(currentYear, currentMonth, 1);
      var dim = daysInMonth(currentYear, currentMonth);

      var prevYear = currentMonth === 1 ? currentYear - 1 : currentYear;
      var prevMonth = currentMonth === 1 ? 12 : currentMonth - 1;
      var prevDim = daysInMonth(prevYear, prevMonth);
      var nextYear = currentMonth === 12 ? currentYear + 1 : currentYear;
      var nextMonth = currentMonth === 12 ? 1 : currentMonth + 1;

      var cells = [];
      // Previous month trailing
      for (var i = 0; i < firstDow; i++) {
        var d = prevDim - firstDow + 1 + i;
        cells.push({y: prevYear, m: prevMonth, d: d, outside: true});
      }
      // Current month
      for (var d = 1; d <= dim; d++) {
        cells.push({y: currentYear, m: currentMonth, d: d, outside: false});
      }
      // Next month leading
      var remaining = 42 - cells.length;
      for (var d = 1; d <= remaining; d++) {
        cells.push({y: nextYear, m: nextMonth, d: d, outside: true});
      }

      // Determine which cell gets tabindex=0
      var focusIdx = -1;
      for (var i = 0; i < cells.length; i++) {
        if (fmtDate(cells[i].y, cells[i].m, cells[i].d) === selected) { focusIdx = i; break; }
      }
      if (focusIdx === -1) {
        // First day of current month
        for (var i = 0; i < cells.length; i++) {
          if (!cells[i].outside) { focusIdx = i; break; }
        }
      }

      // Build 6 week rows
      for (var w = 0; w < 6; w++) {
        var row = document.createElement("div");
        row.className = "mui-calendar__week";
        row.setAttribute("role", "row");

        for (var dow = 0; dow < 7; dow++) {
          var idx = w * 7 + dow;
          var c = cells[idx];
          var dateStr = fmtDate(c.y, c.m, c.d);
          var btn = document.createElement("button");
          btn.type = "button";
          btn.setAttribute("role", "gridcell");
          btn.setAttribute("data-date", dateStr);
          btn.setAttribute("tabindex", idx === focusIdx ? "0" : "-1");

          var cls = "mui-calendar__day";
          if (c.outside) cls += " mui-calendar__day--outside";
          if (dateStr === todayStr) cls += " mui-calendar__day--today";
          if (dateStr === selected) {
            cls += " mui-calendar__day--selected";
            btn.setAttribute("aria-selected", "true");
          } else {
            btn.setAttribute("aria-selected", "false");
          }

          var dis = isDisabled(c.y, c.m, c.d, minDate, maxDate);
          if (dis) btn.disabled = true;

          btn.className = cls;

          if (!c.outside || showOutside) {
            btn.textContent = String(c.d);
          }

          row.appendChild(btn);
        }
        grid.appendChild(row);
      }
    }

    // Navigation
    prevBtn.addEventListener("click", function () {
      currentMonth--;
      if (currentMonth < 1) { currentMonth = 12; currentYear--; }
      renderMonth();
    });

    nextBtn.addEventListener("click", function () {
      currentMonth++;
      if (currentMonth > 12) { currentMonth = 1; currentYear++; }
      renderMonth();
    });

    // Day selection — event delegation
    grid.addEventListener("click", function (e) {
      var day = e.target.closest(".mui-calendar__day");
      if (!day || day.disabled) return;

      // Deselect previous
      var prev = grid.querySelector(".mui-calendar__day--selected");
      if (prev) {
        prev.classList.remove("mui-calendar__day--selected");
        prev.setAttribute("aria-selected", "false");
        prev.setAttribute("tabindex", "-1");
      }

      // Select new
      day.classList.add("mui-calendar__day--selected");
      day.setAttribute("aria-selected", "true");
      day.setAttribute("tabindex", "0");
      day.focus();

      selected = day.getAttribute("data-date");
      root.setAttribute("data-selected", selected);
      if (hidden) hidden.value = selected;

      // Dispatch change event for external listeners
      root.dispatchEvent(new CustomEvent("calendar:change", { detail: { date: selected }, bubbles: true }));
    });

    // Keyboard navigation
    grid.addEventListener("keydown", function (e) {
      var day = e.target.closest(".mui-calendar__day");
      if (!day) return;

      var allDays = grid.querySelectorAll(".mui-calendar__day:not(:disabled)");
      var currentIdx = -1;
      for (var i = 0; i < allDays.length; i++) {
        if (allDays[i] === day) { currentIdx = i; break; }
      }
      if (currentIdx === -1) return;

      var targetIdx = -1;

      if (e.key === "ArrowRight") {
        e.preventDefault();
        targetIdx = currentIdx + 1 < allDays.length ? currentIdx + 1 : currentIdx;
      } else if (e.key === "ArrowLeft") {
        e.preventDefault();
        targetIdx = currentIdx - 1 >= 0 ? currentIdx - 1 : currentIdx;
      } else if (e.key === "ArrowDown") {
        e.preventDefault();
        targetIdx = currentIdx + 7 < allDays.length ? currentIdx + 7 : currentIdx;
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        targetIdx = currentIdx - 7 >= 0 ? currentIdx - 7 : currentIdx;
      } else if (e.key === "Home") {
        e.preventDefault();
        // First day of current month
        for (var i = 0; i < allDays.length; i++) {
          if (!allDays[i].classList.contains("mui-calendar__day--outside")) { targetIdx = i; break; }
        }
      } else if (e.key === "End") {
        e.preventDefault();
        // Last day of current month
        for (var i = allDays.length - 1; i >= 0; i--) {
          if (!allDays[i].classList.contains("mui-calendar__day--outside")) { targetIdx = i; break; }
        }
      } else if (e.key === "PageUp") {
        e.preventDefault();
        currentMonth--;
        if (currentMonth < 1) { currentMonth = 12; currentYear--; }
        renderMonth();
        // Focus first non-disabled day
        var first = grid.querySelector(".mui-calendar__day:not(:disabled):not(.mui-calendar__day--outside)");
        if (first) { first.setAttribute("tabindex", "0"); first.focus(); }
        return;
      } else if (e.key === "PageDown") {
        e.preventDefault();
        currentMonth++;
        if (currentMonth > 12) { currentMonth = 1; currentYear++; }
        renderMonth();
        var first = grid.querySelector(".mui-calendar__day:not(:disabled):not(.mui-calendar__day--outside)");
        if (first) { first.setAttribute("tabindex", "0"); first.focus(); }
        return;
      } else if (e.key === "Enter" || e.key === " ") {
        e.preventDefault();
        day.click();
        return;
      }

      if (targetIdx >= 0 && targetIdx < allDays.length) {
        day.setAttribute("tabindex", "-1");
        allDays[targetIdx].setAttribute("tabindex", "0");
        allDays[targetIdx].focus();
      }
    });
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();


// ============ carousel.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["carousel"] = function (root) {
    var container = root.querySelector(".mui-carousel__container");
    if (!container) return;

    var slides = container.querySelectorAll(".mui-carousel__slide");
    var prev = root.querySelector(".mui-carousel__prev");
    var next = root.querySelector(".mui-carousel__next");
    var dots = root.querySelectorAll(".mui-carousel__dot");
    var currentIndex = 0;
    var total = slides.length;
    var loopEnabled = root.getAttribute("data-loop") === "true";
    var autoPlay = root.getAttribute("data-autoplay") === "true";
    var autoPlayTimer = null;

    function updateDisabled() {
      if (loopEnabled) return;
      if (prev) prev.disabled = currentIndex === 0;
      if (next) next.disabled = currentIndex === total - 1;
    }

    function goTo(index) {
      if (index < 0) {
        index = loopEnabled ? total - 1 : 0;
      }
      if (index >= total) {
        index = loopEnabled ? 0 : total - 1;
      }
      currentIndex = index;
      container.style.transform = "translateX(-" + (currentIndex * 100) + "%)";

      // Update dots
      for (var i = 0; i < dots.length; i++) {
        var isActive = i === currentIndex;
        dots[i].setAttribute("aria-selected", isActive ? "true" : "false");
        if (isActive) {
          dots[i].classList.add("mui-carousel__dot--active");
        } else {
          dots[i].classList.remove("mui-carousel__dot--active");
        }
      }

      // Mark inactive slides as inert — removes them from both the focus
      // order and the a11y tree without the aria-hidden-on-focusable
      // trap (aria-hidden="true" with focusable descendants is a WCAG
      // violation; inert is the modern, browser-supported equivalent).
      for (var j = 0; j < slides.length; j++) {
        slides[j].inert = j !== currentIndex;
      }

      updateDisabled();
    }

    // Arrow clicks
    if (prev) {
      prev.addEventListener("click", function () {
        goTo(currentIndex - 1);
        resetAutoPlay();
      });
    }
    if (next) {
      next.addEventListener("click", function () {
        goTo(currentIndex + 1);
        resetAutoPlay();
      });
    }

    // Dot clicks
    for (var i = 0; i < dots.length; i++) {
      (function (idx) {
        dots[idx].addEventListener("click", function () {
          goTo(idx);
          resetAutoPlay();
        });
      })(i);
    }

    // Keyboard: ArrowLeft/Right on the carousel root
    root.addEventListener("keydown", function (e) {
      if (e.key === "ArrowLeft") {
        e.preventDefault();
        goTo(currentIndex - 1);
        resetAutoPlay();
      } else if (e.key === "ArrowRight") {
        e.preventDefault();
        goTo(currentIndex + 1);
        resetAutoPlay();
      }
    });

    // Auto-play
    function startAutoPlay() {
      if (!autoPlay) return;
      autoPlayTimer = setInterval(function () {
        goTo(currentIndex + 1);
      }, 4000);
    }

    function resetAutoPlay() {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
      startAutoPlay();
    }

    // Pause auto-play on hover/focus
    root.addEventListener("mouseenter", function () {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
    });
    root.addEventListener("mouseleave", function () {
      startAutoPlay();
    });
    root.addEventListener("focusin", function () {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
    });
    root.addEventListener("focusout", function () {
      startAutoPlay();
    });

    // Initialize
    goTo(0);
    startAutoPlay();
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();


// ============ collapsible.js ============
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


// ============ combobox.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

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


// ============ command.js ============
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


// ============ data_table.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["data-table"] = function (root) {
    var pageSize = parseInt(root.getAttribute("data-page-size") || "5", 10);
    var tbody = root.querySelector(".mui-data-table__body");
    var info = root.querySelector(".mui-data-table__info");
    var prevBtn = root.querySelector('[data-action="prev"]');
    var nextBtn = root.querySelector('[data-action="next"]');
    var searchInput = root.querySelector(".mui-data-table__search");
    var headers = root.querySelectorAll(".mui-data-table__th");

    if (!tbody) return;

    // Read all rows from the DOM at init
    var allRowEls = tbody.querySelectorAll("tr[data-row-data]");
    var allRows = [];
    for (var i = 0; i < allRowEls.length; i++) {
      var raw = allRowEls[i].getAttribute("data-row-data");
      try {
        allRows.push(JSON.parse(raw));
      } catch (e) {
        allRows.push([]);
      }
    }

    var filteredRows = allRows.slice();
    var currentPage = 0;
    var sortKey = -1;
    var sortDir = ""; // "", "asc", "desc"

    function renderPage() {
      var total = filteredRows.length;
      var totalPages = Math.max(1, Math.ceil(total / pageSize));
      if (currentPage >= totalPages) currentPage = totalPages - 1;
      if (currentPage < 0) currentPage = 0;

      var start = currentPage * pageSize;
      var end = Math.min(start + pageSize, total);
      var pageRows = filteredRows.slice(start, end);

      // Build tbody HTML
      var html = "";
      for (var i = 0; i < pageRows.length; i++) {
        var row = pageRows[i];
        var rowJson = JSON.stringify(row).replace(/&/g, "&amp;").replace(/"/g, "&quot;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
        html += '<tr class="mui-table__row" data-row-data="' + rowJson + '">';
        for (var j = 0; j < row.length; j++) {
          html += '<td class="mui-table__td">' + escapeHtml(row[j]) + "</td>";
        }
        html += "</tr>";
      }
      tbody.innerHTML = html;

      // Update info
      if (info) {
        if (total === 0) {
          info.textContent = "No results";
        } else {
          info.textContent = "Showing " + (start + 1) + "-" + end + " of " + total;
        }
      }

      // Update button states
      if (prevBtn) prevBtn.disabled = currentPage === 0;
      if (nextBtn) nextBtn.disabled = end >= total;
    }

    function escapeHtml(str) {
      var div = document.createElement("div");
      div.appendChild(document.createTextNode(str));
      return div.innerHTML;
    }

    function applyFilter(query) {
      var q = query.toLowerCase().trim();
      if (q === "") {
        filteredRows = sortRows(allRows.slice());
      } else {
        var matched = [];
        for (var i = 0; i < allRows.length; i++) {
          var row = allRows[i];
          var found = false;
          for (var j = 0; j < row.length; j++) {
            if (row[j].toLowerCase().indexOf(q) !== -1) {
              found = true;
              break;
            }
          }
          if (found) matched.push(row);
        }
        filteredRows = sortRows(matched);
      }
      currentPage = 0;
      renderPage();
    }

    function sortRows(rows) {
      if (sortKey < 0 || sortDir === "") return rows;
      var col = sortKey;
      var dir = sortDir === "asc" ? 1 : -1;
      rows.sort(function (a, b) {
        var va = a[col] || "";
        var vb = b[col] || "";
        // Try numeric comparison for values like $250.00
        var na = parseFloat(va.replace(/[^0-9.\-]/g, ""));
        var nb = parseFloat(vb.replace(/[^0-9.\-]/g, ""));
        if (!isNaN(na) && !isNaN(nb)) {
          return (na - nb) * dir;
        }
        return va.localeCompare(vb) * dir;
      });
      return rows;
    }

    function handleSort(colIndex) {
      if (sortKey === colIndex) {
        // Cycle: asc -> desc -> none
        if (sortDir === "asc") {
          sortDir = "desc";
        } else if (sortDir === "desc") {
          sortDir = "";
          sortKey = -1;
        }
      } else {
        sortKey = colIndex;
        sortDir = "asc";
      }

      // Update header indicators
      for (var i = 0; i < headers.length; i++) {
        headers[i].removeAttribute("data-sort-dir");
        var icon = headers[i].querySelector(".mui-data-table__sort-icon");
        if (icon) icon.innerHTML = "&#8693;";
      }
      if (sortDir !== "" && colIndex >= 0 && colIndex < headers.length) {
        headers[colIndex].setAttribute("data-sort-dir", sortDir);
        var activeIcon = headers[colIndex].querySelector(".mui-data-table__sort-icon");
        if (activeIcon) {
          activeIcon.innerHTML = sortDir === "asc" ? "&#9650;" : "&#9660;";
        }
      }

      // Re-apply filter with new sort
      var query = searchInput ? searchInput.value : "";
      applyFilter(query);
    }

    // Bind header clicks
    for (var h = 0; h < headers.length; h++) {
      if (headers[h].getAttribute("data-sortable") === "true") {
        (function (idx) {
          headers[idx].addEventListener("click", function () {
            handleSort(idx);
          });
        })(h);
      }
    }

    // Bind search
    if (searchInput) {
      searchInput.addEventListener("input", function () {
        applyFilter(searchInput.value);
      });
    }

    // Bind pagination
    if (prevBtn) {
      prevBtn.addEventListener("click", function () {
        if (currentPage > 0) {
          currentPage--;
          renderPage();
        }
      });
    }
    if (nextBtn) {
      nextBtn.addEventListener("click", function () {
        var total = filteredRows.length;
        var totalPages = Math.ceil(total / pageSize);
        if (currentPage < totalPages - 1) {
          currentPage++;
          renderPage();
        }
      });
    }

    // Initial render (JS takes over from server-rendered rows)
    renderPage();
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();


// ============ date_picker.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  var MONTH_NAMES = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
  ];

  var DAY_HEADERS = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];

  function daysInMonth(year, month) {
    if (month === 2) {
      return ((year % 4 === 0 && year % 100 !== 0) || year % 400 === 0) ? 29 : 28;
    }
    var thirtyDays = [4, 6, 9, 11];
    for (var i = 0; i < thirtyDays.length; i++) {
      if (month === thirtyDays[i]) return 30;
    }
    return 31;
  }

  function dayOfWeek(year, month, day) {
    var t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    var y = month < 3 ? year - 1 : year;
    return (y + Math.floor(y / 4) - Math.floor(y / 100) + Math.floor(y / 400) + t[month - 1] + day) % 7;
  }

  function formatDisplay(year, month, day) {
    return MONTH_NAMES[month - 1] + " " + day + ", " + year;
  }

  function formatISO(year, month, day) {
    var m = month < 10 ? "0" + month : "" + month;
    var d = day < 10 ? "0" + day : "" + day;
    return year + "-" + m + "-" + d;
  }

  function renderCalendarHTML(year, month, selectedYear, selectedMonth, selectedDay) {
    var html = '<div class="mui-date-picker__calendar" data-year="' + year + '" data-month="' + month + '">';

    // Header
    html += '<div class="mui-date-picker__cal-header">';
    html += '<button type="button" class="mui-date-picker__nav-btn" data-action="prev-month" aria-label="Previous month">\u2039</button>';
    html += '<span class="mui-date-picker__cal-title">' + MONTH_NAMES[month - 1] + ' ' + year + '</span>';
    html += '<button type="button" class="mui-date-picker__nav-btn" data-action="next-month" aria-label="Next month">\u203a</button>';
    html += '</div>';

    // Day-of-week headers
    html += '<div class="mui-date-picker__day-headers">';
    for (var h = 0; h < DAY_HEADERS.length; h++) {
      html += '<span class="mui-date-picker__day-header">' + DAY_HEADERS[h] + '</span>';
    }
    html += '</div>';

    // Day grid
    html += '<div class="mui-date-picker__days">';
    var firstDow = dayOfWeek(year, month, 1);
    var totalDays = daysInMonth(year, month);

    for (var e = 0; e < firstDow; e++) {
      html += '<span class="mui-date-picker__day mui-date-picker__day--empty"></span>';
    }

    for (var d = 1; d <= totalDays; d++) {
      var isSelected = (year === selectedYear && month === selectedMonth && d === selectedDay);
      var cls = "mui-date-picker__day";
      if (isSelected) cls += " mui-date-picker__day--selected";
      html += '<button type="button" class="' + cls + '" data-day="' + d + '" data-month="' + month + '" data-year="' + year + '">' + d + '</button>';
    }

    html += '</div></div>';
    return html;
  }

  window.MaudUI.behaviors["date-picker"] = function (root) {
    var trigger = root.querySelector(".mui-date-picker__trigger");
    var dropdown = root.querySelector(".mui-date-picker__dropdown");
    var valueEl = root.querySelector(".mui-date-picker__value");
    var hiddenInput = root.querySelector(".mui-date-picker__hidden");

    // Track current state
    var selectedYear = 0;
    var selectedMonth = 0;
    var selectedDay = 0;
    var viewYear = 0;
    var viewMonth = 0;

    // Parse initial selection from hidden input
    var initialValue = hiddenInput ? hiddenInput.value : "";
    if (initialValue) {
      var parts = initialValue.split("-");
      if (parts.length === 3) {
        selectedYear = parseInt(parts[0], 10);
        selectedMonth = parseInt(parts[1], 10);
        selectedDay = parseInt(parts[2], 10);
        viewYear = selectedYear;
        viewMonth = selectedMonth;
      }
    }

    // Default view to April 2026 if no selection
    if (!viewYear) {
      viewYear = 2026;
      viewMonth = 4;
    }

    function open() {
      dropdown.removeAttribute("hidden");
      trigger.setAttribute("aria-expanded", "true");
      rebuildCalendar();
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("keydown", escClose, true);
    }

    function close() {
      dropdown.setAttribute("hidden", "");
      trigger.setAttribute("aria-expanded", "false");
      document.removeEventListener("click", clickOutside, true);
      document.removeEventListener("keydown", escClose, true);
    }

    function clickOutside(e) {
      if (!root.contains(e.target)) close();
    }

    function escClose(e) {
      if (e.key === "Escape") close();
    }

    function rebuildCalendar() {
      dropdown.innerHTML = renderCalendarHTML(viewYear, viewMonth, selectedYear, selectedMonth, selectedDay);
      wireCalendarEvents();
    }

    function wireCalendarEvents() {
      // Month navigation
      var prevBtn = dropdown.querySelector('[data-action="prev-month"]');
      var nextBtn = dropdown.querySelector('[data-action="next-month"]');

      if (prevBtn) {
        prevBtn.addEventListener("click", function (e) {
          e.stopPropagation();
          viewMonth--;
          if (viewMonth < 1) {
            viewMonth = 12;
            viewYear--;
          }
          rebuildCalendar();
        });
      }

      if (nextBtn) {
        nextBtn.addEventListener("click", function (e) {
          e.stopPropagation();
          viewMonth++;
          if (viewMonth > 12) {
            viewMonth = 1;
            viewYear++;
          }
          rebuildCalendar();
        });
      }

      // Day selection
      var days = dropdown.querySelectorAll(".mui-date-picker__day:not(.mui-date-picker__day--empty):not(.mui-date-picker__day--disabled)");
      for (var i = 0; i < days.length; i++) {
        days[i].addEventListener("click", function (e) {
          e.stopPropagation();
          var btn = e.currentTarget;
          selectedDay = parseInt(btn.getAttribute("data-day"), 10);
          selectedMonth = parseInt(btn.getAttribute("data-month"), 10);
          selectedYear = parseInt(btn.getAttribute("data-year"), 10);

          // Update display
          valueEl.textContent = formatDisplay(selectedYear, selectedMonth, selectedDay);
          valueEl.classList.remove("mui-date-picker__value--placeholder");
          hiddenInput.value = formatISO(selectedYear, selectedMonth, selectedDay);

          close();

          // Dispatch change event
          root.dispatchEvent(new CustomEvent("mui-date-change", {
            detail: { year: selectedYear, month: selectedMonth, day: selectedDay, iso: hiddenInput.value },
            bubbles: true
          }));
        });
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
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();


// ============ dialog.js ============
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


// ============ dir.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["dir-toggle"] = function (el) {
    el.addEventListener("click", function () {
      var html = document.documentElement;
      var current = html.getAttribute("dir") || "ltr";
      var next = current === "ltr" ? "rtl" : "ltr";
      html.setAttribute("dir", next);
      el.textContent = next === "ltr" ? "RTL" : "LTR";
      try { localStorage.setItem("mui-dir", next); } catch (e) {}
    });

    // Restore saved direction
    try {
      var saved = localStorage.getItem("mui-dir");
      if (saved && saved !== (document.documentElement.getAttribute("dir") || "ltr")) {
        document.documentElement.setAttribute("dir", saved);
        el.textContent = saved === "ltr" ? "RTL" : "LTR";
      }
    } catch (e) {}
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();


// ============ drawer.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  // Drawer Trigger Behavior
  window.MaudUI.behaviors["drawer-trigger"] = function (el) {
    el.addEventListener("click", function () {
      var target_id = el.getAttribute("data-target");
      var drawer = target_id ? document.getElementById(target_id) : null;
      if (drawer && drawer.showModal) {
        drawer.showModal();
      }
    });
  };

  // Drawer Behavior
  window.MaudUI.behaviors["drawer"] = function (el) {
    // Wire close buttons inside the drawer
    var closeBtns = el.querySelectorAll("[data-mui-close]");
    for (var i = 0; i < closeBtns.length; i++) {
      closeBtns[i].addEventListener("click", function () {
        el.close();
      });
    }

    // Click on backdrop closes drawer (when clicking on dialog element outside content)
    el.addEventListener("click", function (e) {
      if (e.target === el) {
        el.close();
      }
    });

    // Native <dialog> with showModal() handles:
    // - ESC closes the drawer (unless prevented by cancel event)
    // - Focus trapping inside the drawer
  };

  if (window.MaudUI.init) window.MaudUI.init();
})();


// ============ hover_card.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["hover-card"] = function (root) {
    // root is .mui-hover-card with trigger and content inside
    var trigger = root.querySelector(".mui-hover-card__trigger");
    var content = root.querySelector(".mui-hover-card__content");
    if (!trigger || !content) return;

    var openDelay = parseInt(root.getAttribute("data-open-delay") || "300", 10);
    var closeDelay = parseInt(root.getAttribute("data-close-delay") || "200", 10);
    var openTimer = null;
    var closeTimer = null;
    var hideTimer = null;

    function show() {
      clearTimeout(closeTimer);
      clearTimeout(hideTimer);
      openTimer = setTimeout(function () {
        // Remove hidden first so CSS transition can play
        content.removeAttribute("hidden");
        // Force reflow before adding visible state
        void content.offsetHeight;
        content.setAttribute("data-visible", "true");
      }, openDelay);
    }

    function hide() {
      clearTimeout(openTimer);
      closeTimer = setTimeout(function () {
        content.setAttribute("data-visible", "false");
        // Delay hidden to let CSS transition complete
        hideTimer = setTimeout(function () {
          content.setAttribute("hidden", "");
        }, 150); // Match --mui-transition duration
      }, closeDelay);
    }

    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    // Content stays open while hovering the card itself
    content.addEventListener("mouseenter", function () {
      clearTimeout(closeTimer);
      clearTimeout(hideTimer);
    });
    content.addEventListener("mouseleave", hide);
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();


// ============ input_otp.js ============
window.MaudUI.behaviors["input-otp"] = function(root) {
  var slots = root.querySelectorAll(".mui-input-otp__slot");
  var hidden = root.querySelector(".mui-input-otp__value");

  for (var i = 0; i < slots.length; i++) {
    (function(idx) {
      slots[idx].addEventListener("input", function() {
        if (slots[idx].value.length === 1 && idx < slots.length - 1) {
          slots[idx + 1].focus();
        }
        updateHidden();
      });

      slots[idx].addEventListener("keydown", function(e) {
        if (e.key === "Backspace" && !slots[idx].value && idx > 0) {
          slots[idx - 1].focus();
        }
      });
    })(i);
  }

  function updateHidden() {
    var val = "";
    for (var j = 0; j < slots.length; j++) {
      val += slots[j].value;
    }
    if (hidden) {
      hidden.value = val;
    }
  }
};


// ============ menu.js ============
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


// ============ menubar.js ============
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


// ============ navigation_menu.js ============
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


// ============ number_field.js ============
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


// ============ popover.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["popover"] = function (root) {
    var trigger = root.querySelector(".mui-popover__trigger");
    var content = root.querySelector(".mui-popover__content");
    if (!trigger || !content) return;

    // Find the actual interactive element (consumer's button/link). If none,
    // fall back to the trigger span — but avoid setting aria-expanded on a
    // bare span (invalid on a generic role).
    var interactive = trigger.querySelector('button, a, [role="button"]') || trigger;
    interactive.setAttribute("aria-haspopup", "dialog");
    interactive.setAttribute("aria-expanded", "false");
    interactive.setAttribute("aria-controls", content.id || "");

    function toggle() {
      var expanded = interactive.getAttribute("aria-expanded") === "true";
      if (expanded) close();
      else open();
    }

    function open() {
      interactive.setAttribute("aria-expanded", "true");
      // Remove hidden first so CSS transition can play
      content.removeAttribute("hidden");
      // Force reflow before adding visible state
      void content.offsetHeight;
      content.setAttribute("data-visible", "true");
      content.focus();
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("keydown", escClose, true);
    }

    function close() {
      interactive.setAttribute("aria-expanded", "false");
      content.setAttribute("data-visible", "false");
      // Delay hidden to let CSS transition complete
      setTimeout(function () {
        content.setAttribute("hidden", "");
      }, 150); // Match --mui-transition duration
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


// ============ resizable.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["resizable"] = function (root) {
    var handles = root.querySelectorAll(".mui-resizable__handle");
    var panels = root.querySelectorAll(".mui-resizable__panel");
    var direction = root.getAttribute("data-direction") || "horizontal";
    var isHorizontal = direction === "horizontal";

    if (handles.length === 0 || panels.length < 2) return;

    function getTotalSize() {
      var rect = root.getBoundingClientRect();
      return isHorizontal ? rect.width : rect.height;
    }

    function getPanelSizes() {
      var sizes = [];
      for (var i = 0; i < panels.length; i++) {
        var rect = panels[i].getBoundingClientRect();
        sizes.push(isHorizontal ? rect.width : rect.height);
      }
      return sizes;
    }

    function getMinSize(panel) {
      var min = parseFloat(panel.getAttribute("data-min-size") || "10");
      // Convert percentage to pixels
      return (min / 100) * getTotalSize();
    }

    function applyFlexValues(sizes) {
      var total = 0;
      for (var i = 0; i < sizes.length; i++) {
        total += sizes[i];
      }
      for (var i = 0; i < panels.length; i++) {
        var pct = (sizes[i] / total) * 100;
        panels[i].style.flex = pct + " 1 0%";
      }
      // Update aria-valuenow on handles
      for (var h = 0; h < handles.length; h++) {
        var leftSize = sizes[h];
        var leftPct = ((leftSize / total) * 100).toFixed(1);
        handles[h].setAttribute("aria-valuenow", leftPct);
      }
    }

    // Mouse drag
    for (var h = 0; h < handles.length; h++) {
      (function (handleIndex) {
        var handle = handles[handleIndex];
        var panelA = panels[handleIndex];
        var panelB = panels[handleIndex + 1];

        handle.addEventListener("mousedown", function (e) {
          e.preventDefault();
          root.setAttribute("data-dragging", "");

          var sizes = getPanelSizes();
          var startPos = isHorizontal ? e.clientX : e.clientY;
          var startA = sizes[handleIndex];
          var startB = sizes[handleIndex + 1];
          var minA = getMinSize(panelA);
          var minB = getMinSize(panelB);

          function onMove(e) {
            var currentPos = isHorizontal ? e.clientX : e.clientY;
            var delta = currentPos - startPos;

            var newA = startA + delta;
            var newB = startB - delta;

            // Enforce min sizes
            if (newA < minA) {
              newA = minA;
              newB = startA + startB - minA;
            }
            if (newB < minB) {
              newB = minB;
              newA = startA + startB - minB;
            }

            sizes[handleIndex] = newA;
            sizes[handleIndex + 1] = newB;
            applyFlexValues(sizes);
          }

          function onUp() {
            root.removeAttribute("data-dragging");
            document.removeEventListener("mousemove", onMove);
            document.removeEventListener("mouseup", onUp);
          }

          document.addEventListener("mousemove", onMove);
          document.addEventListener("mouseup", onUp, { once: true });
        });

        // Keyboard: arrow keys to resize
        handle.addEventListener("keydown", function (e) {
          var step = getTotalSize() * 0.02; // 2% per keypress
          var sizes = getPanelSizes();
          var minA = getMinSize(panelA);
          var minB = getMinSize(panelB);
          var delta = 0;

          var growKey = isHorizontal ? "ArrowRight" : "ArrowDown";
          var shrinkKey = isHorizontal ? "ArrowLeft" : "ArrowUp";

          if (e.key === growKey) {
            e.preventDefault();
            delta = step;
          } else if (e.key === shrinkKey) {
            e.preventDefault();
            delta = -step;
          } else if (e.key === "Home") {
            e.preventDefault();
            delta = -(sizes[handleIndex] - minA);
          } else if (e.key === "End") {
            e.preventDefault();
            delta = sizes[handleIndex + 1] - minB;
          } else {
            return;
          }

          var newA = sizes[handleIndex] + delta;
          var newB = sizes[handleIndex + 1] - delta;

          if (newA < minA) {
            newA = minA;
            newB = sizes[handleIndex] + sizes[handleIndex + 1] - minA;
          }
          if (newB < minB) {
            newB = minB;
            newA = sizes[handleIndex] + sizes[handleIndex + 1] - minB;
          }

          sizes[handleIndex] = newA;
          sizes[handleIndex + 1] = newB;
          applyFlexValues(sizes);
        });

        // Double-click to reset to equal sizes
        handle.addEventListener("dblclick", function () {
          var equalSize = 100 / panels.length;
          for (var i = 0; i < panels.length; i++) {
            panels[i].style.flex = equalSize + " 1 0%";
          }
        });
      })(h);
    }
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();


// ============ scroll_area.js ============
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


// ============ select.js ============
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


// ============ slider.js ============
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
      root.setAttribute("data-dragging", "true");
      document.addEventListener("mousemove", onDrag);
      document.addEventListener("mouseup", function () {
        dragging = false;
        root.removeAttribute("data-dragging");
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


// ============ switch.js ============
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


// ============ tabs.js ============
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


// ============ theme.js ============
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


// ============ toast.js ============
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


// ============ toggle.js ============
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


// ============ tooltip.js ============
(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["tooltip"] = function (wrapper) {
    // wrapper is .mui-tooltip with a trigger inside and a .mui-tooltip__content sibling
    var trigger = wrapper.querySelector(".mui-tooltip__trigger");
    var content = wrapper.querySelector(".mui-tooltip__content");
    if (!trigger || !content) return;

    var delay = parseInt(wrapper.getAttribute("data-delay") || "500", 10);
    var showTimer = null;
    var hideTimer = null;

    function show() {
      clearTimeout(hideTimer);
      clearTimeout(showTimer);
      showTimer = setTimeout(function () {
        // Remove hidden first so CSS transition can play
        content.removeAttribute("hidden");
        // Force reflow before adding visible state
        void content.offsetHeight;
        content.setAttribute("data-visible", "true");
      }, delay);
    }

    function hide() {
      clearTimeout(showTimer);
      clearTimeout(hideTimer);
      content.setAttribute("data-visible", "false");
      hideTimer = setTimeout(function () {
        content.setAttribute("hidden", "");
      }, 150); // Match --mui-transition duration
    }

    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    trigger.addEventListener("focus", show);
    trigger.addEventListener("blur", hide);
    // Escape dismisses
    trigger.addEventListener("keydown", function (e) {
      if (e.key === "Escape") hide();
    });
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
