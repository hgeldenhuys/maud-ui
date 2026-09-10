// Dependency-free, isolated real Chrome for local regression fixtures.
import {spawn} from 'node:child_process';
import {mkdtempSync, readFileSync, existsSync, rmSync} from 'node:fs';
import {tmpdir} from 'node:os';
import {join} from 'node:path';
import {setTimeout as delay} from 'node:timers/promises';

export async function chrome() {
  const profile = mkdtempSync(join(tmpdir(), 'mui-workflow-test-'));
  const process = spawn(globalThis.process.env.PAGE_SHOT_CHROME || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome', ['--headless', '--disable-gpu', '--disable-extensions', '--no-first-run', '--no-default-browser-check', '--remote-debugging-port=0', `--user-data-dir=${profile}`, 'about:blank'], {stdio:'ignore'});
  let spawnError;
  process.on('error', error => { spawnError = error; });
  try {
    for (let i=0;i<150&&!existsSync(join(profile,'DevToolsActivePort'));i++) { if(spawnError)throw spawnError; await delay(100); }
    const [port] = readFileSync(join(profile,'DevToolsActivePort'),'utf8').split('\n');
    const targets = await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
    const ws = new WebSocket(targets.find(t=>t.type==='page').webSocketDebuggerUrl);
    await new Promise((resolve,reject)=>{ws.onopen=resolve;ws.onerror=reject;});
    let id=0; const pending=new Map(), errors=[];
    ws.onmessage = event => {
      const m = JSON.parse(event.data);
      if (m.id) { const p=pending.get(m.id); if(p){pending.delete(m.id);m.error?p.reject(Error(JSON.stringify(m.error))):p.resolve(m.result);} }
      else if (m.method==='Runtime.exceptionThrown' || m.method==='Page.javascriptDialogOpening') errors.push(m);
    };
    function send(method, params={}) { return new Promise((resolve,reject)=>{const key=++id;pending.set(key,{resolve,reject});ws.send(JSON.stringify({id:key,method,params}));}); }
    async function evaluate(expression) { const r=await send('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true});if(r.exceptionDetails)throw Error(JSON.stringify(r.exceptionDetails));return r.result.value; }
    await send('Page.enable'); await send('Runtime.enable');
    return {
      send, evaluate, errors,
      async goto(url) { await send('Page.navigate',{url}); await this.until(`location.href === ${JSON.stringify(url)} && document.readyState === 'complete'`); await evaluate('document.fonts.ready.then(()=>true)'); },
      async until(expression, timeout=5000) { const start=Date.now(); while(Date.now()-start<timeout){try{if(await evaluate(expression))return;}catch{}await delay(20);}throw Error('Timed out: '+expression); },
      async viewport(width,height=900) { await send('Emulation.setDeviceMetricsOverride',{width,height,deviceScaleFactor:1,mobile:width<600}); },
      async click(selector) {
        const point=await evaluate(`(()=>{const n=document.querySelector(${JSON.stringify(selector)});if(!n)throw Error('Missing click target');n.scrollIntoView({block:'center',behavior:'instant'});const r=n.getBoundingClientRect();if(!r.width||!r.height)throw Error('Hidden click target');return{x:r.x+r.width/2,y:r.y+r.height/2};})()`);
        await send('Input.dispatchMouseEvent',{type:'mousePressed',button:'left',clickCount:1,...point});await send('Input.dispatchMouseEvent',{type:'mouseReleased',button:'left',clickCount:1,...point});
      },
      async fill(selector,text) { await evaluate(`(()=>{const n=document.querySelector(${JSON.stringify(selector)});n.focus();n.select();})()`);await send('Input.insertText',{text}); },
      async key(key,modifiers=0) {
        const code=key.length===1?'Key'+key.toUpperCase():key, windowsVirtualKeyCode=({Enter:13,Escape:27,Tab:9})[key]||key.toUpperCase().charCodeAt(0);
        const text=key==='Enter'?'\r':key.length===1?key:undefined;
        const params={key,code,windowsVirtualKeyCode,modifiers};
        await send('Input.dispatchKeyEvent',{type:text?'keyDown':'rawKeyDown',...params,...(text&&!modifiers?{text,unmodifiedText:text}:{})});await send('Input.dispatchKeyEvent',{type:'keyUp',...params});
      },
      async close() { ws.close();process.kill('SIGTERM');await delay(150);rmSync(profile,{recursive:true,force:true}); }
    };
  } catch (error) { process.kill('SIGTERM');rmSync(profile,{recursive:true,force:true});throw error; }
}
