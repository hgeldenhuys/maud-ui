// Handler/lifecycle tests with a small DOM fixture. Native dialog, layout and AT
// behavior still require browser review; this file deliberately launches no browser.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import vm from "node:vm";
import test from "node:test";

class Node {
  constructor(attrs = {}) { this.attrs = { ...attrs }; this.listeners = {}; this.nodes = {}; this.children = []; this.nodeType = 1; this.isConnected = true; this.hidden = false; }
  hasAttribute(name) { return name in this.attrs; }
  getAttribute(name) { return this.attrs[name] ?? null; }
  setAttribute(name, value) { this.attrs[name] = String(value); }
  removeAttribute(name) { delete this.attrs[name]; }
  querySelectorAll(selector) { return this.nodes[selector] || []; }
  querySelector(selector) { return this.querySelectorAll(selector)[0] || null; }
  addEventListener(name, fn, options = {}) { (this.listeners[name] ||= []).push({ fn, once: options.once }); }
  removeEventListener(name, fn) { this.listeners[name] = (this.listeners[name] || []).filter(item => item.fn !== fn); }
  dispatchEvent(event) {
    event.target ||= this;
    for (const item of [...(this.listeners[event.type] || [])]) {
      if (item.once) this.removeEventListener(event.type, item.fn);
      item.fn(event);
    }
  }
  focus() { document.activeElement = this; }
  getClientRects() { return this.isConnected && !this.hidden ? [{}] : []; }
  closest(selector) { return selector === "a[href]" && this.hasAttribute("href") ? this : null; }
  contains(node) { return this === node || this.children.some(child => child.contains(node)); }
  append(node) {
    if (node.parentNode) node.parentNode.children = node.parentNode.children.filter(child => child !== node);
    this.children.push(node); node.parentNode = this;
  }
  get id() { return this.getAttribute('id') || ''; }
  set id(value) { this.setAttribute('id', value); }
  get firstElementChild() { return this.children[0] || null; }
  get firstChild() { return this.children[0] || null; }
  appendChild(node) { this.append(node); return node; }
  replaceChildren(...nodes) { this.children = []; nodes.forEach(node => this.append(node)); }
  scrollIntoView() { this.scrolled = true; }
  get nextSibling() { return this.parentNode?.children[this.parentNode.children.indexOf(this) + 1] || null; }
  insertBefore(node, reference) {
    this.append(node);
    this.children = this.children.filter(child => child !== node);
    const index = reference ? this.children.indexOf(reference) : this.children.length;
    this.children.splice(index, 0, node);
  }
}
const document = new Node();
document.body = new Node();
document.documentElement = new Node();
const byId = new Map();
document.getElementById = id => byId.get(id) || null;
const media = new Node(); media.matches = true;
const window = new Node(); window.matchMedia = () => media;
const storage = new Map();
const localStorage = { getItem: key => storage.get(key) ?? null, setItem: (key, value) => storage.set(key, value) };
const context = { window, document, localStorage, FormData: class { constructor(form) { return form.values; } }, Element: Node, CustomEvent: class { constructor(type) { this.type = type; } }, getComputedStyle: node => ({ direction: node.direction || "ltr" }), setTimeout, clearTimeout, console };
vm.runInNewContext(readFileSync("static/maud-ui.js", "utf8"), context);
const ui = window.MaudUI;
const event = (type, values = {}) => ({ type, button: 0, ...values, preventDefault() { this.defaultPrevented = true; } });
function dialog() {
  const node = new Node(); node.open = false;
  node.showModal = () => { node.open = true; };
  node.close = () => { node.open = false; node.dispatchEvent(event("close")); };
  return node;
}

