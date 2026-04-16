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
