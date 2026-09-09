(function() {
  "use strict";
  var MaudUI = {
    behaviors: {},
    // Attach a behavior to one element, once.
    attach: function(el) {
      if (el.hasAttribute("data-mui-init")) return;
      var type = el.getAttribute("data-mui");
      if (type && MaudUI.behaviors[type]) {
        MaudUI.behaviors[type](el);
        el.setAttribute("data-mui-init", "");
      }
    },
    // Initialise `root` AND its descendants.
    //
    // Matching `root` itself is load-bearing: htmx sets the swap event's target
    // to the swapped node, and with `hx-swap="outerHTML"` (or an OOB swap of a
    // component's own element) that node IS the `[data-mui]` element rather than
    // its parent. `querySelectorAll` alone looks only at descendants, so those
    // swaps produced markup with no behavior attached — no error, no console
    // warning, just a control that does nothing when clicked.
    init: function(root) {
      if (!root) root = document.body;
      if (root.nodeType === 1 && root.hasAttribute("data-mui")) {
        MaudUI.attach(root);
      }
      var els = root.querySelectorAll("[data-mui]");
      for (var i = 0; i < els.length; i++) {
        MaudUI.attach(els[i]);
      }
    }
  };
  window.MaudUI = MaudUI;
  document.addEventListener("DOMContentLoaded", function() {
    MaudUI.init();
  });
  var SWAP_EVENTS = ["htmx:afterSwap", "htmx:oobAfterSwap"];
  for (var e = 0; e < SWAP_EVENTS.length; e++) {
    document.addEventListener(SWAP_EVENTS[e], function(evt) {
      if (evt.target instanceof Element) MaudUI.init(evt.target);
    });
  }
  document.addEventListener("htmx:historyRestore", function() {
    var stale = document.querySelectorAll("[data-mui][data-mui-init]");
    for (var i = 0; i < stale.length; i++) {
      stale[i].removeAttribute("data-mui-init");
    }
    MaudUI.init(document.body);
  });
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["accordion"] = function(root) {
    var isMultiple = root.getAttribute("data-multiple") === "true";
    var triggers = root.querySelectorAll(".mui-accordion__trigger");
    if (triggers.length === 0) return;
    for (var i = 0; i < triggers.length; i++) {
      var trigger = triggers[i];
      trigger.addEventListener("click", function(event) {
        var currentTrigger = event.currentTarget;
        handleTriggerToggle(currentTrigger, isMultiple);
      });
      trigger.addEventListener("keydown", function(event) {
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
    function handleTriggerToggle(trigger2, isMultiple2) {
      var isExpanded = trigger2.getAttribute("aria-expanded") === "true";
      var contentId = trigger2.getAttribute("aria-controls");
      var content = document.getElementById(contentId);
      if (!content) return;
      if (!isMultiple2 && !isExpanded) {
        for (var i2 = 0; i2 < triggers.length; i2++) {
          var otherTrigger = triggers[i2];
          if (otherTrigger !== trigger2 && otherTrigger.getAttribute("aria-expanded") === "true") {
            var otherContentId = otherTrigger.getAttribute("aria-controls");
            var otherContent = document.getElementById(otherContentId);
            if (otherContent) {
              otherTrigger.setAttribute("aria-expanded", "false");
              otherContent.setAttribute("hidden", "");
            }
          }
        }
      }
      var newState = isExpanded ? "false" : "true";
      trigger2.setAttribute("aria-expanded", newState);
      if (newState === "true") {
        content.removeAttribute("hidden");
      } else {
        content.setAttribute("hidden", "");
      }
    }
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["button-group"] = function(group) {
    var mode = (group.getAttribute("data-mode") || "exclusive").toLowerCase();
    var buttons = Array.prototype.slice.call(group.querySelectorAll("button"));
    if (buttons.length === 0) return;
    function valueOf(btn) {
      return btn.value || btn.getAttribute("data-value") || (btn.textContent || "").trim();
    }
    function emit(btn, pressed) {
      group.dispatchEvent(new CustomEvent("mui:button-group-change", {
        bubbles: true,
        detail: { value: valueOf(btn), pressed, target: btn }
      }));
    }
    group.addEventListener("click", function(e) {
      var btn = e.target && e.target.closest && e.target.closest("button");
      if (!btn || !group.contains(btn) || btn.disabled) return;
      if (mode === "multiple") {
        var pressed = btn.getAttribute("aria-pressed") !== "true";
        btn.setAttribute("aria-pressed", pressed ? "true" : "false");
        emit(btn, pressed);
      } else {
        if (btn.getAttribute("aria-pressed") === "true") {
          emit(btn, true);
          return;
        }
        for (var i = 0; i < buttons.length; i++) {
          var b = buttons[i];
          if (b === btn) b.setAttribute("aria-pressed", "true");
          else b.setAttribute("aria-pressed", "false");
        }
        emit(btn, true);
      }
    });
    if (mode !== "multiple") {
      group.addEventListener("keydown", function(e) {
        if (e.key !== "ArrowLeft" && e.key !== "ArrowRight") return;
        var current = document.activeElement;
        if (!buttons.indexOf || buttons.indexOf(current) === -1) return;
        e.preventDefault();
        var idx = buttons.indexOf(current);
        var dir = e.key === "ArrowRight" ? 1 : -1;
        var next = buttons[(idx + dir + buttons.length) % buttons.length];
        if (next.disabled) return;
        next.focus();
        next.click();
      });
    }
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  var MONTHS = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"
  ];
  function isLeapYear(y) {
    return y % 4 === 0 && y % 100 !== 0 || y % 400 === 0;
  }
  function daysInMonth(y, m) {
    if (m === 2) return isLeapYear(y) ? 29 : 28;
    if (m === 4 || m === 6 || m === 9 || m === 11) return 30;
    return 31;
  }
  function dayOfWeek(y, m, d) {
    var t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    if (m < 3) y--;
    return (y + Math.floor(y / 4) - Math.floor(y / 100) + Math.floor(y / 400) + t[m - 1] + d) % 7;
  }
  function fmtDate(y, m, d) {
    return String(y) + "-" + (m < 10 ? "0" : "") + m + "-" + (d < 10 ? "0" : "") + d;
  }
  function parseDate(s) {
    if (!s) return null;
    var p = s.split("-");
    if (p.length !== 3) return null;
    return { y: parseInt(p[0], 10), m: parseInt(p[1], 10), d: parseInt(p[2], 10) };
  }
  function dateCmp(a, b) {
    if (a.y !== b.y) return a.y < b.y ? -1 : 1;
    if (a.m !== b.m) return a.m < b.m ? -1 : 1;
    if (a.d !== b.d) return a.d < b.d ? -1 : 1;
    return 0;
  }
  function isDisabled(y, m, d, minD, maxD) {
    var cur = { y, m, d };
    if (minD && dateCmp(cur, minD) < 0) return true;
    if (maxD && dateCmp(cur, maxD) > 0) return true;
    return false;
  }
  window.MaudUI.behaviors["calendar"] = function(root) {
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
    var now = /* @__PURE__ */ new Date();
    var todayStr = fmtDate(now.getFullYear(), now.getMonth() + 1, now.getDate());
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
      for (var i = 0; i < firstDow; i++) {
        var d = prevDim - firstDow + 1 + i;
        cells.push({ y: prevYear, m: prevMonth, d, outside: true });
      }
      for (var d = 1; d <= dim; d++) {
        cells.push({ y: currentYear, m: currentMonth, d, outside: false });
      }
      var remaining = 42 - cells.length;
      for (var d = 1; d <= remaining; d++) {
        cells.push({ y: nextYear, m: nextMonth, d, outside: true });
      }
      var focusIdx = -1;
      for (var i = 0; i < cells.length; i++) {
        if (fmtDate(cells[i].y, cells[i].m, cells[i].d) === selected) {
          focusIdx = i;
          break;
        }
      }
      if (focusIdx === -1) {
        for (var i = 0; i < cells.length; i++) {
          if (!cells[i].outside) {
            focusIdx = i;
            break;
          }
        }
      }
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
    prevBtn.addEventListener("click", function() {
      currentMonth--;
      if (currentMonth < 1) {
        currentMonth = 12;
        currentYear--;
      }
      renderMonth();
    });
    nextBtn.addEventListener("click", function() {
      currentMonth++;
      if (currentMonth > 12) {
        currentMonth = 1;
        currentYear++;
      }
      renderMonth();
    });
    grid.addEventListener("click", function(e) {
      var day = e.target.closest(".mui-calendar__day");
      if (!day || day.disabled) return;
      var prev = grid.querySelector(".mui-calendar__day--selected");
      if (prev) {
        prev.classList.remove("mui-calendar__day--selected");
        prev.setAttribute("aria-selected", "false");
        prev.setAttribute("tabindex", "-1");
      }
      day.classList.add("mui-calendar__day--selected");
      day.setAttribute("aria-selected", "true");
      day.setAttribute("tabindex", "0");
      day.focus();
      selected = day.getAttribute("data-date");
      root.setAttribute("data-selected", selected);
      if (hidden) hidden.value = selected;
      root.dispatchEvent(new CustomEvent("calendar:change", { detail: { date: selected }, bubbles: true }));
    });
    grid.addEventListener("keydown", function(e) {
      var day = e.target.closest(".mui-calendar__day");
      if (!day) return;
      var allDays = grid.querySelectorAll(".mui-calendar__day:not(:disabled)");
      var currentIdx = -1;
      for (var i = 0; i < allDays.length; i++) {
        if (allDays[i] === day) {
          currentIdx = i;
          break;
        }
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
        for (var i = 0; i < allDays.length; i++) {
          if (!allDays[i].classList.contains("mui-calendar__day--outside")) {
            targetIdx = i;
            break;
          }
        }
      } else if (e.key === "End") {
        e.preventDefault();
        for (var i = allDays.length - 1; i >= 0; i--) {
          if (!allDays[i].classList.contains("mui-calendar__day--outside")) {
            targetIdx = i;
            break;
          }
        }
      } else if (e.key === "PageUp") {
        e.preventDefault();
        currentMonth--;
        if (currentMonth < 1) {
          currentMonth = 12;
          currentYear--;
        }
        renderMonth();
        var first = grid.querySelector(".mui-calendar__day:not(:disabled):not(.mui-calendar__day--outside)");
        if (first) {
          first.setAttribute("tabindex", "0");
          first.focus();
        }
        return;
      } else if (e.key === "PageDown") {
        e.preventDefault();
        currentMonth++;
        if (currentMonth > 12) {
          currentMonth = 1;
          currentYear++;
        }
        renderMonth();
        var first = grid.querySelector(".mui-calendar__day:not(:disabled):not(.mui-calendar__day--outside)");
        if (first) {
          first.setAttribute("tabindex", "0");
          first.focus();
        }
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
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["carousel"] = function(root) {
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
      container.style.transform = "translateX(-" + currentIndex * 100 + "%)";
      for (var i2 = 0; i2 < dots.length; i2++) {
        var isActive = i2 === currentIndex;
        dots[i2].setAttribute("aria-selected", isActive ? "true" : "false");
        if (isActive) {
          dots[i2].classList.add("mui-carousel__dot--active");
        } else {
          dots[i2].classList.remove("mui-carousel__dot--active");
        }
      }
      for (var j = 0; j < slides.length; j++) {
        slides[j].inert = j !== currentIndex;
      }
      updateDisabled();
    }
    if (prev) {
      prev.addEventListener("click", function() {
        goTo(currentIndex - 1);
        resetAutoPlay();
      });
    }
    if (next) {
      next.addEventListener("click", function() {
        goTo(currentIndex + 1);
        resetAutoPlay();
      });
    }
    for (var i = 0; i < dots.length; i++) {
      (function(idx) {
        dots[idx].addEventListener("click", function() {
          goTo(idx);
          resetAutoPlay();
        });
      })(i);
    }
    root.addEventListener("keydown", function(e) {
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
    function startAutoPlay() {
      if (!autoPlay) return;
      autoPlayTimer = setInterval(function() {
        goTo(currentIndex + 1);
      }, 4e3);
    }
    function resetAutoPlay() {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
      startAutoPlay();
    }
    root.addEventListener("mouseenter", function() {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
    });
    root.addEventListener("mouseleave", function() {
      startAutoPlay();
    });
    root.addEventListener("focusin", function() {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
    });
    root.addEventListener("focusout", function() {
      startAutoPlay();
    });
    goTo(0);
    startAutoPlay();
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["collapsible"] = function(root) {
    var trigger = root.querySelector(".mui-collapsible__trigger");
    var content = root.querySelector(".mui-collapsible__content");
    if (!trigger || !content) return;
    trigger.addEventListener("click", function() {
      toggleCollapsible();
    });
    trigger.addEventListener("keydown", function(event) {
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
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["combobox"] = function(root) {
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
    trigger.addEventListener("click", function() {
      if (trigger.getAttribute("aria-expanded") === "true") {
        close();
      } else {
        open();
      }
    });
    searchInput.addEventListener("input", function() {
      filterOptions(searchInput.value);
    });
    searchInput.addEventListener("keydown", function(e) {
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
    trigger.addEventListener("keydown", function(e) {
      var isOpen = trigger.getAttribute("aria-expanded") === "true";
      if (!isOpen && (e.key === "ArrowDown" || e.key === "ArrowUp" || e.key === "Enter" || e.key === " ")) {
        e.preventDefault();
        open();
      } else if (isOpen && e.key === "Escape") {
        e.preventDefault();
        close();
      }
    });
    list.addEventListener("click", function(e) {
      var option = e.target.closest(".mui-combobox__option");
      if (option) {
        selectOption(option);
      }
    });
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["command-trigger"] = function(el) {
    el.addEventListener("click", function() {
      var targetId = el.getAttribute("data-target");
      var dialog = targetId ? document.getElementById(targetId) : null;
      if (dialog && dialog.showModal) {
        dialog.showModal();
        var search = dialog.querySelector(".mui-command__search");
        if (search) {
          search.value = "";
          search.focus();
        }
        resetFilter(dialog);
      }
    });
  };
  window.MaudUI.behaviors["command"] = function(el) {
    var searchInput = el.querySelector(".mui-command__search");
    var list = el.querySelector(".mui-command__list");
    var emptyEl = el.querySelector(".mui-command__empty");
    var items = el.querySelectorAll(".mui-command__item");
    var groups = el.querySelectorAll(".mui-command__group");
    var activeIndex = -1;
    function visibleItems() {
      var result = [];
      for (var i = 0; i < items.length; i++) {
        if (items[i].getAttribute("data-hidden") !== "true" && !items[i].classList.contains("mui-command__item--disabled")) {
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
      el.dispatchEvent(new CustomEvent("mui-command-select", {
        detail: { label },
        bubbles: true
      }));
    }
    searchInput.addEventListener("input", function() {
      filterItems(searchInput.value);
    });
    searchInput.addEventListener("keydown", function(e) {
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
    });
    list.addEventListener("click", function(e) {
      var item = e.target.closest(".mui-command__item");
      if (item && !item.classList.contains("mui-command__item--disabled")) {
        selectItem(item);
      }
    });
    el.addEventListener("click", function(e) {
      if (e.target === el) {
        el.close();
      }
    });
    el.addEventListener("close", function() {
      searchInput.value = "";
      resetFilter(el);
      unhighlightAll();
      activeIndex = -1;
    });
  };
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
  document.addEventListener("keydown", function(e) {
    if ((e.metaKey || e.ctrlKey) && e.key === "k") {
      e.preventDefault();
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
(function() {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  ui.compareDecimal = (left, right) => {
    function parse(text) {
      const value = text.trim().replace("\u2212", "-");
      const match = /^([+-]?)(?:[$€£¥])?((?:\d{1,3}(?:,\d{3})+|\d+))(?:\.(\d+))?$/.exec(value);
      if (!match) return null;
      const decimals = match[3] || "";
      return { value: BigInt((match[1] === "-" ? "-" : "") + match[2].replaceAll(",", "") + decimals), scale: decimals.length };
    }
    const a = parse(left), b = parse(right);
    if (!a || !b) return null;
    const scale = Math.max(a.scale, b.scale);
    const l = a.value * 10n ** BigInt(scale - a.scale), r = b.value * 10n ** BigInt(scale - b.scale);
    return l < r ? -1 : l > r ? 1 : 0;
  };
  ui.behaviors["data-table"] = (root) => {
    const pageSize = Math.max(1, parseInt(root.getAttribute("data-page-size") || "5", 10) || 5);
    const body = root.querySelector(".mui-data-table__body");
    if (!body) return;
    const info = root.querySelector(".mui-data-table__info");
    const previous = root.querySelector('[data-action="prev"]');
    const next = root.querySelector('[data-action="next"]');
    const search = root.querySelector(".mui-data-table__search");
    const headers = Array.from(root.querySelectorAll(".mui-data-table__th[data-key]"));
    const rows = Array.from(body.querySelectorAll("tr")).map((node) => {
      let values;
      try {
        values = JSON.parse(node.getAttribute("data-row-data"));
      } catch {
      }
      if (!Array.isArray(values)) values = Array.from(node.children).filter((cell) => !cell.classList.contains("mui-data-table__td--select")).map((cell) => cell.textContent.trim());
      return { node, values: values.map(String) };
    });
    let page = 0, column = -1, direction = 0;
    function render() {
      const query = (search?.value || "").trim().toLocaleLowerCase();
      const filtered = rows.filter((row) => row.values.some((value) => value.toLocaleLowerCase().includes(query)) || !query);
      if (direction && column >= 0) filtered.sort((a, b) => {
        const left = a.values[column] || "", right = b.values[column] || "";
        return direction * (ui.compareDecimal(left, right) ?? left.localeCompare(right));
      });
      page = Math.max(0, Math.min(page, Math.ceil(filtered.length / pageSize) - 1));
      rows.forEach((row) => {
        row.node.hidden = true;
      });
      filtered.forEach((row, index) => {
        body.append(row.node);
        row.node.hidden = index < page * pageSize || index >= (page + 1) * pageSize;
      });
      if (info) info.textContent = filtered.length ? `Showing ${page * pageSize + 1}-${Math.min((page + 1) * pageSize, filtered.length)} of ${filtered.length}` : "No results";
      if (previous) previous.disabled = page === 0;
      if (next) next.disabled = (page + 1) * pageSize >= filtered.length;
      headers.forEach((header, index) => {
        if (header.getAttribute("data-sortable") !== "true") return;
        header.setAttribute("aria-sort", index === column && direction ? direction === 1 ? "ascending" : "descending" : "none");
        if (index === column && direction) header.setAttribute("data-sort-dir", direction === 1 ? "asc" : "desc");
        else header.removeAttribute("data-sort-dir");
      });
    }
    headers.forEach((header, index) => {
      if (header.getAttribute("data-sortable") !== "true") return;
      const sort = () => {
        direction = column === index ? direction === 1 ? -1 : direction === -1 ? 0 : 1 : 1;
        column = index;
        page = 0;
        render();
      };
      header.addEventListener("click", sort);
      header.addEventListener("keydown", (event) => {
        if (event.key === "Enter" || event.key === " ") {
          event.preventDefault();
          sort();
        }
      });
    });
    search?.addEventListener("input", () => {
      page = 0;
      render();
    });
    previous?.addEventListener("click", () => {
      page--;
      render();
    });
    next?.addEventListener("click", () => {
      page++;
      render();
    });
    render();
  };
  ui.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  var MONTH_NAMES = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"
  ];
  var DAY_HEADERS = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
  function daysInMonth(year, month) {
    if (month === 2) {
      return year % 4 === 0 && year % 100 !== 0 || year % 400 === 0 ? 29 : 28;
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
    html += '<div class="mui-date-picker__cal-header">';
    html += '<button type="button" class="mui-date-picker__nav-btn" data-action="prev-month" aria-label="Previous month">\u2039</button>';
    html += '<span class="mui-date-picker__cal-title">' + MONTH_NAMES[month - 1] + " " + year + "</span>";
    html += '<button type="button" class="mui-date-picker__nav-btn" data-action="next-month" aria-label="Next month">\u203A</button>';
    html += "</div>";
    html += '<div class="mui-date-picker__day-headers">';
    for (var h = 0; h < DAY_HEADERS.length; h++) {
      html += '<span class="mui-date-picker__day-header">' + DAY_HEADERS[h] + "</span>";
    }
    html += "</div>";
    html += '<div class="mui-date-picker__days">';
    var firstDow = dayOfWeek(year, month, 1);
    var totalDays = daysInMonth(year, month);
    for (var e = 0; e < firstDow; e++) {
      html += '<span class="mui-date-picker__day mui-date-picker__day--empty"></span>';
    }
    for (var d = 1; d <= totalDays; d++) {
      var isSelected = year === selectedYear && month === selectedMonth && d === selectedDay;
      var cls = "mui-date-picker__day";
      if (isSelected) cls += " mui-date-picker__day--selected";
      html += '<button type="button" class="' + cls + '" data-day="' + d + '" data-month="' + month + '" data-year="' + year + '">' + d + "</button>";
    }
    html += "</div></div>";
    return html;
  }
  window.MaudUI.behaviors["date-picker"] = function(root) {
    var trigger = root.querySelector(".mui-date-picker__trigger");
    var dropdown = root.querySelector(".mui-date-picker__dropdown");
    var valueEl = root.querySelector(".mui-date-picker__value");
    var hiddenInput = root.querySelector(".mui-date-picker__hidden");
    var selectedYear = 0;
    var selectedMonth = 0;
    var selectedDay = 0;
    var viewYear = 0;
    var viewMonth = 0;
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
      var prevBtn = dropdown.querySelector('[data-action="prev-month"]');
      var nextBtn = dropdown.querySelector('[data-action="next-month"]');
      if (prevBtn) {
        prevBtn.addEventListener("click", function(e) {
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
        nextBtn.addEventListener("click", function(e) {
          e.stopPropagation();
          viewMonth++;
          if (viewMonth > 12) {
            viewMonth = 1;
            viewYear++;
          }
          rebuildCalendar();
        });
      }
      var days = dropdown.querySelectorAll(".mui-date-picker__day:not(.mui-date-picker__day--empty):not(.mui-date-picker__day--disabled)");
      for (var i = 0; i < days.length; i++) {
        days[i].addEventListener("click", function(e) {
          e.stopPropagation();
          var btn = e.currentTarget;
          selectedDay = parseInt(btn.getAttribute("data-day"), 10);
          selectedMonth = parseInt(btn.getAttribute("data-month"), 10);
          selectedYear = parseInt(btn.getAttribute("data-year"), 10);
          valueEl.textContent = formatDisplay(selectedYear, selectedMonth, selectedDay);
          valueEl.classList.remove("mui-date-picker__value--placeholder");
          hiddenInput.value = formatISO(selectedYear, selectedMonth, selectedDay);
          close();
          root.dispatchEvent(new CustomEvent("mui-date-change", {
            detail: { year: selectedYear, month: selectedMonth, day: selectedDay, iso: hiddenInput.value },
            bubbles: true
          }));
        });
      }
    }
    trigger.addEventListener("click", function() {
      if (trigger.getAttribute("aria-expanded") === "true") {
        close();
      } else {
        open();
      }
    });
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["dialog-trigger"] = function(el) {
    el.addEventListener("click", function() {
      var target_id = el.getAttribute("data-target");
      var dialog = target_id ? document.getElementById(target_id) : null;
      if (dialog && dialog.showModal) {
        dialog.showModal();
      }
    });
  };
  window.MaudUI.behaviors["alert-dialog-trigger"] = function(el) {
    el.addEventListener("click", function() {
      var target_id = el.getAttribute("data-target");
      var dialog = target_id ? document.getElementById(target_id) : null;
      if (dialog && dialog.showModal) {
        dialog.showModal();
      }
    });
  };
  window.MaudUI.behaviors["dialog"] = function(el) {
    var closeBtns = el.querySelectorAll("[data-mui-close]");
    for (var i = 0; i < closeBtns.length; i++) {
      closeBtns[i].addEventListener("click", function() {
        el.close();
      });
    }
    el.addEventListener("click", function(e) {
      if (e.target === el) {
        el.close();
      }
    });
  };
  window.MaudUI.behaviors["alert-dialog"] = function(el) {
    var closeBtns = el.querySelectorAll("[data-mui-close]");
    for (var i = 0; i < closeBtns.length; i++) {
      closeBtns[i].addEventListener("click", function() {
        el.close();
      });
    }
    el.addEventListener("cancel", function(e) {
      e.preventDefault();
    });
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["dir-toggle"] = function(el) {
    el.addEventListener("click", function() {
      var html = document.documentElement;
      var current = html.getAttribute("dir") || "ltr";
      var next = current === "ltr" ? "rtl" : "ltr";
      html.setAttribute("dir", next);
      el.textContent = next === "ltr" ? "RTL" : "LTR";
      try {
        localStorage.setItem("mui-dir", next);
      } catch (e) {
      }
    });
    try {
      var saved = localStorage.getItem("mui-dir");
      if (saved && saved !== (document.documentElement.getAttribute("dir") || "ltr")) {
        document.documentElement.setAttribute("dir", saved);
        el.textContent = saved === "ltr" ? "RTL" : "LTR";
      }
    } catch (e) {
    }
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["drawer-trigger"] = function(el) {
    el.addEventListener("click", function() {
      var target_id = el.getAttribute("data-target");
      var drawer = target_id ? document.getElementById(target_id) : null;
      if (drawer && drawer.showModal) {
        drawer.showModal();
      }
    });
  };
  window.MaudUI.behaviors["drawer"] = function(el) {
    var closeBtns = el.querySelectorAll("[data-mui-close]");
    for (var i = 0; i < closeBtns.length; i++) {
      closeBtns[i].addEventListener("click", function() {
        el.close();
      });
    }
    el.addEventListener("click", function(e) {
      if (e.target === el) {
        el.close();
      }
    });
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["hover-card"] = function(root) {
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
      openTimer = setTimeout(function() {
        content.removeAttribute("hidden");
        void content.offsetHeight;
        content.setAttribute("data-visible", "true");
      }, openDelay);
    }
    function hide() {
      clearTimeout(openTimer);
      closeTimer = setTimeout(function() {
        content.setAttribute("data-visible", "false");
        hideTimer = setTimeout(function() {
          content.setAttribute("hidden", "");
        }, 150);
      }, closeDelay);
    }
    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    content.addEventListener("mouseenter", function() {
      clearTimeout(closeTimer);
      clearTimeout(hideTimer);
    });
    content.addEventListener("mouseleave", hide);
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
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
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["menu"] = function(root) {
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
  window.MaudUI.behaviors["context-menu"] = function(root) {
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
    region.addEventListener("contextmenu", function(e) {
      e.preventDefault();
      open(e.clientX, e.clientY);
    });
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["menubar"] = function(root) {
    var triggers = [];
    var contents = [];
    var menuWrappers = root.querySelectorAll(".mui-menubar__menu");
    var activeIndex = -1;
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
      if (activeIndex >= 0 && activeIndex !== index) {
        closeMenu(activeIndex);
      }
      activeIndex = index;
      triggers[index].setAttribute("aria-expanded", "true");
      contents[index].removeAttribute("hidden");
      for (var i2 = 0; i2 < triggers.length; i2++) {
        triggers[i2].tabIndex = i2 === index ? 0 : -1;
      }
      var firstItem = getMenuItems(index);
      if (firstItem.length > 0) {
        firstItem[0].tabIndex = 0;
        firstItem[0].focus();
      }
      document.addEventListener("click", handleClickOutside, true);
      document.addEventListener("keydown", handleGlobalKeydown, true);
    }
    function closeMenu(index) {
      if (index < 0 || index >= triggers.length) return;
      triggers[index].setAttribute("aria-expanded", "false");
      contents[index].setAttribute("hidden", "");
      var items = getMenuItems(index);
      for (var i2 = 0; i2 < items.length; i2++) {
        items[i2].tabIndex = -1;
      }
    }
    function closeAll() {
      for (var i2 = 0; i2 < triggers.length; i2++) {
        closeMenu(i2);
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
      for (var i2 = 0; i2 < items.length; i2++) {
        if (items[i2] === focused) return i2;
      }
      return -1;
    }
    function focusItem(menuIndex, itemIndex) {
      var items = getMenuItems(menuIndex);
      if (itemIndex < 0 || itemIndex >= items.length) return;
      for (var i2 = 0; i2 < items.length; i2++) {
        items[i2].tabIndex = i2 === itemIndex ? 0 : -1;
      }
      items[itemIndex].focus();
    }
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
    for (var t = 0; t < triggers.length; t++) {
      (function(idx) {
        triggers[idx].addEventListener("click", function(e) {
          e.stopPropagation();
          if (activeIndex === idx) {
            closeAll();
            triggers[idx].focus();
          } else {
            openMenu(idx);
          }
        });
        triggers[idx].addEventListener("mouseenter", function() {
          if (isAnyOpen() && activeIndex !== idx) {
            openMenu(idx);
          }
        });
      })(t);
    }
    for (var c = 0; c < contents.length; c++) {
      (function(menuIdx) {
        contents[menuIdx].addEventListener("click", function(e) {
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
    root.addEventListener("keydown", function(e) {
      if (isAnyOpen()) return;
      var focused = document.activeElement;
      var currentIdx = -1;
      for (var i2 = 0; i2 < triggers.length; i2++) {
        if (triggers[i2] === focused) {
          currentIdx = i2;
          break;
        }
      }
      if (currentIdx < 0) return;
      switch (e.key) {
        case "ArrowRight":
          e.preventDefault();
          var nextIdx = (currentIdx + 1) % triggers.length;
          for (var i2 = 0; i2 < triggers.length; i2++) {
            triggers[i2].tabIndex = i2 === nextIdx ? 0 : -1;
          }
          triggers[nextIdx].focus();
          break;
        case "ArrowLeft":
          e.preventDefault();
          var prevIdx = (currentIdx - 1 + triggers.length) % triggers.length;
          for (var i2 = 0; i2 < triggers.length; i2++) {
            triggers[i2].tabIndex = i2 === prevIdx ? 0 : -1;
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
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["nav-menu"] = function(root) {
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
      for (var i2 = 0; i2 < triggers.length; i2++) {
        closeMenu(i2);
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
      for (var i2 = 0; i2 < links.length; i2++) {
        if (links[i2] === focused) return i2;
      }
      return -1;
    }
    function focusLink(menuIndex, linkIndex) {
      var links = getLinks(menuIndex);
      if (linkIndex >= 0 && linkIndex < links.length) {
        links[linkIndex].focus();
      }
    }
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
            var prevMenu = (activeIndex - 1 + triggers.length) % triggers.length;
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
    for (var t = 0; t < triggers.length; t++) {
      (function(idx) {
        triggers[idx].addEventListener("click", function(e) {
          e.stopPropagation();
          if (activeIndex === idx) {
            closeAll();
            triggers[idx].focus();
          } else {
            openMenu(idx);
          }
        });
        triggers[idx].addEventListener("mouseenter", function() {
          if (isAnyOpen() && activeIndex !== idx) {
            openMenu(idx);
          }
        });
      })(t);
    }
    root.addEventListener("keydown", function(e) {
      if (isAnyOpen()) return;
      var focused = document.activeElement;
      var currentIdx = -1;
      for (var i2 = 0; i2 < triggers.length; i2++) {
        if (triggers[i2] === focused) {
          currentIdx = i2;
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
          var prevIdx = (currentIdx - 1 + triggers.length) % triggers.length;
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
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  function updateValue(input, newVal) {
    var min = input.hasAttribute("min") ? parseFloat(input.min) : -Infinity;
    var max = input.hasAttribute("max") ? parseFloat(input.max) : Infinity;
    var step = parseFloat(input.step) || 1;
    newVal = Math.round(newVal / step) * step;
    newVal = Math.max(min, Math.min(max, newVal));
    var decimals = (step.toString().split(".")[1] || "").length;
    input.value = newVal.toFixed(decimals);
    input.dispatchEvent(new Event("change", { bubbles: true }));
  }
  window.MaudUI.behaviors["number-field-dec"] = function(el) {
    el.addEventListener("click", function() {
      if (el.disabled) return;
      var input = el.parentElement.querySelector("input[type='number']");
      if (!input || input.disabled) return;
      var current = parseFloat(input.value) || 0;
      var step = parseFloat(input.step) || 1;
      updateValue(input, current - step);
    });
  };
  window.MaudUI.behaviors["number-field-inc"] = function(el) {
    el.addEventListener("click", function() {
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
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["popover"] = function(root) {
    var trigger = root.querySelector(".mui-popover__trigger");
    var content = root.querySelector(".mui-popover__content");
    if (!trigger || !content) return;
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
      content.removeAttribute("hidden");
      void content.offsetHeight;
      content.setAttribute("data-visible", "true");
      content.focus();
      document.addEventListener("click", clickOutside, true);
      document.addEventListener("keydown", escClose, true);
    }
    function close() {
      interactive.setAttribute("aria-expanded", "false");
      content.setAttribute("data-visible", "false");
      setTimeout(function() {
        content.setAttribute("hidden", "");
      }, 150);
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
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["resizable"] = function(root) {
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
      return min / 100 * getTotalSize();
    }
    function applyFlexValues(sizes) {
      var total = 0;
      for (var i = 0; i < sizes.length; i++) {
        total += sizes[i];
      }
      for (var i = 0; i < panels.length; i++) {
        var pct = sizes[i] / total * 100;
        panels[i].style.flex = pct + " 1 0%";
      }
      for (var h2 = 0; h2 < handles.length; h2++) {
        var leftSize = sizes[h2];
        var leftPct = (leftSize / total * 100).toFixed(1);
        handles[h2].setAttribute("aria-valuenow", leftPct);
      }
    }
    for (var h = 0; h < handles.length; h++) {
      (function(handleIndex) {
        var handle = handles[handleIndex];
        var panelA = panels[handleIndex];
        var panelB = panels[handleIndex + 1];
        handle.addEventListener("mousedown", function(e) {
          e.preventDefault();
          root.setAttribute("data-dragging", "");
          var sizes = getPanelSizes();
          var startPos = isHorizontal ? e.clientX : e.clientY;
          var startA = sizes[handleIndex];
          var startB = sizes[handleIndex + 1];
          var minA = getMinSize(panelA);
          var minB = getMinSize(panelB);
          function onMove(e2) {
            var currentPos = isHorizontal ? e2.clientX : e2.clientY;
            var delta = currentPos - startPos;
            var newA = startA + delta;
            var newB = startB - delta;
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
        handle.addEventListener("keydown", function(e) {
          var step = getTotalSize() * 0.02;
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
        handle.addEventListener("dblclick", function() {
          var equalSize = 100 / panels.length;
          for (var i = 0; i < panels.length; i++) {
            panels[i].style.flex = equalSize + " 1 0%";
          }
        });
      })(h);
    }
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["scroll-area"] = function(root) {
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
      var thumbHeight = Math.max(ratio * 100, 10);
      var scrollRatio = viewport.scrollTop / (viewport.scrollHeight - viewport.clientHeight);
      var thumbTop = scrollRatio * (100 - thumbHeight);
      thumb.style.height = thumbHeight + "%";
      thumb.style.top = thumbTop + "%";
      scrollbar.classList.add("mui-scroll-area__scrollbar--visible");
      clearTimeout(hideTimer);
      hideTimer = setTimeout(function() {
        scrollbar.classList.remove("mui-scroll-area__scrollbar--visible");
      }, 1e3);
    }
    viewport.addEventListener("scroll", updateThumb);
    var dragging = false;
    var startY = 0;
    var startScroll = 0;
    thumb.addEventListener("mousedown", function(e) {
      e.preventDefault();
      dragging = true;
      startY = e.clientY;
      startScroll = viewport.scrollTop;
      document.addEventListener("mousemove", onDrag);
      document.addEventListener(
        "mouseup",
        function() {
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
      viewport.scrollTop = startScroll + delta / trackHeight * scrollable;
    }
    updateThumb();
    if (window.ResizeObserver) {
      new ResizeObserver(updateThumb).observe(viewport);
    }
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["select"] = function(root) {
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
    trigger.addEventListener("click", function() {
      if (trigger.getAttribute("aria-expanded") === "true") {
        close();
      } else {
        open();
      }
    });
    trigger.addEventListener("keydown", function(e) {
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
        typeAheadTimeout = setTimeout(function() {
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
    dropdown.addEventListener("click", function(e) {
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
(function() {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const groups = /* @__PURE__ */ new WeakMap();
  const read = (key) => {
    try {
      return localStorage.getItem(key);
    } catch {
      return null;
    }
  };
  const write = (key, value) => {
    try {
      localStorage.setItem(key, value);
    } catch {
    }
  };
  function initGroup(group) {
    if (groups.has(group)) return groups.get(group);
    const key = "mui-nav-group:" + (group.getAttribute("data-nav-key") || group.id);
    const saved = read(key);
    const state = { desired: saved === "closed" ? false : saved === "open" ? true : group.open, forced: false };
    if (saved === null && group.querySelector('[aria-current="page"]')) state.desired = true;
    groups.set(group, state);
    group.open = state.desired;
    group.addEventListener("toggle", () => {
      if (state.forced) return;
      state.desired = group.open;
      write(key, group.open ? "open" : "closed");
    });
    return state;
  }
  function forceGroups(root, forced) {
    root.querySelectorAll('[data-mui="nav-group"]').forEach((group) => {
      const state = initGroup(group);
      state.forced = forced;
      group.open = forced || state.desired;
    });
  }
  ui.navigation = { read, write, forceGroups };
  ui.behaviors["nav-group"] = initGroup;
  function updateSidebar(sidebar, collapsed) {
    const desktop = window.matchMedia("(min-width: 64rem)").matches;
    const mode = sidebar.getAttribute("data-collapsible");
    if (mode === "none") collapsed = false;
    sidebar.setAttribute("data-state", collapsed ? "collapsed" : "expanded");
    write("mui-sidebar:" + sidebar.id, collapsed ? "collapsed" : "expanded");
    forceGroups(sidebar, desktop && mode === "icon" && collapsed);
    document.querySelectorAll('[data-mui="sidebar-trigger"]').forEach((trigger) => {
      if (trigger.getAttribute("data-target") === sidebar.id) {
        trigger.setAttribute("aria-controls", sidebar.id);
        trigger.setAttribute("aria-expanded", String(!window.matchMedia("(max-width: 59.99rem)").matches && (!collapsed || !desktop && mode === "icon")));
      }
    });
  }
  function toggle(sidebar, trigger) {
    if (!sidebar) return;
    const phone = window.matchMedia("(max-width: 59.99rem)").matches;
    if (phone) {
      const panel = document.getElementById(sidebar.id + "-drawer");
      if (!panel || typeof panel.showModal !== "function" || panel.open) return;
      panel.dispatchEvent(new CustomEvent("mui:navigation-open"));
      panel.showModal();
      if (trigger) {
        trigger.setAttribute("aria-expanded", "true");
        panel.addEventListener("close", () => {
          trigger.setAttribute("aria-expanded", "false");
          if (trigger.isConnected && trigger.getClientRects().length) trigger.focus();
        }, { once: true });
      }
    } else updateSidebar(sidebar, sidebar.getAttribute("data-state") !== "collapsed");
  }
  ui.behaviors["sidebar"] = (sidebar) => {
    const panel = document.getElementById(sidebar.id + "-drawer");
    const provider = sidebar.closest(".mui-sidebar-provider");
    if (!panel || !provider || typeof panel.showModal !== "function") return;
    const home = panel.parentNode;
    sidebar.setAttribute("data-navigation-ready", "");
    provider.setAttribute("data-navigation-ready", "");
    panel.removeAttribute("open");
    const restore = () => home.insertBefore(sidebar, panel);
    restore();
    sidebar.querySelectorAll(".mui-sidebar__menu-button").forEach((row) => {
      if (!row.hasAttribute("aria-label")) row.setAttribute("aria-label", row.textContent.trim());
      if (!row.hasAttribute("title")) row.setAttribute("title", row.getAttribute("aria-label"));
    });
    const saved = read("mui-sidebar:" + sidebar.id);
    if (saved === "collapsed" || saved === "expanded") sidebar.setAttribute("data-state", saved);
    const sync = () => {
      const phone = window.matchMedia("(max-width: 59.99rem)").matches;
      if (!phone && panel.open) {
        panel.close();
        restore();
      }
      updateSidebar(sidebar, sidebar.getAttribute("data-state") === "collapsed");
    };
    panel.addEventListener("mui:navigation-open", () => {
      forceGroups(sidebar, false);
      panel.append(sidebar);
    });
    panel.addEventListener("close", restore);
    panel.addEventListener("click", (event) => {
      if (event.target === panel || event.target.closest("a[href]")) panel.close();
    });
    const media = [window.matchMedia("(max-width: 59.99rem)"), window.matchMedia("(min-width: 64rem)")];
    const resize = () => {
      if (!provider.isConnected) {
        media.forEach((m) => m.removeEventListener("change", resize));
        return;
      }
      sync();
    };
    media.forEach((m) => m.addEventListener("change", resize));
    sync();
  };
  ui.behaviors["sidebar-trigger"] = (trigger) => {
    const sidebar = document.getElementById(trigger.getAttribute("data-target"));
    trigger.setAttribute("aria-controls", trigger.getAttribute("data-target"));
    trigger.setAttribute("aria-expanded", String(!!sidebar && sidebar.getAttribute("data-state") !== "collapsed" && !window.matchMedia("(max-width: 59.99rem)").matches));
    trigger.addEventListener("click", () => toggle(sidebar, trigger));
  };
  ui.behaviors["sidebar-rail"] = (rail) => rail.addEventListener("click", () => toggle(rail.closest('[data-mui="sidebar"]')));
  if (!window.__muiSidebarShortcutBound) {
    window.__muiSidebarShortcutBound = true;
    document.addEventListener("keydown", (event) => {
      if (!(event.metaKey || event.ctrlKey) || event.key.toLowerCase() !== "b" || event.defaultPrevented) return;
      if (event.target.closest('input, textarea, select, [contenteditable="true"]')) return;
      const provider = event.target.closest(".mui-sidebar-provider");
      const sidebar = provider ? provider.querySelector('[data-mui="sidebar"]') : document.querySelector('[data-mui="sidebar"]');
      if (sidebar) {
        event.preventDefault();
        toggle(sidebar, document.querySelector('[data-mui="sidebar-trigger"]'));
      }
    });
  }
  ui.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["slider"] = function(root) {
    var track = root.querySelector(".mui-slider__track");
    var fill = root.querySelector(".mui-slider__fill");
    var native = root.querySelector(".mui-slider__native");
    var valueLabel = root.querySelector(".mui-slider__value");
    var thumbs = Array.prototype.slice.call(
      root.querySelectorAll(".mui-slider__thumb")
    );
    if (!track || !fill || thumbs.length === 0) return;
    if (root.getAttribute("data-disabled") === "true") return;
    var vertical = root.getAttribute("data-orientation") === "vertical";
    var min = parseFloat(root.getAttribute("data-min") || "0");
    var max = parseFloat(root.getAttribute("data-max") || "100");
    var step = parseFloat(root.getAttribute("data-step") || "1");
    if (!(step > 0)) step = 1;
    var values = thumbs.map(function(t) {
      return parseFloat(t.getAttribute("aria-valuenow") || "0");
    });
    function clamp(v) {
      return Math.max(min, Math.min(max, v));
    }
    function snap(v) {
      return clamp(Math.round((v - min) / step) * step + min);
    }
    function fmt(v) {
      return v % 1 === 0 ? String(v) : String(parseFloat(v.toFixed(4)));
    }
    function pct(v) {
      return max === min ? 0 : (v - min) / (max - min) * 100;
    }
    function paint() {
      for (var i2 = 0; i2 < thumbs.length; i2++) {
        var p = pct(values[i2]);
        if (vertical) thumbs[i2].style.bottom = p + "%";
        else thumbs[i2].style.left = p + "%";
        thumbs[i2].setAttribute("aria-valuenow", fmt(values[i2]));
      }
      var lo, hi;
      if (values.length > 1) {
        lo = Math.min.apply(null, values);
        hi = Math.max.apply(null, values);
      } else {
        lo = min;
        hi = values[0];
      }
      var start = pct(lo);
      var extent = Math.max(0, pct(hi) - start);
      if (vertical) {
        fill.style.bottom = start + "%";
        fill.style.height = extent + "%";
      } else {
        fill.style.left = start + "%";
        fill.style.width = extent + "%";
      }
      if (valueLabel) {
        valueLabel.textContent = values.length > 1 ? fmt(lo) + " \u2014 " + fmt(hi) : fmt(values[0]);
      }
      if (root.hasAttribute("data-values")) {
        root.setAttribute("data-values", values.map(fmt).join(","));
      }
      if (native) {
        native.value = values[0];
        native.dispatchEvent(new Event("input", { bubbles: true }));
        native.dispatchEvent(new Event("change", { bubbles: true }));
      }
    }
    function setThumb(i2, v) {
      v = snap(v);
      if (i2 > 0) v = Math.max(v, values[i2 - 1]);
      if (i2 < values.length - 1) v = Math.min(v, values[i2 + 1]);
      if (v === values[i2]) return;
      values[i2] = v;
      paint();
    }
    function valueFromPointer(e) {
      var rect = track.getBoundingClientRect();
      var p;
      if (vertical) {
        p = rect.height === 0 ? 0 : (rect.bottom - e.clientY) / rect.height;
      } else {
        p = rect.width === 0 ? 0 : (e.clientX - rect.left) / rect.width;
      }
      return snap(min + Math.max(0, Math.min(1, p)) * (max - min));
    }
    function nearestThumb(v) {
      var best = 0;
      var bestDist = Infinity;
      for (var i2 = 0; i2 < values.length; i2++) {
        var d = Math.abs(values[i2] - v);
        if (d < bestDist) {
          bestDist = d;
          best = i2;
        }
      }
      return best;
    }
    function bindThumb(thumb, index) {
      thumb.addEventListener("pointerdown", function(e) {
        if (thumb.getAttribute("aria-disabled") === "true") return;
        e.preventDefault();
        thumb.focus();
        try {
          thumb.setPointerCapture(e.pointerId);
        } catch (_) {
        }
        root.setAttribute("data-dragging", "true");
        function onMove(ev) {
          setThumb(index, valueFromPointer(ev));
        }
        function onUp() {
          root.removeAttribute("data-dragging");
          thumb.removeEventListener("pointermove", onMove);
          thumb.removeEventListener("pointerup", onUp);
          thumb.removeEventListener("pointercancel", onUp);
          try {
            if (thumb.hasPointerCapture(e.pointerId)) {
              thumb.releasePointerCapture(e.pointerId);
            }
          } catch (_) {
          }
        }
        thumb.addEventListener("pointermove", onMove);
        thumb.addEventListener("pointerup", onUp);
        thumb.addEventListener("pointercancel", onUp);
      });
      thumb.addEventListener("keydown", function(e) {
        if (thumb.getAttribute("aria-disabled") === "true") return;
        var v = values[index];
        var big = step * 10;
        var next = null;
        if (e.key === "ArrowRight" || e.key === "ArrowUp") next = v + step;
        else if (e.key === "ArrowLeft" || e.key === "ArrowDown") next = v - step;
        else if (e.key === "PageUp") next = v + big;
        else if (e.key === "PageDown") next = v - big;
        else if (e.key === "Home") next = min;
        else if (e.key === "End") next = max;
        if (next === null) return;
        e.preventDefault();
        setThumb(index, next);
      });
    }
    for (var i = 0; i < thumbs.length; i++) bindThumb(thumbs[i], i);
    track.addEventListener("pointerdown", function(e) {
      if (e.target !== track && e.target !== fill) return;
      var v = valueFromPointer(e);
      var idx = nearestThumb(v);
      setThumb(idx, v);
      thumbs[idx].focus();
    });
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["switch"] = function(el) {
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
    el.addEventListener("keydown", function(e) {
      if (e.code === "Space" || e.code === "Enter") {
        e.preventDefault();
        toggle();
      }
    });
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["tabs"] = function(root) {
    var tabs = root.querySelectorAll('[role="tab"]');
    var panels = root.querySelectorAll('[role="tabpanel"]');
    function activateTab(index) {
      for (var i2 = 0; i2 < tabs.length; i2++) {
        tabs[i2].setAttribute("aria-selected", "false");
        tabs[i2].setAttribute("tabindex", "-1");
        if (panels[i2]) {
          panels[i2].setAttribute("hidden", "");
        }
      }
      if (tabs[index]) {
        tabs[index].setAttribute("aria-selected", "true");
        tabs[index].setAttribute("tabindex", "0");
        tabs[index].focus();
      }
      if (panels[index]) {
        panels[index].removeAttribute("hidden");
      }
    }
    for (var i = 0; i < tabs.length; i++) {
      (function(tabIndex) {
        tabs[tabIndex].addEventListener("click", function() {
          activateTab(tabIndex);
        });
        tabs[tabIndex].addEventListener("keydown", function(e) {
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
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["theme-toggle"] = function(el) {
    el.addEventListener("click", function() {
      var html = document.documentElement;
      var current = html.getAttribute("data-theme") || "dark";
      var next = current === "dark" ? "light" : "dark";
      html.setAttribute("data-theme", next);
      el.textContent = next === "dark" ? "Light mode" : "Dark mode";
      try {
        localStorage.setItem("mui-theme", next);
      } catch (e) {
      }
    });
    try {
      var saved = localStorage.getItem("mui-theme");
      if (saved && saved !== (document.documentElement.getAttribute("data-theme") || "dark")) {
        document.documentElement.setAttribute("data-theme", saved);
        el.textContent = saved === "dark" ? "Light mode" : "Dark mode";
      }
    } catch (e) {
    }
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["toast"] = function(el) {
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
    if (el.hasAttribute("data-mui-exiting")) return;
    el.setAttribute("data-mui-exiting", "");
    el.classList.add("mui-toast--exit");
    const style = getComputedStyle(el);
    const milliseconds = (value) => value.trim().endsWith("ms") ? parseFloat(value) : parseFloat(value) * 1e3;
    const duration = milliseconds((style.animationDuration || "0s").split(",")[0]);
    const delay = milliseconds((style.animationDelay || "0s").split(",")[0]);
    if (!duration || window.matchMedia("(prefers-reduced-motion: reduce)").matches) {
      el.remove();
      return;
    }
    const finish = (event) => {
      if (!event || event.target === el) {
        clearTimeout(timer);
        el.removeEventListener("animationend", finish);
        el.remove();
      }
    };
    const timer = setTimeout(() => finish(), duration + delay);
    el.addEventListener("animationend", finish);
  }
  function buildToastNode(opts) {
    const variant = opts.variant || "info";
    const title = opts.title || "";
    const description = opts.description || "";
    const duration = opts.duration_ms != null ? opts.duration_ms : 5e3;
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
    closeEl.textContent = "\xD7";
    toast.appendChild(closeEl);
    return toast;
  }
  window.MaudUI.toast = function(opts) {
    const viewport = document.getElementById("mui-toast-viewport");
    if (!viewport) {
      console.warn("Toast viewport not found. Call maud_ui::primitives::toast::viewport() in your page.");
      return;
    }
    viewport.appendChild(buildToastNode(opts));
    if (window.MaudUI.init) window.MaudUI.init(viewport);
  };
  function dispatchToSonner(opts) {
    const viewport = document.querySelector(".mui-sonner");
    if (!viewport) {
      console.warn("Sonner viewport not found. Call maud_ui::primitives::sonner::viewport(position) in your page.");
      return;
    }
    viewport.appendChild(buildToastNode(opts));
    if (window.MaudUI.init) window.MaudUI.init(viewport);
  }
  window.MaudUI.sonner = dispatchToSonner;
  window.addEventListener("mui:sonner-toast", function(e) {
    if (e && e.detail) dispatchToSonner(e.detail);
  });
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["toggle"] = function(el) {
    function toggle() {
      if (el.hasAttribute("disabled")) return;
      var isPressed = el.getAttribute("aria-pressed") === "true";
      el.setAttribute("aria-pressed", !isPressed ? "true" : "false");
    }
    el.addEventListener("click", toggle);
    el.addEventListener("keydown", function(e) {
      if (e.code === "Space" || e.code === "Enter") {
        e.preventDefault();
        toggle();
      }
    });
  };
  window.MaudUI.behaviors["toggle-group"] = function(el) {
    var items = el.querySelectorAll(".mui-toggle-group__item");
    var isMultiple = el.getAttribute("data-multiple") === "true";
    var isDisabled = el.hasAttribute("data-disabled");
    function updateTabindex() {
      var focusable = null;
      for (var i2 = 0; i2 < items.length; i2++) {
        if (items[i2].getAttribute("aria-pressed") === "true") {
          focusable = items[i2];
          break;
        }
      }
      if (!focusable && items.length > 0) {
        focusable = items[0];
      }
      for (var i2 = 0; i2 < items.length; i2++) {
        items[i2].tabIndex = items[i2] === focusable ? 0 : -1;
      }
    }
    function pressItem(item) {
      if (isDisabled || item.hasAttribute("disabled")) return;
      var isPressed = item.getAttribute("aria-pressed") === "true";
      var newPressed = !isPressed;
      if (isMultiple) {
        item.setAttribute("aria-pressed", newPressed ? "true" : "false");
      } else {
        for (var i2 = 0; i2 < items.length; i2++) {
          items[i2].setAttribute("aria-pressed", "false");
        }
        item.setAttribute("aria-pressed", "true");
      }
      updateTabindex();
    }
    function handleArrowKey(direction) {
      var focusedIndex = -1;
      for (var i2 = 0; i2 < items.length; i2++) {
        if (items[i2] === document.activeElement) {
          focusedIndex = i2;
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
      (function(item) {
        item.addEventListener("click", function() {
          pressItem(item);
        });
        item.addEventListener("keydown", function(e) {
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
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;
  window.MaudUI.behaviors["tooltip"] = function(wrapper) {
    var trigger = wrapper.querySelector(".mui-tooltip__trigger");
    var content = wrapper.querySelector(".mui-tooltip__content");
    if (!trigger || !content) return;
    var delay = parseInt(wrapper.getAttribute("data-delay") || "500", 10);
    var showTimer = null;
    var hideTimer = null;
    function show() {
      clearTimeout(hideTimer);
      clearTimeout(showTimer);
      showTimer = setTimeout(function() {
        content.removeAttribute("hidden");
        void content.offsetHeight;
        content.setAttribute("data-visible", "true");
      }, delay);
    }
    function hide() {
      clearTimeout(showTimer);
      clearTimeout(hideTimer);
      content.setAttribute("data-visible", "false");
      hideTimer = setTimeout(function() {
        content.setAttribute("hidden", "");
      }, 150);
    }
    trigger.addEventListener("mouseenter", show);
    trigger.addEventListener("mouseleave", hide);
    trigger.addEventListener("focus", show);
    trigger.addEventListener("blur", hide);
    trigger.addEventListener("keydown", function(e) {
      if (e.key === "Escape") hide();
    });
  };
  if (window.MaudUI.init) window.MaudUI.init();
})();
(function() {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = /* @__PURE__ */ new WeakSet();
  ui.behaviors["banking-demo"] = (root) => {
    if (initialized.has(root)) return;
    initialized.add(root);
    let trigger;
    root.addEventListener("click", (event) => {
      const link = event.target.closest("[data-bank-review]");
      if (link && !event.metaKey && !event.ctrlKey && !event.shiftKey && !event.altKey && event.button === 0) {
        const panel = document.getElementById(link.getAttribute("href").slice(1));
        if (panel && root.contains(panel)) {
          event.preventDefault();
          trigger = link;
          panel.open = true;
          panel.querySelector("summary").focus();
        }
      }
      const close = event.target.closest("[data-bank-close]");
      if (close) {
        const panel = close.closest("details");
        panel.open = false;
        if (trigger?.isConnected) trigger.focus();
        else panel.querySelector("summary").focus();
      }
    });
    const shell = root.closest(".mui-block--shell");
    const search = shell?.querySelector('.mui-page-header input[type="search"]');
    const transactions = root.querySelector(".mui-data-table__search");
    search?.closest("form")?.addEventListener("submit", (event) => {
      event.preventDefault();
      transactions.value = search.value;
      transactions.dispatchEvent(new Event("input", { bubbles: true }));
      search.closest("details").open = false;
      transactions.focus();
    });
  };
  ui.init();
})();
window.MaudUI.brandData = { "tokens": [{ "name": "--mui-brand-accent", "label": "Accent", "kind": "color", "default": "#285bc5" }, { "name": "--mui-brand-accent-ink", "label": "Text on accent", "kind": "color", "default": "#ffffff" }, { "name": "--mui-brand-font-heading", "label": "Heading font", "kind": "text", "default": '"Avenir Next", "Nunito Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif' }, { "name": "--mui-brand-font-body", "label": "Body font", "kind": "text", "default": '"Avenir Next", "Nunito Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", system-ui, sans-serif' }, { "name": "--mui-brand-radius", "label": "Corner radius", "kind": "text", "default": "0.375rem" }, { "name": "--mui-brand-density", "label": "Density", "kind": "density", "default": "1" }, { "name": "--mui-brand-logo-mask", "label": "Logo mask (CSS url)", "kind": "text", "default": 'url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%2032%2032%22%3E%3Cpath%20d%3D%22M16%203%203%2010v3h26v-3ZM5%2016h4v10H5Zm9%200h4v10h-4Zm9%200h4v10h-4ZM3%2028h26v3H3Z%22%2F%3E%3C%2Fsvg%3E")' }, { "name": "--mui-brand-logo-size", "label": "Logo size", "kind": "text", "default": "2rem" }, { "name": "--mui-brand-logo-radius", "label": "Logo corner radius", "kind": "text", "default": "0" }], "presets": [{ "key": "lodge", "label": "Lodge", "wordmark": "Garden House", "tagline": "Independent hospitality", "values": ["#466752", "#ffffff", "Georgia, serif", '"Avenir Next", system-ui, sans-serif', "0.5rem", "1", 'url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%2032%2032%22%3E%3Cpath%20d%3D%22m16%203%2011%2012h-6l7%209H18v5h-4v-5H4l7-9H5Z%22%2F%3E%3C%2Fsvg%3E")', "2rem", "0"] }, { "key": "bank", "label": "Bank", "wordmark": "Northline", "tagline": "Business banking", "values": ["#24428c", "#ffffff", '"Avenir Next", system-ui, sans-serif', '"Avenir Next", system-ui, sans-serif', "0.25rem", "0", 'url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%2032%2032%22%3E%3Cpath%20d%3D%22M16%203%203%2010v3h26v-3ZM5%2016h4v10H5Zm9%200h4v10h-4Zm9%200h4v10h-4ZM3%2028h26v3H3Z%22%2F%3E%3C%2Fsvg%3E")', "1.75rem", "0"] }, { "key": "clinic", "label": "Clinic", "wordmark": "Commonwell", "tagline": "Care for your community", "values": ["#076b67", "#ffffff", '"Trebuchet MS", system-ui, sans-serif', '"Avenir Next", system-ui, sans-serif', "0.75rem", "2", 'url("data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%2032%2032%22%3E%3Cpath%20d%3D%22M12%203h8v9h9v8h-9v9h-8v-9H3v-8h9Z%22%2F%3E%3C%2Fsvg%3E")', "2rem", "0.25rem"] }] };
(function() {
  "use strict";
  const ui = window.MaudUI, data = ui?.brandData;
  if (!data) return;
  const root = document.documentElement;
  const modes = ["compact", "comfortable", "spacious"];
  const read = (key) => {
    try {
      return localStorage.getItem(key);
    } catch {
      return null;
    }
  };
  const write = (key, value) => {
    try {
      localStorage.setItem(key, value);
    } catch {
    }
  };
  let values = data.tokens.map((token) => token.default);
  let current = "";
  const savedDensity = read("mui-density");
  ui.densityChosen = modes.includes(savedDensity);
  function refresh() {
    const output = document.getElementById("mui-theme-export");
    if (output) output.textContent = buildCss();
    document.querySelectorAll("[data-brand-token]").forEach((input) => {
      const index = data.tokens.findIndex((token) => token.name === input.getAttribute("data-brand-token"));
      if (index >= 0 && input !== document.activeElement) input.value = values[index];
    });
    document.querySelectorAll("[data-brand-select]").forEach((select) => {
      select.value = current;
    });
  }
  function buildCss() {
    return ":root {\n" + data.tokens.map((token, index) => "  " + token.name + ": " + values[index] + ";").join("\n") + "\n}\n";
  }
  function valid(token, value) {
    if (typeof value !== "string" || !value.trim()) return false;
    if (token.kind === "density") return ["0", "1", "2"].includes(value);
    if (token.kind === "color") return /^#[0-9a-f]{6}$/i.test(value);
    const property = token.name.includes("font-") ? "font-family" : token.name.endsWith("mask") ? "mask-image" : token.name.includes("radius") ? "border-radius" : "width";
    return window.CSS?.supports(property, value) ?? false;
  }
  function persist() {
    write("mui-brand", JSON.stringify({ current, values }));
  }
  function setDensity(mode, persistChoice = true) {
    if (!modes.includes(mode)) return;
    root.setAttribute("data-density", mode);
    document.querySelectorAll("[data-mui-density-scope]").forEach((scope) => scope.setAttribute("data-mui-density-scope", mode));
    document.querySelectorAll('[data-mui="density-control"]').forEach((select) => {
      select.value = mode;
    });
    values[5] = String(modes.indexOf(mode));
    if (persistChoice) {
      ui.densityChosen = true;
      write("mui-density", mode);
      persist();
    }
    refresh();
  }
  ui.setDensity = setDensity;
  function apply(next, name, save = true) {
    if (next.length !== data.tokens.length || !next.every((v, i) => valid(data.tokens[i], v))) return false;
    let old = {};
    try {
      old = JSON.parse(read("mui-theme-overrides") || "{}");
    } catch {
    }
    Object.keys(old).forEach((key) => {
      if (key.startsWith("mui-") && !key.startsWith("mui-brand-")) root.style.removeProperty("--" + key);
    });
    values = [...next];
    current = name;
    root.removeAttribute("data-brand");
    data.tokens.forEach((token, i) => root.style.setProperty(token.name, values[i]));
    const brand = data.presets.find((preset) => preset.key === name);
    if (brand) document.querySelectorAll("[data-brand-live]").forEach((preview) => {
      preview.querySelector(".mui-brand-mark__wordmark").textContent = brand.wordmark;
      preview.querySelector(".mui-brand-mark__tagline").textContent = brand.tagline;
    });
    setDensity(modes[Number(values[5])], false);
    if (save) persist();
    document.dispatchEvent(new CustomEvent("mui:brand-change"));
    refresh();
    return true;
  }
  ui.brand = { buildCss, apply, valid };
  try {
    const saved = JSON.parse(read("mui-brand") || "null");
    if (Array.isArray(saved?.values)) apply(saved.values, saved.current, false);
  } catch {
  }
  if (ui.densityChosen) setDensity(savedDensity, false);
  const initialized = /* @__PURE__ */ new WeakSet();
  ui.behaviors["density-control"] = (select) => {
    if (initialized.has(select)) return;
    initialized.add(select);
    select.value = root.getAttribute("data-density") || "comfortable";
    select.addEventListener("change", () => setDensity(select.value));
  };
  ui.behaviors["brand-customizer"] = (editor) => {
    if (initialized.has(editor)) return;
    initialized.add(editor);
    let saved;
    try {
      saved = JSON.parse(read("mui-brand") || "null");
    } catch {
    }
    if (!saved || !Array.isArray(saved.values) || !apply(saved.values, saved.current, false)) apply(data.presets[0].values, data.presets[0].key, false);
    if (ui.densityChosen) setDensity(savedDensity, false);
    editor.querySelector("[data-brand-select]").addEventListener("change", (event) => {
      const preset = data.presets.find((preset2) => preset2.key === event.target.value);
      if (preset) {
        apply(preset.values, preset.key);
        write("mui-density", modes[Number(preset.values[5])]);
        ui.densityChosen = true;
      }
    });
    editor.addEventListener("input", (event) => {
      const index = data.tokens.findIndex((token) => token.name === event.target.getAttribute("data-brand-token"));
      if (index < 0) return;
      const next = [...values];
      next[index] = event.target.value;
      const accepted = valid(data.tokens[index], next[index]);
      event.target.setAttribute("aria-invalid", String(!accepted));
      if (accepted) {
        apply(next, "");
        if (index === 5) setDensity(modes[Number(next[index])]);
      }
    });
    editor.querySelector("[data-brand-example-action]").addEventListener("click", () => {
      editor.querySelector("[data-brand-status]").textContent = "Action received. This preview uses your current brand and density.";
    });
  };
  ui.init();
})();
(function() {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = /* @__PURE__ */ new WeakSet();
  ui.behaviors["status-chip-group"] = function(group) {
    if (initialized.has(group)) return;
    initialized.add(group);
    group.addEventListener("keydown", function(event) {
      if (event.altKey || event.metaKey || event.ctrlKey || event.shiftKey) return;
      const links = Array.from(group.querySelectorAll("a[href]"));
      const index = links.indexOf(event.target);
      if (index < 0) return;
      const rtl = getComputedStyle(group).direction === "rtl";
      let next;
      if (event.key === "Home") next = 0;
      else if (event.key === "End") next = links.length - 1;
      else if (event.key === "ArrowRight") next = index + (rtl ? -1 : 1);
      else if (event.key === "ArrowLeft") next = index + (rtl ? 1 : -1);
      else if (event.key === "ArrowDown") next = index + 1;
      else if (event.key === "ArrowUp") next = index - 1;
      else return;
      event.preventDefault();
      links[(next + links.length) % links.length].focus();
    });
  };
  ui.behaviors["navigation-trigger"] = function(trigger) {
    if (initialized.has(trigger)) return;
    initialized.add(trigger);
    trigger.setAttribute("aria-expanded", "false");
    trigger.addEventListener("click", function(event) {
      if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
      const dialog = document.getElementById(trigger.getAttribute("aria-controls"));
      if (!dialog || typeof dialog.showModal !== "function") return;
      event.preventDefault();
      if (dialog.open) return;
      dialog.dispatchEvent(new CustomEvent("mui:navigation-open"));
      dialog.showModal();
      trigger.setAttribute("aria-expanded", "true");
      dialog.addEventListener("close", function() {
        trigger.setAttribute("aria-expanded", "false");
        if (trigger.isConnected && trigger.getClientRects().length) trigger.focus();
      }, { once: true });
    });
  };
  ui.behaviors["shell-navigation"] = function(shell) {
    if (initialized.has(shell)) return;
    initialized.add(shell);
    const sidebar = shell.querySelector(".mui-block--shell__sidebar");
    const dialog = shell.querySelector(".mui-navigation-dialog");
    const main = shell.querySelector(".mui-block--shell__main");
    if (!sidebar || !dialog || typeof dialog.showModal !== "function") return;
    shell.setAttribute("data-mui-navigation-ready", "");
    const fallback = shell.querySelector(".mui-navigation-fallback");
    if (fallback) {
      fallback.open = false;
      fallback.hidden = true;
    }
    dialog.removeAttribute("open");
    shell.insertBefore(sidebar, main);
    const headerBrand = shell.querySelector(".mui-app-header .mui-brand-mark");
    const brandHome = headerBrand?.parentNode;
    const brandNext = headerBrand?.nextSibling;
    const drawerBrand = sidebar.querySelector(".mui-block--shell__brand");
    const restore = () => {
      shell.insertBefore(sidebar, main);
      if (headerBrand && brandHome) brandHome.insertBefore(headerBrand, brandNext);
    };
    dialog.addEventListener("mui:navigation-open", () => {
      if (headerBrand && drawerBrand) drawerBrand.insertBefore(headerBrand, drawerBrand.firstChild);
      dialog.append(sidebar);
    });
    dialog.addEventListener("close", restore);
    dialog.addEventListener("click", function(event) {
      if (event.target === dialog || event.target.closest("a[href]")) dialog.close();
    });
    const media = window.matchMedia("(max-width: 63.99rem)");
    const resize = () => {
      if (!shell.isConnected) {
        media.removeEventListener("change", resize);
        return;
      }
      if (!media.matches && dialog.open) {
        const focused = sidebar.contains(document.activeElement) ? document.activeElement : null;
        dialog.close();
        restore();
        if (focused) focused.focus();
      }
    };
    media.addEventListener("change", resize);
    const header = main.querySelector(".mui-page-header");
    const mobileControls = sidebar.querySelector(".mui-block--shell__mobile-controls");
    const where = sidebar.querySelector(".mui-block--shell__where");
    const context = header?.querySelector(".mui-page-header__context");
    const breadcrumb = context?.querySelector(".mui-breadcrumb");
    const settings = header?.querySelector(".mui-page-header__settings");
    const headerControls = shell.querySelector(".mui-page-header__controls");
    const wide = window.matchMedia("(min-width: 80rem)");
    function syncHeader() {
      if (!shell.isConnected) {
        media.removeEventListener("change", syncHeader);
        wide.removeEventListener("change", syncHeader);
        return;
      }
      if (!header) return;
      const focused = document.activeElement;
      if (headerControls && mobileControls) {
        const target = media.matches ? mobileControls : settings || header;
        if (headerControls.parentNode !== target) target.append(headerControls);
      }
      if (breadcrumb && where && context) {
        const target = media.matches ? where : context;
        if (breadcrumb.parentNode !== target) target.append(breadcrumb);
      }
      if (settings) settings.open = wide.matches;
      if (focused && sidebar.contains(focused) && media.matches && !dialog.open) {
        shell.querySelector('[data-mui="navigation-trigger"]')?.focus();
      }
    }
    media.addEventListener("change", syncHeader);
    wide.addEventListener("change", syncHeader);
    syncHeader();
    const prefs = ui.navigation;
    const railMedia = window.matchMedia("(min-width: 64rem)");
    const rail = shell.querySelector('[data-mui="shell-rail"]');
    const key = "mui-shell-rail:" + shell.id;
    const saved = prefs && prefs.read(key);
    if (saved === "true" || saved === "false") shell.setAttribute("data-collapsed", saved);
    function syncRail() {
      const collapsed = !!rail && railMedia.matches && shell.getAttribute("data-collapsed") === "true";
      if (rail) rail.setAttribute("aria-expanded", String(!collapsed));
      if (prefs) prefs.forceGroups(sidebar, collapsed);
    }
    if (rail) rail.addEventListener("click", function() {
      const next = shell.getAttribute("data-collapsed") !== "true";
      if (next) {
        const input = sidebar.querySelector("[data-mui-nav-search]");
        if (input) input.value = "";
        sidebar.querySelectorAll("li, .mui-block--shell__nav-group").forEach((row) => {
          row.hidden = false;
        });
      }
      shell.setAttribute("data-collapsed", String(next));
      if (prefs) prefs.write(key, String(next));
      syncRail();
    });
    const resizeRail = () => {
      if (!shell.isConnected) {
        railMedia.removeEventListener("change", resizeRail);
        return;
      }
      syncRail();
    };
    railMedia.addEventListener("change", resizeRail);
    dialog.addEventListener("mui:navigation-open", () => {
      if (prefs) prefs.forceGroups(sidebar, false);
    });
    dialog.addEventListener("close", syncRail);
    syncRail();
    const search = sidebar.querySelector("[data-mui-nav-search]");
    if (search) search.addEventListener("input", function() {
      const query = search.value.trim().toLocaleLowerCase();
      if (prefs) prefs.forceGroups(sidebar, !!query);
      sidebar.querySelectorAll(".mui-block--shell__nav-group").forEach((group) => {
        const rows = Array.from(group.querySelectorAll("li"));
        rows.forEach((row) => {
          row.hidden = !row.textContent.toLocaleLowerCase().includes(query);
        });
        group.hidden = rows.every((row) => row.hidden);
      });
    });
  };
  ui.init();
})();
(function() {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  function focusSearch(target, source) {
    const disclosure = target.closest(".mui-page-header__search-disclosure");
    if (disclosure) disclosure.open = true;
    const dialog = source.closest("dialog");
    if (dialog?.open) {
      dialog.addEventListener("close", () => target.focus(), { once: true });
      dialog.close();
    } else target.focus();
  }
  ui.behaviors["workspace-search"] = (trigger) => {
    trigger.addEventListener("click", () => {
      const target = trigger.closest(".mui-block--shell")?.querySelector('.mui-worklist-header input[type="search"]');
      if (target) focusSearch(target, trigger);
    });
  };
  ui.behaviors["page-search"] = (input) => {
    input.setAttribute("aria-keyshortcuts", "Meta+K Control+K");
  };
  if (!window.__muiPageSearchBound) {
    window.__muiPageSearchBound = true;
    document.addEventListener("keydown", (event) => {
      if (event.defaultPrevented || !(event.metaKey || event.ctrlKey) || event.key.toLowerCase() !== "k") return;
      const scope = event.target.closest(".mui-block--shell") || document;
      const target = scope.querySelector('[data-mui="page-search"], .mui-worklist-header input[type="search"]');
      if (target && (target.getClientRects().length || target.closest(".mui-page-header__search-disclosure")?.getClientRects().length)) {
        event.preventDefault();
        focusSearch(target, event.target);
      }
    });
  }
  ui.behaviors["workspace-demo"] = (demo) => {
    const shell = demo.closest(".mui-block--shell");
    const search = demo.querySelector('.mui-worklist-header input[type="search"]');
    const chips = Array.from(demo.querySelectorAll(".mui-status-chip-group__chip"));
    const table = demo.querySelector("tbody");
    const panel = demo.querySelector(".mui-workspace-example__record");
    const create = demo.querySelector("[data-demo-create]");
    const dialog = create?.closest("dialog");
    const feedback = demo.querySelector('[role="status"]');
    const title = demo.querySelector("[data-demo-title]");
    const subtitle = demo.querySelector(".mui-record-header__subtitle");
    const status = demo.querySelector("[data-demo-status]");
    if (!shell || !search || !table || !panel || !create || !dialog) return;
    let selectedFilter = 0;
    let serial = 2051;
    const filterNames = ["", "Arriving", "Checked in", "Needs review"];
    function rows() {
      return Array.from(table.querySelectorAll("tr"));
    }
    function applyFilter(announce = true) {
      const query = search.value.trim().toLocaleLowerCase();
      let visible = 0;
      rows().forEach((row) => {
        const guest = row.querySelector("[data-guest]");
        const matches = (!selectedFilter || guest.getAttribute("data-status") === filterNames[selectedFilter]) && row.textContent.toLocaleLowerCase().includes(query);
        row.hidden = !matches;
        if (matches) visible++;
      });
      chips.forEach((chip, i) => {
        if (i === selectedFilter) chip.setAttribute("aria-current", "page");
        else chip.removeAttribute("aria-current");
      });
      demo.querySelector(".mui-workspace-example__empty").hidden = visible > 0;
      if (announce) feedback.textContent = visible + (visible === 1 ? " reservation shown." : " reservations shown.");
    }
    function selectRow(row) {
      const guest = row.querySelector("[data-guest]");
      title.textContent = guest.getAttribute("data-guest");
      subtitle.textContent = guest.getAttribute("data-reference") + " \xB7 " + guest.getAttribute("data-room") + " \xB7 " + row.children[3].textContent + " nights";
      status.replaceChildren(row.children[2].firstElementChild.cloneNode(true));
      panel.focus({ preventScroll: true });
      panel.scrollIntoView({ block: "nearest", behavior: "auto" });
      feedback.textContent = "Viewing " + guest.getAttribute("data-guest") + "\u2019s reservation.";
    }
    function updateCounts() {
      chips.forEach((chip, i) => {
        const count = rows().filter((row) => !i || row.querySelector("[data-guest]").getAttribute("data-status") === filterNames[i]).length;
        chip.querySelector(".mui-status-chip-group__count").textContent = String(count);
        chip.querySelector(".mui-sr-only").textContent = " " + count + " items";
      });
      demo.querySelector(".mui-worklist-header__count").textContent = rows().length + " reservations \xB7 Tuesday, 8 September";
    }
    search.addEventListener("input", () => applyFilter());
    search.closest("form").addEventListener("submit", (event) => {
      event.preventDefault();
      applyFilter();
    });
    demo.addEventListener("click", (event) => {
      if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.altKey || event.shiftKey) return;
      const link = event.target.closest("a[href]");
      if (!link) return;
      const href = link.getAttribute("href");
      const filter = chips.findIndex((chip) => chip.getAttribute("href") === href);
      if (filter >= 0) {
        event.preventDefault();
        selectedFilter = filter;
        applyFilter();
        if (!link.closest(".mui-status-chip-group")) search.focus();
      } else if (link.hasAttribute("data-demo-view")) {
        event.preventDefault();
        selectRow(link.closest("tr"));
      } else if (href === "#" + dialog.id && typeof dialog.showModal === "function") {
        event.preventDefault();
        dialog.showModal();
        dialog.addEventListener("close", () => {
          if (link.isConnected) link.focus();
        }, { once: true });
      }
    });
    create.addEventListener("submit", (event) => {
      event.preventDefault();
      if (!create.reportValidity()) return;
      const fields = new FormData(create);
      const name = String(fields.get("guest") || "").trim();
      if (!name) {
        create.elements.guest.focus();
        return;
      }
      const room = String(fields.get("room"));
      const row = table.querySelector("tr").cloneNode(true);
      const guest = row.querySelector("[data-guest]");
      const reference = "RS-" + serial++;
      row.hidden = false;
      guest.textContent = name;
      for (const [key, value] of Object.entries({ guest: name, reference, status: "Arriving", room })) guest.setAttribute("data-" + key, value);
      row.querySelector(".mui-workspace-example__reference").textContent = reference;
      row.children[1].textContent = room;
      row.children[2].firstElementChild.textContent = "Arriving";
      row.children[2].firstElementChild.className = "mui-badge mui-badge--info";
      row.children[3].textContent = String(fields.get("nights"));
      row.querySelector("[data-demo-view]").setAttribute("aria-label", "View " + name + "\u2019s reservation");
      table.append(row);
      selectedFilter = 0;
      search.value = "";
      updateCounts();
      applyFilter(false);
      create.reset();
      dialog.close();
      feedback.textContent = name + " added to this example. Changes stay on this page.";
    });
  };
  ui.init();
})();
(function() {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = /* @__PURE__ */ new WeakSet();
  ui.behaviors["header-search"] = (details) => {
    if (initialized.has(details)) return;
    initialized.add(details);
    const summary = details.querySelector("summary");
    const input = details.querySelector('input[type="search"]');
    const desktop = window.matchMedia?.("(min-width: 64rem)");
    const sync = () => {
      if (!details.isConnected) {
        desktop?.removeEventListener("change", sync);
        return;
      }
      details.open = desktop.matches;
    };
    if (desktop) {
      desktop.addEventListener("change", sync);
      sync();
    }
    document.addEventListener("keydown", (event) => {
      if (!details.isConnected || !(event.metaKey || event.ctrlKey) || event.key.toLowerCase() !== "k") return;
      event.preventDefault();
      details.open = true;
      input?.focus();
    });
    details.addEventListener("toggle", () => {
      if (details.open && details.contains(document.activeElement)) input?.focus();
    });
    details.addEventListener("keydown", (event) => {
      if (event.key !== "Escape" || !details.open) return;
      event.preventDefault();
      if (!desktop?.matches) {
        details.open = false;
        summary.focus();
      }
    });
  };
  ui.init();
})();
(function() {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = /* @__PURE__ */ new WeakSet();
  function ordinal(value) {
    if (!/^\d{4}-\d{2}-\d{2}$/.test(value)) return null;
    const [year, month, day] = value.split("-").map(Number);
    if (!year || month < 1 || month > 12) return null;
    const leap = year % 4 === 0 && (year % 100 !== 0 || year % 400 === 0);
    const months = [31, leap ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if (!day || day > months[month - 1]) return null;
    const previous = year - 1;
    return previous * 365 + Math.floor(previous / 4) - Math.floor(previous / 100) + Math.floor(previous / 400) + months.slice(0, month - 1).reduce((a, b) => a + b, 0) + day;
  }
  function bounded(input, value) {
    const min = ordinal(input.min || ""), max = ordinal(input.max || "");
    return value !== null && (min === null || value >= min) && (max === null || value <= max);
  }
  ui.behaviors["date-range"] = (range) => {
    if (initialized.has(range)) return;
    const start = range.querySelector('[data-range-field="start"]');
    const end = range.querySelector('[data-range-field="end"]');
    const output = range.querySelector("output");
    if (!start || !end || !output) return;
    initialized.add(range);
    function update() {
      const first = ordinal(start.value), last = ordinal(end.value);
      const incomplete = !start.value || !end.value;
      const invalid = !incomplete && (!bounded(start, first) || !bounded(end, last) || last <= first);
      const count = last - first;
      output.textContent = incomplete ? range.getAttribute("data-incomplete") : invalid ? range.getAttribute("data-invalid") : count + " " + range.getAttribute(count === 1 ? "data-night" : "data-nights");
      output.setAttribute("data-range-invalid", String(invalid));
      for (const input of [start, end]) {
        if (invalid) input.setAttribute("aria-invalid", "true");
        else input.removeAttribute("aria-invalid");
      }
      end.setCustomValidity(invalid && !range.disabled && !end.disabled && !end.readOnly ? range.getAttribute("data-invalid") : "");
    }
    range.addEventListener("input", update);
    range.addEventListener("change", update);
    update();
    range.__muiUpdateDateRange = update;
  };
  if (!window.__muiRecordKitResetBound) {
    window.__muiRecordKitResetBound = true;
    document.addEventListener("reset", (event) => {
      setTimeout(() => {
        if (event.defaultPrevented) return;
        document.querySelectorAll('[data-mui="date-range"]').forEach((range) => {
          if (range.querySelector('[data-range-field="start"]')?.form === event.target || range.querySelector('[data-range-field="end"]')?.form === event.target) range.__muiUpdateDateRange?.();
        });
      }, 0);
    });
  }
  ui.behaviors["attention-banner"] = (banner) => {
    if (initialized.has(banner)) return;
    const dismiss = banner.querySelector(".mui-attention-banner__dismiss");
    if (!dismiss) return;
    initialized.add(banner);
    dismiss.hidden = false;
    dismiss.addEventListener("click", () => {
      const controls = Array.from(document.querySelectorAll("a[href], button, input, select, textarea, summary, [tabindex]"));
      const usable = (node) => node && !banner.contains(node) && !node.disabled && node.getAttribute("aria-disabled") !== "true" && node.getClientRects().length;
      const tabbable = (node) => usable(node) && (node.getAttribute("tabindex") === null || Number(node.getAttribute("tabindex")) >= 0);
      const explicit = document.getElementById(dismiss.getAttribute("data-dismiss-focus"));
      const index = controls.indexOf(dismiss);
      const target = usable(explicit) ? explicit : controls.slice(index + 1).find(tabbable) || controls.slice(0, Math.max(0, index)).reverse().find(tabbable);
      banner.hidden = true;
      if (target) target.focus();
      else {
        const parent = banner.parentElement;
        if (parent) {
          const previous = parent.getAttribute("tabindex");
          parent.setAttribute("tabindex", "-1");
          parent.focus();
          if (previous === null) parent.removeAttribute("tabindex");
          else parent.setAttribute("tabindex", previous);
        }
      }
    });
  };
  ui.init();
})();
(function() {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = /* @__PURE__ */ new WeakSet();
  ui.behaviors["workspace-pages"] = (root) => {
    if (initialized.has(root)) return;
    const tabs = Array.from(root.querySelectorAll("[data-workspace-tab]"));
    const panels = Array.from(root.querySelectorAll("[data-workspace-panel]"));
    if (!tabs.length || tabs.length !== panels.length) return;
    initialized.add(root);
    root.querySelector(".lp__workspace-tabs").setAttribute("role", "tablist");
    function activate(index, focus) {
      tabs.forEach((tab, i) => {
        tab.setAttribute("role", "tab");
        tab.setAttribute("aria-controls", panels[i].id);
        tab.setAttribute("aria-selected", String(i === index));
        tab.setAttribute("tabindex", i === index ? "0" : "-1");
        panels[i].setAttribute("role", "tabpanel");
        panels[i].hidden = i !== index;
      });
      const density = panels[index].getAttribute("data-default-density");
      if (density && !ui.densityChosen) ui.setDensity?.(density, false);
      if (focus) tabs[index].focus();
    }
    const initial = tabs.findIndex((tab) => tab.getAttribute("href") === window.location?.hash);
    activate(initial < 0 ? 0 : initial, false);
    root.addEventListener("click", (event) => {
      if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.altKey || event.shiftKey) return;
      const link = event.target.closest("a[href]");
      const index = link ? tabs.findIndex((tab) => tab.getAttribute("href") === link.getAttribute("href")) : -1;
      if (index < 0) return;
      event.preventDefault();
      activate(index, !tabs.includes(link));
    });
    root.addEventListener("keydown", (event) => {
      if (event.metaKey || event.ctrlKey || event.altKey || event.shiftKey) return;
      const index = tabs.indexOf(event.target);
      if (index < 0) return;
      const rtl = getComputedStyle(root).direction === "rtl";
      let next;
      if (event.key === "Home") next = 0;
      else if (event.key === "End") next = tabs.length - 1;
      else if (event.key === "ArrowRight") next = index + (rtl ? -1 : 1);
      else if (event.key === "ArrowLeft") next = index + (rtl ? 1 : -1);
      else return;
      event.preventDefault();
      activate((next + tabs.length) % tabs.length, true);
    });
  };
  ui.init();
})();
