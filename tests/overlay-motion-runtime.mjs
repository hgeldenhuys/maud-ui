import test from 'node:test';
import assert from 'node:assert/strict';
import vm from 'node:vm';
import {readFileSync} from 'node:fs';
function fixture(reduced=false) {
  const timers=new Map(); let serial=0;
  const window={MaudUI:{},matchMedia:()=>({matches:reduced})};
  vm.runInNewContext(readFileSync('static/behaviors/overlay_motion.js','utf8'),{
    window,getComputedStyle:()=>({getPropertyValue:()=> '100ms'}),
    setTimeout:(fn,ms)=>{const id=++serial;timers.set(id,{fn,ms});return id},clearTimeout:id=>timers.delete(id),
  });
  const attrs=new Map(), listeners=new Map();
  const panel={hidden:false,inert:false,getAttribute:n=>attrs.get(n)??null,setAttribute:(n,v)=>attrs.set(n,v),removeAttribute:n=>attrs.delete(n),addEventListener:(n,f)=>listeners.set(n,f),removeEventListener:n=>listeners.delete(n)};
  return {ui:window.MaudUI,panel,timers,listeners,hide:()=>{panel.hidden=true}};
}
test('exit is inert, ignores descendant/unrelated animation events, and hides on completion',()=>{
  const f=fixture();f.ui.closeOverlay(f.panel,f.hide);
  assert.equal(f.panel.getAttribute('data-state'),'closing');assert(f.panel.inert);assert(!f.panel.hidden);
  const end=f.listeners.get('animationend');
  end({target:{},animationName:'mui-overlay-exit'});assert(!f.panel.hidden);
  end({target:f.panel,animationName:'other'});assert(!f.panel.hidden);
  end({target:f.panel,animationName:'mui-overlay-exit'});assert(f.panel.hidden);assert(!f.panel.inert);assert.equal(f.timers.size,0);
});
test('timeout, repeated close, cancellation and reopening cannot leave a stale hide',()=>{
  const f=fixture();f.panel.setAttribute('data-state','open');f.ui.closeOverlay(f.panel,f.hide);f.ui.closeOverlay(f.panel,f.hide);
  assert.equal(f.timers.size,1);assert.equal([...f.timers.values()][0].ms,100);
  f.ui.cancelOverlayExit(f.panel);assert.equal(f.timers.size,0);assert(!f.panel.inert);assert.equal(f.panel.getAttribute('data-state'),'open');
  f.ui.closeOverlay(f.panel,f.hide);[...f.timers.values()][0].fn();assert(f.panel.hidden);assert.equal(f.listeners.size,0);
});
test('reduced motion hides synchronously without a timer',()=>{
  const f=fixture(true);f.ui.closeOverlay(f.panel,f.hide);assert(f.panel.hidden);assert.equal(f.timers.size,0);
});