test("filter arrows wrap, honor RTL, preserve selection and allow modified keys", () => {
  const group = new Node({ "data-mui": "status-chip-group" });
  const links = [new Node({ href: "/a", "aria-current": "page" }), new Node({ href: "/b" }), new Node({ href: "/c" })];
  group.nodes["a[href]"] = links;
  ui.init(group); ui.init(group);
  assert.equal(group.listeners.keydown.length, 1);
  group.dispatchEvent(event("keydown", { key: "ArrowLeft", target: links[0] })); assert.equal(document.activeElement, links[2]);
  group.direction = "rtl";
  group.dispatchEvent(event("keydown", { key: "ArrowRight", target: links[0] })); assert.equal(document.activeElement, links[2]);
  group.dispatchEvent(event("keydown", { key: "Home", target: links[2] })); assert.equal(document.activeElement, links[0]);
  group.dispatchEvent(event("keydown", { key: "End", target: links[0] })); assert.equal(document.activeElement, links[2]);
  const modified = event("keydown", { key: "Home", target: links[2], ctrlKey: true }); group.dispatchEvent(modified);
  assert(!modified.defaultPrevented); assert.equal(links[0].getAttribute("aria-current"), "page");
});

test("More retains native fallback, opens a dialog, and restores its ARIA/focus state", () => {
  const trigger = new Node({ "data-mui": "navigation-trigger", "aria-controls": "more", href: "/navigation" });
  ui.init(trigger);
  const fallback = event("click"); trigger.dispatchEvent(fallback); assert(!fallback.defaultPrevented);
  const panel = dialog(); byId.set("more", panel);
  const modified = event("click", { ctrlKey: true }); trigger.dispatchEvent(modified); assert(!modified.defaultPrevented);
  const click = event("click"); trigger.dispatchEvent(click);
  assert(click.defaultPrevented && panel.open); assert.equal(trigger.getAttribute("aria-expanded"), "true");
  panel.close(); assert.equal(trigger.getAttribute("aria-expanded"), "false"); assert.equal(document.activeElement, trigger);
});

test("shell moves one sidebar into the drawer, restores it, and closes on desktop", () => {
  const shell = new Node({ "data-mui": "shell-navigation" }), sidebar = new Node(), main = new Node(), panel = dialog();
  shell.append(sidebar); shell.append(main); shell.append(panel);
  shell.nodes[".mui-block--shell__sidebar"] = [sidebar]; shell.nodes[".mui-navigation-dialog"] = [panel]; shell.nodes[".mui-block--shell__main"] = [main];
  ui.init(shell);
  panel.dispatchEvent(event("mui:navigation-open")); panel.showModal(); assert.equal(sidebar.parentNode, panel);
  panel.close(); assert.equal(sidebar.parentNode, shell); assert.equal(shell.children.filter(n => n === sidebar).length, 1);
  panel.dispatchEvent(event("mui:navigation-open")); panel.showModal();
  media.matches = false; media.dispatchEvent(event("change")); assert(!panel.open); assert.equal(sidebar.parentNode, shell);
  shell.isConnected = false; media.dispatchEvent(event("change")); assert.equal(media.listeners.change.length, 0);
});

test("built runtime initializes an outerHTML replacement through the htmx lifecycle", () => {
  const replacement = new Node({ "data-mui": "status-chip-group" }); replacement.nodes["a[href]"] = [];
  document.dispatchEvent(event("htmx:afterSwap", { target: replacement }));
  assert(replacement.hasAttribute("data-mui-init")); assert.equal(replacement.listeners.keydown.length, 1);
  document.dispatchEvent(event("htmx:oobAfterSwap", { target: replacement })); assert.equal(replacement.listeners.keydown.length, 1);
});

