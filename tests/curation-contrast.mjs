// Token arithmetic, not a browser or a screenshot/contrast sweep.
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import vm from "node:vm";
const source = readFileSync("src/showcase/mod.rs", "utf8");
const presetSource = source.split("var PRESETS = ")[1].split(";\n")[0];
const presets = vm.runInNewContext(`(${presetSource})`);
const matchStart = source.indexOf("  function matchesPreset(");
const matchEnd = source.indexOf("  function refreshExport()", matchStart);
const matchesPreset = vm.runInNewContext(`${source.slice(matchStart, matchEnd)}\nmatchesPreset`);
assert(matchesPreset(presets.dark, "dark", {}));
assert(!matchesPreset(presets.dark, "light", {}));
assert(!matchesPreset(presets.dark, "dark", { "mui-radius-sm": "6px" }));
const emerald = Object.fromEntries(Object.entries(presets.emerald).filter(([key]) => !key.startsWith("_")));
assert(matchesPreset(presets.emerald, "dark", emerald));
assert(!matchesPreset(presets.emerald, "dark", { ...emerald, "mui-accent": "#ffffff" }));
const css = readFileSync("static/styles/tokens.css", "utf8");
const tokenValues = text => Object.fromEntries([...text.matchAll(/--(mui-[a-z0-9-]+):\s*([^;]+);/gi)].map(m => [m[1], m[2].trim()]));
const shared = tokenValues(css.split(':root {')[1].split('\n}')[0]);
const dark = { ...shared, ...tokenValues(css.split(':root,\n[data-theme="dark"] {')[1].split('\n}')[0]) };
const light = { ...shared, ...tokenValues(css.split('[data-theme="light"] {')[1].split('\n}')[0]) };
const resolve = (value, tokens) => value.replace(/var\(--(mui-[a-z0-9-]+)\)/g, (_, key) => resolve(tokens[key], tokens));
const luminance = hex => {
  let rgb = hex.slice(1);
  if (rgb.length === 3) rgb = [...rgb].map(c => c + c).join("");
  const channels = [0, 2, 4].map(i => parseInt(rgb.slice(i, i + 2), 16) / 255).map(v => v <= 0.04045 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4);
  return channels[0] * 0.2126 + channels[1] * 0.7152 + channels[2] * 0.0722;
};
const contrast = (a, b) => { const x = luminance(a), y = luminance(b); return (Math.max(x, y) + 0.05) / (Math.min(x, y) + 0.05); };
let checks = 0;
const failures = [];
for (const [name, preset] of Object.entries(presets)) {
  const raw = { ...(preset._base === "light" ? light : dark), ...preset };
  const tokens = Object.fromEntries(Object.entries(raw).map(([key, value]) => [key, resolve(value, raw)]));
  for (const surface of ["mui-bg", "mui-bg-card", "mui-bg-input"]) {
    for (const ink of ["mui-text", "mui-text-secondary", "mui-text-muted", "mui-text-subtle", "mui-accent-text", "mui-info-text", "mui-success-text", "mui-warning-text", "mui-danger-text"]) {
      const ratio = contrast(tokens[ink], tokens[surface]);
      if (ratio < 4.5) failures.push(`${name}: ${ink} / ${surface} = ${ratio.toFixed(2)}`); checks++;
    }
    for (const boundary of ["mui-border-control", "mui-border-focus"]) {
      const ratio = contrast(tokens[boundary], tokens[surface]);
      if (ratio < 3) failures.push(`${name}: ${boundary} / ${surface} = ${ratio.toFixed(2)}`); checks++;
    }
  }
  const activeRatio = contrast(tokens['mui-accent-text'], tokens['mui-accent-soft']);
  checks++; if (activeRatio < 4.5) failures.push(`${name}: active text/tint = ${activeRatio.toFixed(2)}`);
  for (const tone of ["info", "success", "warning", "danger", "violet", "rose"]) {
    const fillRatio = contrast(tokens[`mui-${tone}`], tokens[`mui-${tone}-fg`]);
    if (fillRatio < 4.5) failures.push(`${name}: ${tone} fill / ink = ${fillRatio.toFixed(2)}`); checks++;
    const ratio = contrast(tokens[`mui-${tone}-text`], tokens[`mui-${tone}-bg`]);
    if (ratio < 4.5) failures.push(`${name}: ${tone} text / background = ${ratio.toFixed(2)}`); checks++;
  }
  for (const [fill, ink] of [["mui-accent", "mui-accent-fg"], ["mui-accent-hover", "mui-accent-fg"], ["mui-danger-hover", "mui-danger-fg"]]) {
    const ratio = contrast(tokens[fill], tokens[ink]);
    if (ratio < 4.5) failures.push(`${name}: ${ink} / ${fill} = ${ratio.toFixed(2)}`); checks++;
  }
}
assert.equal(failures.length, 0, failures.join("\n"));
for (const match of source.matchAll(/data-preset="([a-z0-9-]+)"/g)) assert(presets[match[1]], `Unwired preset ${match[1]}`);
const controls = [...source.split('// ── Left column: controls')[1].split('// ── Right column: live preview')[0].matchAll(/\("(mui-[a-z0-9-]+)"/g)].map(m => m[1]);
const registry = [...source.split("var TOKENS = [")[1].split("]; ")[0].split("];")[0].matchAll(/name: '(mui-[a-z0-9-]+)'/g)].map(m => m[1]);
assert.deepEqual(controls.sort(), registry.sort(), "theme inputs and persistence must agree");
const styles = new Map([["--mui-old-custom-token", "old"]]);
const storage = new Map();
let baseTheme;
const state = {
  TOKENS: registry.map(name => ({ name })), PRESETS: presets,
  overrides: { "mui-old-custom-token": "old" },
  document: { documentElement: { style: { removeProperty: key => styles.delete(key) }, setAttribute: (_, value) => { baseTheme = value; } } },
  localStorage: { setItem: (key, value) => storage.set(key, value) },
  applyToken: (key, value) => styles.set("--" + key, value),
  syncControlsFromComputed() {}, save() {}, refreshExport() {},
};
const switching = source.slice(source.indexOf("  function clearOverrides()"), source.indexOf("  function syncControlsFromComputed()"));
vm.runInNewContext(`${switching}\napplyPreset('emerald');`, state);
assert(!styles.has("--mui-old-custom-token"));
assert.equal(styles.get("--mui-accent-fg"), "#022c22");
vm.runInNewContext("applyPreset('light');", state);
assert.equal(styles.size, 0, "base presets must clear every previous preset override");
assert.equal(baseTheme, "light");
assert.equal(storage.get("mui-theme"), "light");
console.log(`${checks} contrast checks passed across ${Object.keys(presets).length} presets; theme controls, reset and persistence passed.`);
