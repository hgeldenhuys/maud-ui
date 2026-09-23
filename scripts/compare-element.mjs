// compare-element — the same element on the live site and on a local build, side by side.
//
//   node scripts/compare-element.mjs <route> <selector> [--candidate http://localhost:3456]
//        [--reference https://maudui.herman.engineer] [--theme light|dark] [--width 1440]
//        [--pad 24] [--out compare.png]
//
// Writes one PNG: reference on the left, candidate on the right, each cropped to the element's
// box plus padding, and prints both boxes. Built 2026-09-23 after the same comparison was
// hand-rolled four times in one session (combobox, popover, gallery nav, shell nav rows):
// visual-check.mjs answers "did anything change", this answers "show me that one thing".
// The route is written once; the reference gets a trailing slash because the live site is a
// static export that 404s some routes without it.
import {chrome} from '../tests/chrome-cdp.mjs';
import {writeFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import {tmpdir} from 'node:os';
import {join} from 'node:path';

const args = process.argv.slice(2);
const usage = 'usage: node scripts/compare-element.mjs <route> <selector> [--candidate URL] [--reference URL] [--theme light|dark] [--width N] [--pad N] [--out file.png]';
const opt = {candidate: 'http://localhost:3456', reference: 'https://maudui.herman.engineer', theme: 'dark', width: '1440', pad: '24', out: 'compare.png'};
const pos = [];
for (let i = 0; i < args.length; i++) {
  if (args[i].startsWith('--')) {
    const key = args[i].slice(2);
    if (!(key in opt) || args[i + 1] === undefined) { console.error(usage); process.exit(2); }
    opt[key] = args[++i];
  } else pos.push(args[i]);
}
if (pos.length !== 2) { console.error(usage); process.exit(2); }
const [route, selector] = pos;
if (!['light', 'dark'].includes(opt.theme)) { console.error(`--theme must be light or dark, not ${opt.theme}`); process.exit(2); }

const b = await chrome();
const shots = [];
try {
  await b.send('Emulation.setEmulatedMedia', {features: [{name: 'prefers-color-scheme', value: opt.theme}]});
  await b.send('Page.addScriptToEvaluateOnNewDocument', {source: `try{localStorage.setItem('mui-theme',${JSON.stringify(opt.theme)})}catch{}`});
  await b.viewport(Number(opt.width), 900);
  for (const [label, base] of [['reference', opt.reference], ['candidate', opt.candidate]]) {
    const path = route.replace(/^\/?/, '/');
    const url = base.replace(/\/$/, '') + (label === 'reference' && !path.endsWith('/') ? path + '/' : path);
    await b.send('Page.navigate', {url});
    await b.until(`document.readyState === 'complete' && location.href !== 'about:blank'`, 20000);
    await b.evaluate('document.fonts.ready.then(()=>true)');
    const box = await b.evaluate(`(()=>{const n=document.querySelector(${JSON.stringify(selector)}); if(!n) return null;
      document.documentElement.style.scrollBehavior='auto'; n.scrollIntoView({block:'center',behavior:'instant'});
      const r=n.getBoundingClientRect(); return {x:r.x+scrollX,y:r.y+scrollY,w:r.width,h:r.height}})()`);
    if (!box) throw Error(`${label}: no element matches ${selector} at ${url}`);
    const p = Number(opt.pad);
    const clip = {x: Math.max(0, box.x - p), y: Math.max(0, box.y - p), width: Math.min(Number(opt.width), box.w + 2 * p), height: Math.min(1600, box.h + 2 * p), scale: 1};
    const {data} = await b.send('Page.captureScreenshot', {format: 'png', captureBeyondViewport: true, clip});
    const file = join(tmpdir(), `compare-${label}-${process.pid}.png`);
    writeFileSync(file, Buffer.from(data, 'base64'));
    shots.push(file);
    console.log(`${label.padEnd(9)} ${url}  ${Math.round(box.w)}x${Math.round(box.h)} at ${Math.round(box.x)},${Math.round(box.y)}`);
  }
} finally {
  await b.close();
}
// Pillow joins the two crops; the house pixel-diff tooling already depends on it.
const python = process.env.MAUD_UI_PYTHON || 'python3';
execFileSync(python, ['-c', `
import sys
from PIL import Image
a, b = Image.open(sys.argv[1]).convert('RGB'), Image.open(sys.argv[2]).convert('RGB')
o = Image.new('RGB', (a.width + b.width + 16, max(a.height, b.height)), (220, 38, 38))
o.paste(a, (0, 0)); o.paste(b, (a.width + 16, 0)); o.save(sys.argv[3])
`, shots[0], shots[1], opt.out], {stdio: 'inherit'});
console.log(`wrote ${opt.out} (reference left, candidate right)`);
