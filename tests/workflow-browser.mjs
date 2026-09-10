// Real input events, native validation/dialogs, and a local HTTP failure/retry server.
// Run after: cargo run --example workflow_fixture && node examples/build-assets.mjs
import assert from 'node:assert/strict';
import {createServer} from 'node:http';
import {readFileSync, writeFileSync, mkdirSync, existsSync} from 'node:fs';
import {setTimeout as delay} from 'node:timers/promises';
import {chrome} from './chrome-cdp.mjs';
const requests=[], cases=[];
const server=createServer(async (req,res)=>{
  const url=new URL(req.url,'http://localhost');
  if(req.method==='POST') {
    let body='';for await(const chunk of req)body+=chunk;
    const entries=[...new URLSearchParams(body)], data=Object.fromEntries(entries);requests.push({path:url.pathname,data,entries});await delay(350);
    const failure=data.email?.startsWith('taken') || data.name==='Offline' || data.booking==='fail';
    res.writeHead(failure?422:200,{'Content-Type':'application/json'});
    res.end(JSON.stringify(failure?{status:'error',message:data.booking?'Check-in could not be completed.':'The guest could not be saved.',description:'Your entries are still here. Review the details and try again.',details:'Request N6-422: <diagnostic> & retry',errors:data.email?.startsWith('taken')?{email:'This email already belongs to a guest.'}:{}}:{status:'success',message:data.booking?'Leila Morgan is checked in.':'Guest saved.',description:data.booking?'Garden suite · 10–11 September.':'Leila Morgan is ready for a booking.'}));return;
  }
  const path=url.pathname==='/css/maud-ui.css'?'static/maud-ui.css':url.pathname==='/js/maud-ui.js'?'static/maud-ui.js':url.pathname==='/js/maud-ui.min.js'?'static/maud-ui.min.js':url.pathname.startsWith('/fixtures/')?'docs'+url.pathname:null;
  if(!path||!existsSync(path)||path.includes('..')){res.writeHead(404);res.end();return;}
  let body=readFileSync(path);
  if(url.searchParams.has('min')) body=Buffer.from(body.toString().replace('/js/maud-ui.js','/js/maud-ui.min.js'));
  res.setHeader('Content-Type',path.endsWith('.css')?'text/css':path.endsWith('.js')?'text/javascript':'text/html');res.end(body);
});
await new Promise(resolve=>server.listen(0,'127.0.0.1',resolve));
const origin=`http://127.0.0.1:${server.address().port}`;
const browser=await chrome();
const $=expression=>browser.evaluate(expression);
const truth=async(expression,label)=>assert(await $(expression),label||expression);
const equal=async(expression,expected,label)=>{const actual=await $(expression);assert.deepEqual(typeof actual==='string'&&actual.length>500?actual.slice(0,500):actual,expected,label||expression);};
const done=async n=>browser.until(`window.fixtureFinished === ${n}`);
mkdirSync('docs/night-6-shots',{recursive:true});
async function shot(name){const {data}=await browser.send('Page.captureScreenshot',{format:'png'});writeFileSync(`docs/night-6-shots/${name}.png`,Buffer.from(data,'base64'));}
try {
  for(const brand of ['lodge','neutral']) for(const density of ['compact','comfortable','spacious']) for(const width of [1280,390]) for(const theme of ['light','dark']) {
    const label=`${brand}/${density}/${width}/${theme}`;
    await browser.viewport(width,width===390?844:900);
    await browser.goto(`${origin}/fixtures/workflow-${brand}-${density}.html${theme==='dark'?'?min=1':''}`);
    await $(`document.documentElement.dataset.theme=${JSON.stringify(theme)}`);
    await truth(`(()=>{const ids=[...document.querySelectorAll('[id]')].map(n=>n.id);return new Set(ids).size===ids.length;})()`,`${label}: duplicate IDs`);
    await truth('document.documentElement.scrollWidth <= innerWidth',`${label}: horizontal overflow`);
    await equal('document.querySelectorAll(".mui-wizard-header [aria-current=step]").length',1);
    await truth(`[...document.querySelectorAll('.mui-search-results__item')].every(n=>[...n.children].every(c=>c.getBoundingClientRect().right<=n.getBoundingClientRect().right+1))`,`${label}: result bounds`);
    await browser.click('#save-guest');
    await equal('document.activeElement.id','name');
    await truth(`document.getElementById('name').getAttribute('aria-describedby').split(' ').every(id=>document.getElementById(id)?.textContent.trim())`);
    await equal('window.fixtureSubmits',0);
    await browser.fill('#name','Leila Morgan');await browser.fill('#email','invalid-email');
    await browser.click('#save-guest');await equal('document.activeElement.id','email');
    await truth(`document.getElementById('email').getAttribute('aria-describedby').includes('privacy') && document.getElementById('email').getAttribute('aria-describedby').includes('email-desc')`);
    await browser.fill('#email','taken@example.com');
    await browser.click('#save-guest');await equal('document.activeElement.id','method');
    await $(`document.getElementById('method').value='Cash';document.getElementById('method').dispatchEvent(new Event('change',{bubbles:true}))`);
    await browser.click('label:has(input[value="terrace"])');
    await equal('document.querySelectorAll(".mui-radio-group .mui-field__error").length',0);
    await truth(`[...document.querySelectorAll('.mui-radio-group input')].every(n=>!n.hasAttribute('aria-invalid'))`);
    await browser.click('#save-guest');
    await truth(`document.getElementById('save-guest').disabled && document.getElementById('cancel-edit').disabled && document.getElementById('guest-form').getAttribute('aria-busy')==='true'`);
    await truth(`document.getElementById('save-guest').textContent.includes('Saving guest…')`);
    await equal('window.fixturePayload',{name:'Leila Morgan',email:'taken@example.com',method:'Cash',room:'terrace',intent:'create'});
    await $(`document.getElementById('guest-form').dispatchEvent(new SubmitEvent('submit',{bubbles:true,cancelable:true,submitter:document.getElementById('save-guest')}))`);
    await equal('window.fixtureSubmits',1);
    await done(1);await equal('document.activeElement.id','email');
    await truth(`document.getElementById('guest-result').textContent.startsWith('The guest could not be saved.')`);
    await truth(`!document.querySelector('#guest-result details').open && document.querySelector('#guest-result pre').textContent.includes('<diagnostic>')`);
    await truth(`document.getElementById('email').value==='taken@example.com' && !document.getElementById('save-guest').disabled && document.getElementById('locked-submit').disabled`);
    await equal('document.querySelectorAll("[data-mui-submitter-mirror]").length',0);
    if(brand==='lodge'&&density==='comfortable') await shot(`library-form-error-${width}-${theme}`);
    await browser.fill('#email','leila@example.com');await browser.key('Enter');await done(2);
    await equal('document.querySelector("#guest-result .mui-notice__message").textContent','Guest saved.');
    await equal(`document.getElementById('save-guest').getAttribute('aria-disabled')`,'false');
    await truth(`!document.getElementById('email').hasAttribute('aria-invalid') && document.getElementById('email').getAttribute('aria-describedby').split(' ').sort().join(' ')==='email-desc privacy'`);
    await browser.fill('#name','Offline');await browser.click('#save-guest');await done(3);await equal('document.activeElement.id','guest-result');
    await browser.click('#cancel-edit');await equal('document.getElementById("name").value','');await truth('document.getElementById("guest-result").hidden');await equal('window.fixtureSubmits',3);
    await browser.click('[data-target="check-in"]');
    await equal('document.activeElement.textContent','Keep booking');await truth('document.querySelector("#check-in:modal") !== null');
    await browser.key('Tab',8);await equal('document.activeElement.textContent','Check in guest'); // Shift+Tab wraps within modal.
    await browser.key('Escape');await browser.until('!document.getElementById("check-in").open');
    await equal('document.activeElement.dataset.target','check-in');
    await browser.click('[data-target="check-in"]');await browser.key('Enter');await browser.until('!document.getElementById("check-in").open');await equal('window.fixtureSubmits',3);
    await browser.click('[data-target="check-in"]');await $(`document.querySelector('#check-in input[name=booking]').value='fail'`);
    await browser.key('Tab');await browser.key('Enter');await done(4);await truth('document.getElementById("check-in").open');
    await equal('document.activeElement.id','check-in-form-result');
    // The form and outside result regions have distinct IDs; diagnostics remain inside the modal.
    await browser.key('Escape');await browser.until('!document.getElementById("check-in").open');
    await browser.click('[data-target="check-in"]');await $(`document.querySelector('#check-in input[name=booking]').value='B-410'`);
    await browser.key('Tab');await browser.key('Enter');await done(5);await browser.until('!document.getElementById("check-in").open');
    await equal('document.activeElement.dataset.target','check-in');
    await browser.click('#next-step');await equal('document.activeElement.id','step-title');
    await equal('document.querySelector(".mui-wizard-header [aria-current=step]").textContent','3Check in');
    await $(`document.querySelector('.mui-search-results__item').focus()`);await browser.key('Enter');await truth(`location.hash==='#guest-section'`);
    await browser.click('#no-results a');await truth(`location.hash==='#search-results'`);
    await browser.click('.mui-worklist-empty a');await truth(`location.hash==='#new-note'`);
    if(width===390) {
      await browser.click('[data-mui="navigation-trigger"]');await truth('document.querySelector(".mui-navigation-dialog").open');await browser.key('Escape');
      await browser.until('!document.querySelector(".mui-navigation-dialog").open && document.activeElement.getAttribute("aria-expanded")==="false"');
      await browser.click('[data-mui="navigation-trigger"]');
      await $(`document.querySelector('.mui-navigation-dialog').addEventListener('cancel',e=>{e.preventDefault();e.target.removeAttribute('open');},{once:true})`);
      await browser.key('Escape');await browser.until('document.activeElement.getAttribute("aria-expanded")==="false"');
      await truth(`!document.querySelector('.mui-navigation-dialog .mui-block--shell__sidebar')`);
    }
    cases.push({brand,density,width,theme,checks:'validation, server errors, payload, duplicate submit, retry, cancel, notice, modal, wizard, search, empty, drawer'});
    console.log(`PASS ${label}`);
  }
  // Server field fragments focus their error, preserve help, and retain isolation.
  await browser.viewport(1280);await browser.goto(`${origin}/fixtures/workflow-lodge-comfortable.html`);
  await $(`(()=>{const old=document.getElementById('email').closest('.mui-field'), field=old.cloneNode(true);field.removeAttribute('data-mui-init');const error=document.createElement('p');error.id='email-err';error.className='mui-field__error';error.textContent='Use another email.';field.append(error);old.replaceWith(field);MaudUI.init(field);field.dispatchEvent(new CustomEvent('htmx:afterSettle',{bubbles:true}));})()`);
  await equal('document.activeElement.id','email');
  await truth(`document.getElementById('email').getAttribute('aria-describedby').includes('privacy') && document.getElementById('email').getAttribute('aria-describedby').includes('email-err')`);
  await $(`MaudUI.formFeedback(document.getElementById('guest-form'),{status:'error',message:'Review the guest.',errors:{email:'Check email.',name:'Check name.'}})`);
  await equal('document.activeElement.id','name');
  await $(`MaudUI.formFeedback(document.getElementById('check-in-form'),{status:'success',message:'Unrelated form saved.'})`);
  await equal(`document.getElementById('name').getAttribute('aria-invalid')`,'true');
  // A restored pending snapshot must become usable, including its original button content.
  await $(`(()=>{const old=document.getElementById('guest-form');MaudUI.formPending(old,true,document.getElementById('save-guest'));const fresh=old.cloneNode(true);fresh.removeAttribute('data-mui-init');fresh.querySelectorAll('[data-mui-init]').forEach(n=>n.removeAttribute('data-mui-init'));old.replaceWith(fresh);MaudUI.init(fresh);MaudUI.init(fresh);})()`);
  await truth(`!document.getElementById('save-guest').disabled && document.getElementById('locked-submit').disabled && !document.getElementById('guest-form').hasAttribute('aria-busy')`);
  await equal('document.querySelectorAll("[data-mui-submitter-mirror]").length',0);
  await equal('document.getElementById("save-guest").textContent','Save guest');
  await $(`(()=>{const form=document.getElementById('guest-form');form.addEventListener('reset',e=>e.preventDefault(),{once:true});form.reset();})()`);
  await equal(`document.getElementById('name').getAttribute('aria-invalid')`,'true');
  await $(`MaudUI.formPending(document.getElementById('guest-form'),true,document.getElementById('save-guest'));window.dispatchEvent(new PageTransitionEvent('pageshow',{persisted:true}))`);
  await truth(`!document.getElementById('save-guest').disabled && document.getElementById('locked-submit').disabled`);
  // Custom unwrapped controls must not overwrite an existing help node with the generated error ID.
  await $(`(()=>{const form=document.createElement('form');form.id='plain-form';form.dataset.mui='form-feedback';form.innerHTML='<label for="plain">Reference</label><input id="plain" name="plain" aria-describedby="plain-feedback"><p id="plain-feedback">Original help</p>';document.querySelector('main').append(form);MaudUI.init(form);MaudUI.formFeedback(form,{status:'error',message:'Check the reference.',errors:{plain:'Reference was not found.'}});})()`);
  await equal('document.activeElement.id','plain');await equal('document.getElementById("plain-feedback").textContent','Original help');
  await equal(`document.getElementById('plain').getAttribute('aria-describedby').split(' ').length`,2);
  await browser.fill('#plain','B-410');await equal('document.getElementById("plain").getAttribute("aria-describedby")','plain-feedback');
  console.log('PASS server fragments, first-error order, form isolation, restored pending state, cancelled reset, bfcache and custom-control hints');
  // A normal browser POST keeps the named submitter and the selected radio once.
  await browser.viewport(1280);await browser.goto(`${origin}/fixtures/workflow-lodge-comfortable.html?native=1`);
  await browser.fill('#name','Native Leila');await browser.fill('#email','native@example.com');
  await $(`document.getElementById('method').value='Cash'`);await browser.click('label:has(input[value="garden"])');await browser.click('#save-guest');
  await browser.until(`location.pathname === '/fixtures/save' && document.body.textContent.includes('Guest saved.')`);
  assert.equal(requests.at(-1).entries.filter(([key])=>key==='intent').length,1);
  assert.equal(requests.at(-1).data.intent,'create');assert.equal(requests.at(-1).data.room,'garden');
  assert.equal(browser.errors.length,0,JSON.stringify(browser.errors));
  assert.equal(requests.length,24*5+1);
  writeFileSync('docs/night-6-browser-results.json',JSON.stringify({cases,lifecycleChecks:7,nativePost:true,requests:requests.length,javascriptErrors:browser.errors},null,2)+'\n');
  console.log('24 rendered workflow cases plus native POST passed; 121 local HTTP submissions; no JavaScript exceptions or native confirm prompts.');
} finally { await browser.close();await new Promise(resolve=>server.close(resolve)); }
