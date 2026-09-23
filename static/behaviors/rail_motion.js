// Sidebar collapse with motion (0.19.5). Collapsing a sidebar to its icon rail used to
// swap layouts in one frame: the column snapped from 240px to 64px and every label
// vanished at once. When the browser supports view transitions (and the reader has not
// asked for reduced motion), the change now runs as one: the sidebar's width glides,
// its labels clip away as it narrows, and the content slides with it. Elsewhere the
// swap stays instant, exactly as before.
(function () {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  let serial = 0;
  function reduced() {
    return window.matchMedia("(prefers-reduced-motion: reduce)").matches;
  }
  // parts: the elements whose box changes (the sidebar and the column beside it).
  ui.railTransition = function (parts, change) {
    const live = parts.filter(Boolean).filter(el => el.getClientRects().length);
    if (typeof document.startViewTransition !== "function" || reduced() || !live.length) { change(); return; }
    const id = ++serial;
    live.forEach((el, i) => {
      el.style.viewTransitionName = "mui-rail-" + id + "-" + i;
      // The first part is the sidebar (labels cross-fade away); the rest are content that
      // reflows, where a cross-fade shows old and new text at once, so they only slide.
      el.style.viewTransitionClass = i === 0 ? "mui-rail" : "mui-rail mui-rail-slide";
    });
    const clear = () => live.forEach(el => { el.style.viewTransitionName = ""; el.style.viewTransitionClass = ""; });
    try {
      // Typed, so the stylesheet can hold the rest of the page still for THIS transition
      // only (an app's own view transitions are untouched). Older engines take no types.
      let transition;
      try { transition = document.startViewTransition({ update: change, types: ["mui-rail"] }); }
      catch (e) { transition = document.startViewTransition(change); }
      transition.finished.then(clear, clear);
    } catch (error) { clear(); change(); }
  };
})();
