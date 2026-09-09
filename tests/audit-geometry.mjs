// Real Chromium geometry, using an installed Puppeteer module; no package installation.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import { pathToFileURL } from 'node:url';
const args=process.argv.slice(2), value=key=>args[args.indexOf(key)+1];
const module=process.env.MUI_BROWSER_MODULE || 'puppeteer';
const {default:puppeteer}=await import(module.startsWith('/')?pathToFileURL(module).href:module);
const browser=await puppeteer.launch({headless:true});const results=[];
try {
  for (const width of [1280,390]) for (const scheme of ['light','dark']) {
    const page=await browser.newPage();
    await page.setViewport({width,height:900,hasTouch:width===390});
    await page.emulateMediaFeatures([{name:'prefers-color-scheme',value:scheme}]);
    if (args.includes('--url')) {
      await page.setExtraHTTPHeaders({'x-rsc-role':args.includes('--role')?value('--role'):'receptionist'});
      await page.goto(value('--url'),{waitUntil:'networkidle2'});
    } else await page.setContent(fs.readFileSync(value('--html'),'utf8'),{waitUntil:'load'});
    const measure=()=>page.evaluate(()=>{
      const details=document.querySelector('[data-mui="header-search"]'),input=details.querySelector('input'),button=details.querySelector('button');
      const rect=n=>{const r=n.getBoundingClientRect();return {x:r.x,y:r.y,w:r.width,h:r.height,visible:n.checkVisibility()};};
      const a=rect(input),b=rect(button),hit=document.elementFromPoint(b.x+b.w/2,b.y+b.h/2);
      return {open:details.open,input:a,button:b,hit:!!hit&&(hit===button||button.contains(hit)),overlap:Math.max(0,Math.min(a.x+a.w,b.x+b.w)-Math.max(a.x,b.x))*Math.max(0,Math.min(a.y+a.h,b.y+b.h)-Math.max(a.y,b.y))};
    });
    const closed=await measure();
    if (width===390) { assert(!closed.open);assert.equal(closed.button.w,0);assert.equal(closed.input.w,0); }
    else assert(closed.open && closed.input.visible, 'desktop search must be a real visible input');
    if (width===390) await page.click('[data-mui="header-search"] > summary');
    const open=await measure();const verify=m=>{assert(m.input.visible&&m.button.visible);assert(m.hit);assert.equal(m.overlap,0);assert(m.button.w>40);assert(m.button.h>=(width===390?44:32));};verify(open);
    await page.evaluate(()=>{const b=document.querySelector('.mui-page-header__submit');b.style.position='absolute';b.style.inset='0 auto auto 0';});
    const bad=await measure();assert.throws(()=>verify(bad),'overlap mutation must fail');
    await page.evaluate(()=>document.querySelector('.mui-page-header__submit').removeAttribute('style'));
    await page.focus('[data-mui="header-search"] input');await page.keyboard.type('Amara');await page.keyboard.press('Escape');
    const recovered=await page.evaluate(()=>({open:document.querySelector('[data-mui="header-search"]').open,focused:document.activeElement.matches('[data-mui="header-search"] > summary'),value:document.querySelector('[data-mui="header-search"] input').value}));
    if(width===390)assert(!recovered.open&&recovered.focused);else assert(recovered.open);assert(recovered.value.includes('Amara'));
    results.push({width,scheme,closed,open,negative:'overlap rejected',escape:'focus and query retained'});await page.close();
  }
  if(args.includes('--output'))fs.writeFileSync(value('--output'),JSON.stringify(results,null,2)+'\n');
  console.log(`Rendered search geometry: ${results.length} viewport/scheme cases; hidden boxes absent, open hit targets separate, touch sizes, Escape and overlap mutations passed`);
} finally { await browser.close(); }
