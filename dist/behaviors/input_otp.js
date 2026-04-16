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
