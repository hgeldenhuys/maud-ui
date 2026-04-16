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
