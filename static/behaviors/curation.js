(function () {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = new WeakSet();

  ui.behaviors["status-chip-group"] = function (group) {
    if (initialized.has(group)) return;
    initialized.add(group);
    group.addEventListener("keydown", function (event) {
      if (event.altKey || event.metaKey || event.ctrlKey || event.shiftKey) return;
      const links = Array.from(group.querySelectorAll("a[href]"));
      const index = links.indexOf(event.target);
      if (index < 0) return;
      const rtl = getComputedStyle(group).direction === "rtl";
      let next;
      if (event.key === "Home") next = 0;
      else if (event.key === "End") next = links.length - 1;
      else if (event.key === "ArrowRight") next = index + (rtl ? -1 : 1);
      else if (event.key === "ArrowLeft") next = index + (rtl ? 1 : -1);
      else if (event.key === "ArrowDown") next = index + 1;
      else if (event.key === "ArrowUp") next = index - 1;
      else return;
      event.preventDefault();
      links[(next + links.length) % links.length].focus();
    });
  };

  ui.behaviors["navigation-trigger"] = function (trigger) {
    if (initialized.has(trigger)) return;
    initialized.add(trigger);
    trigger.setAttribute("aria-expanded", "false");
    trigger.addEventListener("click", function (event) {
      if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return;
      const dialog = document.getElementById(trigger.getAttribute("aria-controls"));
      if (!dialog || typeof dialog.showModal !== "function") return;
      event.preventDefault();
      if (dialog.open) return;
      dialog.dispatchEvent(new CustomEvent("mui:navigation-open"));
      dialog.showModal();
      trigger.setAttribute("aria-expanded", "true");
      dialog.addEventListener("close", function () {
        trigger.setAttribute("aria-expanded", "false");
        if (trigger.isConnected && trigger.getClientRects().length) trigger.focus();
      }, { once: true });
    });
  };

  ui.behaviors["shell-navigation"] = function (shell) {
    if (initialized.has(shell)) return;
    initialized.add(shell);
    const sidebar = shell.querySelector(".mui-block--shell__sidebar");
    const dialog = shell.querySelector(".mui-navigation-dialog");
    const main = shell.querySelector(".mui-block--shell__main");
    if (!sidebar || !dialog || typeof dialog.showModal !== "function") return;
    shell.setAttribute("data-mui-navigation-ready", "");
    // A history snapshot may contain the open drawer; restore one canonical sidebar.
    dialog.removeAttribute("open");
    shell.insertBefore(sidebar, main);
    const restore = () => shell.insertBefore(sidebar, main);
    dialog.addEventListener("mui:navigation-open", () => dialog.append(sidebar));
    dialog.addEventListener("close", restore);
    dialog.addEventListener("click", function (event) {
      if (event.target === dialog || event.target.closest("a[href]")) dialog.close();
    });
    const media = window.matchMedia("(max-width: 59.99rem)");
    const resize = () => {
      if (!shell.isConnected) { media.removeEventListener("change", resize); return; }
      if (!media.matches && dialog.open) {
        const focused = sidebar.contains(document.activeElement) ? document.activeElement : null;
        dialog.close();
        restore();
        if (focused) focused.focus();
      }
    };
    media.addEventListener("change", resize);
    const search = sidebar.querySelector("[data-mui-nav-search]");
    if (search) search.addEventListener("input", function () {
      const query = search.value.trim().toLocaleLowerCase();
      sidebar.querySelectorAll(".mui-block--shell__nav-group").forEach(group => {
        const rows = Array.from(group.querySelectorAll("li"));
        rows.forEach(row => { row.hidden = !row.textContent.toLocaleLowerCase().includes(query); });
        group.hidden = rows.every(row => row.hidden);
      });
    });
  };
  ui.init();
})();
