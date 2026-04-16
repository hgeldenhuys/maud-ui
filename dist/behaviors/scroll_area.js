(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["scroll-area"] = function (root) {
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
      var thumbHeight = Math.max(ratio * 100, 10); // min 10% height
      var scrollRatio = viewport.scrollTop / (viewport.scrollHeight - viewport.clientHeight);
      var thumbTop = scrollRatio * (100 - thumbHeight);
      thumb.style.height = thumbHeight + "%";
      thumb.style.top = thumbTop + "%";

      // Show scrollbar, hide after 1s idle
      scrollbar.classList.add("mui-scroll-area__scrollbar--visible");
      clearTimeout(hideTimer);
      hideTimer = setTimeout(function () {
        scrollbar.classList.remove("mui-scroll-area__scrollbar--visible");
      }, 1000);
    }

    viewport.addEventListener("scroll", updateThumb);

    // Thumb drag
    var dragging = false;
    var startY = 0;
    var startScroll = 0;

    thumb.addEventListener("mousedown", function (e) {
      e.preventDefault();
      dragging = true;
      startY = e.clientY;
      startScroll = viewport.scrollTop;
      document.addEventListener("mousemove", onDrag);
      document.addEventListener(
        "mouseup",
        function () {
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
      viewport.scrollTop = startScroll + (delta / trackHeight) * scrollable;
    }

    // Initial update
    updateThumb();
    // Observe content changes
    if (window.ResizeObserver) {
      new ResizeObserver(updateThumb).observe(viewport);
    }
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
