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
