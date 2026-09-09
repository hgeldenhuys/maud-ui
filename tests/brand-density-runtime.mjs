// Run the actual shipped bundles with DOM fixtures. No browser is launched.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';

function fixture(file, initial = {}) {
  let document;
  class Node {
    constructor(attrs = {}) { this.attrs = { ...attrs }; this.nodes = {}; this.listeners = {}; this.children = []; this.hidden = false; this.nodeType = 1; this.isConnected = true; this.value = ''; this.properties = new Map(); this.style = { setProperty: (k,v) => this.properties.set(k,v), removeProperty: k => this.properties.delete(k) }; }
    hasAttribute(name) { return name in this.attrs; }
    getAttribute(name) { return this.attrs[name] ?? null; }
    setAttribute(name, value) { this.attrs[name] = String(value); }
    removeAttribute(name) { delete this.attrs[name]; }
    querySelectorAll(selector) { return this.nodes[selector] || []; }
    querySelector(selector) { return this.querySelectorAll(selector)[0] || null; }
    addEventListener(type, fn) { (this.listeners[type] ||= []).push(fn); }
    removeEventListener(type, fn) { this.listeners[type] = (this.listeners[type] || []).filter(f => f !== fn); }
    dispatchEvent(event) { event.target ||= this; event.preventDefault ||= () => { event.defaultPrevented = true; }; for (const fn of this.listeners[event.type] || []) fn(event); return event; }
    closest(selector) { return this.parents?.[selector] || null; }
    focus() { document.activeElement = this; }
    contains(child) { return child === this || this.children.includes(child); }
    getClientRects() { return this.hidden ? [] : [{}]; }
  }
  document = new Node(); document.body = new Node(); document.documentElement = new Node();
  const ids = new Map(); document.getElementById = id => ids.get(id) || null;
  const storage = new Map(Object.entries(initial));
  const window = new Node(); window.matchMedia = () => new Node(); window.CSS = { supports: (_, v) => !/[{}]/.test(v) && v !== 'invalid' };
  vm.runInNewContext(readFileSync(file, 'utf8'), { window, document, Element: Node, console, setTimeout, clearTimeout, getComputedStyle: () => ({ direction: 'ltr' }), localStorage: { getItem: k => storage.get(k) ?? null, setItem: (k,v) => storage.set(k,v) }, CustomEvent: class { constructor(type) { this.type = type; } } });
  function editor() {
    const root = new Node({ 'data-mui': 'brand-customizer' }), select = new Node(), action = new Node(), status = new Node(), preview = new Node(), wordmark = new Node(), tagline = new Node(), output = new Node();
    const fields = window.MaudUI.brandData.tokens.map(token => new Node({ 'data-brand-token': token.name }));
    root.nodes['[data-brand-select]'] = [select]; root.nodes['[data-brand-example-action]'] = [action]; root.nodes['[data-brand-status]'] = [status];
    document.nodes['[data-brand-token]'] = fields; document.nodes['[data-brand-select]'] = [select]; document.nodes['[data-brand-live]'] = [preview];
    preview.nodes['.mui-brand-mark__wordmark'] = [wordmark]; preview.nodes['.mui-brand-mark__tagline'] = [tagline]; ids.set('mui-theme-export', output);
    window.MaudUI.init(root);
    return { root, select, fields, output, wordmark, tagline, status, action, choose: value => { select.value = value; select.dispatchEvent({ type: 'change' }); } };
  }
  return { Node, document, window, ui: window.MaudUI, storage, ids, editor };
}
for (const file of ['static/maud-ui.js','static/maud-ui.min.js']) {
  test(`${file}: all three brands change live and export exactly nine names`, () => {
    const f = fixture(file), e = f.editor();
    for (const brand of f.ui.brandData.presets) {
      e.choose(brand.key);
      assert.equal(e.wordmark.textContent, brand.wordmark); assert.equal(e.tagline.textContent, brand.tagline);
      const names = [...e.output.textContent.matchAll(/^  (--mui-[\w-]+):/gm)].map(m => m[1]);
      assert.deepEqual(names, Array.from(f.ui.brandData.tokens, token => token.name));
      assert(e.output.textContent.includes('data:image/svg+xml')); assert(!names.includes('--mui-bg'));
      for (const [i, token] of f.ui.brandData.tokens.entries()) assert.equal(f.document.documentElement.properties.get(token.name), brand.values[i]);
    }
    assert.equal(f.document.documentElement.getAttribute('data-density'), 'spacious');
    e.action.dispatchEvent({ type: 'click' }); assert(e.status.textContent.includes('Action received'));
  });
  test(`${file}: custom edits validate, persist, reload, and reject unrelated keys`, () => {
    const f = fixture(file), e = f.editor(); e.choose('bank');
    const font = e.fields[2]; font.value = 'Georgia, serif'; e.root.dispatchEvent({ type: 'input', target: font });
    const before = e.output.textContent; font.value = 'invalid'; e.root.dispatchEvent({ type: 'input', target: font });
    assert.equal(font.getAttribute('aria-invalid'), 'true'); assert.equal(e.output.textContent, before);
    const reloaded = fixture(file, Object.fromEntries(f.storage)), r = reloaded.editor(); assert.equal(r.fields[2].value, 'Georgia, serif'); assert.equal(reloaded.document.documentElement.getAttribute('data-density'), 'compact');
    assert.equal(f.ui.brand.apply(['#fff'], 'bad'), false);
    const corrupt = fixture(file, { 'mui-brand': '{bad json}', 'mui-density': 'tiny' }); assert.doesNotThrow(() => corrupt.editor());
  });
  test(`${file}: document density survives reload, updates scoped previews, and attaches once`, () => {
    const f = fixture(file, { 'mui-density': 'compact' }); const select = new f.Node({ 'data-mui': 'density-control' }), scope = new f.Node({ 'data-mui-density-scope': 'compact' });
    f.document.nodes['[data-mui="density-control"]'] = [select]; f.document.nodes['[data-mui-density-scope]'] = [scope];
    f.ui.init(select); f.ui.behaviors['density-control'](select); assert.equal(select.listeners.change.length, 1);
    select.value = 'spacious'; select.dispatchEvent({ type: 'change' });
    assert.equal(f.document.documentElement.getAttribute('data-density'), 'spacious'); assert.equal(scope.getAttribute('data-mui-density-scope'), 'spacious'); assert.equal(f.storage.get('mui-density'), 'spacious');
    assert(f.ui.brand.buildCss().includes('--mui-brand-density: 2;')); f.ui.setDensity('invalid'); assert.equal(f.storage.get('mui-density'), 'spacious');
  });
  test(`${file}: decimal sorting preserves cents, signs, scale and integers above 2^53`, () => {
    const { ui } = fixture(file);
    for (const [a,b,result] of [['$9,950.01','$9,950.10',-1],['−$9,950.01','−$9,950.10',1],['9007199254740992.01','9007199254740992.02',-1],['$2.0','$2.000',0],['-0.01','0',-1],['$10','$2',1],['12,34.00','2',null],['','0',null],['Unknown','0',null]]) assert.equal(ui.compareDecimal(a,b), result, `${a} / ${b}`);
  });
  test(`${file}: header disclosure focuses its own input and Escape preserves the query`, () => {
    const f = fixture(file), details = new f.Node({ 'data-mui': 'header-search' }), summary = new f.Node(), input = new f.Node();
    input.value = 'Northline'; details.nodes.summary = [summary]; details.nodes['input[type="search"]'] = [input]; details.children = [summary,input];
    f.ui.init(details); f.ui.behaviors['header-search'](details); assert.equal(details.listeners.keydown.length,1);
    summary.focus(); details.open = true; details.dispatchEvent({ type: 'toggle' }); assert.equal(f.document.activeElement,input);
    const event = details.dispatchEvent({ type: 'keydown', key: 'Escape' }); assert(event.defaultPrevented); assert(!details.open); assert.equal(f.document.activeElement,summary); assert.equal(input.value,'Northline');
  });
  test(`${file}: KYC review opens the original case and returns focus on closing`, () => {
    const f = fixture(file), root = new f.Node({ 'data-mui': 'banking-demo' }), link = new f.Node({ href: '#case' }), panel = new f.Node(), summary = new f.Node(), close = new f.Node();
    root.children = [panel]; panel.nodes.summary = [summary]; f.ids.set('case',panel);
    link.parents = { '[data-bank-review]': link }; close.parents = { '[data-bank-close]': close, details: panel };
    f.ui.init(root); f.ui.init(root); assert.equal(root.listeners.click.length,1);
    const native = root.dispatchEvent({ type:'click',target:link,button:0,ctrlKey:true }); assert(!native.defaultPrevented); assert(!panel.open);
    root.dispatchEvent({ type:'click',target:link,button:0 }); assert(panel.open); assert.equal(f.document.activeElement,summary);
    root.dispatchEvent({ type:'click',target:close,button:0 }); assert(!panel.open); assert.equal(f.document.activeElement,link);
  });
}
