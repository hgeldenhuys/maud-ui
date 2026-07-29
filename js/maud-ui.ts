// ⚠ NOT SHIPPED — THIS FILE IS NOT BUILT AND NOT SERVED.
//
// `js/build.mjs` builds the runtime from `dist/maud-ui.js.bak`, never from this
// file. Editing here changes nothing a browser will ever run, and the two have
// already diverged (this copy has no `htmx:oobAfterSwap` / `htmx:historyRestore`
// handling and its `init()` misses the swapped node itself).
//
// It is kept only as the typed reference for a future migration. Until
// `build.mjs` reads it, treat `dist/maud-ui.js.bak` as the source of truth.
//
// maud-ui runtime — behaviors register into MaudUI.behaviors and self-initialize
// via `data-mui="<name>"` attributes. Wave 2/3 agents add behaviors here.

type BehaviorFn = (el: Element) => void;

interface MaudUIInterface {
  behaviors: Record<string, BehaviorFn>;
  init(root?: Element): void;
}

declare global {
  interface Window { MaudUI: MaudUIInterface; }
}

const MaudUI: MaudUIInterface = {
  behaviors: {},
  init(root: Element = document.body) {
    root.querySelectorAll<HTMLElement>("[data-mui]").forEach((el) => {
      if (el.hasAttribute("data-mui-init")) return;
      const type = el.getAttribute("data-mui");
      if (type && MaudUI.behaviors[type]) {
        MaudUI.behaviors[type](el);
        el.setAttribute("data-mui-init", "");
      }
    });
  },
};

window.MaudUI = MaudUI;

document.addEventListener("DOMContentLoaded", () => MaudUI.init());
document.addEventListener("htmx:afterSwap", (e) => {
  if (e.target instanceof Element) MaudUI.init(e.target);
});
