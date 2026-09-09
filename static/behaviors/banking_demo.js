(function () {
  'use strict';
  const ui = window.MaudUI;
  if (!ui) return;
  const initialized = new WeakSet();
  ui.behaviors['banking-demo'] = root => {
    if (initialized.has(root)) return; initialized.add(root);
    let trigger;
    root.addEventListener('click', event => {
      const link = event.target.closest('[data-bank-review]');
      if (link && !event.metaKey && !event.ctrlKey && !event.shiftKey && !event.altKey && event.button === 0) {
        const panel = document.getElementById(link.getAttribute('href').slice(1));
        if (panel && root.contains(panel)) { event.preventDefault(); trigger = link; panel.open = true; panel.querySelector('summary').focus(); }
      }
      const close = event.target.closest('[data-bank-close]');
      if (close) { const panel = close.closest('details'); panel.open = false; if (trigger?.isConnected) trigger.focus(); else panel.querySelector('summary').focus(); }
    });
    const shell = root.closest('.mui-block--shell');
    const search = shell?.querySelector('.mui-page-header input[type="search"]');
    const transactions = root.querySelector('.mui-data-table__search');
    search?.closest('form')?.addEventListener('submit', event => {
      event.preventDefault(); transactions.value = search.value; transactions.dispatchEvent(new Event('input', { bubbles: true }));
      search.closest('details').open = false; transactions.focus();
    });
  };
  ui.init();
})();
