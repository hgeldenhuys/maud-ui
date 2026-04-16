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
