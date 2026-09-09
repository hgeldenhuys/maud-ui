// Browser-free cascade arithmetic over BOTH emitted stylesheets. This models the
// selectors/properties used by these fixtures, not font shaping or browser layout.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

function parse(text, context = []) {
  const result = [];
  text = text.replace(/\/\*[\s\S]*?\*\//g, '');
  let cursor = 0;
  while (cursor < text.length) {
    const open = text.indexOf('{', cursor); if (open < 0) break;
    const selector = text.slice(cursor, open).trim();
    let end = open + 1, depth = 1, quote = '';
    for (; end < text.length && depth; end++) {
      const c = text[end];
      if (quote) { if (c === quote && text[end - 1] !== '\\') quote = ''; }
      else if (c === '"' || c === "'") quote = c;
      else if (c === '{') depth++; else if (c === '}') depth--;
    }
    const body = text.slice(open + 1, end - 1);
    if (selector.startsWith('@')) result.push(...parse(body, [...context, selector]));
    else result.push({ selector, body, context });
    cursor = end;
  }
  return result;
}
function splitSelectors(selector) { return selector.split(/,(?![^()]*\))/).map(s => s.trim()); }
function compoundMatches(selector, node) {
  if (!node) return false;
  let matches = true;
  selector = selector.replace(/:not\(([^()]+)\)/g, (_, part) => { if (compoundMatches(part, node)) matches = false; return ''; });
  // Fixtures use resting controls. Pseudo-elements/state selectors cannot match them.
  if (selector.includes(':')) return false;
  const tag = selector.match(/^[a-z][\w-]*/i)?.[0];
  if (tag && tag !== node.tag) matches = false;
  for (const [, name] of selector.matchAll(/\.([\w-]+)/g)) if (!node.classes.includes(name)) matches = false;
  for (const [, name, value] of selector.matchAll(/\[([\w-]+)(?:=["']?([^\]"']+)["']?)?\]/g)) {
    if (!(name in node.attrs) || (value !== undefined && node.attrs[name] !== value)) matches = false;
  }
  return matches && !/[#~+|]/.test(selector);
}
function matches(selector, node) {
  const parts = selector.split(/\s+/); let current = node;
  if (!compoundMatches(parts.pop(), current)) return false;
  while (parts.length) {
    let part = parts.pop();
    if (part === '>') { part = parts.pop(); current = current.parent; if (!compoundMatches(part, current)) return false; }
    else { current = current.parent; while (current && !compoundMatches(part, current)) current = current.parent; if (!current) return false; }
  }
  return true;
}
const el = (tag, classes, attrs = {}, parent = null) => ({ tag, classes: classes.split(' ').filter(Boolean), attrs, parent });
function active(context, env) {
  return context.every(rule => {
    if (rule.startsWith('@media')) {
      if (rule.includes('pointer')) return /pointer:\s*coarse/.test(rule) && env.coarse;
      if (rule.includes('prefers-') || rule.includes('forced-colors')) return false;
      const max = rule.match(/max-width:\s*([\d.]+)(rem|px)/), min = rule.match(/min-width:\s*([\d.]+)(rem|px)/);
      const viewport = env.viewportWidth ?? env.width;
      return (!max || viewport <= Number(max[1]) * (max[2] === 'rem' ? 16 : 1)) && (!min || viewport >= Number(min[1]) * (min[2] === 'rem' ? 16 : 1));
    }
    if (rule.startsWith('@container')) {
      const max = rule.match(/max-width:\s*([\d.]+)rem/);
      return !!max && env.width <= Number(max[1]) * 16;
    }
    return false;
  });
}
function computed(rules, node, env) {
  let owner = node; while (owner.parent) owner = owner.parent;
  const theme = owner.attrs['data-theme'] || 'dark';
  const tokens = Object.fromEntries(rules.filter(r => !r.context.length && splitSelectors(r.selector).some(s => {
    const normalized = s.replaceAll('\"', '').replaceAll("'", '');
    return normalized === ':root' || normalized === '[data-theme]' || normalized === `[data-theme=${theme}]` || (owner.attrs['data-density'] && normalized === `:root[data-density=${owner.attrs['data-density']}]`) || (owner.attrs['data-brand'] && (normalized === '[data-brand]' || normalized === `[data-brand=${owner.attrs['data-brand']}]`));
  })).flatMap(r => [...r.body.matchAll(/(--[\w-]+):\s*([^;]+)(?:;|$)/g)].map(m => [m[1], m[2].trim()])));
  function resolve(value) {
    while (value.includes('var(')) {
      const start = value.indexOf('var('); let end = start + 4, depth = 1;
      for (; end < value.length && depth; end++) { if (value[end] === '(') depth++; if (value[end] === ')') depth--; }
      const [key, ...fallback] = value.slice(start + 4, end - 1).split(',');
      const raw = tokens[key.trim()] ?? fallback.join(',').trim(); assert(raw, `Unresolved ${key}`);
      value = value.slice(0, start) + resolve(raw) + value.slice(end);
    }
    let previous;
    do {
      previous = value;
      value = value.replace(/\b(calc|max|min)\(([^()]*)\)/g, (whole, fn, expression) => {
        if (!/^[\d.\s+*/,-]*(?:(?:rem|px)[\d.\s+*/,-]*)*$/.test(expression)) return whole;
        const numbers = expression.replace(/([\d.]+)rem/g, (_, v) => Number(v) * 16).replaceAll('px', '');
        const compute = part => Function(`"use strict"; return (${part})`)();
        return (fn === 'calc' ? compute(numbers) : Math[fn](...numbers.split(',').map(compute))) + 'px';
      });
    } while (value !== previous);
    return value;
  }
  const properties = new Map();
  function assign(name, value, specificity, order) {
    const last = properties.get(name);
    if (!last || specificity > last.specificity || (specificity === last.specificity && order >= last.order)) properties.set(name, { value, specificity, order });
  }
  rules.forEach((rule, order) => {
    if (!active(rule.context, env)) return;
    for (const selector of splitSelectors(rule.selector).filter(s => matches(s, node))) {
      // :not contributes its argument; our fixture selectors contain no :is/:where.
      const specificity = (selector.match(/[.#\[]/g)?.length || 0) * 10 + (selector.match(/(?:^|\s|>)\s*[a-z][\w-]*/g)?.length || 0);
      for (const declaration of rule.body.split(';')) {
        const colon = declaration.indexOf(':'); if (colon < 0) continue;
        const name = declaration.slice(0, colon).trim(), raw = declaration.slice(colon + 1).trim();
        if (!/^(?:padding(?:-(?:block|inline|top|right|bottom|left))?|height|min-height|align-self|font-size|line-height|border|box-sizing|grid-auto-flow|grid-template-columns|flex-wrap|display|white-space)$/.test(name)) continue;
        const value = resolve(raw);
        assign(name, value, specificity, order);
        if (name === 'padding') {
          const p = value.split(/\s+/); const [top, right = top, bottom = top, left = right] = p;
          for (const [key, val] of Object.entries({ top, right, bottom, left })) assign('padding-' + key, val, specificity, order);
        } else if (name === 'padding-block' || name === 'padding-inline') {
          const [a, b = a] = value.split(/\s+/), sides = name === 'padding-block' ? ['top', 'bottom'] : ['left', 'right'];
          assign('padding-' + sides[0], a, specificity, order); assign('padding-' + sides[1], b, specificity, order);
        }
      }
    }
  });
  return Object.fromEntries([...properties].map(([name, entry]) => [name, entry.value]));
}
const px = value => value === 'auto' || value === undefined ? 0 : Number.parseFloat(value) * (value.endsWith('rem') ? 16 : 1);
function height(style) {
  const content = px(style['font-size']) * Number(style['line-height']) + px(style['padding-top']) + px(style['padding-bottom']) + 2 * px(style.border);
  return Math.max(px(style['min-height']), style.height === 'auto' ? content : px(style.height));
}
for (const file of ['static/maud-ui.css', 'static/maud-ui.min.css']) {
  const rules = parse(readFileSync(file, 'utf8'));
  test(`${file}: standalone Row remains 32px with zero block padding; Sm remains available`, () => {
    for (const size of ['row', 'sm']) {
      const node = el('button', `mui-btn mui-btn--outline mui-btn--${size}`);
      const style = computed(rules, node, { width: 1280 });
      assert.equal(px(style['padding-top']), size === 'row' ? 0 : 4); assert.equal(px(style['padding-bottom']), size === 'row' ? 0 : 4); assert.equal(height(style), 32);
      assert.equal(height(computed(rules, node, { width: 390, coarse: true })), 44);
    }
  });
  test(`${file}: Save/Cancel use 32/32/36px, More stays 32px, and touch targets stay 44px`, () => {
    for (const theme of ['light', 'dark']) for (const density of ['row', 'compact', 'comfortable']) {
      const root = el('div', '', { 'data-theme': theme }), row = el('div', 'mui-action-row', { 'data-density': density }, root);
      const controls = [el('button', 'mui-btn mui-btn--primary', {}, row), el('a', 'mui-btn mui-btn--outline mui-btn--row', {}, row), el('summary', 'mui-btn mui-btn--outline mui-action-row__trigger', {}, el('details', 'mui-action-row__more', {}, row)), el('button', 'mui-btn mui-btn--primary mui-btn--md', {}, el('form', 'mui-block-action', {}, row))];
      for (const control of controls) {
        const desktop = computed(rules, control, { width: 1280 }); assert.equal(height(desktop), density === 'comfortable' && control.tag !== 'summary' ? 36 : 32, `${density} ${control.tag}`);
        assert.equal(desktop['white-space'], 'normal');
        assert.equal(height(computed(rules, control, { width: 390, coarse: true })), 44);
      }
    }
  });
  test(`${file}: timeline, grouped rows and date fields adapt on phones`, () => {
    const cases = [
      [el('ol', 'mui-record-timeline__items'), 'grid-auto-flow', 'column', 'row'],
      [el('li', 'mui-grouped-worklist__row'), 'grid-template-columns', 'minmax(10rem, 2fr) minmax(10rem, 2fr) minmax(0, 1fr) auto', 'minmax(0, 1fr) auto'],
      [el('div', 'mui-date-range__fields'), 'grid-template-columns', 'minmax(0, 1fr) auto minmax(0, 1fr)', 'minmax(0, 1fr)'],
    ];
    const normalize = value => value.replace(/\s+/g, '').replaceAll('0px', '0');
    for (const [node, prop, desktop, phone] of cases) {
      assert.equal(normalize(computed(rules, node, { width: 900 })[prop]), normalize(desktop));
      assert.equal(normalize(computed(rules, node, { width: 290 })[prop]), normalize(phone));
    }
    assert.equal(computed(rules, el('div', 'mui-action-row'), { width: 290 })['flex-wrap'], 'wrap');
  });
  test(`${file}: a narrow desktop timeline stays horizontal and cannot stretch to its grid sibling`, () => {
    const items = el('ol', 'mui-record-timeline__items');
    for (const viewportWidth of [768, 1280]) assert.equal(computed(rules, items, { width: 290, viewportWidth })['grid-auto-flow'], 'column');
    assert.equal(computed(rules, items, { width: 290, viewportWidth: 767 })['grid-auto-flow'], 'row');
    const frame = computed(rules, el('section', 'mui-record-timeline'), { width: 290, viewportWidth: 1280 });
    assert.equal(frame.height, 'fit-content'); assert.equal(frame['min-height'], '0'); assert.equal(frame['align-self'], 'start');
  });
  test(`${file}: document density changes controls, type, card insets and rows together`, () => {
    const sequence = [];
    for (const density of ['compact', 'comfortable', 'spacious']) {
      const root = el('html', '', { 'data-theme': 'light', 'data-density': density });
      const control = computed(rules, el('input', 'mui-input', {}, root), { width: 1280 });
      const card = computed(rules, el('section', 'mui-record-money', {}, root), { width: 1280 });
      const cell = computed(rules, el('td', 'mui-table__td', {}, root), { width: 1280 });
      sequence.push([height(control), px(control['font-size']), px(card['padding-top']), px(cell['padding-top'])]);
      assert.equal(height(computed(rules, el('input', 'mui-input', {}, root), { width: 390, coarse: true })), 44);
    }
    assert.deepEqual(sequence, [[32,13,16,6],[36,14,20,8],[40,15,24,10]]);
  });
  test(`${file}: chips follow document density while count bubbles and badges retain compact text`, () => {
    for (const [density, expected] of [['compact',32],['comfortable',32],['spacious',36]]) {
      const root = el('html', '', { 'data-density': density });
      const chip = el('a', 'mui-status-chip-group__chip', {}, root);
      assert.equal(height(computed(rules, chip, { width: 1280 })), expected);
      assert.equal(height(computed(rules, chip, { width: 390, coarse: true })), 44);
      assert.equal(px(computed(rules, el('span', 'mui-status-chip-group__count', {}, chip), { width: 1280 }).height), 18);
      assert.equal(px(computed(rules, el('span', 'mui-badge', {}, root), { width: 1280 }).height), 22);
    }
  });
  test(`${file}: desktop page context stays one 56px row; one task occupies one row`, () => {
    for (const width of [1024,1280]) {
      const header = computed(rules, el('header', 'mui-page-header'), { width });
      assert.equal(header.display, 'grid'); assert.equal(header['flex-wrap'], 'nowrap'); assert.equal(px(header.height), 56);
    }
    assert.equal(computed(rules, el('header', 'mui-page-header'), { width: 1023 }).display, 'flex');
    const grid = el('ul', 'mui-task-grid', { 'data-count': '1' });
    assert.equal(computed(rules, grid, { width: 1280 })['grid-template-columns'].replaceAll(' ', '').replaceAll('0px','0'), 'minmax(0,1fr)');
  });
}