for (const bundle of ['static/maud-ui.js', 'static/maud-ui.min.js']) {
  test(`${bundle}: the same sidebar returns to its stretching column after every drawer cycle`, () => {
    const localWindow = new Node(), localMedia = new Node(), localDocument = new Node();
    localMedia.matches = true; localWindow.matchMedia = () => localMedia;
    localDocument.documentElement = new Node(); localDocument.body = new Node(); localDocument.getElementById = () => null;
    vm.runInNewContext(readFileSync(bundle, 'utf8'), { ...context, window: localWindow, document: localDocument });
    const shell = new Node({'data-mui':'shell-navigation'}), body = new Node(), column = new Node(), sidebar = new Node(), main = new Node(), panel = dialog();
    const input = new Node(), link = new Node({href:'/booking','aria-current':'page'}); input.value = 'Original value';
    sidebar.append(input); sidebar.append(link); column.append(sidebar); body.append(column); body.append(main); shell.append(body); shell.append(panel);
    shell.nodes['.mui-block--shell__sidebar'] = [sidebar]; shell.nodes['.mui-block--shell__sidebar-column'] = [column];
    shell.nodes['.mui-block--shell__main'] = [main]; shell.nodes['.mui-navigation-dialog'] = [panel];
    localWindow.MaudUI.init(shell); localWindow.MaudUI.init(shell);
    for (let i = 0; i < 3; i++) {
      panel.dispatchEvent(event('mui:navigation-open')); panel.showModal(); assert.equal(sidebar.parentNode, panel);
      panel.close(); assert.equal(sidebar.parentNode, column); assert.equal(column.children.length, 1);
    }
    panel.dispatchEvent(event('mui:navigation-open')); panel.showModal();
    localMedia.matches = false; localMedia.dispatchEvent(event('change'));
    assert(!panel.open); assert.equal(sidebar.parentNode, column); assert.equal(input.value, 'Original value');
    assert.equal(sidebar.children[1], link); assert.equal(body.children[1], main);
    shell.isConnected = false; localMedia.dispatchEvent(event('change'));
    assert.equal(localMedia.listeners.change.length, 0);
  });
}

test("gallery phone menu moves one site nav, keeps search usable, and restores focus/layout state", () => {
  const header = new Node(), siteNav = new Node(), tools = new Node(), sidebar = new Node(), componentNav = new Node();
  const menu = new Node(), main = new Node(), backdrop = new Node(), search = new Node();
  const first = new Node({ href: "/gallery" }), last = new Node({ href: "/table" });
  const phone = new Node(), narrow = new Node(); phone.matches = true; narrow.matches = true;
  header.append(menu); header.append(search); header.append(siteNav); header.append(tools);
  siteNav.append(first); sidebar.append(componentNav); componentNav.append(last);
  const focusables = 'a[href], button, input, select, summary, [tabindex="0"]';
  header.nodes[focusables] = [menu, search]; sidebar.nodes[focusables] = [first, last]; sidebar.nodes['a[href]'] = [first, last];
  document.nodes['main.mui-gallery__main'] = [main]; document.nodes['.mui-showcase__header'] = [header];
  for (const [id, node] of Object.entries({ 'mui-drawer-toggle': menu, 'mui-gallery-navigation': sidebar, 'mui-site-navigation': siteNav, 'mui-drawer-backdrop': backdrop })) byId.set(id, node);
  window.matchMedia = query => query.includes('40rem') ? phone : narrow;
  const source = readFileSync('src/showcase/mod.rs', 'utf8');
  const start = source.indexOf('    // ── Mobile drawer ─');
  const end = source.indexOf('    // ── Command palette ─', start);
  vm.runInNewContext(source.slice(start, end), { ...context, search });
  assert.equal(siteNav.parentNode, sidebar);
  menu.dispatchEvent(event('click'));
  assert.equal(menu.getAttribute('aria-expanded'), 'true'); assert.equal(main.inert, true);
  assert.equal(document.activeElement, first);
  last.focus(); const tab = event('keydown', { key: 'Tab' }); document.dispatchEvent(tab);
  assert(tab.defaultPrevented); assert.equal(document.activeElement, menu);
  document.dispatchEvent(event('keydown', { key: 'Escape' }));
  assert.equal(main.inert, false); assert.equal(menu.getAttribute('aria-expanded'), 'false');
  assert.equal(document.activeElement, menu);
  search.value = 'table'; search.focus(); search.dispatchEvent(event('input'));
  assert.equal(main.inert, true); assert.equal(document.activeElement, search);
  phone.matches = false; phone.dispatchEvent(event('change'));
  assert.equal(siteNav.parentNode, header); assert.equal(siteNav.nextSibling, tools);
  narrow.matches = false; narrow.dispatchEvent(event('change'));
  assert.equal(main.inert, false); assert(!document.documentElement.hasAttribute('data-mui-drawer'));
  phone.matches = true; phone.dispatchEvent(event('change'));
  assert.equal(sidebar.children.filter(child => child === siteNav).length, 1);
});

