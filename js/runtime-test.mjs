// Runtime behaviour tests for the shipped JS bundle.
//
// Runs `dist/maud-ui.js` inside real headless Chrome and asserts how behaviors
// get attached. There is no DOM shim and nothing is mocked: the bundle under
// test is the exact file consumers load, driven by the same engine they use.
//
// Run with `bun run test:js`. Needs Chrome or Chromium — the same binary
// `js/build-og.mjs` looks for.
//
// Why this exists: on 2026-07-28 `init()` was found to reach only DESCENDANTS
// of the node it was handed. htmx sets a swap event's target to the swapped
// node itself, so `hx-swap="outerHTML"` on a component — and every OOB swap —
// produced live-looking markup with no behavior attached. No error, no console
// warning. The bug was invisible to `cargo test`, which never executes JS.

import { spawn } from "node:child_process";
import { existsSync, readFileSync, writeFileSync, unlinkSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

const ROOT = process.cwd();
const BUNDLE = join(ROOT, "dist/maud-ui.js");

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
    `no Chrome or Chromium binary found — this script executes the JS bundle in a real browser and needs one.\n` +
      `Looked in:\n${CHROME_CANDIDATES.map((p) => `  ${p}`).join("\n")}\n` +
      `Fix: install Chrome, or point CHROME_PATH at an existing binary:\n` +
      `  CHROME_PATH=/path/to/chrome bun run test:js`,
  );
}

if (!existsSync(BUNDLE)) {
  console.error(
    `dist/maud-ui.js is missing — this script tests the BUILT bundle, not the sources.\n` +
      `Fix: build it first:\n  bun run build`,
  );
  process.exit(1);
}

// The cases. Each returns a plain boolean; the page collects them into JSON.
// Written as source text so they run inside the browser alongside the bundle.
const CASES = `
  function mkEl(name) {
    var el = document.createElement("div");
    el.setAttribute("data-mui", name || "probe");
    return el;
  }
  function fresh(name) {
    // A behavior that counts how many times it was applied, per element.
    window.MaudUI.behaviors[name] = function (el) {
      el.__count = (el.__count || 0) + 1;
    };
  }
  var host = document.createElement("div");
  document.body.appendChild(host);
  var results = {};

  // 1. Baseline: the swapped node CONTAINS the component.
  fresh("probe");
  var wrap = document.createElement("div");
  var inner = mkEl();
  wrap.appendChild(inner);
  host.appendChild(wrap);
  window.MaudUI.init(wrap);
  results.descendant_is_attached = inner.__count === 1;

  // 2. THE REGRESSION: the swapped node IS the component
  //    (hx-swap="outerHTML" on the component's own element).
  var self = mkEl();
  host.appendChild(self);
  window.MaudUI.init(self);
  results.root_itself_is_attached = self.__count === 1;

  // 3. Idempotence — init twice must not double-bind.
  window.MaudUI.init(self);
  window.MaudUI.init(host);
  results.no_double_bind = self.__count === 1;

  // 4. Out-of-band swap. htmx does not raise afterSwap for the OOB node.
  var oob = mkEl();
  host.appendChild(oob);
  oob.dispatchEvent(new CustomEvent("htmx:oobAfterSwap", { bubbles: true }));
  results.oob_swap_is_attached = oob.__count === 1;

  // 5. Ordinary swap still works.
  var swapped = mkEl();
  host.appendChild(swapped);
  swapped.dispatchEvent(new CustomEvent("htmx:afterSwap", { bubbles: true }));
  results.after_swap_is_attached = swapped.__count === 1;

  // 6. History restore. htmx caches innerHTML INCLUDING the data-mui-init
  //    guard, so restored markup carries the attribute but none of the
  //    listeners. The guard has to be cleared or the page is inert.
  var restored = mkEl();
  restored.setAttribute("data-mui-init", "");  // as it comes back from cache
  host.appendChild(restored);
  document.dispatchEvent(new CustomEvent("htmx:historyRestore"));
  results.history_restore_reattaches = restored.__count === 1;

  // 7. An unknown behavior name must not be marked initialised — otherwise a
  //    behavior that registers later can never claim the element.
  var unknown = mkEl("no-such-behavior");
  host.appendChild(unknown);
  window.MaudUI.init(unknown);
  results.unknown_behavior_left_unmarked = !unknown.hasAttribute("data-mui-init");
`;

const page = `<!doctype html><meta charset="utf-8"><title>maud-ui runtime tests</title>
<body>
<script>${readFileSync(BUNDLE, "utf8")}</script>
<script>
  var results = {};
  try {
    ${CASES}
    window.__results = results;
  } catch (err) {
    window.__results = { __error: String(err && err.stack || err) };
  }
  var out = document.createElement("pre");
  out.id = "results";
  out.textContent = JSON.stringify(window.__results);
  document.body.appendChild(out);
</script>
</body>`;

const pagePath = join(tmpdir(), `maud-ui-runtime-test-${process.pid}.html`);
writeFileSync(pagePath, page);

const chrome = findChrome();

function dumpDom(url) {
  return new Promise((resolve, reject) => {
    const child = spawn(
      chrome,
      [
        "--headless",
        "--disable-gpu",
        "--no-sandbox",
        "--virtual-time-budget=3000",
        "--dump-dom",
        url,
      ],
      { stdio: ["ignore", "pipe", "pipe"] },
    );
    let out = "";
    let err = "";
    child.stdout.on("data", (d) => (out += d));
    child.stderr.on("data", (d) => (err += d));
    child.on("error", reject);
    child.on("exit", (code) =>
      code === 0
        ? resolve(out)
        : reject(new Error(`chrome exited ${code}\n${err.slice(0, 500)}`)),
    );
  });
}

const dom = await dumpDom(`file://${pagePath}`);
unlinkSync(pagePath);

const match = dom.match(/<pre id="results">([\s\S]*?)<\/pre>/);
if (!match) {
  console.error(
    "the test page produced no results element — the bundle likely threw before the harness ran.\n" +
      "Fix: open the generated page in a browser and read the console. Dumped DOM follows:\n" +
      dom.slice(0, 1000),
  );
  process.exit(1);
}

const decode = (s) =>
  s
    .replace(/&quot;/g, '"')
    .replace(/&lt;/g, "<")
    .replace(/&gt;/g, ">")
    .replace(/&amp;/g, "&");

const results = JSON.parse(decode(match[1]));

if (results.__error) {
  console.error(`the harness threw inside the browser:\n${results.__error}`);
  process.exit(1);
}

const names = Object.keys(results);
const failed = names.filter((k) => results[k] !== true);

for (const name of names) {
  console.log(`  ${results[name] === true ? "ok  " : "FAIL"}  ${name}`);
}

if (failed.length) {
  console.error(
    `\n${failed.length} of ${names.length} runtime assertion(s) failed: ${failed.join(", ")}\n` +
      `The runtime base is dist/maud-ui.js.bak (NOT js/maud-ui.ts, which is not built).\n` +
      `After editing it run \`bun run build\` to regenerate dist/maud-ui.js, then re-run this.`,
  );
  process.exit(1);
}

console.log(`\n${names.length} runtime assertions passed (${chrome.split("/").pop()})`);
