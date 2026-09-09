// Limited, browser-free cascade arithmetic; no font shaping or layout engine.
import assert from 'node:assert/strict';

export function parse(text, context = []) {
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
  if (/::?before$/.test(selector) && node.pseudo === 'before') selector = selector.replace(/::?before$/, '');
  else if (node.pseudo) return false;
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
  const parts = selector.replace(/\s*>\s*/g, ' > ').split(/\s+/); let current = node;
  if (!compoundMatches(parts.pop(), current)) return false;
  while (parts.length) {
    let part = parts.pop();
    if (part === '>') { part = parts.pop(); current = current.parent; if (!compoundMatches(part, current)) return false; }
    else { current = current.parent; while (current && !compoundMatches(part, current)) current = current.parent; if (!current) return false; }
  }
  return true;
}
export const el = (tag, classes, attrs = {}, parent = null) => ({ tag, classes: classes.split(' ').filter(Boolean), attrs, parent });
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
export function computed(rules, node, env) {
  let owner = node; while (owner.parent) owner = owner.parent;
  const theme = owner.attrs['data-theme'] || 'dark';
  const tokens = Object.fromEntries(rules.filter(r => !r.context.length && splitSelectors(r.selector).some(s => {
    const normalized = s.replaceAll('\"', '').replaceAll("'", '');
    return normalized === ':root' || normalized === '[data-theme]' || normalized === `[data-theme=${theme}]` || (owner.attrs['data-density'] && (normalized === `:root[data-density=${owner.attrs['data-density']}]` || normalized === `[data-density=${owner.attrs['data-density']}]:not(.mui-action-row)`)) || (owner.attrs['data-brand'] && (normalized === '[data-brand]' || normalized === `[data-brand=${owner.attrs['data-brand']}]`));
  })).flatMap(r => [...r.body.matchAll(/(--[\w-]+):\s*([^;]+)(?:;|$)/g)].map(m => [m[1], m[2].trim()])));
  // Rebind tokens at real ancestor scopes (content density and fixed chrome).
  const ancestry = []; for (let current = node; current; current = current.parent) ancestry.unshift(current);
  for (const ancestor of ancestry) {
    const local = new Map();
    rules.forEach((rule, order) => {
      if (!active(rule.context, env)) return;
      for (const selector of splitSelectors(rule.selector).filter(s => matches(s, ancestor))) {
        const rank = (selector.match(/[.#\[]/g)?.length || 0) * 10;
        for (const [, name, value] of rule.body.matchAll(/(--[\w-]+):\s*([^;]+)(?:;|$)/g)) {
          const last = local.get(name);
          if (!last || rank > last.rank || (rank === last.rank && order >= last.order)) local.set(name, {value: value.trim(), rank, order});
        }
      }
    });
    for (const [name, entry] of local) tokens[name] = entry.value;
    // Inline numeric depth and explicit brand overrides from the rendered fixture.
    for (const [, name, value] of (ancestor.attrs.style || '').matchAll(/(--[\w-]+):\s*([^;]+)(?:;|$)/g)) tokens[name] = value.trim();
  }
  function resolve(value) {
    if (value === undefined) return '';
    while (value.includes('var(')) {
      const start = value.indexOf('var('); let end = start + 4, depth = 1;
      for (; end < value.length && depth; end++) { if (value[end] === '(') depth++; if (value[end] === ')') depth--; }
      const [key, ...fallback] = value.slice(start + 4, end - 1).split(',');
      const raw = tokens[key.trim()] ?? fallback.join(',').trim(); assert(raw, `Unresolved ${key}`);
      value = value.slice(0, start) + resolve(raw) + value.slice(end);
    }
    value = value.replace(/env\([^,]+,\s*([^)]+)\)/g, "$1");
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
        if (!/^(?:padding(?:-(?:block|inline|top|right|bottom|left))?|height|min-height|max-height|width|min-width|align-self|align-items|font-size|font-weight|line-height|border(?:-radius)?|box-sizing|grid-auto-flow|grid-template-columns|grid-template-rows|grid-row|flex-wrap|flex-direction|flex-shrink|display|white-space|position|top|contain|overflow-y|overscroll-behavior|gap|margin(?:-block)?|padding-inline-start|padding-inline-end|background|color|inset-inline-start)$/.test(name)) continue;
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
  return { ...Object.fromEntries([...properties].map(([name, entry]) => [name, entry.value])), token: name => resolve(tokens[name]) };
}
export const px = value => value === 'auto' || value === undefined ? 0 : Number.parseFloat(value) * (value.endsWith('rem') ? 16 : 1);
export function height(style) {
  const content = px(style['font-size']) * Number(style['line-height']) + px(style['padding-top']) + px(style['padding-bottom']) + 2 * px(style.border);
  return Math.max(px(style['min-height']), style.height === 'auto' ? content : px(style.height));
}
