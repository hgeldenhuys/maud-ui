(function () {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const groups = new WeakMap();
  const read = key => { try { return localStorage.getItem(key); } catch { return null; } };
  const write = (key, value) => { try { localStorage.setItem(key, value); } catch {} };
  function initGroup(group) {
    if (group.querySelector('[aria-current="page"]')) group.setAttribute('data-contains-current', 'true');
    else group.removeAttribute('data-contains-current');
    if (groups.has(group)) return groups.get(group);
    const key = "mui-nav-group:" + (group.getAttribute("data-nav-key") || group.id);
    const saved = read(key);
    const state = { desired: saved === "closed" ? false : saved === "open" ? true : group.open, forced: false };
    // Keep a selected destination discoverable on first load of a route.
    if (saved === null && group.querySelector('[aria-current="page"]')) state.desired = true;
    groups.set(group, state);
    group.open = state.desired;
    group.addEventListener("toggle", () => {
      if (state.forced) return;
      state.desired = group.open;
      write(key, group.open ? "open" : "closed");
    });
    return state;
  }
  function forceGroups(root, forced) {
    root.querySelectorAll('[data-mui="nav-group"]').forEach(group => {
      const state = initGroup(group);
      state.forced = forced;
      group.open = forced || state.desired;
    });
  }
  ui.navigation = { read, write, forceGroups };
  ui.behaviors["nav-group"] = initGroup;

  function updateSidebar(sidebar, collapsed) {
    const desktop = window.matchMedia("(min-width: 64rem)").matches;
    const mode = sidebar.getAttribute("data-collapsible");
    if (mode === "none") collapsed = false;
    sidebar.setAttribute("data-state", collapsed ? "collapsed" : "expanded");
    write("mui-sidebar:" + sidebar.id, collapsed ? "collapsed" : "expanded");
    forceGroups(sidebar, desktop && mode === "icon" && collapsed);
    document.querySelectorAll('[data-mui="sidebar-trigger"]').forEach(trigger => {
      if (trigger.getAttribute("data-target") === sidebar.id) {
        trigger.setAttribute("aria-controls", sidebar.id);
        trigger.setAttribute("aria-expanded", String(!window.matchMedia("(max-width: 59.99rem)").matches && (!collapsed || (!desktop && mode === "icon"))));
      }
    });
  }
  function toggle(sidebar, trigger) {
    if (!sidebar) return;
    const phone = window.matchMedia("(max-width: 59.99rem)").matches;
    if (phone) {
      const panel = document.getElementById(sidebar.id + "-drawer");
      if (!panel || typeof panel.showModal !== "function" || panel.open) return;
      panel.dispatchEvent(new CustomEvent("mui:navigation-open"));
      panel.showModal();
      if (trigger) {
        trigger.setAttribute("aria-expanded", "true");
        panel.addEventListener("close", () => {
          trigger.setAttribute("aria-expanded", "false");
          if (trigger.isConnected && trigger.getClientRects().length) trigger.focus();
        }, { once: true });
      }
    } else updateSidebar(sidebar, sidebar.getAttribute("data-state") !== "collapsed");
  }
  ui.behaviors["sidebar"] = sidebar => {
    const panel = document.getElementById(sidebar.id + "-drawer");
    const provider = sidebar.closest('.mui-sidebar-provider');
    if (!panel || !provider || typeof panel.showModal !== "function") return;
    const home = panel.parentNode;
    sidebar.setAttribute("data-navigation-ready", "");
    provider.setAttribute("data-navigation-ready", "");
    panel.removeAttribute("open");
    const restore = () => home.insertBefore(sidebar, panel);
    restore();
    sidebar.querySelectorAll('.mui-sidebar__menu-button').forEach(row => {
      if (!row.hasAttribute('aria-label')) row.setAttribute('aria-label', row.textContent.trim());
      if (!row.hasAttribute('title')) row.setAttribute('title', row.getAttribute('aria-label'));
    });
    const saved = read("mui-sidebar:" + sidebar.id);
    if (saved === "collapsed" || saved === "expanded") sidebar.setAttribute("data-state", saved);
    const sync = () => {
      const phone = window.matchMedia("(max-width: 59.99rem)").matches;
      if (!phone && panel.open) { panel.close(); restore(); }
      updateSidebar(sidebar, sidebar.getAttribute("data-state") === "collapsed");
    };
    panel.addEventListener("mui:navigation-open", () => { forceGroups(sidebar, false); panel.append(sidebar); });
    panel.addEventListener("close", restore);
    panel.addEventListener("click", event => { if (event.target === panel || event.target.closest('a[href]')) panel.close(); });
    const media = [window.matchMedia("(max-width: 59.99rem)"), window.matchMedia("(min-width: 64rem)")];
    const resize = () => {
      if (!provider.isConnected) { media.forEach(m => m.removeEventListener("change", resize)); return; }
      sync();
    };
    media.forEach(m => m.addEventListener("change", resize));
    sync();
  };
  ui.behaviors["sidebar-trigger"] = trigger => {
    const sidebar = document.getElementById(trigger.getAttribute("data-target"));
    trigger.setAttribute("aria-controls", trigger.getAttribute("data-target"));
    trigger.setAttribute("aria-expanded", String(!!sidebar && sidebar.getAttribute("data-state") !== "collapsed" && !window.matchMedia("(max-width: 59.99rem)").matches));
    trigger.addEventListener("click", () => toggle(sidebar, trigger));
  };
  ui.behaviors["sidebar-rail"] = rail => rail.addEventListener("click", () => toggle(rail.closest('[data-mui="sidebar"]')));
  if (!window.__muiSidebarShortcutBound) {
    window.__muiSidebarShortcutBound = true;
    document.addEventListener("keydown", event => {
      if (!(event.metaKey || event.ctrlKey) || event.key.toLowerCase() !== "b" || event.defaultPrevented) return;
      if (event.target.closest('input, textarea, select, [contenteditable="true"]')) return;
      const provider = event.target.closest('.mui-sidebar-provider');
      const sidebar = provider ? provider.querySelector('[data-mui="sidebar"]') : document.querySelector('[data-mui="sidebar"]');
      if (sidebar) { event.preventDefault(); toggle(sidebar, document.querySelector('[data-mui="sidebar-trigger"]')); }
    });
  }
  ui.init();
})();