test("theme copy reports success only after the clipboard write and offers a failure fallback", async () => {
  const source = readFileSync('src/showcase/mod.rs', 'utf8');
  const start = source.indexOf("  document.getElementById('mui-theme-copy')?.addEventListener");
  const end = source.indexOf("  document.getElementById('mui-theme-download')", start);
  const copy = new Node(), status = new Node();
  byId.set('mui-theme-copy', copy); byId.set('mui-theme-export-status', status);
  let complete;
  const clipboard = { writeText: () => new Promise(resolve => { complete = resolve; }) };
  vm.runInNewContext(source.slice(start, end), { document, navigator: { clipboard }, buildCss: () => ':root {}' });
  const pending = copy.listeners.click[0].fn();
  assert.equal(status.textContent, undefined);
  complete(); await pending; assert.equal(status.textContent, 'CSS copied.');
  clipboard.writeText = async () => { throw new Error('Permission denied'); };
  await copy.listeners.click[0].fn();
  assert.match(status.textContent, /Select the CSS above or download/);
});

test("group preferences survive rail expansion, restore on exit, and degrade when storage is unavailable", () => {
  const group = new Node({ 'data-mui': 'nav-group', 'data-nav-key': 'work' }); group.open = true;
  storage.set('mui-nav-group:work', 'closed');
  group.nodes['[aria-current="page"]'] = [new Node()];
  ui.init(group); assert.equal(group.open, false);
  const root = new Node(); root.nodes['[data-mui="nav-group"]'] = [group];
  ui.navigation.forceGroups(root, true); assert.equal(group.open, true);
  group.dispatchEvent(event('toggle')); assert.equal(storage.get('mui-nav-group:work'), 'closed');
  ui.navigation.forceGroups(root, false); assert.equal(group.open, false);
  group.open = true; group.dispatchEvent(event('toggle')); assert.equal(storage.get('mui-nav-group:work'), 'open');
  const replacement = new Node({ 'data-mui': 'nav-group', 'data-nav-key': 'work' }); replacement.open = false;
  ui.init(replacement); assert.equal(replacement.open, true);
  const originalGet = localStorage.getItem; localStorage.getItem = () => { throw Error('blocked'); };
  const denied = new Node({ 'data-mui': 'nav-group', 'data-nav-key': 'denied' }); denied.open = true;
  assert.doesNotThrow(() => ui.init(denied)); assert.equal(denied.open, true);
  localStorage.getItem = originalGet;
});

