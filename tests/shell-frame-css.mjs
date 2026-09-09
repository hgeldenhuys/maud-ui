// Contract arithmetic over real Rust-rendered fixtures and both emitted bundles.
// Browser geometry is separately covered by shell-frame-browser.mjs (supervisor).
import assert from 'node:assert/strict';
import { readFileSync, writeFileSync } from 'node:fs';
import test from 'node:test';
import { parse, el, computed, px, height } from './css-cascade.mjs';

const modes = ['compact', 'comfortable', 'spacious'];
function markup(path) {
  const nodes = [], stack = [], voids = new Set('meta link input img br hr source wbr'.split(' '));
  for (const [, end, tag, tail] of readFileSync(path, 'utf8').matchAll(/<(\/?)([a-z][\w-]*)\b([^>]*)>/g)) {
    if (end) { while (stack.length && stack.pop().tag !== tag) {} continue; }
    const attrs = Object.fromEntries([...tail.matchAll(/([^\s=]+)(?:="([^"]*)")?/g)].map(m => [m[1], m[2] || '']));
    const node = el(tag, attrs.class || '', attrs, stack.at(-1)); nodes.push(node);
    if (!voids.has(tag)) stack.push(node);
  }
  return { nodes, root: nodes[0], one: name => { const node = nodes.find(n => n.classes.includes(name)); assert(node, name); return node; } };
}
const environment = {width: 1280, viewportHeight: 900};
function frame(rules, density, long = false) {
  const fixture = markup(`docs/fixtures/shell-frame-${density}-${long ? 'long' : 'short'}.html`);
  const style = name => computed(rules, fixture.one(name), environment);
  const shell = style('mui-block--shell'), body = style('mui-block--shell__body'), column = style('mui-block--shell__sidebar-column'), sidebar = style('mui-block--shell__sidebar');
  assert.equal(shell['min-height'], '100dvh'); assert.equal(shell['grid-template-rows'], 'auto 1fr auto');
  assert.equal(body['align-items'], 'stretch'); assert.equal(column['align-self'], 'stretch'); assert.equal(column.contain, 'size');
  assert.equal(column.display, 'grid'); assert.equal(column['grid-template-rows'].replace(/\s/g,''), 'minmax(0,1fr)');
  assert.equal(sidebar['align-self'], 'stretch');
  assert.equal(sidebar.position, 'sticky'); assert.equal(sidebar['overflow-y'], 'auto'); assert.equal(sidebar['max-height'], '100dvh');
  const masthead = style('mui-app-header'), footer = style('mui-app-footer'), page = style('mui-page-header'), content = style('mui-block--shell__content');
  const width = px(body.token('--mui-shell-sidebar-width')), x = width + px(content['padding-left']), y = height(masthead) + height(page) + px(content['padding-top']);
  return { fixture, style, signature: [height(masthead), 0, height(masthead), width, x, y, height(footer)], masthead: height(masthead), footer: height(footer) };
}
function stillFrame(rules) {
  const signatures = modes.map(mode => frame(rules, mode).signature);
  assert.deepEqual(signatures[0], [64, 0, 64, 240, 264, 144, 64]);
  assert(signatures.every(s => JSON.stringify(s) === JSON.stringify(signatures[0])), JSON.stringify(signatures));
}
const evidence = [];
for (const file of ['static/maud-ui.css', 'static/maud-ui.min.css']) {
  const source = readFileSync(file, 'utf8'), rules = parse(source);
  test(`${file}: Full is reserved for avatar and status-dot shapes`, () => {
    const uses = rules.filter(rule => rule.body.includes('var(--mui-radius-full)'));
    assert(uses.length > 0);
    for (const rule of uses) assert(/avatar|skeleton--circle|__dot|status-dot|__marker|__pulse|__title::?before/.test(rule.selector), rule.selector);
  });
  test(`${file}: density never moves the shell frame; regressions are rejected`, () => {
    stillFrame(rules);
    assert.throws(() => stillFrame(parse(source + '\n[data-density] .mui-block--shell__content { padding: var(--mui-card-inset-inline); }')));
    assert.throws(() => stillFrame(parse(source + '\n.mui-block--shell { min-height: 32rem; }')));
    assert.throws(() => stillFrame(parse(source + '\n.mui-block--shell__sidebar-column { contain: none; }')));
  });
  test(`${file}: short and long body rows fill the viewport and the sidebar column`, () => {
    for (const density of modes) for (const long of [false, true]) {
      const result = frame(rules, density, long);
      // Grid's auto / 1fr / auto contract for any short or long intrinsic content.
      // These heights are test inputs, not invented browser measurements.
      const intrinsicBody = long ? 2200 : 550;
      const bodyHeight = Math.max(environment.viewportHeight - result.masthead - result.footer, intrinsicBody);
      const columnHeight = bodyHeight; // align-self: stretch + contain: size above
      const footerBottom = result.masthead + bodyHeight + result.footer;
      assert(footerBottom >= environment.viewportHeight); assert.equal(columnHeight, bodyHeight);
      if (!long) assert.equal(footerBottom, 900);
      const rows = result.fixture.nodes.filter(n => n.tag === 'tr'); assert(rows.length > (long ? 60 : 2));
      evidence.push({file, density, length: long ? 'long' : 'short', frame: result.signature, intrinsic_body_input: intrinsicBody, body_height: bodyHeight, sidebar_column_height: columnHeight, footer_bottom: footerBottom});
    }
  });
  test(`${file}: container density changes controls but not frame; stack owns one gap`, () => {
    const values = [];
    for (const density of modes) {
      const fixture = markup(`docs/fixtures/shell-frame-${density}-scoped.html`);
      const input = fixture.nodes.find(n => n.tag === 'input' && n.attrs.name === 'guest');
      values.push(height(computed(rules, input, environment)));
      assert.equal(height(computed(rules, fixture.one('mui-app-header'), environment)), 64);
      const stack = computed(rules, fixture.one('mui-page-stack'), environment);
      assert.equal(stack.display, 'flex'); assert.equal(px(stack.gap), 16);
      for (const child of fixture.nodes.filter(n => n.parent === fixture.one('mui-page-stack'))) assert.equal(computed(rules, child, environment)['margin-block'], '0');
      const phoneBody = computed(rules, fixture.one('mui-block--shell__body'), { width: 390, coarse: true });
      assert.equal(phoneBody['grid-template-columns'].replace(/\s/g,''), 'minmax(0,1fr)');
    }
    assert.deepEqual(values, [32, 36, 40]);
  });
  test(`${file}: brand radius clamps controls and gives sibling surfaces the same corner`, () => {
    for (const brand of ['0', '.375rem', '.75rem', '4rem']) for (const density of modes) {
      const fixture = markup(`docs/fixtures/shell-frame-${density}-short.html`); fixture.root.attrs.style = `--mui-brand-radius:${brand}`;
      const root = computed(rules, fixture.root, environment), radius = px(brand);
      assert.equal(px(root.token('--mui-radius-sm')), radius * .5);
      assert.equal(px(root.token('--mui-radius-md')), radius);
      assert.equal(px(root.token('--mui-radius-lg')), Math.min(radius * 1.5, 12));
      for (const node of fixture.nodes.filter(n => n.classes.some(c => ['mui-btn','mui-input','mui-native-select'].includes(c)))) {
        const control = computed(rules, node, environment);
        assert(px(control['border-radius']) <= height(control) / 3);
      }
      const action = el('div', 'mui-action-row', {'data-density':'row'}, fixture.root);
      for (const node of [el('button','mui-btn mui-btn--primary',{},action), el('button','mui-switch mui-switch--sm',{},fixture.root)]) {
        const control = computed(rules, node, environment);
        assert(px(control['border-radius']) <= height(control) / 3);
      }
      for (const name of ['mui-attention-banner','mui-card','mui-table-wrapper']) assert.equal(px(computed(rules, fixture.one(name), environment)['border-radius']), Math.min(radius * 1.5, 12));
    }
    const wrong = parse(source + '\n[data-density][data-theme] { --mui-radius-lg: calc(var(--mui-brand-radius) * 2); }');
    const fixture = markup('docs/fixtures/shell-frame-compact-short.html');
    assert.throws(() => assert(px(computed(wrong, fixture.one('mui-card'), environment)['border-radius']) <= 12));
  });
  test(`${file}: current item is flat and edge-aligned; its parent has no fill`, () => {
    const fixture = markup('docs/fixtures/shell-frame-compact-short.html');
    const group = fixture.nodes.find(n => n.attrs['data-contains-current'] === 'true');
    const summary = fixture.nodes.find(n => n.parent === group && n.tag === 'summary');
    const parent = computed(rules, summary, environment);
    assert.equal(parent.background, 'transparent'); assert.equal(parent['font-weight'], '600'); assert.equal(parent['border-radius'], '0');
    assert.equal(parent.color, parent.token('--mui-text-primary'));
    const row = fixture.nodes.find(n => n.classes.includes('mui-block--shell__nav-item') && n.attrs['aria-current'] === 'page');
    assert.equal(computed(rules, row, environment)['border-radius'], '0');
    const marker = computed(rules, {...row, pseudo: 'before'}, environment);
    assert.equal(px(marker.width), 2); assert.equal(marker['inset-inline-start'], '0');
    assert.equal(px(computed(rules, fixture.one('mui-block--shell__nav'), environment)['padding-left']), 0);
    const wrong = parse(source + '\n.mui-block--shell__nav-group[data-contains-current="true"] > .mui-block--shell__nav-group-label { background: red; }');
    assert.throws(() => assert.equal(computed(wrong, summary, environment).background, 'transparent'));
  });
}
test.after(() => writeFileSync('docs/night-4-frame-contract.json', JSON.stringify({method:'CSS cascade and grid contract arithmetic; no browser measurements', cases:evidence}, null, 2) + '\n'));
