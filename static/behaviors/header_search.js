(function () {
  'use strict';
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = new WeakSet();
  ui.behaviors['header-search'] = details => {
    if (initialized.has(details)) return;
    initialized.add(details);
    const summary = details.querySelector('summary');
    const input = details.querySelector('input[type="search"]');
    details.addEventListener('toggle', () => { if (details.open && details.contains(document.activeElement)) input?.focus(); });
    details.addEventListener('keydown', event => {
      if (event.key === 'Escape' && details.open) { event.preventDefault(); details.open = false; summary.focus(); }
    });
  };
  ui.init();
})();
