// Real Chrome/CDP driver for the explicitly authorized Round 6 app workflows.
// Persistent browser; each invocation performs one observed action. No packages installed.
import {spawn} from 'node:child_process';
import {readFileSync,writeFileSync,mkdirSync,mkdtempSync,appendFileSync,existsSync,openSync,rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';

const stateFile='target/night-6-browser.json', log='docs/night-6-live-events.jsonl';
const [command,...args]=process.argv.slice(2);
if(command==='start') {
  mkdirSync('target',{recursive:true}); mkdirSync('docs/night-6-shots',{recursive:true});
  const profile=mkdtempSync(join(tmpdir(),'maud-night6-'));
  const chrome=process.env.PAGE_SHOT_CHROME || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
  const child=spawn(chrome,['--headless','--disable-gpu','--disable-extensions','--no-first-run','--no-default-browser-check','--remote-debugging-port=0',`--user-data-dir=${profile}`,'about:blank'],{detached:true,stdio:['ignore','ignore',openSync('target/night-6-chrome.log','a')]});
  child.unref();
  for(let i=0;i<150&&!existsSync(join(profile,'DevToolsActivePort'));i++) await delay(100);
  const [port]=readFileSync(join(profile,'DevToolsActivePort'),'utf8').split('\n');
  writeFileSync(stateFile,JSON.stringify({port:Number(port),pid:child.pid,profile}));
  console.log(JSON.stringify({started:true,port:Number(port),pid:child.pid})); process.exit(0);
}
const state=JSON.parse(readFileSync(stateFile,'utf8'));
if(command==='stop') {
  process.kill(state.pid,'SIGTERM'); await delay(200); rmSync(state.profile,{recursive:true,force:true}); rmSync(stateFile);
  console.log('Closed this task’s Chrome and removed its temporary profile.'); process.exit(0);
}
const targets=await (await fetch(`http://127.0.0.1:${state.port}/json/list`)).json();
const target=targets.find(t=>t.id===state.targetId)||targets.find(t=>t.type==='page'&&!t.url.startsWith('chrome:'));
if(!state.targetId){state.targetId=target.id;writeFileSync(stateFile,JSON.stringify(state));}
const ws=new WebSocket(target.webSocketDebuggerUrl), pending=new Map(), events=[];
await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject;});
let id=0;
ws.onmessage=event=>{
  const message=JSON.parse(event.data);
  if(message.id){const item=pending.get(message.id);if(item){pending.delete(message.id);message.error?item.reject(Error(JSON.stringify(message.error))):item.resolve(message.result);}}
  else if(message.method==='Page.javascriptDialogOpening') events.push({dialog:message.params});
  else if(message.method==='Network.responseReceived'&&message.params.response.status>=400) events.push({url:message.params.response.url,status:message.params.response.status});
};
function send(method,params={}){return new Promise((resolve,reject)=>{const key=++id;pending.set(key,{resolve,reject});ws.send(JSON.stringify({id:key,method,params}));});}
async function evaluate(expression){const result=await send('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true});if(result.exceptionDetails)throw Error(JSON.stringify(result.exceptionDetails));return result.result.value;}
async function observe(){return evaluate(`(()=>{
 const visible=n=>!!(n.getClientRects().length)&&getComputedStyle(n).visibility!=='hidden';
 const name=n=>n.getAttribute('aria-label')||n.labels?.[0]?.innerText||n.innerText||n.placeholder||n.name;
 return {url:location.href,title:document.title,viewport:{width:innerWidth,height:innerHeight,overflow:document.documentElement.scrollWidth-innerWidth},focus:{tag:document.activeElement.tagName,id:document.activeElement.id,name:document.activeElement.name},text:document.body.innerText.slice(0,11000),
 forms:[...document.forms].map(n=>({action:n.action,method:n.method,id:n.id})),
 controls:[...document.querySelectorAll('input,select,textarea,button,summary')].filter(visible).map(n=>({tag:n.tagName,type:n.type,id:n.id,name:n.name,label:name(n)?.trim(),value:n.type==='password'?'[redacted]':n.value,required:n.required,disabled:n.disabled,invalid:n.getAttribute('aria-invalid'),describedby:n.getAttribute('aria-describedby'),options:n.tagName==='SELECT'?[...n.options].map(o=>({value:o.value,label:o.text})):undefined})),
 links:[...document.querySelectorAll('a[href]')].filter(visible).map(n=>({label:n.innerText.trim()||n.getAttribute('aria-label'),href:n.getAttribute('href')})).slice(0,90)};
})()`);}
await send('Page.enable'); await send('Runtime.enable'); await send('Network.enable');
let result;
try {
  if(command==='goto') { await send('Page.navigate',{url:args[0]}); await delay(900); }
  else if(command==='role') { for(const url of ['https://ayios-lodge.kapable.kapable.run','https://ayios-neutral.kapable.kapable.run']) await send('Network.setCookie',{name:'rsc-demo-role',value:args[0],url,path:'/'}); }
  else if(command==='viewport') {
    const width=Number(args[0]); await send('Emulation.setDeviceMetricsOverride',{width,height:Number(args[1]||900),deviceScaleFactor:1,mobile:width<600});
    await send('Emulation.setTouchEmulationEnabled',{enabled:width<600,maxTouchPoints:1}); await delay(250);
  }
  else if(command==='click') {
    const point=await evaluate(`(()=>{const n=document.querySelector(${JSON.stringify(args[0])});if(!n)throw Error('Missing target');n.scrollIntoView({block:'center',behavior:'instant'});const r=n.getBoundingClientRect();return {x:r.x+r.width/2,y:r.y+r.height/2};})()`);
    await send('Input.dispatchMouseEvent',{type:'mousePressed',button:'left',clickCount:1,...point});await send('Input.dispatchMouseEvent',{type:'mouseReleased',button:'left',clickCount:1,...point});await delay(800);
  }
  else if(command==='fill') {
    await evaluate(`(()=>{const n=document.querySelector(${JSON.stringify(args[0])});if(!n)throw Error('Missing field');n.focus();n.select?.();})()`);
    await send('Input.insertText',{text:args[1]});
  }
  else if(command==='select') {
    await evaluate(`(()=>{const n=document.querySelector(${JSON.stringify(args[0])});if(!n)throw Error('Missing select');n.value=${JSON.stringify(args[1])};n.dispatchEvent(new Event('input',{bubbles:true}));n.dispatchEvent(new Event('change',{bubbles:true}));})()`);
  }
  else if(command==='press') {
    const key=args[0], code=key===' '?'Space':key.length===1?'Key'+key.toUpperCase():key;
    const windowsVirtualKeyCode=({Enter:13,Escape:27,Tab:9,' ':32})[key]||key.toUpperCase().charCodeAt(0);
    const params={key,code,windowsVirtualKeyCode,modifiers:Number(args[1]||0)};
    const text=key==='Enter'?'\r':key.length===1?key:undefined;
    await send('Input.dispatchKeyEvent',{type:text?'keyDown':'rawKeyDown',...params,...(text&&!params.modifiers?{text,unmodifiedText:text}:{})});await send('Input.dispatchKeyEvent',{type:'keyUp',...params});await delay(700);
  }
  else if(command==='eval') result=await evaluate(args.join(' '));
  else if(command==='dialog') { await send('Page.handleJavaScriptDialog',{accept:args[0]==='accept'}); await delay(600); }
  else if(command==='shot') {
    const shot=await send('Page.captureScreenshot',{format:'png',captureBeyondViewport:false});const file=`docs/night-6-shots/${args[0]}.png`;writeFileSync(file,Buffer.from(shot.data,'base64'));result={screenshot:file};
  }
  else if(command!=='observe') throw Error('Unknown command: '+command);
  result ??= await observe();
  const record={at:new Date().toISOString(),command,args,result,events};appendFileSync(log,JSON.stringify(record)+'\n');
  console.log(JSON.stringify({result,events},null,2));
} finally {ws.close();}