test("shell rail persists once, clears a hidden filter, and restores a single navigation node", () => {
  const oldMatchMedia = window.matchMedia;
  const phone = new Node(), desktop = new Node(); phone.matches = false; desktop.matches = true;
  window.matchMedia = query => query.includes('64rem') ? desktop : phone;
  const shell = new Node({ id: 'rail-test', 'data-mui': 'shell-navigation', 'data-collapsed': 'false' });
  const sidebar = new Node(), main = new Node(), panel = dialog(), rail = new Node(), input = new Node(); input.value = 'billing';
  const hiddenRow = new Node(); hiddenRow.hidden = true;
  shell.append(sidebar); shell.append(main); shell.append(panel);
  shell.nodes['.mui-block--shell__sidebar'] = [sidebar]; shell.nodes['.mui-navigation-dialog'] = [panel]; shell.nodes['.mui-block--shell__main'] = [main];
  shell.nodes['[data-mui="shell-rail"]'] = [rail]; sidebar.nodes['[data-mui-nav-search]'] = [input]; sidebar.nodes['li, .mui-block--shell__nav-group'] = [hiddenRow];
  ui.init(shell); ui.init(shell);
  rail.dispatchEvent(event('click'));
  assert.equal(rail.listeners.click.length, 1);
  assert.equal(shell.getAttribute('data-collapsed'), 'true'); assert.equal(rail.getAttribute('aria-expanded'), 'false');
  assert.equal(storage.get('mui-shell-rail:rail-test'), 'true'); assert.equal(input.value, ''); assert.equal(hiddenRow.hidden, false);
  desktop.matches = false; desktop.dispatchEvent(event('change')); assert.equal(rail.getAttribute('aria-expanded'), 'true');
  shell.isConnected = false; desktop.dispatchEvent(event('change')); phone.dispatchEvent(event('change'));
  window.matchMedia = oldMatchMedia;
});

test("workspace filters, record selection and local create agree on counts and retain text safety", () => {
  const shell = new Node({ id: 'example' }), demo = new Node({ 'data-mui': 'workspace-demo' });
  const search = new Node(), searchForm = new Node(), table = new Node(), record = new Node(), form = new Node(), panel = dialog();
  const feedback = new Node(), title = new Node(), subtitle = new Node(), status = new Node(), empty = new Node(), countSentence = new Node();
  panel.id = 'example-new'; search.value = ''; search.closest = () => searchForm; form.closest = () => panel; demo.closest = () => shell;
  form.reportValidity = () => true; form.reset = () => { form.resetCalled = true; };
  const labels = ['All', 'Arriving', 'Checked in', 'Needs review'];
  const chips = labels.map((label, i) => { const chip = new Node({ href: '#example-filter-' + i }); chip.nodes['.mui-status-chip-group__count'] = [new Node()]; chip.nodes['.mui-sr-only'] = [new Node()]; return chip; });
  function row(name, state, room = 'Garden suite', nights = '2', reference = 'RS-2048') {
    const row = new Node(); row.textContent = name + ' ' + room + ' ' + state;
    const guest = new Node({ 'data-guest': name, 'data-status': state, 'data-room': room, 'data-reference': reference });
    const ref = new Node(), badge = new Node(), view = new Node({ href: '#example-record', 'data-demo-view': '' }); badge.textContent = state;
    const cells = [new Node(), new Node(), new Node(), new Node(), new Node()]; cells[2].append(badge); cells[3].textContent = nights; cells.forEach(cell => row.append(cell));
    view.closest = selector => selector === 'tr' ? row : selector === 'a[href]' ? view : null;
    row.nodes['[data-guest]'] = [guest]; row.nodes['.mui-workspace-example__reference'] = [ref]; row.nodes['[data-demo-view]'] = [view];
    badge.cloneNode = () => { const cloned = new Node(); cloned.textContent = badge.textContent; return cloned; };
    row.cloneNode = () => rowFactory();
    function rowFactory() { return makeRow(name, state, room, nights, reference); }
    return row;
  }
  const makeRow = row;
  const first = row('Amira Khan', 'Arriving'), second = row('Lina Chen', 'Checked in'); table.append(first); table.append(second);
  table.querySelectorAll = selector => selector === 'tr' ? table.children : [];
  for (const [selector, node] of Object.entries({
    '.mui-worklist-header input[type="search"]': search, 'tbody': table, '.mui-workspace-example__record': record, '[data-demo-create]': form,
    '[role="status"]': feedback, '[data-demo-title]': title, '.mui-record-header__subtitle': subtitle, '[data-demo-status]': status,
    '.mui-workspace-example__empty': empty, '.mui-worklist-header__count': countSentence,
  })) demo.nodes[selector] = [node];
  demo.nodes['.mui-status-chip-group__chip'] = chips;
  ui.init(demo);
  search.value = 'lina'; search.dispatchEvent(event('input')); assert.equal(first.hidden, true); assert.equal(second.hidden, false); assert.equal(feedback.textContent, '1 reservation shown.');
  search.value = ''; demo.dispatchEvent(event('click', { target: chips[1] })); assert.equal(first.hidden, false); assert.equal(second.hidden, true); assert.equal(chips[1].getAttribute('aria-current'), 'page');
  demo.dispatchEvent(event('click', { target: first.querySelector('[data-demo-view]') })); assert.equal(title.textContent, 'Amira Khan'); assert.equal(document.activeElement, record); assert.equal(record.scrolled, true);
  form.values = new Map([['guest', '<img src=x onerror=alert(1)>'], ['room', 'Terrace suite'], ['nights', '3']]);
  form.dispatchEvent(event('submit'));
  assert.equal(table.children.length, 3); assert.equal(chips[0].querySelector('.mui-status-chip-group__count').textContent, '3');
  assert.equal(chips[1].querySelector('.mui-status-chip-group__count').textContent, '2');
  assert.equal(table.children[2].querySelector('[data-guest]').textContent, '<img src=x onerror=alert(1)>');
  assert.equal(table.children[2].children[3].textContent, '3'); assert.equal(form.resetCalled, true);
  assert(feedback.textContent.includes('Changes stay on this page.')); assert.equal(search.value, '');
});

