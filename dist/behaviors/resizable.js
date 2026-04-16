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
