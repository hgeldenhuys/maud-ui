// Static export for the maud-ui gallery.
// Starts the showcase binary, curls every route into public/, copies dist/
// assets, then shuts the server down. Run via `bun run build:static`.
//
// This is how we ship the gallery as static HTML on Kapable / Cloudflare
// Pages / any vanilla web host — no axum server at runtime, everything
// pre-rendered at build time.

import { spawn } from "node:child_process";
import { readFileSync, writeFileSync, mkdirSync, cpSync, rmSync, existsSync } from "node:fs";
import { join, dirname } from "node:path";

const PORT = 3458;
const ROOT = process.cwd();
const PUBLIC_DIR = join(ROOT, "public");

// Component slugs — PARSED from src/showcase/mod.rs rather than duplicated.
// This list used to be hand-maintained with a "keep in lockstep" comment, which
// made it a ninth registration surface: a component added everywhere else still
// silently failed to ship a static page. COMPONENT_NAMES is the single source of
// truth; tests/registration_parity.rs guards every other copy of it.
const COMPONENTS = (() => {
  const src = readFileSync(join(ROOT, "src/showcase/mod.rs"), "utf8");
  const m = src.match(/pub const COMPONENT_NAMES: &\[&str\] = &\[([\s\S]*?)\];/);
  if (!m) throw new Error("could not find COMPONENT_NAMES in src/showcase/mod.rs");
  const slugs = [...m[1].matchAll(/"([a-z0-9_]+)"/g)].map((x) => x[1]);
  if (slugs.length === 0) throw new Error("COMPONENT_NAMES parsed as empty");
  return slugs;
})();

// Blocks — PARSED from BLOCK_NAMES in src/blocks/mod.rs, same as components.
// The hand-kept copy that lived here listed 10 of 31 blocks, so 21 block pages
// (record-header, worklist-header, …) 404'd on the site while the gallery's own
// links pointed at them (found 2026-09-22 from a dead "Guest inbox" link).
const BLOCKS = (() => {
  const src = readFileSync(join(ROOT, "src/blocks/mod.rs"), "utf8");
  const m = src.match(/pub const BLOCK_NAMES: &\[&str\] = &\[([\s\S]*?)\];/);
  if (!m) throw new Error("could not find BLOCK_NAMES in src/blocks/mod.rs");
  const slugs = [...m[1].matchAll(/"([a-z0-9-]+)"/g)].map((x) => x[1]);
  if (slugs.length === 0) throw new Error("BLOCK_NAMES parsed as empty");
  return slugs;
})();

function log(msg) { process.stdout.write(`[export-static] ${msg}\n`); }

async function fetchText(url) {
  const r = await fetch(url);
  if (!r.ok) throw new Error(`${url} → HTTP ${r.status}`);
  return await r.text();
}

async function waitReady(url, timeoutMs = 30000) {
  const until = Date.now() + timeoutMs;
  while (Date.now() < until) {
    try {
      const r = await fetch(url);
      if (r.ok) return;
    } catch { /* keep polling */ }
    await new Promise(r => setTimeout(r, 200));
  }
  throw new Error(`server never came up at ${url}`);
}

async function main() {
  // 1. Build the showcase binary (needed for include_str! refresh if you
  //    just ran `node js/build.mjs`). Cheap if already built.
  log("building showcase binary...");
  await new Promise((resolve, reject) => {
    const c = spawn("cargo", ["build", "--example", "showcase", "--release"], { stdio: "inherit" });
    c.on("exit", code => code === 0 ? resolve() : reject(new Error(`cargo exit ${code}`)));
  });

  // 2. Boot the showcase on a known port.
  log(`booting showcase on :${PORT}...`);
  const server = spawn(
    // CARGO_TARGET_DIR moves the binary; a hard-coded ./target was ENOENT under it (2026-09-23).
    join(process.env.CARGO_TARGET_DIR || join(ROOT, "target"), "release/examples/showcase"),
    [],
    { env: { ...process.env, ADDR: `127.0.0.1:${PORT}` }, stdio: "ignore" },
  );
  try {
    await waitReady(`http://127.0.0.1:${PORT}/`);

    // 3. Clean + recreate public/.
    if (existsSync(PUBLIC_DIR)) rmSync(PUBLIC_DIR, { recursive: true, force: true });
    mkdirSync(join(PUBLIC_DIR, "css"), { recursive: true });
    mkdirSync(join(PUBLIC_DIR, "js"), { recursive: true });

    // 4. Fetch root + getting-started + blocks index + integrations +
    //    every component route + every block route.
    const routes = [
      "/",
      "/gallery",
      "/getting-started",
      "/theme",
      "/blocks",
      "/integrations/monaco-editor",
      "/integrations/xyflow",
      "/integrations/excalidraw",
      "/integrations/xterm",
      "/integrations/fullcalendar",
      "/integrations/leaflet",
      "/integrations/tiptap",
      "/integrations/threejs",
      "/integrations/ag-grid",
      "/integrations/mermaid",
      "/integrations/echarts",
      "/integrations/wavesurfer",
      "/integrations/pdfjs",
      "/integrations/cytoscape",
      "/integrations/sortable",
      ...COMPONENTS.map(s => `/${s}`),
      ...BLOCKS.map(s => `/blocks/${s}`),
    ];
    log(`fetching ${routes.length} routes...`);
    for (const r of routes) {
      const html = await fetchText(`http://127.0.0.1:${PORT}${r}`);
      const dest = r === "/"
        ? join(PUBLIC_DIR, "index.html")
        : join(PUBLIC_DIR, r.slice(1), "index.html");
      mkdirSync(dirname(dest), { recursive: true });
      writeFileSync(dest, html);
    }

    // 5. Copy bundled CSS + JS so /css/maud-ui.min.css and
    //    /js/maud-ui.min.js (referenced by the HTML) resolve.
    // static/ holds the CURRENT bundles (build-assets.mjs writes them); dist/ is
    // the 0.7 snapshot and shipped a year-old stylesheet under a fresh page tree
    // (2026-09-22). The font sits next to the CSS, where its @font-face expects it.
    for (const f of ["maud-ui.css", "maud-ui.min.css", "maud-ui.js", "maud-ui.min.js"]) {
      cpSync(join(ROOT, "static", f), join(PUBLIC_DIR, f.endsWith(".css") ? "css" : "js", f));
    }
    mkdirSync(join(PUBLIC_DIR, "css", "fonts"), { recursive: true });
    cpSync(join(ROOT, "static", "fonts", "InterVariable.woff2"), join(PUBLIC_DIR, "css", "fonts", "InterVariable.woff2"));
    {
    }

    // 6. Brand assets at the ROOT of the site. These are copied rather than
    //    fetched as routes because the route loop writes every path as
    //    `<path>/index.html`, which would turn /og.png into og.png/index.html —
    //    a directory where crawlers expect an image. They must sit at exactly
    //    the paths page_head() and the absolute og:image URL point at.
    for (const f of ["favicon.svg", "apple-touch-icon.png", "og.png"]) {
      const src = join(ROOT, "assets", f);
      if (!existsSync(src)) {
        throw new Error(
          `assets/${f} is missing, so the static site would 404 on /${f} ` +
            `while every page still links to it. Fix: run \`bun run build:og\` ` +
            `(regenerates og.png and apple-touch-icon.png), or restore ` +
            `assets/favicon.svg from git.`,
        );
      }
      cpSync(src, join(PUBLIC_DIR, f));
    }

    log(`done — ${routes.length} pages + assets in public/`);
  } finally {
    server.kill("SIGTERM");
  }
}

main().catch(err => { console.error(err); process.exit(1); });
