#!/usr/bin/env node
// Caller starts the candidate gallery. No baselines or npm dependencies.
import {chrome} from '../tests/chrome-cdp.mjs';
import {mkdir, mkdtemp, readFile, writeFile} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import {dirname, join, resolve} from 'node:path';
import {fileURLToPath} from 'node:url';
import {execFile} from 'node:child_process';
import {promisify} from 'node:util';

const exec = promisify(execFile);
const root = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const usage = 'node scripts/visual-check.mjs --candidate <url> [--reference https://maudui.herman.engineer] [--out <dir>] [--routes tests/visual-routes.txt] [--max-pixels 16] [--inject-candidate-css <css>]\n  --inject-candidate-css is for proving the check can fail: it adds CSS to candidate pages only.';
const noiseCSS = '*,*::before,*::after{transition:none!important;animation:none!important;caret-color:transparent!important}html,*{scroll-behavior:auto!important}';

function options() {
  const result = {reference: 'https://maudui.herman.engineer', routes: join(root, 'tests/visual-routes.txt'), maxPixels: '16', injectCandidateCss: ''};
  for (let i = 2; i < process.argv.length; i += 2) {
    const key = process.argv[i].replace(/^--/, '');
    if (!['candidate', 'reference', 'out', 'routes', 'max-pixels', 'inject-candidate-css'].includes(key) || !process.argv[i].startsWith('--') || !process.argv[i + 1]) throw Error(usage);
    result[{'max-pixels': 'maxPixels', 'inject-candidate-css': 'injectCandidateCss'}[key] || key] = process.argv[i + 1];
  }
  if (!result.candidate) throw Error(usage);
  for (const key of ['candidate', 'reference']) {
    const url = new URL(result[key]);
    if (!['http:', 'https:'].includes(url.protocol)) throw Error(`${key} must be an HTTP(S) URL`);
  }
  if (!/^\d+$/.test(result.maxPixels)) throw Error('--max-pixels must be a whole number: the most changed pixels a capture may have and still pass (default 16)');
  return result;
}

function parseRoutes(source) {
  const seen = new Set();
  return source.split(/\r?\n/).flatMap((line, index) => {
    // # inside a selector is an ID, not a comment. Comments occupy their own lines.
    line = line.trim();
    if (!line || line.startsWith('#')) return [];
    const match = /^(\/\S*)(?:\s+(.*))?$/.exec(line);
    if (!match || match[1].startsWith('//')) throw Error(`Invalid route at line ${index + 1}`);
    const route = {path: match[1], actions: []};
    const tail = match[2] || '';
    // Selectors may contain spaces and quotes; a new action starts at focus=/click=.
    const actions = [...tail.matchAll(/(?:^|\s+)(focus|click)=([\s\S]*?)(?=\s+(?:focus|click)=|$)/g)];
    if (tail && (!actions.length || actions[0].index !== 0)) throw Error(`Invalid action at line ${index + 1}`);
    for (const action of actions) {
      const selector = action[2].trim();
      if (!selector) throw Error(`Empty selector at line ${index + 1}`);
      route.actions.push({kind: action[1], selector});
    }
    // Encode the entire path so /blocks/foo cannot collide with /blocks-foo.
    route.slug = route.path === '/' ? 'home' : encodeURIComponent(route.path.slice(1));
    if (seen.has(route.slug)) throw Error(`Duplicate route/slug at line ${index + 1}: ${route.path}`);
    seen.add(route.slug);
    return [route];
  });
}

async function bounded(promise, label, ms = 30000) {
  let timer;
  try {
    return await Promise.race([promise, new Promise((_, reject) => {
      timer = setTimeout(() => reject(Error(`${label} timed out after ${ms / 1000}s`)), ms);
    })]);
  } finally { clearTimeout(timer); }
}

