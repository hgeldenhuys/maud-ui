// Sweep every exported page for horizontal overflow, at any width.
//
//   bun run sweep:overflow                    # 390px
//   bun run sweep:overflow -- 360 390 1280    # several widths
//
// Exits non-zero if any page overflows, so it can gate a change.
//
// Why this exists: on 2026-07-28 a session hand-wrote this same probe SIX times
// while chasing "the site is broken on mobile" — once per question asked, each
// time slightly differently, each time thrown away. The measurement is three
// lines; everything expensive around it (a server on a free port, an on-origin
// harness page, per-route iframes, waiting for layout) is what kept getting
// rebuilt. That is a script, not a skill and not a lesson.
//
// Two traps it encodes, both of which cost real time that day:
//
//   * Iframes must be SAME-ORIGIN. The deployed site sends frame-denial headers
//     even to itself, so a sweep pointed at the live URL returns an empty
//     contentDocument for every route and reads as "every page is broken". It
//     is the instrument failing, not the site. Sweep a local server; verify the
//     deployed site by byte-comparing what it serves against public/.
//
//   * An element inside `overflow-x: auto` is CONTAINED and must not count. A
//     naive "is anything wider than the viewport" probe flags every code block
//     and every scrollable table on the site.

import { spawn } from "node:child_process";
import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import { join } from "node:path";

const ROOT = process.cwd();
const PUBLIC = join(ROOT, "public");

const widths = process.argv.slice(2).map(Number).filter((n) => n > 0);
const WIDTHS = widths.length ? widths : [390];

const CHROME_CANDIDATES = [
  process.env.CHROME_PATH,
  "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
  "/Applications/Chromium.app/Contents/MacOS/Chromium",
  "/usr/bin/google-chrome",
  "/usr/bin/chromium",
  "/usr/bin/chromium-browser",
].filter(Boolean);

function findChrome() {
  const found = CHROME_CANDIDATES.find((p) => existsSync(p));
  if (found) return found;
  throw new Error(
    `no Chrome or Chromium binary found — this script measures real layout and needs one.\n` +
      `Looked in:\n${CHROME_CANDIDATES.map((p) => `  ${p}`).join("\n")}\n` +
      `Fix: install Chrome, or point CHROME_PATH at an existing binary:\n` +
      `  CHROME_PATH=/path/to/chrome bun run sweep:overflow`,
  );
}

if (!existsSync(PUBLIC)) {
  console.error(
    `public/ is missing — this script sweeps the STATIC EXPORT, not the sources.\n` +
      `Fix: bun run build && bun run build:static`,
  );
  process.exit(1);
}

// Routes = every directory in public/ holding an index.html, plus "/".
function routes() {
  const out = ["/"];
  for (const name of readdirSync(PUBLIC)) {
    const dir = join(PUBLIC, name);
    if (!statSync(dir).isDirectory()) continue;
    if (existsSync(join(dir, "index.html"))) out.push(`/${name}`);
    // one level deeper — blocks/, integrations/
    for (const sub of readdirSync(dir)) {
      const subdir = join(dir, sub);
      if (statSync(subdir).isDirectory() && existsSync(join(subdir, "index.html"))) {
        out.push(`/${name}/${sub}`);
      }
    }
  }
  return out;
}

const ROUTES = routes();

// The harness page is served from the SAME ORIGIN as the routes it frames, so
// contentDocument is readable. Serving it from the sweep server (rather than
// writing a file into public/) keeps the repo tree clean.
const harness = (width) => `<!doctype html><meta charset="utf-8"><title>sweep</title><body>
<script>
(async () => {
  const routes = ${JSON.stringify(ROUTES)};
  const W = ${width};
  const probe = (route) => new Promise((resolve) => {
    const f = document.createElement('iframe');
    f.style.cssText = 'position:fixed;left:-9999px;top:0;width:' + W + 'px;height:900px;border:0';
    f.src = route;
    let done = false;
    const fin = (v) => { if (done) return; done = true; try { f.remove(); } catch (e) {} resolve(v); };
    const t = setTimeout(() => fin({ route, error: 'timeout' }), 15000);
    f.onload = () => setTimeout(() => {
      try {
        const d = f.contentDocument, w = f.contentWindow;
        if (!d) { clearTimeout(t); return fin({ route, error: 'no-document (framing blocked?)' }); }
        const vw = d.documentElement.clientWidth;
        const contained = (el) => {
          let c = el.parentElement;
          while (c && c !== d.body) {
            const ox = w.getComputedStyle(c).overflowX;
            if (ox === 'auto' || ox === 'scroll' || ox === 'hidden') return true;
            c = c.parentElement;
          }
          return false;
        };
        let worst = null;
        for (const el of d.querySelectorAll('body *')) {
          const r = el.getBoundingClientRect();
          if (r.width === 0 && r.height === 0) continue;
          if (r.right > vw + 1 && !contained(el) && (!worst || r.right > worst.right)) {
            worst = {
              sel: el.tagName.toLowerCase() + (el.className && typeof el.className === 'string'
                ? '.' + el.className.trim().split(/\\s+/).slice(0, 2).join('.') : ''),
              right: Math.round(r.right)
            };
          }
        }
        clearTimeout(t);
        fin({ route, overflow: d.documentElement.scrollWidth - vw, worst });
      } catch (e) { clearTimeout(t); fin({ route, error: String(e).slice(0, 60) }); }
    }, 700);
    document.body.appendChild(f);
  });
  const results = [];
  for (let i = 0; i < routes.length; i += 4) {
    results.push(...await Promise.all(routes.slice(i, i + 4).map(probe)));
  }
  const pre = document.createElement('pre');
  pre.id = 'out';
  pre.textContent = JSON.stringify(results);
  document.body.appendChild(pre);
})();
</script></body>`;

