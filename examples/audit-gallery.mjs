// Browser-free route, asset, HTML-identity and inline-script verification.
// Run against a freshly compiled gallery: node examples/audit-gallery.mjs http://127.0.0.1:PORT
import { readFileSync, writeFileSync } from 'node:fs';
import vm from 'node:vm';
import assert from 'node:assert/strict';
const origin = process.argv[2];
assert(origin, 'Pass the local gallery origin');
const landing = await (await fetch(origin)).text();
const index = landing.match(/window\.__MUI_PALETTE__ = (\[[\s\S]*?\]);/);
assert(index, 'Missing command-palette registry');
const routes = [...new Set([...index[1].matchAll(/u: ("(?:[^"\\]|\\.)*")/g)].map(m => JSON.parse(m[1])))];
const scripts = new Map(), pages = [];
for (let i = 0; i < routes.length; i += 6) {
  const results = await Promise.allSettled(routes.slice(i, i + 6).map(async path => {
    const response = await fetch(origin + path);
    const html = await response.text();
    assert.equal(response.status, 200, path);
    assert(/<!doctype html>/i.test(html), `${path}: not HTML`);
    assert(!/<title>Not Found/i.test(html), `${path}: missing showcase`);
    // Count only actual opening tags; documentation contains escaped HTML examples.
    const markup = html.replace(/<(script|style)\b[^>]*>[\s\S]*?<\/\1>/gi, '');
    const ids = [...markup.matchAll(/<[a-z][\w:-]*\b[^>]*?\sid="([^"]+)"[^>]*>/gi)].map(m => m[1]);
    const duplicates = [...new Set(ids.filter((id, index) => ids.indexOf(id) !== index))];
    for (const match of html.matchAll(/<script\b([^>]*)>([\s\S]*?)<\/script>/gi)) {
      if (/\bsrc=/.test(match[1]) || /type="(?:application\/ld\+json|application\/json|importmap)"/.test(match[1]) || !match[2].trim()) continue;
      if (!scripts.has(match[2])) {
        if (/type="module"/.test(match[1])) {
          const { transform } = await import('esbuild'); await transform(match[2], { loader: 'js', format: 'esm' });
        } else new vm.Script(match[2], { filename: path });
        scripts.set(match[2], path);
      }
    }
    return { path, status: response.status, bytes: Buffer.byteLength(html), ids: ids.length, duplicate_ids: duplicates };
  }));
  for (const result of results) { if (result.status === 'rejected') throw result.reason; pages.push(result.value); }
}
const assets = [];
for (const [url, file] of [['/css/maud-ui.css', 'static/maud-ui.css'], ['/js/maud-ui.js', 'static/maud-ui.js'], ['/og.png', 'static/og.png']]) {
  const response = await fetch(origin + url); assert.equal(response.status, 200, url);
  const bytes = Buffer.from(await response.arrayBuffer()); assert(bytes.equals(readFileSync(file)), `${url}: stale compiled asset`);
  assets.push({ path: url, bytes: bytes.length, matches: file });
}
for (const page of pages) assert.equal(page.duplicate_ids.length, 0, `${page.path}: duplicate IDs ${page.duplicate_ids}`);
writeFileSync(process.argv[3] || 'docs/design-http-audit.json', JSON.stringify({ origin, pages: pages.length, assets, unique_inline_scripts: scripts.size, results: pages }, null, 2) + '\n');
console.log(`${pages.length} pages returned 200 with unique IDs; ${assets.length} assets byte-exact; ${scripts.size} unique inline scripts parsed.`);
