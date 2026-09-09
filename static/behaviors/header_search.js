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
    const desktop = window.matchMedia?.('(min-width: 64rem)');
    const sync = () => {
      if (!details.isConnected) { desktop?.removeEventListener('change', sync); return; }
      details.open = desktop.matches;
    };
    if (desktop) { desktop.addEventListener('change', sync); sync(); }
    document.addEventListener('keydown', event => {
      if (!details.isConnected || !(event.metaKey || event.ctrlKey) || event.key.toLowerCase() !== 'k') return;
      event.preventDefault(); details.open = true; input?.focus();
    });
    details.addEventListener('toggle', () => { if (details.open && details.contains(document.activeElement)) input?.focus(); });
    details.addEventListener('keydown', event => {
      if (event.key !== 'Escape' || !details.open) return;
      // A search input's native Escape clears its query. Keep that value at
      // both breakpoints; only the phone disclosure needs to close.
      event.preventDefault();
      if (!desktop?.matches) { details.open = false; summary.focus(); }
    });
  };
  ui.init();
})();
