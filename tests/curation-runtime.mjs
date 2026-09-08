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
  insertBefore(node, reference) {
    this.append(node);
    this.children = this.children.filter(child => child !== node);
    this.children.splice(Math.max(0, this.children.indexOf(reference)), 0, node);
  }
}
const document = new Node();
document.body = new Node();
document.documentElement = new Node();
const byId = new Map();
document.getElementById = id => byId.get(id) || null;
const media = new Node(); media.matches = true;
const window = new Node(); window.matchMedia = () => media;
const context = { window, document, Element: Node, CustomEvent: class { constructor(type) { this.type = type; } }, getComputedStyle: node => ({ direction: node.direction || "ltr" }), setTimeout, clearTimeout, console };
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
