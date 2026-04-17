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

// Component slugs — keep in lockstep with COMPONENT_NAMES in
// src/showcase/mod.rs. Any new component added there must be added here
// too, otherwise its page won't ship.
const COMPONENTS = [
  "accordion","alert","alert_dialog","aspect_ratio","avatar","badge","breadcrumb",
  "button","button_group","calendar","card","carousel","chart","checkbox",
  "collapsible","combobox","command","context_menu","data_table","date_picker",
  "dialog","drawer","empty_state","field","fieldset","hover_card","input",
  "input_group","input_otp","kbd","label","menu","menubar","meter","native_select",
  "navigation_menu","number_field","pagination","popover","progress","radio",
  "radio_group","resizable","scroll_area","select","separator","skeleton","slider",
  "spinner","switch","table","tabs","textarea","toast","toggle","toggle_group",
  "tooltip","typography",
];

// Blocks — pre-composed templates. Mirrors BLOCK_NAMES in src/blocks/mod.rs.
const BLOCKS = ["auth-login", "shell-sidebar"];

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
    "./target/release/examples/showcase",
    [],
    { env: { ...process.env, ADDR: `127.0.0.1:${PORT}` }, stdio: "ignore" },
  );
  try {
    await waitReady(`http://127.0.0.1:${PORT}/`);

    // 3. Clean + recreate public/.
    if (existsSync(PUBLIC_DIR)) rmSync(PUBLIC_DIR, { recursive: true, force: true });
    mkdirSync(join(PUBLIC_DIR, "css"), { recursive: true });
    mkdirSync(join(PUBLIC_DIR, "js"), { recursive: true });

    // 4. Fetch root + getting-started + blocks index + every component
    //    route + every block route.
    const routes = [
      "/",
      "/getting-started",
      "/blocks",
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
    for (const f of ["maud-ui.css", "maud-ui.min.css", "maud-ui.js", "maud-ui.min.js"]) {
      cpSync(join(ROOT, "dist", f), join(PUBLIC_DIR, f.endsWith(".css") ? "css" : "js", f));
    }

    log(`done — ${routes.length} pages + assets in public/`);
  } finally {
    server.kill("SIGTERM");
  }
}

main().catch(err => { console.error(err); process.exit(1); });