test("gallery palette groups recents, rejects unknown stored routes, keeps keyboard selection and restores focus", () => {
  const panel = dialog(), input = new Node(), list = new Node(), opener = new Node(), closer = new Node(), origin = new Node();
  class PaletteNode extends Node {
    constructor() { super(); this.classes = new Set(); this.classList = { toggle: (name, enabled) => enabled ? this.classes.add(name) : this.classes.delete(name) }; }
    closest(selector) { return selector === '[data-index]' && this.hasAttribute('data-index') ? this : super.closest(selector); }
  }
  list.querySelectorAll = selector => selector === '[role="option"]' ? list.children.filter(node => node.getAttribute('role') === 'option') : [];
  list.querySelector = selector => selector === '[aria-selected="true"]' ? list.children.find(node => node.getAttribute('aria-selected') === 'true') : null;
  document.createElement = () => new PaletteNode();
  for (const [id, node] of Object.entries({ 'mui-palette': panel, 'mui-palette-input': input, 'mui-palette-list': list, 'mui-palette-open': opener, 'mui-palette-close': closer })) byId.set(id, node);
  const data = [{ l: 'Home', u: '/', k: 'page' }, { l: 'Table', u: '/table', k: 'component' }, { l: 'Page header', u: '/blocks/shell-page-header', k: 'block' }];
  window.__MUI_PALETTE__ = data; window.location = { href: '/' };
  storage.set('mui-palette-recent', JSON.stringify(['/table', 'javascript:alert(1)', '/table', null, { u: '/table' }]));
  const source = readFileSync('src/showcase/mod.rs', 'utf8');
  const start = source.indexOf('    // ── Command palette ─', source.indexOf('fn showcase_js'));
  const end = source.indexOf('    // ── Theme + direction', start);
  let drawerClosed = false;
  vm.runInNewContext(source.slice(start, end), { ...context, htmlEl: document.documentElement, setDrawer: () => { drawerClosed = true; } });
  origin.focus(); opener.dispatchEvent(event('click'));
  assert(panel.open && drawerClosed); assert.equal(document.activeElement, input); assert.equal(input.getAttribute('aria-expanded'), 'true');
  assert.equal(list.children[0].textContent, 'Recent'); assert.equal(list.querySelectorAll('[role="option"]').length, 3);
  assert.equal(list.querySelector('[aria-selected="true"]').getAttribute('data-index'), '0');
  input.dispatchEvent(event('keydown', { key: 'ArrowDown' })); assert.equal(input.getAttribute('aria-activedescendant'), 'mui-palette-option-1');
  input.value = 'not a destination'; input.dispatchEvent(event('input')); assert.equal(list.querySelectorAll('[role="option"]').length, 0); assert.equal(input.getAttribute('aria-activedescendant'), null);
  input.dispatchEvent(event('keydown', { key: 'Enter' })); assert.equal(window.location.href, '/');
  input.value = 'table'; input.dispatchEvent(event('input')); input.dispatchEvent(event('keydown', { key: 'Enter' }));
  assert.equal(window.location.href, '/table'); assert.equal(panel.open, false); assert.equal(document.activeElement, origin);
  assert.deepEqual(JSON.parse(storage.get('mui-palette-recent')), ['/table']);
  assert.equal(input.getAttribute('aria-expanded'), 'false');
});

