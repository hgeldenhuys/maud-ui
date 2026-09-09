// Supervisor-only rendered geometry check. The design lane does not launch browsers.
// MUI_BROWSER_MODULE=/path/to/puppeteer node tests/shell-frame-browser.mjs http://127.0.0.1:23449
import assert from 'node:assert/strict';
import { writeFileSync } from 'node:fs';
import { pathToFileURL } from 'node:url';
const module = process.env.MUI_BROWSER_MODULE || 'puppeteer';
const { default: puppeteer } = await import(module.startsWith('/') ? pathToFileURL(module).href : module);
const origin = process.argv[2] || 'http://127.0.0.1:23449';
const browser = await puppeteer.launch({headless: true}), results = [];
const modes = ['compact', 'comfortable', 'spacious'];
try {
  for (const width of [1280, 390]) for (const theme of ['light', 'dark']) for (const length of ['short', 'long']) {
    const signatures = [];
    for (const density of modes) {
      const page = await browser.newPage();
      await page.setViewport({width, height: 900, hasTouch: width === 390});
      await page.goto(`${origin}/fixtures/shell-frame-${density}-${length}.html`, {waitUntil: 'networkidle0'});
      await page.evaluate(theme => document.documentElement.setAttribute('data-theme', theme), theme);
      const measure = () => page.evaluate(() => {
        const node = name => document.querySelector('.' + name);
        const rect = name => { const r = node(name).getBoundingClientRect(); return {x:r.x, y:r.y, width:r.width, height:r.height, bottom:r.bottom}; };
        const content = node('mui-block--shell__content').firstElementChild.getBoundingClientRect();
        return {masthead:rect('mui-app-header'), body:rect('mui-block--shell__body'), column:rect('mui-block--shell__sidebar-column'), sidebar:rect('mui-block--shell__sidebar'), main:rect('mui-block--shell__main'), footer:rect('mui-app-footer'), content:{x:content.x,y:content.y}, viewport:innerHeight};
      });
      const geometry = await measure();
      assert(geometry.footer.bottom >= geometry.viewport - 1);
      if (length === 'short') assert(Math.abs(geometry.footer.bottom - geometry.viewport) < 1, 'short page has no surplus shell height');
      const signature = [geometry.masthead.height, geometry.main.x, geometry.main.y, geometry.content.x, geometry.content.y, geometry.footer.height];
      if (width === 1280) {
        assert(Math.abs(geometry.column.height - geometry.body.height) < 1);
        assert(geometry.sidebar.height <= 901);
        signature.push(geometry.sidebar.x, geometry.sidebar.y, geometry.column.width);
        assert(await page.$eval('.mui-block--shell__sidebar', n => n.scrollHeight > n.clientHeight), 'long menu scrolls independently');
      } else {
        assert.equal(geometry.column.height, 0);
        await page.click('[data-mui="navigation-trigger"]');
        assert(await page.$eval('.mui-navigation-dialog', n => n.open));
        await page.keyboard.press('Escape');
        await page.waitForFunction(() => document.querySelector('.mui-block--shell__sidebar').parentElement.matches('.mui-block--shell__sidebar-column'));
      }
      signatures.push(signature);
      const shape = await page.evaluate(() => {
        const banner = document.querySelector('.mui-attention-banner'), card = document.querySelector('.mui-card'), group = document.querySelector('[data-contains-current="true"] > summary');
        const radius = n => parseFloat(getComputedStyle(n).borderTopLeftRadius);
        const controls = [...document.querySelectorAll('.mui-btn,.mui-input,.mui-native-select,.mui-status-chip-group__chip')].filter(n=>n.checkVisibility());
        return {gap:card.getBoundingClientRect().top-banner.getBoundingClientRect().bottom, banner:radius(banner), card:radius(card), controls:controls.map(n=>[radius(n),n.getBoundingClientRect().height]), parent:getComputedStyle(group).backgroundColor};
      });
      assert(Math.abs(shape.gap - 16) < 1); assert.equal(shape.banner, 12); assert.equal(shape.card, 12);
      assert(shape.controls.every(([r,h]) => r <= h / 3)); assert.equal(shape.parent, 'rgba(0, 0, 0, 0)');
      if (width === 1280 && density === 'compact' && length === 'short') {
        await page.$eval('.mui-block--shell__content', n => {n.style.padding = '20px';});
        const broken = await measure(); assert.notEqual(broken.content.x, geometry.content.x, 'old density padding must move the frame');
        await page.$eval('.mui-block--shell__content', n => n.removeAttribute('style'));
        await page.evaluate(() => document.documentElement.style.setProperty('--mui-radius-lg', '24px'));
        const badRadius = await page.$eval('.mui-card', n => parseFloat(getComputedStyle(n).borderTopLeftRadius));
        assert.throws(() => assert(badRadius <= 12), 'the old 24px corner must fail the clamp');
      }
      results.push({width,theme,length,density,geometry,shape}); await page.close();
    }
    for (const signature of signatures.slice(1)) assert.deepEqual(signature, signatures[0], 'density moved the frame');
  }
  const nojs = await browser.newPage(); await nojs.setJavaScriptEnabled(false); await nojs.setViewport({width:390,height:900});
  await nojs.goto(`${origin}/fixtures/shell-frame-compact-short.html`);
  assert(await nojs.$eval('.mui-block--shell__sidebar', n => n.checkVisibility()));
  await nojs.click('.mui-navigation-fallback > summary');
  assert(!(await nojs.$eval('.mui-block--shell__sidebar', n => n.checkVisibility())));
  await nojs.close();
  writeFileSync('docs/night-4-browser-geometry.json', JSON.stringify(results,null,2)+'\n');
  console.log('24 rendered viewport/theme/density/length cases passed; frame, footer, sidebar, gaps, radius, current parent and no-JS navigation verified.');
} finally { await browser.close(); }
