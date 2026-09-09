// Real rendered shell checks. Use MUI_BROWSER_MODULE for an existing Puppeteer install.
import assert from 'node:assert/strict';
import fs from 'node:fs';
import {pathToFileURL} from 'node:url';
const args=process.argv.slice(2),value=key=>args[args.indexOf(key)+1];
const module=process.env.MUI_BROWSER_MODULE||'puppeteer';
const {default:puppeteer}=await import(module.startsWith('/')?pathToFileURL(module).href:module);
const url=args.includes('--url')?value('--url'):pathToFileURL(value('--html')).href;
const tabs=args.includes('--tabs'), role=args.includes('--role')?value('--role'):null;
const browser=await puppeteer.launch({headless:true});const results=[];
async function open(width,scheme,js=true){
  const page=await browser.newPage();await page.setViewport({width,height:900,hasTouch:width===390});
  page.navigationErrors=[];page.on('pageerror',error=>page.navigationErrors.push(error.message));
  await page.setJavaScriptEnabled(js);await page.emulateMediaFeatures([{name:'prefers-color-scheme',value:scheme}]);
  if(role)await page.setExtraHTTPHeaders({'x-rsc-role':role});
  await page.goto(url,{waitUntil:'networkidle2'});return page;
}
const sidebar='.mui-block--shell__sidebar',dialog='.mui-navigation-dialog';
const links=page=>page.$$eval(sidebar+' a[href]',nodes=>nodes.map(n=>n.getAttribute('href')));
try{
 for(const width of [1280,1024,390])for(const scheme of ['light','dark']){
  const page=await open(width,scheme);const errors=page.navigationErrors;
  assert.equal(await page.$$('button.mui-block--shell__trigger').then(a=>a.length),Number(!tabs),'hamburger iff no tabs');
  assert.equal(await page.$$(sidebar).then(a=>a.length),1);
  const original=await links(page);assert(original.length>0);
  await page.evaluate(()=>{window.__originalSidebar=document.querySelector('.mui-block--shell__sidebar');});
  const header=await page.$eval('.mui-page-header',n=>({height:n.getBoundingClientRect().height,right:n.getBoundingClientRect().right}));
  assert.equal(header.height,56);assert(header.right<=width+1,'header overflows viewport');
  assert(await page.$eval('[data-mui="header-search"] input',n=>n.getAttribute('aria-label')||n.closest('label')),'named search');
  if(width===390){
   const trigger=tabs?'.mui-bottom-tab-bar [data-mui="navigation-trigger"]':'button.mui-block--shell__trigger';
   const box=await page.$eval(trigger,n=>{const r=n.getBoundingClientRect();return {w:r.width,h:r.height};});
   assert(box.w>=44&&box.h>=44);assert(await page.$eval('.mui-page-header__title',n=>n.checkVisibility()));
   assert.equal(await page.$$('.mui-page-header .mui-breadcrumb').then(a=>a.length),0,'phone breadcrumb belongs in drawer');
   assert(await page.$eval('[data-mui="header-search"] > summary',n=>n.checkVisibility()),'one phone search action');
   const show=async()=>{await page.click(trigger);await page.waitForFunction(()=>document.querySelector('.mui-navigation-dialog').open);await page.waitForFunction(()=>!document.querySelector('.mui-navigation-dialog').getAnimations({subtree:true}).some(animation=>animation.playState==='running'));};
   await show();assert.deepEqual(await links(page),original);
   assert(await page.evaluate(()=>document.querySelector('.mui-navigation-dialog').contains(window.__originalSidebar)),'same sidebar node');
   assert.equal(await page.$eval(trigger,n=>n.getAttribute('aria-expanded')),'true');
   for(let i=0;i<5;i++){await page.keyboard.press('Tab');assert(await page.evaluate(()=>document.querySelector('.mui-navigation-dialog').contains(document.activeElement)),'modal focus containment');}
   if(args.includes('--screenshots'))await page.screenshot({path:`${value('--screenshots')}/nav-${role||'kit'}-${width}-${scheme}-drawer.png`});
   await page.keyboard.press('Escape');await page.waitForFunction(()=>!document.querySelector('.mui-navigation-dialog').open && document.querySelector('.mui-block--shell__sidebar').parentElement.matches('.mui-block--shell'));
   assert(await page.$eval(trigger,n=>document.activeElement===n),'Escape restores focus');
   assert(await page.evaluate(()=>window.__originalSidebar.parentElement.matches('.mui-block--shell')));
   await show();await page.mouse.click(width-2,450);await page.waitForFunction(()=>!document.querySelector('.mui-navigation-dialog').open && document.querySelector('.mui-block--shell__sidebar').parentElement.matches('.mui-block--shell'));
   await show();await page.$eval(sidebar,n=>n.addEventListener('click',e=>e.preventDefault(),{once:true}));
   await page.click(sidebar+' a[href]');await page.waitForFunction(()=>!document.querySelector('.mui-navigation-dialog').open && document.querySelector('.mui-block--shell__sidebar').parentElement.matches('.mui-block--shell'));
   assert.deepEqual(await links(page),original);
  }else{
   assert(await page.$eval('[data-mui="header-search"] input',n=>n.checkVisibility()),'desktop real search');
   assert((await page.$eval('.mui-page-header__search',n=>n.getBoundingClientRect().width))<=320);
   const settings=await page.$eval('.mui-page-header__settings',n=>({open:n.open,summary:n.querySelector('summary').checkVisibility()}));
   assert.equal(settings.summary,width===1024);assert.equal(settings.open,width===1280);
   const group=await page.$('[data-mui="nav-group"]');
   if(group){
    const key=await group.evaluate(n=>n.getAttribute('data-nav-key'));
    await group.evaluate(n=>{n.open=false;});
    await page.waitForFunction(key=>localStorage.getItem('mui-nav-group:'+key)==='closed',{},key);
    await page.reload({waitUntil:'networkidle2'});
    assert(!(await page.$eval('[data-mui="nav-group"]',n=>n.open)),'group remembers an explicit close');
    await page.$eval('[data-mui="nav-group"]',n=>{n.open=true;});
   }
   const rail=await page.$('[data-mui="shell-rail"]');
   if(rail){
    await rail.click();assert.equal(await page.$eval('.mui-block--shell',n=>n.getAttribute('data-collapsed')),'true');
    assert((await page.$eval(sidebar,n=>n.getBoundingClientRect().width))<100);
    await page.reload({waitUntil:'networkidle2'});assert.equal(await page.$eval('.mui-block--shell',n=>n.getAttribute('data-collapsed')),'true','rail persists');
    await page.click('[data-mui="shell-rail"]');
   }
  }
  await page.keyboard.down('Control');await page.keyboard.press('k');await page.keyboard.up('Control');
  assert(await page.evaluate(()=>document.activeElement.matches('[data-mui="header-search"] input')),'Ctrl+K focuses original search');
  if(width===390)await page.keyboard.press('Escape');
  if(args.includes('--screenshots'))await page.screenshot({path:`${value('--screenshots')}/nav-${role||'kit'}-${width}-${scheme}.png`});
  assert.deepEqual(errors,[]);results.push({width,scheme,links:original.length,tabs});await page.close();
 }
 const nojs=await open(390,'light',false);const fallback='.mui-navigation-fallback';
 assert.equal(await nojs.$$(sidebar).then(a=>a.length),1);const original=await links(nojs);
 assert(await nojs.$eval(sidebar,n=>n.checkVisibility()),'native fallback initially exposes navigation');
 await nojs.click(fallback+' > summary');assert(!(await nojs.$eval(sidebar,n=>n.checkVisibility())));
 await nojs.click(fallback+' > summary');assert(await nojs.$eval(sidebar,n=>n.checkVisibility()));assert.deepEqual(await links(nojs),original);
 await nojs.close();
 if(args.includes('--output'))fs.writeFileSync(value('--output'),JSON.stringify(results,null,2)+'\n');
 console.log(`Navigation browser: ${results.length} viewport/theme cases; native drawer identity, links, focus, Escape/scrim/link close, rail persistence, search and no-JS details passed`);
}finally{await browser.close();}