test("data table sorting and pagination preserve original rich cells, numeric alignment and selected controls", () => {
  const root = new Node({ 'data-mui': 'data-table', 'data-page-size': '1' });
  const body = new Node(), info = new Node(), previous = new Node(), next = new Node(), search = new Node(); search.value = '';
  const nameHeader = new Node({ 'data-key': 'name', 'data-sortable': 'true' });
  const amountHeader = new Node({ 'data-key': 'amount', 'data-sortable': 'true', 'data-align': 'right' });
  function richRow(name, value) {
    const row = new Node(), selectCell = new Node(), nameCell = new Node(), amount = new Node({ 'data-align': 'right' });
    selectCell.classList = { contains: () => true }; selectCell.checked = true;
    nameCell.textContent = name; amount.textContent = value;
    nameCell.classList = amount.classList = { contains: () => false };
    row.append(selectCell); row.append(nameCell); row.append(amount); return row;
  }
  const alpha = richRow('Alpha', '$250.00'), beta = richRow('Beta', '$100.00'); body.append(alpha); body.append(beta);
  Object.defineProperty(body, 'innerHTML', { set() { throw Error('original row nodes must survive'); } });
  body.querySelectorAll = selector => selector === 'tr' ? body.children : [];
  for (const [selector, node] of Object.entries({ '.mui-data-table__body': body, '.mui-data-table__info': info, '[data-action="prev"]': previous, '[data-action="next"]': next, '.mui-data-table__search': search })) root.nodes[selector] = [node];
  root.nodes['.mui-data-table__th[data-key]'] = [nameHeader, amountHeader];
  ui.init(root);
  assert.equal(alpha.hidden, false); assert.equal(beta.hidden, true);
  amountHeader.dispatchEvent(event('keydown', { key: 'Enter' }));
  assert.equal(amountHeader.getAttribute('aria-sort'), 'ascending'); assert.equal(beta.hidden, false); assert.equal(alpha.hidden, true);
  assert.equal(body.children[0], beta); assert.equal(beta.children[2].getAttribute('data-align'), 'right'); assert.equal(beta.children[0].checked, true);
  next.dispatchEvent(event('click')); assert.equal(alpha.hidden, false); assert.equal(info.textContent, 'Showing 2-2 of 2');
  search.value = 'beta'; search.dispatchEvent(event('input')); assert.equal(beta.hidden, false); assert.equal(next.disabled, true); assert.equal(info.textContent, 'Showing 1-1 of 1');
});

