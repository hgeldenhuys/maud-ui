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