async function capture(browser, base, route, theme, width, path, extraCSS = '') {
  const url = new URL(route.path, base).href;
  await browser.viewport(width, 900);
  // Headless Chrome has no focused window, so :focus styles never paint and the focus=
  // action captured an unfocused field (proved 2026-09-23). Emulate a focused page.
  await browser.send('Emulation.setFocusEmulationEnabled', {enabled: true});
  await browser.send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-color-scheme', value: theme}]});
  const {identifier} = await browser.send('Page.addScriptToEvaluateOnNewDocument', {
    source: `try { localStorage.setItem('mui-theme', ${JSON.stringify(theme)}); } catch {}`,
  });
  try {
    // A blank document prevents the old page's readyState satisfying navigation's wait.
    await browser.goto('about:blank');
    const navigation = await browser.send('Page.navigate', {url});
    if (navigation.errorText) throw Error(navigation.errorText);
    await browser.until(`location.href !== 'about:blank' && document.readyState === 'complete'`, 20000);
    const status = await browser.evaluate(`(async () => {
      const entry = performance.getEntriesByType('navigation')[0];
      if (entry && entry.responseStatus) return entry.responseStatus;
      const response = await fetch(location.href, {cache: 'no-store'});
      if (response.body) await response.body.cancel();
      return response.status;
    })()`);
    if (status === 404) return {status};
    if (status < 200 || status >= 400) throw Error(`HTTP ${status} at ${url}`);
    await browser.evaluate(`(async () => {
      const style = document.createElement('style');
      style.textContent = ${JSON.stringify(noiseCSS + extraCSS)};
      document.head.appendChild(style);
      await document.fonts.ready;
      scrollTo(0, 0);
      await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    })()`);
    const before = await browser.evaluate('document.documentElement.scrollHeight');
    for (const {kind, selector} of route.actions) {
      // A DOM click, not a mouse event: the harness's mouse click scrolls the target into
      // view, which moved the sticky header and sidebar differently per run and made
      // /tabs differ by ~21k pixels between two identical builds (2026-09-23).
      if (kind === 'click') await browser.evaluate(`(() => {
        const element = document.querySelector(${JSON.stringify(selector)});
        if (!element) throw Error('Missing click target: ' + ${JSON.stringify(selector)});
        element.click();
      })()`);
      else await browser.evaluate(`(() => {
        const element = document.querySelector(${JSON.stringify(selector)});
        if (!element) throw Error('Missing focus target: ' + ${JSON.stringify(selector)});
        element.focus({preventScroll: true});
        if (document.activeElement !== element) throw Error('Could not focus: ' + ${JSON.stringify(selector)});
      })()`);
    }
    await browser.evaluate(`(async () => {
      scrollTo({top: 0, left: 0, behavior: 'instant'});
      await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    })()`);
    const after = await browser.evaluate('document.documentElement.scrollHeight');
    const height = Math.min(after, 4000);
    const {data} = await browser.send('Page.captureScreenshot', {
      format: 'png', captureBeyondViewport: true, fromSurface: true,
      clip: {x: 0, y: 0, width, height, scale: 1},
    });
    await writeFile(path, Buffer.from(data, 'base64'));
    return {status, before, after, height};
  } finally {
    await browser.send('Page.removeScriptToEvaluateOnNewDocument', {identifier});
  }
}

