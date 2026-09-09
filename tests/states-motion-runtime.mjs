// Execute both emitted bundles with DOM and timer fixtures; no browser is launched.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';

function fixture(file, options = {}) {
  let document, serial = 0;
  const timers = new Map();
  class Node {
    constructor(attrs = {}) { this.attrs = { ...attrs }; this.nodes = {}; this.listeners = {}; this.hidden = false; this.nodeType = 1; this.classes = new Set(); this.classList = { add: name => this.classes.add(name) }; }
    hasAttribute(name) { return name in this.attrs; }
    getAttribute(name) { return this.attrs[name] ?? null; }
    setAttribute(name, value) { this.attrs[name] = String(value); }
    removeAttribute(name) { delete this.attrs[name]; }
    querySelectorAll(selector) { return this.nodes[selector] || []; }
    querySelector(selector) { return this.querySelectorAll(selector)[0] || null; }
    addEventListener(name, fn) { (this.listeners[name] ||= []).push(fn); }
    removeEventListener(name, fn) { this.listeners[name] = (this.listeners[name] || []).filter(f => f !== fn); }
    dispatchEvent(event) { event.target ||= this; event.preventDefault ||= () => { event.defaultPrevented = true; }; for (const fn of [...(this.listeners[event.type] || [])]) fn(event); return event; }
    closest(selector) { return selector === 'a[href]' && this.hasAttribute('href') ? this : null; }
    focus() { document.activeElement = this; }
    remove() { this.removed = true; }
  }
  document = new Node(); document.body = new Node(); document.documentElement = new Node();
  const window = new Node(); window.location = { hash: options.hash || '' }; window.matchMedia = () => ({ ...new Node(), matches: !!options.reduced });
  vm.runInNewContext(readFileSync(file, 'utf8'), {
    window, document, Element: Node, console,
    localStorage: { getItem() { return null; }, setItem() {} },
    setTimeout: (fn, delay) => { const id = ++serial; timers.set(id, { fn, delay }); return id; }, clearTimeout: id => timers.delete(id),
    getComputedStyle: () => ({ direction: options.direction || 'ltr', animationDuration: options.duration ?? '120ms', animationDelay: options.delay ?? '0s' }),
  });
  function workspace(init = true) {
    const root = new Node({ 'data-mui': 'workspace-pages' }), nav = new Node();
    const tabs = ['worklist', 'record'].map(name => new Node({ href: `#${name}` }));
    const panels = ['worklist', 'record'].map(id => Object.assign(new Node(), { id, value: 'Retained draft' }));
    root.nodes['[data-workspace-tab]'] = tabs; root.nodes['[data-workspace-panel]'] = panels; root.nodes['.lp__workspace-tabs'] = [nav];
    if (init) window.MaudUI.init(root);
    const click = (index, attrs = {}) => root.dispatchEvent({ type: 'click', target: tabs[index], button: 0, ...attrs });
    const key = (index, key, attrs = {}) => root.dispatchEvent({ type: 'keydown', target: tabs[index], key, ...attrs });
    return { root, nav, tabs, panels, click, key };
  }
  function toast() {
    const root = new Node({ 'data-mui': 'toast', 'data-duration': '0' }), close = new Node(); root.nodes['.mui-toast__close'] = [close];
    window.MaudUI.init(root);
    return { root, close, dismiss: () => close.dispatchEvent({ type: 'click' }) };
  }
  return { Node, document, window, timers, workspace, toast };
}
for (const file of ['static/maud-ui.js', 'static/maud-ui.min.js']) {
  test(`${file}: workspace reveals both panels without JS; enhancement supplies one selected tab`, () => {
    const f = fixture(file), w = f.workspace(false);
    assert(w.panels.every(p => !p.hidden)); f.window.MaudUI.init(w.root);
    assert.equal(w.nav.getAttribute('role'), 'tablist');
    assert.equal(w.tabs[0].getAttribute('aria-controls'), 'worklist');
    assert.equal(w.tabs[0].getAttribute('aria-selected'), 'true'); assert(w.panels[1].hidden);
    w.click(1); assert(w.panels[0].hidden); assert(!w.panels[1].hidden);
    assert.equal(w.panels[0].value, 'Retained draft');
    const back = new f.Node({ href: '#worklist' }); w.root.dispatchEvent({ type: 'click', target: back, button: 0 });
    assert.equal(f.document.activeElement, w.tabs[0]); assert(!w.panels[0].hidden);
  });
  test(`${file}: workspace honors deep links, keyboard traversal, RTL and modified native links`, () => {
    const f = fixture(file, { hash: '#record' }), w = f.workspace(); assert(w.panels[0].hidden);
    w.key(1, 'Home'); assert.equal(f.document.activeElement, w.tabs[0]);
    w.key(0, 'End'); assert.equal(f.document.activeElement, w.tabs[1]);
    w.key(1, 'ArrowRight'); assert.equal(f.document.activeElement, w.tabs[0]);
    w.key(0, 'ArrowLeft'); assert.equal(f.document.activeElement, w.tabs[1]);
    for (const attrs of [{ ctrlKey: true }, { metaKey: true }, { button: 1 }, { defaultPrevented: true }]) {
      const e = w.click(0, attrs); assert(w.panels[0].hidden); if (!attrs.defaultPrevented) assert(!e.defaultPrevented);
    }
    const r = fixture(file, { direction: 'rtl' }), rw = r.workspace(); rw.key(0, 'ArrowLeft'); assert.equal(r.document.activeElement, rw.tabs[1]);
  });
  test(`${file}: workspace initialization survives htmx swaps and history without duplicate listeners`, () => {
    const f = fixture(file), w = f.workspace(false);
    f.document.dispatchEvent({ type: 'htmx:afterSwap', target: w.root }); f.window.MaudUI.init(w.root);
    f.document.nodes['[data-mui][data-mui-init]'] = [w.root]; f.document.body.nodes['[data-mui]'] = [w.root];
    f.document.dispatchEvent({ type: 'htmx:historyRestore' });
    assert.equal(w.root.listeners.click.length, 1); assert.equal(w.root.listeners.keydown.length, 1);
  });
  test(`${file}: toast exit follows computed motion timing and only its own animation completion`, () => {
    const f = fixture(file, { duration: '0.24s', delay: '30ms' }), t = f.toast(); t.dismiss(); t.dismiss();
    assert(t.root.classes.has('mui-toast--exit')); assert.equal(f.timers.size, 1); assert.equal([...f.timers.values()][0].delay, 270);
    t.root.dispatchEvent({ type: 'animationend', target: t.close }); assert(!t.root.removed);
    t.root.dispatchEvent({ type: 'animationend' }); assert(t.root.removed); assert.equal(f.timers.size, 0);
    assert.equal(t.root.listeners.animationend.length, 0);
  });
  test(`${file}: toast fallback removes after the token duration; reduced or zero motion removes immediately`, () => {
    const f = fixture(file), t = f.toast(); t.dismiss(); [...f.timers.values()][0].fn(); assert(t.root.removed);
    for (const options of [{ reduced: true }, { duration: '0s' }]) {
      const f = fixture(file, options), t = f.toast(); t.dismiss(); assert(t.root.removed); assert.equal(f.timers.size, 0);
    }
  });
}
