// The gallery's command palette must close and STAY closed when it was opened from the
// header search box. Reported 2026-09-23: closing handed focus back to the box, whose
// focus handler reopened the palette before the guard in the dialog's 'close' event ran,
// so Escape and click-outside looked like they did nothing.
//
//   node tests/palette-browser.mjs http://localhost:3456
import {chrome} from './chrome-cdp.mjs';
import {setTimeout as delay} from 'node:timers/promises';

const base = (process.argv[2] || 'http://localhost:3456').replace(/\/$/, '');
const failures = [];
const b = await chrome();
try {
  await b.send('Emulation.setFocusEmulationEnabled', {enabled: true});
  await b.viewport(1440, 900);
  await b.goto(`${base}/gallery`);
  const open = () => b.evaluate("document.getElementById('mui-palette').open");
  const openFromBox = async () => { await b.click('#mui-search'); await delay(300); return open(); };
  const check = async (name, expected) => {
    // Sample twice: the failure was a close followed by an instant reopen.
    await delay(100); const first = await open();
    await delay(500); const second = await open();
    if (first !== expected || second !== expected) failures.push(`${name}: open was ${first} then ${second}, expected ${expected}`);
  };

  if (!(await openFromBox())) failures.push('clicking the header search box did not open the palette');
  await b.key('Escape');
  await check('Escape', false);

  if (!(await openFromBox())) failures.push('clicking the box a second time did not reopen the palette');
  for (const type of ['mousePressed', 'mouseReleased']) await b.send('Input.dispatchMouseEvent', {type, x: 30, y: 860, button: 'left', clickCount: 1});
  await check('click outside', false);

  if (!(await openFromBox())) failures.push('clicking the box after a click-outside did not reopen the palette');
} finally {
  await b.close();
}
for (const f of failures) console.log(`FAIL ${f}`);
console.log(failures.length ? `${failures.length} palette check(s) failed` : 'PASS palette closes and stays closed (Escape, click outside), and reopens from the box');
process.exitCode = failures.length ? 1 : 0;
