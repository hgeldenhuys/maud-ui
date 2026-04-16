(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["carousel"] = function (root) {
    var container = root.querySelector(".mui-carousel__container");
    if (!container) return;

    var slides = container.querySelectorAll(".mui-carousel__slide");
    var prev = root.querySelector(".mui-carousel__prev");
    var next = root.querySelector(".mui-carousel__next");
    var dots = root.querySelectorAll(".mui-carousel__dot");
    var currentIndex = 0;
    var total = slides.length;
    var loopEnabled = root.getAttribute("data-loop") === "true";
    var autoPlay = root.getAttribute("data-autoplay") === "true";
    var autoPlayTimer = null;

    function updateDisabled() {
      if (loopEnabled) return;
      if (prev) prev.disabled = currentIndex === 0;
      if (next) next.disabled = currentIndex === total - 1;
    }

    function goTo(index) {
      if (index < 0) {
        index = loopEnabled ? total - 1 : 0;
      }
      if (index >= total) {
        index = loopEnabled ? 0 : total - 1;
      }
      currentIndex = index;
      container.style.transform = "translateX(-" + (currentIndex * 100) + "%)";

      // Update dots
      for (var i = 0; i < dots.length; i++) {
        var isActive = i === currentIndex;
        dots[i].setAttribute("aria-selected", isActive ? "true" : "false");
        if (isActive) {
          dots[i].classList.add("mui-carousel__dot--active");
        } else {
          dots[i].classList.remove("mui-carousel__dot--active");
        }
      }

      // Update slide aria-hidden
      for (var j = 0; j < slides.length; j++) {
        slides[j].setAttribute("aria-hidden", j === currentIndex ? "false" : "true");
      }

      updateDisabled();
    }

    // Arrow clicks
    if (prev) {
      prev.addEventListener("click", function () {
        goTo(currentIndex - 1);
        resetAutoPlay();
      });
    }
    if (next) {
      next.addEventListener("click", function () {
        goTo(currentIndex + 1);
        resetAutoPlay();
      });
    }

    // Dot clicks
    for (var i = 0; i < dots.length; i++) {
      (function (idx) {
        dots[idx].addEventListener("click", function () {
          goTo(idx);
          resetAutoPlay();
        });
      })(i);
    }

    // Keyboard: ArrowLeft/Right on the carousel root
    root.addEventListener("keydown", function (e) {
      if (e.key === "ArrowLeft") {
        e.preventDefault();
        goTo(currentIndex - 1);
        resetAutoPlay();
      } else if (e.key === "ArrowRight") {
        e.preventDefault();
        goTo(currentIndex + 1);
        resetAutoPlay();
      }
    });

    // Auto-play
    function startAutoPlay() {
      if (!autoPlay) return;
      autoPlayTimer = setInterval(function () {
        goTo(currentIndex + 1);
      }, 4000);
    }

    function resetAutoPlay() {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
      startAutoPlay();
    }

    // Pause auto-play on hover/focus
    root.addEventListener("mouseenter", function () {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
    });
    root.addEventListener("mouseleave", function () {
      startAutoPlay();
    });
    root.addEventListener("focusin", function () {
      if (autoPlayTimer) {
        clearInterval(autoPlayTimer);
        autoPlayTimer = null;
      }
    });
    root.addEventListener("focusout", function () {
      startAutoPlay();
    });

    // Initialize
    goTo(0);
    startAutoPlay();
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
