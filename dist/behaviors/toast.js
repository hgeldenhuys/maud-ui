(function () {
  if (!window.MaudUI || !window.MaudUI.behaviors) return;

  window.MaudUI.behaviors["toast"] = function (el) {
    // el is the toast element — wire close button + auto-dismiss
    const closeBtn = el.querySelector(".mui-toast__close");
    if (closeBtn) {
      closeBtn.addEventListener("click", () => dismissToast(el));
    }

    const duration = parseInt(el.getAttribute("data-duration") || "5000", 10);
    if (duration > 0) {
      let timer = setTimeout(() => dismissToast(el), duration);
      el.addEventListener("mouseenter", () => clearTimeout(timer));
    }
  };

  function dismissToast(el) {
    el.classList.add("mui-toast--exit");
    setTimeout(() => el.remove(), 200);
  }

  // Imperative API: window.MaudUI.toast({ title, description, variant, duration_ms })
  window.MaudUI.toast = function (opts) {
    const viewport = document.getElementById("mui-toast-viewport");
    if (!viewport) {
      console.warn("Toast viewport not found. Call maud_ui::primitives::toast::viewport() in your page.");
      return;
    }

    const variant = opts.variant || "info";
    const title = opts.title || "";
    const description = opts.description || "";
    const duration = opts.duration_ms != null ? opts.duration_ms : 5000;

    const toast = document.createElement("div");
    toast.className = "mui-toast mui-toast--" + variant;
    toast.setAttribute("role", variant === "danger" ? "alert" : "status");
    toast.setAttribute("aria-live", variant === "danger" ? "assertive" : "polite");
    toast.setAttribute("data-mui", "toast");
    toast.setAttribute("data-duration", String(duration));

    const titleEl = document.createElement("div");
    titleEl.className = "mui-toast__title";
    titleEl.textContent = title;
    toast.appendChild(titleEl);

    if (description) {
      const descEl = document.createElement("div");
      descEl.className = "mui-toast__description";
      descEl.textContent = description;
      toast.appendChild(descEl);
    }

    const closeEl = document.createElement("button");
    closeEl.type = "button";
    closeEl.className = "mui-toast__close";
    closeEl.setAttribute("aria-label", "Dismiss");
    closeEl.textContent = "×";
    toast.appendChild(closeEl);

    viewport.appendChild(toast);

    // Trigger behavior attachment if MaudUI.init exists
    if (window.MaudUI.init) window.MaudUI.init(viewport);
  };

  // Re-init in case DOMContentLoaded already fired
  if (window.MaudUI.init) window.MaudUI.init();
})();
