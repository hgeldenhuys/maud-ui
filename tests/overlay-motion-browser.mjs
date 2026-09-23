// Usage: node tests/overlay-motion-browser.mjs http://localhost:3456
// Selectors are taken from public/<page>/index.html; see docs/overlay-motion.md.
import assert from 'node:assert/strict';
import {setTimeout as delay} from 'node:timers/promises';
import {chrome} from './chrome-cdp.mjs';
const base = (process.argv[2] || 'http://localhost:3456').replace(/\/$/, '');
const cases = [
  ...[['dialog','demo-dialog-edit-profile'], ['alert_dialog','demo-alert-delete'],
    ...['right','left','top','bottom'].map(side => ['sheet',`demo-sheet-${side}`]),
    ...[1,2,3].map(n => ['drawer',`demo-drawer-${n}`])]
    .map(([page,id]) => ({name:id,page,trigger:`[data-target="${id}"]`,panel:`#${id}`,native:true})),
  {name:'navigation More',page:'bottom_tab_bar',trigger:'[data-mui="navigation-trigger"][aria-controls="tab-bar-more"]',panel:'#tab-bar-more',native:true,mobile:true},
  {name:'menu / dropdown menu',page:'menu',trigger:'#demo-menu-file-trigger',panel:'#demo-menu-file-items'},
  {name:'context menu',page:'context_menu',trigger:'.mui-context-menu__region',panel:'#demo-ctx-1-menu',context:true},
  {name:'menubar',page:'menubar',trigger:'[aria-controls="demo-menubar-1-menu-0"]',panel:'#demo-menubar-1-menu-0'},
  {name:'popover',page:'popover',trigger:'.mui-popover__trigger button',panel:'#demo-pop-1-content'},
  {name:'hover card',page:'hover_card',trigger:'.mui-hover-card__trigger',panel:'#demo-hover-1-card',hover:true},
  {name:'action-row More',page:'blocks/action-row',trigger:'.mui-action-row__more summary',panel:'.mui-action-row__overflow',more:true},
  {name:'toast',page:'toast',trigger:'button[onclick*="Profile updated"]',panel:'#mui-toast-viewport .mui-toast',toast:true},
];
let browser, failures = 0;
try {
  browser = await chrome();
  for (const c of cases) {
    const problems = [];
    for (const reduced of [false,true]) {
      try {
        await browser.send('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:reduced?'reduce':'no-preference'}]});
        await browser.viewport(c.mobile?390:1280);
        await browser.goto(`${base}/${c.page}`);
        await browser.evaluate(`window.__overlayTrigger=document.querySelector(${JSON.stringify(c.trigger)}); if(!__overlayTrigger) throw Error('missing trigger'); window.__overlayBefore=document.activeElement;`);
        if (c.context || c.hover) {
          await browser.evaluate(`__overlayTrigger.dispatchEvent(new MouseEvent(${JSON.stringify(c.context?'contextmenu':'mouseenter')},{bubbles:true,clientX:300,clientY:300}));`);
        } else await browser.click(c.trigger);
        await browser.until(`(()=>{const n=document.querySelector(${JSON.stringify(c.panel)});return n && n.getClientRects().length && getComputedStyle(n).opacity==='1'})()`);
        // Let entry settle, including the native starting-style transition.
        await delay(300);
        await browser.evaluate(`window.__overlayPanel=document.querySelector(${JSON.stringify(c.panel)}); window.__overlaySample=()=>{const n=__overlayPanel,s=getComputedStyle(n);const r=n.getBoundingClientRect();return {w:Math.round(r.width),h:Math.round(r.height),rendered:n.isConnected&&!!n.getClientRects().length&&s.display!=='none',opacity:Number(s.opacity),focus:document.activeElement===(${!!c.context}?__overlayBefore:__overlayTrigger),backdrop:Number(getComputedStyle(n,'::backdrop').opacity)}};`);
        const openSample=await browser.evaluate('__overlaySample()');
        // Capture on the close event's own clock, not after CDP round trips. Every animation
        // frame for 150ms is sampled and a mid-fade frame is kept: a single sample at 40ms was
        // flaky under load (the fade can start a frame late) and stopped a real release
        // (2026-09-23, toast). An instant close still fails: it never shows a mid-fade frame.
        await browser.evaluate(`window.__overlayCapture=new Promise(resolve=>{
          const n=__overlayPanel; let started=false;
          const start=()=>{if(started)return;started=true;observer.disconnect();const t0=performance.now();let early=null;const sample=()=>{const s=__overlaySample();if(!early||(s.rendered&&s.opacity>0&&s.opacity<1&&!(early.rendered&&early.opacity>0&&early.opacity<1)))early=s;if(${reduced?'true':'performance.now()-t0<150'})requestAnimationFrame(sample);else setTimeout(()=>resolve({early,late:__overlaySample()}),${reduced?400:250})};${reduced?'early=__overlaySample();setTimeout(()=>resolve({early,late:__overlaySample()}),400)':'requestAnimationFrame(sample)'}};
          const observer=new MutationObserver(()=>{if(n.dataset.state==='closing'||n.classList.contains('mui-toast--exit')||n.hidden||!n.isConnected||(${!!c.native}&&!n.open)||(${!!c.more}&&!n.closest('details').open))start()});
          observer.observe(document.body,{subtree:true,attributes:true,childList:true});
          n.addEventListener('close',start,{once:true});
          setTimeout(()=>{if(!started){observer.disconnect();resolve({error:'close did not start'})}},2000);
        }); true`);  // \`; true\`: the harness awaits a returned promise, which blocked here until the 2 s timeout
        if(c.toast) await browser.click(`${c.panel} .mui-toast__close`);
        else if(c.hover) await browser.evaluate("__overlayTrigger.dispatchEvent(new MouseEvent('mouseleave')); ");
        else await browser.key('Escape');
        const result=await browser.evaluate('window.__overlayCapture');
        assert(!result.error,result.error);
        if(reduced) assert(!result.early.rendered,'reduced motion did not hide immediately');
        else {
          assert(result.early.rendered && result.early.opacity>0 && result.early.opacity<1,`40ms sample: ${JSON.stringify(result.early)}`);
          if(c.native) assert(result.early.backdrop>0 && result.early.backdrop<1,'backdrop did not fade');
          // The panel must fade as it was, not re-lay-out: dialogs lost their column layout
          // mid-fade when it lived only under [open] (2026-09-23).
          // A slight scale-out (hover card, ~1%) is motion; a re-layout moves far more than 3%.
          assert(Math.abs(result.early.w-openSample.w)<=Math.max(1,openSample.w*0.03) && Math.abs(result.early.h-openSample.h)<=Math.max(1,openSample.h*0.03),`panel changed size while closing: open ${openSample.w}x${openSample.h}, closing ${result.early.w}x${result.early.h}`);
        }
        assert(!result.late.rendered,'still rendered at 400ms');
        const focusable = await browser.evaluate(`(()=>{const previous=document.activeElement;for(const n of [__overlayPanel,...__overlayPanel.querySelectorAll('a,button,input,select,textarea,[tabindex]')]){n.focus();if(document.activeElement===n)return true;}previous?.focus();return false})()`);
        assert(!focusable,'closed panel remains focusable');
        // Hover cards have no focus contract, and a dismissed toast has no trigger to return
        // to; both are out of this check by decision (maud-ui state file, 2026-09-23).
        if(!c.hover && !c.toast) assert(result.late.focus,'focus did not return to the trigger (context menu: to what had focus before it opened)');
      } catch(error) { problems.push(`${reduced?'reduced':'normal'}: ${error.message}`); }
    }
    if(problems.length) { failures++; console.log(`FAIL ${c.name}: ${problems.join('; ')}`); }
    else console.log(`PASS ${c.name}`);
  }
} catch(error) { failures++; console.log(`FAIL browser setup: ${error.message}`); }
finally { if(browser)await browser.close(); }
process.exitCode=failures?1:0;
