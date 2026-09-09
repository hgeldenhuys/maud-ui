// Browser-free cascade arithmetic over BOTH emitted stylesheets. This models the
// selectors/properties used by these fixtures, not font shaping or browser layout.
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

import { parse, el, computed, px, height } from './css-cascade.mjs';

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
    assert.equal(computed(rules, el('header', 'mui-page-header'), { width: 1023 }).display, 'grid');
    const grid = el('ul', 'mui-task-grid', { 'data-count': '1' });
    assert.equal(computed(rules, grid, { width: 1280 })['grid-template-columns'].replaceAll(' ', '').replaceAll('0px','0'), 'minmax(0,1fr)');
  });
}
