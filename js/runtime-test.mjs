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
const STYLES = join(ROOT, "dist/maud-ui.css");
// Layout cases run against the REAL exported page, so the markup under test is
// the markup that ships rather than a hand-written approximation that drifts.
const EXPORTED_DIALOG_PAGE = join(ROOT, "public/dialog/index.html");

/// Pull one `<dialog id="...">…</dialog>` out of an exported page.
function extractDialog(html, id) {
  const open = html.indexOf(`<dialog class="mui-dialog" id="${id}"`);
  if (open === -1) {
    throw new Error(
      `no dialog with id "${id}" in public/dialog/index.html — the layout cases read the\n` +
        `exported page so they test shipped markup. If the demo was renamed, update the id here;\n` +
        `if public/ is stale, run \`bun run build:static\`.`,
    );
  }
  const close = html.indexOf("</dialog>", open);
  return html.slice(open, close + "</dialog>".length);
}

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

  // ── Layout ────────────────────────────────────────────────────────────
  // A form inside a dialog must fill the dialog's content column. It did not:
  // .mui-field carries max-width: 24rem as a measure guard for a field on an
  // open page, and inside a 32rem dialog that became a second, narrower cap —
  // the title and description spanned the full 464px column while every field
  // stopped at 384px, an 80px ragged edge down the right of the form. Reported
  // by eye, twice, because nothing measured it.
  function dialogFieldGaps(id) {
    var dlg = document.getElementById(id);
    if (!dlg) return null;
    dlg.showModal();
    var body = dlg.querySelector(".mui-dialog__body");
    var bcs = getComputedStyle(body);
    var br = body.getBoundingClientRect();
    var colRight = br.right - parseFloat(bcs.paddingRight);
    var gaps = [];
    var targets = dlg.querySelectorAll(".mui-field, .mui-input, .mui-select__trigger, .mui-textarea");
    for (var i = 0; i < targets.length; i++) {
      gaps.push(colRight - targets[i].getBoundingClientRect().right);
    }
    dlg.close();
    return { count: targets.length, worst: gaps.length ? Math.max.apply(null, gaps) : 0 };
  }

  // 1px of tolerance for sub-pixel rounding; the bug was 80px, so this is not
  // a threshold anything can hide behind.
  var edit = dialogFieldGaps("demo-dialog-edit-profile");
  results.dialog_form_fills_the_column =
    !!edit && edit.count > 0 && edit.worst <= 1;

  var share = dialogFieldGaps("demo-dialog-share-doc");
  results.share_dialog_form_fills_the_column =
    !!share && share.count > 0 && share.worst <= 1;

  // The cap must SURVIVE outside a dialog — removing it altogether would let a
  // form stretch to the full window width on an open page.
  var loose = document.createElement("div");
  loose.className = "mui-field";
  document.body.appendChild(loose);
  results.field_keeps_its_measure_outside_a_dialog =
    getComputedStyle(loose).maxWidth !== "none";
  loose.remove();
`;

const exportedPage = readFileSync(EXPORTED_DIALOG_PAGE, "utf8");

const page = `<!doctype html><meta charset="utf-8"><title>maud-ui runtime tests</title>
<style>${readFileSync(STYLES, "utf8")}</style>
<style>
  /* Kill the 150ms enter animation. It transforms scale(0.95)→scale(1), so a
     measurement taken mid-flight reads every box 5% narrow — which is exactly
     how a first pass at this bug produced numbers that were all subtly wrong. */
  dialog.mui-dialog, dialog.mui-alert-dialog { animation: none !important; }
</style>
<body>
${extractDialog(exportedPage, "demo-dialog-edit-profile")}
${extractDialog(exportedPage, "demo-dialog-share-doc")}
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
  // Point at the right source. A layout failure is CSS; a behaviour failure is
  // the runtime base — and that base is NOT the file most people open first.
  const layoutFailed = failed.filter((k) => /column|measure/.test(k));
  const behaviourFailed = failed.filter((k) => !/column|measure/.test(k));

  let hint = "";
  if (behaviourFailed.length) {
    hint +=
      `\nBehaviour (${behaviourFailed.join(", ")}):\n` +
      `  Source is dist/maud-ui.js.bak — NOT js/maud-ui.ts, which is never built.\n` +
      `  Edit it, then \`bun run build\` to regenerate dist/maud-ui.js.`;
  }
  if (layoutFailed.length) {
    hint +=
      `\nLayout (${layoutFailed.join(", ")}):\n` +
      `  Source is css/, most likely a width or max-width cap. These cases measure the\n` +
      `  REAL exported dialog from public/dialog/index.html, so also confirm public/ is\n` +
      `  current (\`bun run build:static\`) before hunting a CSS cause that isn't there.\n` +
      `  Edit css/, then \`bun run build\` to regenerate dist/maud-ui.css.`;
  }

  console.error(
    `\n${failed.length} of ${names.length} assertion(s) failed.${hint}`,
  );
  process.exit(1);
}

console.log(`\n${names.length} runtime assertions passed (${chrome.split("/").pop()})`);
