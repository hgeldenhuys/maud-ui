(function () {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = new WeakSet();
  ui.behaviors['workspace-pages'] = root => {
    if (initialized.has(root)) return;
    const tabs = Array.from(root.querySelectorAll('[data-workspace-tab]'));
    const panels = Array.from(root.querySelectorAll('[data-workspace-panel]'));
    if (!tabs.length || tabs.length !== panels.length) return;
    initialized.add(root);
    root.querySelector('.lp__workspace-tabs').setAttribute('role', 'tablist');
    function activate(index, focus) {
      tabs.forEach((tab, i) => {
        tab.setAttribute('role', 'tab'); tab.setAttribute('aria-controls', panels[i].id);
        tab.setAttribute('aria-selected', String(i === index)); tab.setAttribute('tabindex', i === index ? '0' : '-1');
        panels[i].setAttribute('role', 'tabpanel'); panels[i].hidden = i !== index;
      });
      if (focus) tabs[index].focus();
    }
    const initial = tabs.findIndex(tab => tab.getAttribute('href') === window.location?.hash);
    activate(initial < 0 ? 0 : initial, false);
    root.addEventListener('click', event => {
      if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.altKey || event.shiftKey) return;
      const link = event.target.closest('a[href]');
      const index = link ? tabs.findIndex(tab => tab.getAttribute('href') === link.getAttribute('href')) : -1;
      if (index < 0) return;
      event.preventDefault(); activate(index, !tabs.includes(link));
    });
    root.addEventListener('keydown', event => {
      if (event.metaKey || event.ctrlKey || event.altKey || event.shiftKey) return;
      const index = tabs.indexOf(event.target); if (index < 0) return;
      const rtl = getComputedStyle(root).direction === 'rtl';
      let next;
      if (event.key === 'Home') next = 0;
      else if (event.key === 'End') next = tabs.length - 1;
      else if (event.key === 'ArrowRight') next = index + (rtl ? -1 : 1);
      else if (event.key === 'ArrowLeft') next = index + (rtl ? 1 : -1);
      else return;
      event.preventDefault(); activate((next + tabs.length) % tabs.length, true);
    });
  };
  ui.init();
})();
