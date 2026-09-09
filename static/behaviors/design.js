(function () {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  ui.behaviors["workspace-search"] = trigger => {
    trigger.addEventListener("click", () => trigger.closest('.mui-block--shell')?.querySelector('.mui-worklist-header input[type="search"]')?.focus());
  };
  ui.behaviors["page-search"] = input => {
    input.setAttribute('aria-keyshortcuts', 'Meta+K Control+K');
  };
  if (!window.__muiPageSearchBound) {
    window.__muiPageSearchBound = true;
    document.addEventListener('keydown', event => {
      if (event.defaultPrevented || !(event.metaKey || event.ctrlKey) || event.key.toLowerCase() !== 'k') return;
      const scope = event.target.closest('.mui-block--shell') || document;
      const target = scope.querySelector('[data-mui="page-search"], .mui-worklist-header input[type="search"]');
      if (target && target.getClientRects().length) { event.preventDefault(); target.focus(); }
    });
  }
  ui.behaviors["workspace-demo"] = demo => {
    const shell = demo.closest('.mui-block--shell');
    const search = demo.querySelector('.mui-worklist-header input[type="search"]');
    const chips = Array.from(demo.querySelectorAll('.mui-status-chip-group__chip'));
    const table = demo.querySelector('tbody');
    const panel = demo.querySelector('.mui-workspace-example__record');
    const create = demo.querySelector('[data-demo-create]');
    const dialog = create?.closest('dialog');
    const feedback = demo.querySelector('[role="status"]');
    const title = demo.querySelector('[data-demo-title]');
    const subtitle = demo.querySelector('.mui-record-header__subtitle');
    const status = demo.querySelector('[data-demo-status]');
    if (!shell || !search || !table || !panel || !create || !dialog) return;
    let selectedFilter = 0;
    let serial = 2051;
    const filterNames = ['', 'Arriving', 'Checked in', 'Needs review'];
    function rows() { return Array.from(table.querySelectorAll('tr')); }
    function applyFilter(announce = true) {
      const query = search.value.trim().toLocaleLowerCase();
      let visible = 0;
      rows().forEach(row => {
        const guest = row.querySelector('[data-guest]');
        const matches = (!selectedFilter || guest.getAttribute('data-status') === filterNames[selectedFilter]) && row.textContent.toLocaleLowerCase().includes(query);
        row.hidden = !matches;
        if (matches) visible++;
      });
      chips.forEach((chip, i) => {
        if (i === selectedFilter) chip.setAttribute('aria-current', 'page'); else chip.removeAttribute('aria-current');
      });
      demo.querySelector('.mui-workspace-example__empty').hidden = visible > 0;
      if (announce) feedback.textContent = visible + (visible === 1 ? ' reservation shown.' : ' reservations shown.');
    }
    function selectRow(row) {
      const guest = row.querySelector('[data-guest]');
      title.textContent = guest.getAttribute('data-guest');
      subtitle.textContent = guest.getAttribute('data-reference') + ' · ' + guest.getAttribute('data-room') + ' · ' + row.children[3].textContent + ' nights';
      status.replaceChildren(row.children[2].firstElementChild.cloneNode(true));
      panel.focus({ preventScroll: true });
      panel.scrollIntoView({ block: 'nearest', behavior: 'auto' });
      feedback.textContent = 'Viewing ' + guest.getAttribute('data-guest') + '’s reservation.';
    }
    function updateCounts() {
      chips.forEach((chip, i) => {
        const count = rows().filter(row => !i || row.querySelector('[data-guest]').getAttribute('data-status') === filterNames[i]).length;
        chip.querySelector('.mui-status-chip-group__count').textContent = String(count);
        chip.querySelector('.mui-sr-only').textContent = ' ' + count + ' items';
      });
      demo.querySelector('.mui-worklist-header__count').textContent = rows().length + ' reservations · Tuesday, 8 September';
    }
    search.addEventListener('input', () => applyFilter());
    search.closest('form').addEventListener('submit', event => { event.preventDefault(); applyFilter(); });
    demo.addEventListener('click', event => {
      if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.altKey || event.shiftKey) return;
      const link = event.target.closest('a[href]');
      if (!link) return;
      const href = link.getAttribute('href');
      const filter = chips.findIndex(chip => chip.getAttribute('href') === href);
      if (filter >= 0) { event.preventDefault(); selectedFilter = filter; applyFilter(); if (!link.closest('.mui-status-chip-group')) search.focus(); }
      else if (link.hasAttribute('data-demo-view')) { event.preventDefault(); selectRow(link.closest('tr')); }
      else if (href === '#' + dialog.id && typeof dialog.showModal === 'function') {
        event.preventDefault(); dialog.showModal();
        dialog.addEventListener('close', () => { if (link.isConnected) link.focus(); }, { once: true });
      }
    });
    create.addEventListener('submit', event => {
      event.preventDefault();
      if (!create.reportValidity()) return;
      const fields = new FormData(create);
      const name = String(fields.get('guest') || '').trim();
      if (!name) { create.elements.guest.focus(); return; }
      const room = String(fields.get('room'));
      const row = table.querySelector('tr').cloneNode(true);
      const guest = row.querySelector('[data-guest]');
      const reference = 'RS-' + serial++;
      row.hidden = false;
      guest.textContent = name;
      for (const [key, value] of Object.entries({ guest: name, reference, status: 'Arriving', room })) guest.setAttribute('data-' + key, value);
      row.querySelector('.mui-workspace-example__reference').textContent = reference;
      row.children[1].textContent = room;
      row.children[2].firstElementChild.textContent = 'Arriving';
      row.children[2].firstElementChild.className = 'mui-badge mui-badge--info';
      row.children[3].textContent = String(fields.get('nights'));
      row.querySelector('[data-demo-view]').setAttribute('aria-label', 'View ' + name + '’s reservation');
      table.append(row);
      selectedFilter = 0; search.value = ''; updateCounts(); applyFilter(false);
      create.reset(); dialog.close();
      feedback.textContent = name + ' added to this example. Changes stay on this page.';
    });
  };
  ui.init();
})();
