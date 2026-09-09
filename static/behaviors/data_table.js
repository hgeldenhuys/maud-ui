(function () {
  "use strict";
  const ui = window.MaudUI;
  if (!ui) return;
  // Exact decimal comparison: no IEEE-754 rounding, including values beyond 2^53.
  ui.compareDecimal = (left, right) => {
    function parse(text) {
      const value = text.trim().replace('−', '-');
      const match = /^([+-]?)(?:[$€£¥])?((?:\d{1,3}(?:,\d{3})+|\d+))(?:\.(\d+))?$/.exec(value);
      if (!match) return null;
      const decimals = match[3] || '';
      return { value: BigInt((match[1] === '-' ? '-' : '') + match[2].replaceAll(',', '') + decimals), scale: decimals.length };
    }
    const a = parse(left), b = parse(right);
    if (!a || !b) return null;
    const scale = Math.max(a.scale, b.scale);
    const l = a.value * 10n ** BigInt(scale - a.scale), r = b.value * 10n ** BigInt(scale - b.scale);
    return l < r ? -1 : l > r ? 1 : 0;
  };
  ui.behaviors["data-table"] = root => {
    const pageSize = Math.max(1, parseInt(root.getAttribute('data-page-size') || '5', 10) || 5);
    const body = root.querySelector('.mui-data-table__body');
    if (!body) return;
    const info = root.querySelector('.mui-data-table__info');
    const previous = root.querySelector('[data-action="prev"]');
    const next = root.querySelector('[data-action="next"]');
    const search = root.querySelector('.mui-data-table__search');
    const headers = Array.from(root.querySelectorAll('.mui-data-table__th[data-key]'));
    // Keep original nodes: replacing innerHTML loses rich fields, alignment and selection.
    const rows = Array.from(body.querySelectorAll('tr')).map(node => {
      let values;
      try { values = JSON.parse(node.getAttribute('data-row-data')); } catch {}
      if (!Array.isArray(values)) values = Array.from(node.children).filter(cell => !cell.classList.contains('mui-data-table__td--select')).map(cell => cell.textContent.trim());
      return { node, values: values.map(String) };
    });
    let page = 0, column = -1, direction = 0;
    function render() {
      const query = (search?.value || '').trim().toLocaleLowerCase();
      const filtered = rows.filter(row => row.values.some(value => value.toLocaleLowerCase().includes(query)) || !query);
      if (direction && column >= 0) filtered.sort((a, b) => {
        const left = a.values[column] || '', right = b.values[column] || '';
        return direction * (ui.compareDecimal(left, right) ?? left.localeCompare(right));
      });
      page = Math.max(0, Math.min(page, Math.ceil(filtered.length / pageSize) - 1));
      rows.forEach(row => { row.node.hidden = true; });
      filtered.forEach((row, index) => { body.append(row.node); row.node.hidden = index < page * pageSize || index >= (page + 1) * pageSize; });
      if (info) info.textContent = filtered.length ? `Showing ${page * pageSize + 1}-${Math.min((page + 1) * pageSize, filtered.length)} of ${filtered.length}` : 'No results';
      if (previous) previous.disabled = page === 0;
      if (next) next.disabled = (page + 1) * pageSize >= filtered.length;
      headers.forEach((header, index) => {
        if (header.getAttribute('data-sortable') !== 'true') return;
        header.setAttribute('aria-sort', index === column && direction ? direction === 1 ? 'ascending' : 'descending' : 'none');
        if (index === column && direction) header.setAttribute('data-sort-dir', direction === 1 ? 'asc' : 'desc'); else header.removeAttribute('data-sort-dir');
      });
    }
    headers.forEach((header, index) => {
      if (header.getAttribute('data-sortable') !== 'true') return;
      const sort = () => { direction = column === index ? direction === 1 ? -1 : direction === -1 ? 0 : 1 : 1; column = index; page = 0; render(); };
      header.addEventListener('click', sort);
      header.addEventListener('keydown', event => { if (event.key === 'Enter' || event.key === ' ') { event.preventDefault(); sort(); } });
    });
    search?.addEventListener('input', () => { page = 0; render(); });
    previous?.addEventListener('click', () => { page--; render(); });
    next?.addEventListener('click', () => { page++; render(); });
    render();
  };
  ui.init();
})();
