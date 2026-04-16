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