const MIME = { ".html": "text/html", ".css": "text/css", ".js": "text/javascript",
  ".svg": "image/svg+xml", ".png": "image/png", ".json": "application/json" };

let currentWidth = WIDTHS[0];
const server = Bun.serve({
  port: 0,
  fetch(req) {
    const path = new URL(req.url).pathname;
    if (path === "/__sweep") {
      return new Response(harness(currentWidth), { headers: { "content-type": "text/html" } });
    }
    let file = join(PUBLIC, path);
    if (existsSync(file) && statSync(file).isDirectory()) file = join(file, "index.html");
    if (!existsSync(file)) return new Response("not found", { status: 404 });
    const ext = file.slice(file.lastIndexOf("."));
    return new Response(Bun.file(file), {
      headers: { "content-type": MIME[ext] || "application/octet-stream" },
    });
  },
});

const chrome = findChrome();

function dumpDom(url) {
  return new Promise((resolve, reject) => {
    const child = spawn(chrome, ["--headless", "--disable-gpu", "--no-sandbox",
      "--virtual-time-budget=60000", "--dump-dom", url], { stdio: ["ignore", "pipe", "pipe"] });
    let out = "";
    child.stdout.on("data", (d) => (out += d));
    child.on("error", reject);
    child.on("exit", (c) => (c === 0 ? resolve(out) : reject(new Error(`chrome exited ${c}`))));
  });
}

const decode = (s) => s.replace(/&quot;/g, '"').replace(/&lt;/g, "<")
  .replace(/&gt;/g, ">").replace(/&amp;/g, "&");

let anyFailed = false;

for (const width of WIDTHS) {
  currentWidth = width;
  const dom = await dumpDom(`http://localhost:${server.port}/__sweep`);
  const m = dom.match(/<pre id="out">([\s\S]*?)<\/pre>/);
  if (!m) {
    console.error(`  ${width}px — harness produced no output; the page likely threw.`);
    anyFailed = true;
    continue;
  }
  const results = JSON.parse(decode(m[1]));
  const bad = results.filter((r) => r.error || r.overflow > 1);
  const label = `${width}px`.padEnd(8);
  if (!bad.length) {
    console.log(`  ok    ${label} ${results.length}/${results.length} pages clean`);
  } else {
    anyFailed = true;
    console.log(`  FAIL  ${label} ${results.length - bad.length}/${results.length} clean — ${bad.length} overflowing:`);
    for (const b of bad.sort((x, y) => (y.overflow || 0) - (x.overflow || 0))) {
      console.log(
        b.error
          ? `          ${b.route} — ${b.error}`
          : `          ${b.route}  +${b.overflow}px  ${b.worst ? b.worst.sel : "(no uncontained element — check html/body)"}`,
      );
    }
  }
}

server.stop(true);

if (anyFailed) {
  console.error(
    `\nA page is wider than the viewport, so the whole document scrolls sideways.\n` +
      `The named element is the widest one NOT inside an overflow-x container — start there.\n` +
      `Common causes: a grid/flex child with the default min-width:auto that refuses to shrink,\n` +
      `an unbreakable string (a long URL or path) under overflow-wrap:normal, or a fixed\n` +
      `min-width on a component. See docs/testing.md.`,
  );
  process.exit(1);
}
console.log(`\nAll ${ROUTES.length} pages clean at ${WIDTHS.join("px, ")}px.`);