const escapeHTML = value => String(value).replace(/[&<>"']/g, c => ({'&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;'}[c]));

async function report(out, rows, opts) {
  const sorted = [...rows].sort((a, b) => (a.match ?? Infinity) - (b.match ?? Infinity));
  const content = sorted.map(row => {
    const images = ['ref', 'cand', 'diff'].map(kind => {
      if (!row.images[kind]) return `<td>${kind === 'ref' && row.newPage ? '404 — new page' : 'Unavailable'}</td>`;
      const path = escapeHTML(row.images[kind].split('/').map(encodeURIComponent).join('/'));
      return `<td><a href="${path}"><img loading="lazy" src="${path}" alt="${kind}: ${escapeHTML(row.path)} ${row.theme} ${row.width}"></a></td>`;
    }).join('');
    const heights = ['ref', 'cand'].flatMap(kind => {
      const capture = row[kind];
      if (capture?.before === undefined) return [];
      return [`${kind}: ${capture.before} → ${capture.after}px${capture.before !== capture.after ? ' — HEIGHT CHANGE' : ''}${capture.after > 4000 ? ' (image capped at 4000px)' : ''}`];
    });
    return `<tr><th scope="row">${escapeHTML(row.path)}<br>${row.theme} ${row.width}<br><strong>${row.match === undefined ? (row.newPage ? 'NEW PAGE' : 'ERROR') : row.match.toFixed(2) + '%'}</strong>${[...heights, ...row.notes].map(note => `<p>${escapeHTML(note)}</p>`).join('')}</th>${images}</tr>`;
  }).join('\n');
  await writeFile(join(out, 'index.html'), `<!doctype html>
<html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>maud-ui visual check</title><style>
:root{color-scheme:light dark;font:15px system-ui}body{margin:24px}table{border-collapse:collapse;width:100%;table-layout:fixed}th,td{border:1px solid #888;padding:8px;vertical-align:top;overflow-wrap:anywhere}th:first-child{width:18%}img{width:100%;height:auto;display:block}p{font-weight:normal}a{color:inherit}
</style><h1>maud-ui visual check</h1>
<p>Reference: ${escapeHTML(opts.reference)}<br>Candidate: ${escapeHTML(opts.candidate)}<br>Fails above ${escapeHTML(opts.maxPixels)} changed pixels. Worst pixel match first. Height changes are findings independent of pixel scores. Click images for full resolution.</p>
<table><thead><tr><th>Capture / findings</th><th>Reference</th><th>Candidate</th><th>Heatmap</th></tr></thead><tbody>${content}</tbody></table></html>`);
}

async function main() {
  const opts = options();
  const out = opts.out ? resolve(opts.out) : await mkdtemp(join(tmpdir(), 'maud-ui-visual-'));
  await mkdir(out, {recursive: true});
  console.log(`Output: ${out}`);
  // pixel-diff.py needs Pillow, and the first python3 on PATH often lacks it (Homebrew's does).
  const python = process.env.MAUD_UI_PYTHON || 'python3';
  try { await exec(python, ['-c', 'import PIL'], {timeout: 30000}); }
  catch { throw Error(`${python} cannot import PIL, which scripts/pixel-diff.py needs: run \`${python} -m pip install pillow\`, or set MAUD_UI_PYTHON to a python that has it (e.g. /opt/homebrew/bin/python3.11)`); }
  const routes = parseRoutes(await readFile(opts.routes, 'utf8'));
  if (!routes.length) throw Error('No routes to capture');
  const rows = [];
  let browser, failures = 0, below = 0, compared = 0, resized = 0;
  try {
    browser = await chrome();
    for (const route of routes) for (const theme of ['light', 'dark']) for (const width of [1440, 390]) {
      const row = {...route, theme, width, images: {}, notes: []};
      rows.push(row);
      await mkdir(join(out, route.slug), {recursive: true});
      let side = 'ref';
      try {
        for (const kind of ['ref', 'cand']) {
          side = kind;
          const relative = `${route.slug}/${theme}-${width}-${kind}.png`;
          row[kind] = await bounded(capture(browser, kind === 'ref' ? opts.reference : opts.candidate, route, theme, width, join(out, relative), kind === 'cand' ? opts.injectCandidateCss : ''), `${kind} capture`);
          if (row[kind].status !== 404) row.images[kind] = relative;
          if (row[kind].before !== row[kind].after) {
            console.error(`HEIGHT CHANGE ${route.path} ${theme} ${width} ${kind}: ${row[kind].before} → ${row[kind].after}px`);
          }
        }
        if (row.cand.status === 404) throw Error('Candidate returned HTTP 404');
        if (row.ref.status === 404) {
          row.newPage = true;
          below++;  // nothing to compare against, so it needs the same human look as a change
          console.log(`NEW PAGE ${route.path} ${theme} ${width} (reference HTTP 404): review it in the report`);
          continue;
        }
        // An action (a tab click, a focus) that resizes the page is the 0.19.3 "tab switch
        // resized the page" regression: pixels can still match, so it is its own failure.
        const refGrow = row.ref.after - row.ref.before, candGrow = row.cand.after - row.cand.before;
        if (candGrow !== refGrow) {
          resized++;
          const note = `ACTION RESIZED THE PAGE by ${candGrow}px (previous release: ${refGrow}px)`;
          row.notes.push(note);
          console.log(`${note}  ${route.path} ${theme} ${width}`);
        }
        if (row.ref.after !== row.cand.after) {
          const note = `HEIGHT MISMATCH ref ${row.ref.after}px cand ${row.cand.after}px`;
          row.notes.push(note);
          console.error(`${note}: ${route.path} ${theme} ${width}`);
        }
        side = 'diff';
        const diff = `${route.slug}/${theme}-${width}-diff.png`;
        const {stdout} = await exec(python, [join(root, 'scripts/pixel-diff.py'), join(out, row.images.ref), join(out, row.images.cand), '--out', join(out, diff), '--tol', '2'], {timeout: 30000});
        const match = /^match (\d+(?:\.\d+)?)%/m.exec(stdout);
        if (!match) throw Error(`Unrecognized pixel-diff output: ${stdout}`);
        row.match = Number(match[1]);
        row.images.diff = diff;
        compared++;
        // --tol 2, not pixel-diff's default 24: a light-accent shift from #6e79d6 to #5e6ad2 is
        // 16 per channel and passed unseen at 24. Captures are deterministic; at 2 the only noise
        // is zero pages of 84 (at 0 it is four), measured 2026-09-23.
        const differ = /^differ (\d+) pixels/m.exec(stdout);
        if (!differ) throw Error(`pixel-diff printed no 'differ N pixels' line: ${stdout}`);
        row.differ = Number(differ[1]);
        if (row.differ > Number(opts.maxPixels)) {
          below++;
          console.log(`${row.differ} px changed (${row.match.toFixed(2)}%)  ${route.path === '/' ? '/' : route.path.slice(1)}  ${theme} ${width}`);
        }
      } catch (error) {
        failures++;
        row.notes.push(`${side}: ${error.message}`);
        console.error(`ERROR ${route.path} ${theme} ${width} ${side}: ${error.message}`);
        // Recreate after errors: a timed-out CDP operation must not affect the next capture.
        await browser.close();
        browser = undefined;
        browser = await chrome();
      }
    }
  } catch (error) {
    failures++;
    console.error(`ERROR ${rows.at(-1)?.path || routes[0].path}: ${error.message}`);
    if (!rows.length) rows.push({...routes[0], theme: 'light', width: 1440, images: {}, notes: [error.message]});
    else rows.at(-1).notes.push(error.message);
  } finally {
    try {
      if (browser) await browser.close();
    } catch (error) {
      failures++;
      console.error(`ERROR closing Chrome: ${error.message}`);
    }
    await report(out, rows, opts);
  }
  console.log(`${below} of ${compared} captures changed more than ${opts.maxPixels} pixels; ${resized} resized by an action; report: ${join(out, 'index.html')}`);
  process.exitCode = failures ? 2 : (below || resized) ? 1 : 0;
}

main().catch(error => { console.error(`ERROR: ${error.message}`); process.exitCode = 2; });
