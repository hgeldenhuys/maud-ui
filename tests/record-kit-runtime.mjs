// Run the actual shipped bundles in a small DOM fixture. Native radio/date keyboard
// behavior and browser layout still need supervisor review; this launches no browser.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import vm from 'node:vm';
import test from 'node:test';

function fixture(file) {
  let document;
  class Node {
    constructor(attrs = {}) { this.attrs = { ...attrs }; this.nodes = {}; this.listeners = {}; this.children = []; this.nodeType = 1; this.hidden = false; this.isConnected = true; this.disabled = false; this.readOnly = false; this.value = ''; }
    hasAttribute(name) { return name in this.attrs; }
    getAttribute(name) { return this.attrs[name] ?? null; }
    setAttribute(name, value) { this.attrs[name] = String(value); }
    removeAttribute(name) { delete this.attrs[name]; }
    querySelectorAll(selector) { return this.nodes[selector] || []; }
    querySelector(selector) { return this.querySelectorAll(selector)[0] || null; }
    addEventListener(name, fn) { (this.listeners[name] ||= []).push(fn); }
    removeEventListener(name, fn) { this.listeners[name] = (this.listeners[name] || []).filter(f => f !== fn); }
    dispatchEvent(event) { event.target ||= this; for (const fn of this.listeners[event.type] || []) fn(event); }
    closest() { return null; }
    contains(node) { return node === this || this.children.some(child => child.contains(node)); }
    getClientRects() { return this.hidden || !this.isConnected || this.parentElement?.hidden ? [] : [{}]; }
    focus() { document.activeElement = this; }
    setCustomValidity(message) { this.validationMessage = message; }
    append(child) { this.children.push(child); child.parentElement = this; }
  }
  document = new Node(); document.body = new Node(); document.documentElement = new Node();
  const ids = new Map(); document.getElementById = id => ids.get(id) || null;
  const window = new Node(); window.matchMedia = () => new Node();
  vm.runInNewContext(readFileSync(file, 'utf8'), { window, document, Element: Node, localStorage: { getItem() { return null; }, setItem() {} }, setTimeout, clearTimeout, console, getComputedStyle: () => ({ direction: 'ltr' }), CustomEvent: class { constructor(type) { this.type = type; } } });
  const ui = window.MaudUI;
  function dates(first = '2026-09-08', last = '2026-09-11', init = true) {
    const range = new Node({ 'data-mui': 'date-range', 'data-night': 'night', 'data-nights': 'nights', 'data-incomplete': 'Choose both dates', 'data-invalid': 'Invalid interval' });
    const start = new Node(), end = new Node(), output = new Node(), form = new Node();
    start.value = first; end.value = last; start.form = end.form = form;
    range.nodes['[data-range-field="start"]'] = [start]; range.nodes['[data-range-field="end"]'] = [end]; range.nodes.output = [output];
    range.append(start); range.append(end); range.append(output);
    document.nodes['[data-mui="date-range"]'] = [...document.querySelectorAll('[data-mui="date-range"]'), range];
    if (init) ui.init(range);
    return { range, start, end, output, form, change: () => range.dispatchEvent({ type: 'input' }) };
  }
  function banner(init = true) {
    const banner = new Node({ 'data-mui': 'attention-banner' }), dismiss = new Node(), next = new Node(), previous = new Node(), parent = new Node();
    dismiss.hidden = true; banner.nodes['.mui-attention-banner__dismiss'] = [dismiss]; banner.append(dismiss); parent.append(banner);
    document.nodes['a[href], button, input, select, textarea, summary, [tabindex]'] = [previous, dismiss, next];
    if (init) ui.init(banner);
    return { banner, dismiss, next, previous, parent, click: () => dismiss.dispatchEvent({ type: 'click' }) };
  }
  return { Node, document, window, ui, ids, dates, banner };
}
for (const file of ['static/maud-ui.js', 'static/maud-ui.min.js']) {
  test(`${file}: live date counts, singular and both event types`, () => {
    const f = fixture(file), d = f.dates();
    assert.equal(d.output.textContent, '3 nights');
    d.end.value = '2026-09-09'; d.change(); assert.equal(d.output.textContent, '1 night');
    d.range.setAttribute('data-nights', 'nuits'); d.end.value = '2026-09-12'; d.range.dispatchEvent({ type: 'change' }); assert.equal(d.output.textContent, '4 nuits');
    assert.equal(d.end.validationMessage, ''); assert(!d.start.hasAttribute('aria-invalid'));
  });
  test(`${file}: calendar boundaries never depend on timezone or Date year coercion`, () => {
    const f = fixture(file);
    for (const [first, last, result] of [
      ['2024-02-28', '2024-03-01', '2 nights'], ['1900-02-28', '1900-03-01', '1 night'],
      ['2000-02-28', '2000-03-01', '2 nights'], ['0099-12-31', '0100-01-01', '1 night'],
      ['2026-03-07', '2026-03-09', '2 nights'], ['2026-10-31', '2026-11-02', '2 nights'],
      ['0001-01-01', '9999-12-31', '3652058 nights'],
    ]) assert.equal(f.dates(first, last).output.textContent, result, `${first} to ${last}`);
  });
  test(`${file}: invalid, reversed and same-day values block submission until corrected`, () => {
    const f = fixture(file);
    for (const [first, last] of [['2026-02-29', '2026-03-01'], ['0000-01-01', '2026-09-11'], ['2026-09-11', '2026-09-11'], ['2026-09-12', '2026-09-11'], ['2026-9-08', '2026-09-11'], ['2026-04-31', '2026-09-11'], ['２０２６-09-08', '2026-09-11']]) {
      const d = f.dates(first, last); assert.equal(d.output.textContent, 'Invalid interval'); assert.equal(d.end.validationMessage, 'Invalid interval'); assert.equal(d.start.getAttribute('aria-invalid'), 'true');
      d.start.value = '2026-09-08'; d.end.value = '2026-09-11'; d.change(); assert.equal(d.end.validationMessage, ''); assert(!d.end.hasAttribute('aria-invalid'));
    }
  });
  test(`${file}: incomplete and inclusive min/max keep native required validation separate`, () => {
    const f = fixture(file), d = f.dates(); d.start.min = '2026-09-08'; d.end.max = '2026-09-11'; d.change(); assert.equal(d.output.textContent, '3 nights');
    d.start.min = '2026-09-09'; d.change(); assert.equal(d.end.validationMessage, 'Invalid interval');
    d.start.value = ''; d.change(); assert.equal(d.output.textContent, 'Choose both dates'); assert.equal(d.end.validationMessage, '');
    d.start.min = ''; d.start.value = '2026-09-08'; d.end.value = '2026-09-12'; d.change(); assert.equal(d.end.validationMessage, 'Invalid interval');
    d.end.max = 'malformed'; d.change(); assert.equal(d.output.textContent, '4 nights');
  });
  test(`${file}: readonly and disabled values do not add a submit block`, () => {
    for (const target of ['range', 'end']) {
      const d = fixture(file).dates('2026-09-12', '2026-09-11'); d[target].disabled = true; d.change(); assert.equal(d.end.validationMessage, '');
    }
    const d = fixture(file).dates('2026-09-12', '2026-09-11'); d.end.readOnly = true; d.change(); assert.equal(d.end.validationMessage, '');
  });
  test(`${file}: reset runs after values restore, respects cancellation and external form owners`, async () => {
    const f = fixture(file), d = f.dates('2026-09-12', '2026-09-11'), unrelated = f.dates();
    d.start.form = new f.Node(); // The end belongs to an external form; no closest(form) assumption.
    f.document.dispatchEvent({ type: 'reset', target: d.form }); d.start.value = '2026-09-08';
    await new Promise(resolve => setTimeout(resolve, 5)); assert.equal(d.output.textContent, '3 nights'); assert.equal(d.end.validationMessage, '');
    assert.equal(unrelated.output.textContent, '3 nights');
    d.start.value = '2026-09-12'; d.change(); const cancelled = { type: 'reset', target: d.form, defaultPrevented: true }; f.document.dispatchEvent(cancelled);
    await new Promise(resolve => setTimeout(resolve, 5)); assert.equal(d.output.textContent, 'Invalid interval');
  });
  test(`${file}: date initialization is idempotent across swaps and history restore`, () => {
    const f = fixture(file), d = f.dates(undefined, undefined, false);
    f.document.dispatchEvent({ type: 'htmx:afterSwap', target: d.range }); f.ui.init(d.range);
    assert.equal(d.range.listeners.input.length, 1); assert(!d.range.listeners.keydown);
    const restored = f.dates(undefined, undefined, false); restored.range.setAttribute('data-mui-init', '');
    f.document.nodes['[data-mui][data-mui-init]'] = [d.range, restored.range]; f.document.body.nodes['[data-mui]'] = [d.range, restored.range];
    f.document.dispatchEvent({ type: 'htmx:historyRestore' });
    assert.equal(d.range.listeners.input.length, 1); assert.equal(restored.range.listeners.input.length, 1); assert.equal(f.document.listeners.reset.length, 1);
  });
  test(`${file}: dismissal reveals progressively, hides locally and focuses a supplied target`, () => {
    const f = fixture(file), b = f.banner(false); assert(b.dismiss.hidden); f.ui.init(b.banner); assert(!b.dismiss.hidden);
    const target = new f.Node(); f.ids.set('record', target); b.dismiss.setAttribute('data-dismiss-focus', 'record'); b.click(); assert(b.banner.hidden); assert.equal(f.document.activeElement, target); assert(!b.next.hidden);
  });
  test(`${file}: dismissal skips hidden/disabled controls, then restores parent focus`, () => {
    const f = fixture(file), b = f.banner(); b.next.disabled = true; b.click(); assert.equal(f.document.activeElement, b.previous);
    const c = f.banner(); c.next.hidden = true; c.previous.hidden = true; c.click(); assert.equal(f.document.activeElement, c.parent); assert(!c.parent.hasAttribute('tabindex'));
    const e = f.banner(); e.next.hidden = true; e.previous.hidden = true; e.parent.setAttribute('tabindex', '0'); e.click(); assert.equal(e.parent.getAttribute('tabindex'), '0');
  });
  test(`${file}: dismissal rejects an internal target and attaches once to OOB swaps`, () => {
    const f = fixture(file), b = f.banner(false); f.ids.set('inside', b.dismiss); b.dismiss.setAttribute('data-dismiss-focus', 'inside');
    f.document.dispatchEvent({ type: 'htmx:oobAfterSwap', target: b.banner }); f.ui.init(b.banner); f.ui.behaviors['attention-banner'](b.banner);
    assert.equal(b.dismiss.listeners.click.length, 1); b.click(); assert.equal(f.document.activeElement, b.next);
  });
}
