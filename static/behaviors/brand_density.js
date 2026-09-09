(function () {
  'use strict';
  const ui = window.MaudUI, data = ui?.brandData;
  if (!data) return;
  const root = document.documentElement;
  const modes = ['compact', 'comfortable', 'spacious'];
  const read = key => { try { return localStorage.getItem(key); } catch { return null; } };
  const write = (key, value) => { try { localStorage.setItem(key, value); } catch {} };
  let values = data.tokens.map(token => token.default);
  let current = '';
  const savedDensity = read('mui-density');
  ui.densityChosen = modes.includes(savedDensity);
  function refresh() {
    const output = document.getElementById('mui-theme-export');
    if (output) output.textContent = buildCss();
    document.querySelectorAll('[data-brand-token]').forEach(input => {
      const index = data.tokens.findIndex(token => token.name === input.getAttribute('data-brand-token'));
      if (index >= 0 && input !== document.activeElement) input.value = values[index];
    });
    document.querySelectorAll('[data-brand-select]').forEach(select => { select.value = current; });
  }
  function buildCss() {
    const radii = [];
    document.querySelectorAll('[data-brand-radius-sample]').forEach(sample => {
      const key = sample.getAttribute('data-brand-radius-sample');
      const value = getComputedStyle(sample).borderTopLeftRadius;
      if (!value) return;
      radii.push(key + ' = ' + value);
      const label = document.querySelector('[data-brand-radius-value="' + key + '"]');
      if (label) label.textContent = value;
    });
    const formula = 'Radius: sm = brand * 0.5; md = brand; lg = min(brand * 1.5, 12px). Controls = min(sm, 8px).';
    return '/* ' + formula + (radii.length ? ' Resolved: ' + radii.join('; ') + '.' : '') + ' */\n:root {\n' + data.tokens.map((token, index) => '  ' + token.name + ': ' + values[index] + ';').join('\n') + '\n}\n';
  }
  function valid(token, value) {
    if (typeof value !== 'string' || !value.trim()) return false;
    if (token.kind === 'density') return ['0', '1', '2'].includes(value);
    if (token.kind === 'color') return /^#[0-9a-f]{6}$/i.test(value);
    if (token.name === '--mui-brand-radius') return value === '0' || (window.CSS?.supports('width', value) && window.CSS.supports('width', 'calc(' + value + ' * 0.5)'));
    const property = token.name.includes('font-') ? 'font-family' : token.name.endsWith('mask') ? 'mask-image' : token.name.includes('radius') ? 'border-radius' : 'width';
    return window.CSS?.supports(property, value) ?? false;
  }
  function persist() { write('mui-brand', JSON.stringify({ current, values })); }
  function setDensity(mode, persistChoice = true) {
    if (!modes.includes(mode)) return;
    root.setAttribute('data-density', mode);
    document.querySelectorAll('[data-mui-density-scope]').forEach(scope => scope.setAttribute('data-mui-density-scope', mode));
    document.querySelectorAll('[data-mui="density-control"]').forEach(select => { select.value = mode; });
    values[5] = String(modes.indexOf(mode));
    if (persistChoice) { ui.densityChosen = true; write('mui-density', mode); persist(); }
    refresh();
  }
  ui.setDensity = setDensity;
  function apply(next, name, save = true) {
    if (next.length !== data.tokens.length || !next.every((v, i) => valid(data.tokens[i], v))) return false;
    // Legacy theme overrides are an advanced preview, separate from the nine-token brand.
    let old = {};
    try { old = JSON.parse(read('mui-theme-overrides') || '{}'); } catch {}
    Object.keys(old).forEach(key => { if (key.startsWith('mui-') && !key.startsWith('mui-brand-')) root.style.removeProperty('--' + key); });
    values = [...next]; current = name;
    root.removeAttribute('data-brand');
    data.tokens.forEach((token, i) => root.style.setProperty(token.name, values[i]));
    const brand = data.presets.find(preset => preset.key === name);
    if (brand) document.querySelectorAll('[data-brand-live]').forEach(preview => {
      preview.querySelector('.mui-brand-mark__wordmark').textContent = brand.wordmark;
      preview.querySelector('.mui-brand-mark__tagline').textContent = brand.tagline;
    });
    setDensity(modes[Number(values[5])], false);
    if (save) persist();
    document.dispatchEvent(new CustomEvent('mui:brand-change'));
    refresh(); return true;
  }
  ui.brand = { buildCss, apply, valid };
  try {
    const saved = JSON.parse(read('mui-brand') || 'null');
    if (Array.isArray(saved?.values)) apply(saved.values, saved.current, false);
  } catch {}
  if (ui.densityChosen) setDensity(savedDensity, false);
  const initialized = new WeakSet();
  ui.behaviors['density-control'] = select => {
    if (initialized.has(select)) return; initialized.add(select);
    select.value = root.getAttribute('data-density') || 'comfortable';
    select.addEventListener('change', () => setDensity(select.value));
  };
  ui.behaviors['brand-customizer'] = editor => {
    if (initialized.has(editor)) return; initialized.add(editor);
    let saved;
    try { saved = JSON.parse(read('mui-brand') || 'null'); } catch {}
    if (!saved || !Array.isArray(saved.values) || !apply(saved.values, saved.current, false)) apply(data.presets[0].values, data.presets[0].key, false);
    if (ui.densityChosen) setDensity(savedDensity, false);
    editor.querySelector('[data-brand-select]').addEventListener('change', event => {
      const preset = data.presets.find(preset => preset.key === event.target.value);
      if (preset) { apply(preset.values, preset.key); write('mui-density', modes[Number(preset.values[5])]); ui.densityChosen = true; }
    });
    editor.addEventListener('input', event => {
      const index = data.tokens.findIndex(token => token.name === event.target.getAttribute('data-brand-token'));
      if (index < 0) return;
      const next = [...values]; next[index] = event.target.value;
      const accepted = valid(data.tokens[index], next[index]);
      event.target.setAttribute('aria-invalid', String(!accepted));
      if (accepted) { apply(next, ''); if (index === 5) setDensity(modes[Number(next[index])]); }
    });
    editor.querySelector('[data-brand-example-action]').addEventListener('click', () => { editor.querySelector('[data-brand-status]').textContent = 'Action received. This preview uses your current brand and density.'; });
  };
  ui.init();
})();
