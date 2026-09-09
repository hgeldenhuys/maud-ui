// Source contracts only. This does not claim browser layout or animation measurements.
import assert from 'node:assert/strict';
import { readFileSync, readdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const walk = dir => readdirSync(dir, { withFileTypes: true }).flatMap(entry => entry.isDirectory() ? walk(join(dir, entry.name)) : [join(dir, entry.name)]);
const files = [...walk('src'), ...walk('static/styles')].filter(path => /\.(rs|css)$/.test(path));
const violations = [];
for (const file of files) {
  const source = readFileSync(file, 'utf8');
  // Token declarations are the single legitimate home for numeric type sizes.
  if (file !== 'static/styles/tokens.css' && /\b(?:font-size|fontSize)['"]?\s*[:=]\s*['"]?[.\d]/.test(source)) violations.push(`${file}: fixed font size`);
  if (/style="[^"\n]*font-size\s*:/.test(source)) violations.push(`${file}: local font-size declaration`);
  if (/\b(?:animation|transition)(?:-duration|-delay)?\s*:[^;{}]*\b(?:[1-9]\d*(?:\.\d+)?|0\.\d+)(?:ms|s)\b/.test(source)) violations.push(`${file}: untokenized motion`);
}
assert.deepEqual(violations, []);
const tokens = readFileSync('static/styles/tokens.css', 'utf8');
const durations = [...tokens.matchAll(/(--mui-motion-[\w-]+):\s*(\d+(?:\.\d+)?m?s);/g)].map(match => match[1]);
assert.deepEqual(durations, ['--mui-motion-fast', '--mui-motion-enter']);
for (const file of ['static/maud-ui.css', 'static/maud-ui.min.css']) {
  const css = readFileSync(file, 'utf8');
  const definitions = new Set([...css.matchAll(/@keyframes\s+(mui-[\w-]+)/g)].map(match => match[1]));
  for (const match of css.matchAll(/animation(?:-name)?:\s*(mui-[\w-]+)/g)) assert(definitions.has(match[1]), `${file}: undefined ${match[1]}`);
  for (const name of ['surface-enter', 'backdrop-enter', 'notice-enter', 'notice-exit', 'shimmer']) assert(definitions.has(`mui-${name}`), `${file}: ${name}`);
  assert(!definitions.has('mui-panel-enter'), 'Old surface animation survived');
  const final = css.lastIndexOf('@media (prefers-reduced-motion: reduce)') >= 0 ? css.lastIndexOf('@media (prefers-reduced-motion: reduce)') : css.lastIndexOf('@media(prefers-reduced-motion:reduce)');
  assert(final >= 0);
  const reduced = css.slice(final, final + 400).replaceAll(/\s/g, '');
  for (const rule of ['animation:none!important', 'transition:none!important']) {
    assert(reduced.includes(rule), `${file}: missing reduced-motion ${rule}`);
  }
  assert(/\.mui-skeleton::?after\{display:none;?\}/.test(reduced));
}
const result = { source_files: files.length, fixed_font_sizes: 0, local_font_size_declarations: 0, untokenized_motion: 0, duration_tokens: durations, bundles_checked: 2, browser_review: 'Not performed; supervisor review required.' };
writeFileSync(process.argv[2] || 'docs/states-motion-audit.json', JSON.stringify(result, null, 2) + '\n');
console.log(`${files.length} source files checked: no local font sizes or untokenized motion; two duration tokens and reduced-motion contracts pass in both bundles.`);