test("phone header keeps search and moves original breadcrumb/toolbar without copying", () => {
  const previous = window.matchMedia;
  const phone = new Node(), wide = new Node(), rail = new Node();
  phone.matches = false; wide.matches = true; rail.matches = true;
  window.matchMedia = query => query.includes('63.99rem') ? phone : query.includes('80rem') ? wide : rail;
  const shell = new Node({ 'data-mui': 'shell-navigation' }), sidebar = new Node(), main = new Node(), panel = dialog();
  const header = new Node(), context = new Node(), breadcrumb = new Node(), search = new Node(), settings = new Node(), controls = new Node(), mobile = new Node(), where = new Node(), trigger = new Node();
  const input = new Node(), language = new Node(); input.value='Amira';language.value='fr';
  search.append(input);controls.append(language);context.append(breadcrumb);settings.append(controls);
  header.append(context);header.append(search);header.append(settings);main.append(header);sidebar.append(where);sidebar.append(mobile);
  shell.append(sidebar);shell.append(main);shell.append(panel);
  shell.nodes['.mui-block--shell__sidebar']=[sidebar];shell.nodes['.mui-navigation-dialog']=[panel];shell.nodes['.mui-block--shell__main']=[main];
  main.nodes['.mui-page-header']=[header];header.nodes['.mui-page-header__context']=[context];header.nodes['.mui-page-header__settings']=[settings];
  context.nodes['.mui-breadcrumb']=[breadcrumb];sidebar.nodes['.mui-block--shell__mobile-controls']=[mobile];sidebar.nodes['.mui-block--shell__where']=[where];
  shell.nodes['.mui-page-header__controls']=[controls];shell.nodes['[data-mui="navigation-trigger"]']=[trigger];
  ui.init(shell);input.focus();phone.matches=true;phone.dispatchEvent(event('change'));
  assert.equal(search.parentNode,header);assert.equal(breadcrumb.parentNode,where);assert.equal(controls.parentNode,mobile);assert.equal(document.activeElement,input);
  panel.dispatchEvent(event('mui:navigation-open'));panel.showModal();assert.equal(sidebar.parentNode,panel);
  phone.matches=false;phone.dispatchEvent(event('change'));
  assert(!panel.open);assert.equal(sidebar.parentNode,shell);assert.equal(breadcrumb.parentNode,context);assert.equal(controls.parentNode,settings);
  assert.equal(input.value,'Amira');assert.equal(language.value,'fr');
  shell.isConnected=false;phone.dispatchEvent(event('change'));wide.dispatchEvent(event('change'));rail.dispatchEvent(event('change'));
  assert.equal(phone.listeners.change.length,0);window.matchMedia=previous;
});

test("workspace search closes the modal before focusing the only guest search", () => {
  const trigger = new Node({ 'data-mui': 'workspace-search' }), shell = new Node(), panel = dialog(), input = new Node(), menu = new Node();
  trigger.closest = selector => selector === 'dialog' ? panel : shell;
  shell.nodes['.mui-worklist-header input[type="search"]'] = [input];
  panel.addEventListener('close', () => menu.focus()); panel.showModal(); ui.init(trigger);
  trigger.dispatchEvent(event('click'));
  assert(!panel.open); assert.equal(document.activeElement, input);
});

test("gallery initial, saved and toggled themes use one root in both viewport modes", () => {
  const source = readFileSync('src/showcase/mod.rs', 'utf8');
  const start = source.indexOf('    function setTheme(theme)');
  const end = source.indexOf('    // ── Swatch click-to-copy', start);
  for (const width of [390, 1280]) for (const saved of [null, 'light', 'dark', 'invalid']) {
    const root = new Node({ 'data-theme': 'light' }), button = new Node();
    const local = new Map(saved ? [['mui-theme', saved]] : []);
    vm.runInNewContext(source.slice(start, end), {
      document: { documentElement: root, getElementById: id => id === 'theme-toggle' ? button : null },
      localStorage: { getItem: key => local.get(key), setItem: (key, value) => local.set(key, value) },
      window: { innerWidth: width },
    });
    const initial = saved === 'dark' ? 'dark' : 'light';
    assert.equal(root.getAttribute('data-theme'), initial);
    button.dispatchEvent(event('click', { stopImmediatePropagation() {} }));
    assert.equal(root.getAttribute('data-theme'), initial === 'dark' ? 'light' : 'dark');
    assert.equal(local.get('mui-theme'), root.getAttribute('data-theme'));
    button.dispatchEvent(event('click', { stopImmediatePropagation() {} }));
    assert.equal(root.getAttribute('data-theme'), initial);
  }
});
