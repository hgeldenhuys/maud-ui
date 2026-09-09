// Prepared for supervisor execution. The Design Lead lane must not launch Chrome.
// MUI_BROWSER_MODULE=/path/to/puppeteer node tests/record-page-browser.mjs http://127.0.0.1:23450
import assert from 'node:assert/strict';
import {writeFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const module=process.env.MUI_BROWSER_MODULE || 'puppeteer';
const {default:puppeteer}=await import(module.startsWith('/')?pathToFileURL(module).href:module);
const origin=process.argv[2] || 'http://127.0.0.1:23450';
const browser=await puppeteer.launch({headless:true}), results=[];
const near=(actual,expected,label)=>assert(Math.abs(actual-expected)<1,`${label}: ${actual}, expected ${expected}`);
async function open(file,width,theme) {
  const page=await browser.newPage();
  await page.setViewport({width,height:900,hasTouch:width===390});
  await page.goto(`${origin}/fixtures/${file}`,{waitUntil:'networkidle0'});
  await page.evaluate(async theme=>{ document.documentElement.dataset.theme=theme; await document.fonts.ready; },theme);
  return page;
}
try {
  for (const width of [1280,390]) for (const theme of ['light','dark']) {
    for (const density of ['compact','comfortable','spacious']) for (const variant of ['guest','long']) {
      const file=`record-page-${density}-${variant}.html`, page=await open(file,width,theme);
      const geometry=await page.evaluate(()=>{
        const $=selector=>document.querySelector(selector), rect=node=>{const r=node.getBoundingClientRect(); return {x:r.x,y:r.y,right:r.right,bottom:r.bottom,width:r.width,height:r.height};};
        const header=$('.mui-record-header'), subtitle=$('.mui-record-header__subtitle'), actions=$('.mui-record-header .mui-action-row');
        const fields=$('.mui-record-facts'), groups=[...fields.querySelectorAll('.mui-record-facts__group')];
        const related=$('.mui-related-list__item'), input=$('.mui-page-header__search-field input'), badge=$('.mui-page-header__search-field kbd');
        const current=$('.mui-page-header__context [data-position="current"]');
        const text=current?.querySelector('span,a'), range=document.createRange(); if(text) range.selectNodeContents(text);
        const css=getComputedStyle(input), canvas=document.createElement('canvas'), context=canvas.getContext('2d'); context.font=css.font;
        const columns=getComputedStyle(fields.querySelector('dl')).gridTemplateColumns.split(' ').length;
        return {overflow:document.documentElement.scrollWidth>innerWidth,header:rect(header),subtitle:rect(subtitle),actions:rect(actions),facts:rect(fields),columns,
          groups:groups.map(n=>({radius:getComputedStyle(n).borderRadius,single:n.dataset.single})),
          masked:[...fields.querySelectorAll('.mui-record-facts__masked')].map(n=>({hint:n.textContent,font:getComputedStyle(n.parentElement).fontFamily})),
          related:{text:related.innerText,box:rect(related),children:[...related.children].map(rect)},
          search:rect($('.mui-page-header__search')),input:rect(input),badge:rect(badge),badgeVisible:badge.checkVisibility(),paddingEnd:parseFloat(css.paddingInlineEnd),paddingStart:parseFloat(css.paddingInlineStart),placeholderWidth:context.measureText(input.placeholder).width,
          current:text?rect(current):null,currentText:text?[...range.getClientRects()].map(r=>({right:r.right})):[],
          groupCount:groups.length,surfaceCount:document.querySelectorAll('.mui-record-facts').length};
      });
      assert(!geometry.overflow,`${file}: horizontal page overflow`);
      assert.equal(geometry.groupCount,4); assert.equal(geometry.surfaceCount,1);
      assert(geometry.groups.every(g=>parseFloat(g.radius)===0)); assert.equal(geometry.groups.filter(g=>g.single==='true').length,1);
      assert.equal(geometry.columns,width===1280?4:1);
      assert.equal(geometry.masked.length,2); assert(geometry.masked.every(m=>m.hint==='masked' && /mono|consolas/i.test(m.font)));
      assert(geometry.actions.y>=geometry.subtitle.bottom); near(geometry.facts.y-geometry.header.bottom,16,'header to facts gap');
      assert.equal((geometry.related.text.match(/UI-B05/g)||[]).length,1);
      assert(!/COMPOSITION|nested|via guest/.test(geometry.related.text));
      assert(geometry.related.children.every(child=>child.right<=geometry.related.box.right+1));
      if(width===1280) {
        assert(geometry.badgeVisible); near(geometry.paddingEnd,52,'reserved input padding');
        assert(geometry.badge.right<=geometry.input.right);
        assert(geometry.input.width-geometry.paddingStart-geometry.paddingEnd>=geometry.placeholderWidth);
        assert(geometry.currentText.every(r=>r.right<=geometry.search.x-1),'current breadcrumb paints under search');
      } else {
        await page.click('.mui-page-header__search-toggle');
        assert(await page.$eval('.mui-page-header__search-field input',n=>n.checkVisibility()));
        assert(!(await page.$eval('.mui-page-header__search-field kbd',n=>n.checkVisibility())));
      }
      assert(!(await page.$eval('.mui-record-header .mui-action-row__more',n=>n.open)));
      await page.click('.mui-record-header .mui-action-row__trigger');
      assert(await page.$eval('.mui-record-header .mui-action-row__more',n=>n.open));
      assert(await page.$eval('.mui-record-header .mui-action-row__overflow a',n=>n.checkVisibility()));
      results.push({file,width,theme,geometry}); await page.close();
    }
    const cards=await open('record-page-three-cards.html',width,theme);
    const gaps=await cards.$$eval('.mui-card',nodes=>nodes.slice(1).map((n,i)=>n.getBoundingClientRect().top-nodes[i].getBoundingClientRect().bottom));
    assert.equal(gaps.length,2); gaps.forEach(gap=>near(gap,16,'sibling card gap'));
    await cards.$eval('.mui-card',n=>n.parentElement.style.gap='0');
    const broken=await cards.$$eval('.mui-card',nodes=>nodes[1].getBoundingClientRect().top-nodes[0].getBoundingClientRect().bottom);
    assert.throws(()=>near(broken,16,'injected touching-card regression'));
    results.push({file:'record-page-three-cards.html',width,theme,gaps}); await cards.close();
    const minimal=await open('record-page-minimal.html',width,theme);
    assert(await minimal.$eval('.mui-record-facts',n=>parseFloat(getComputedStyle(n).borderTopWidth)===0));
    assert.equal(await minimal.$$eval('.mui-related-list__empty',nodes=>nodes.length),1);
    assert.equal(await minimal.$$eval('.mui-related-list__item',nodes=>nodes.length),0);
    results.push({file:'record-page-minimal.html',width,theme,single_fact_border:0,quiet_empty_lines:1}); await minimal.close();
  }
  writeFileSync('docs/night-5-browser-geometry.json',JSON.stringify(results,null,2)+'\n');
  console.log('32 rendered cases passed: record sections, masking, related rows, title/actions, search/crumb bounds, single-fact record and sibling card gaps.');
} finally { await browser.close(); }
