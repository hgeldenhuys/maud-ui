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
