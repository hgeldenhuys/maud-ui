// Rasterise assets/og-source.html → assets/og.png at exactly 1200x630.
//
// Run via `bun run build:og`. The PNG is COMMITTED: link crawlers (X, Slack,
// Discord, iMessage, Mastodon) fetch a URL and will not run a build step, so
// the bytes have to exist in the repo and in the static export.
//
// Only re-run when assets/og-source.html changes. tests/registration_parity.rs
// asserts the component count baked into that HTML still matches
// COMPONENT_NAMES, so a component added without regenerating this card fails
// the suite rather than shipping a card that quietly understates the library.

import { spawn } from "node:child_process";
import { existsSync, statSync, readFileSync, writeFileSync, rmSync } from "node:fs";
import { join } from "node:path";
import { tmpdir } from "node:os";

const ROOT = process.cwd();
const SOURCE = join(ROOT, "assets/og-source.html");
const DEST = join(ROOT, "assets/og.png");
const WIDTH = 1200;
const HEIGHT = 630;

// iOS ignores rel="icon" for a home-screen tile and it does not honour an SVG's
// prefers-color-scheme, so the transparent mark alone lands as a black-on-black
// square. This is the opaque fallback: the light-theme accent on a white tile,
// which is what iOS composites against anyway.
const TOUCH_DEST = join(ROOT, "assets/apple-touch-icon.png");
const TOUCH_SIZE = 180;

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
    `no Chrome or Chromium binary found — this script rasterises HTML and needs one.\n` +
      `Looked in:\n${CHROME_CANDIDATES.map((p) => `  ${p}`).join("\n")}\n` +
      `Fix: install Chrome, or point CHROME_PATH at an existing binary:\n` +
      `  CHROME_PATH=/path/to/chrome bun run build:og`,
  );
}

if (!existsSync(SOURCE)) {
  console.error(
    `assets/og-source.html is missing — it is the input this script renders.\n` +
      `Fix: restore it from git (git checkout assets/og-source.html).`,
  );
  process.exit(1);
}

const chrome = findChrome();

function shoot(url, dest, w, h) {
  return new Promise((resolve, reject) => {
    const child = spawn(
      chrome,
      [
        "--headless",
        "--disable-gpu",
        "--hide-scrollbars",
        "--force-device-scale-factor=1",
        `--window-size=${w},${h}`,
        "--virtual-time-budget=4000",
        `--screenshot=${dest}`,
        url,
      ],
      { stdio: ["ignore", "ignore", "pipe"] },
    );
    let stderr = "";
    child.stderr.on("data", (d) => (stderr += d));
    child.on("exit", (code) =>
      code === 0
        ? resolve()
        : reject(new Error(`chrome exited ${code}. stderr:\n${stderr.trim()}`)),
    );
  });
}

console.log(`[build-og] rendering ${WIDTH}x${HEIGHT} with ${chrome}`);
await shoot(`file://${SOURCE}`, DEST, WIDTH, HEIGHT);
if (!existsSync(DEST)) {
  console.error(
    `chrome reported success but ${DEST} does not exist.\n` +
      `Fix: run the same command by hand to see what it printed.`,
  );
  process.exit(1);
}
console.log(`[build-og] wrote assets/og.png (${statSync(DEST).size} B)`);

// Apple touch icon — the mark on an opaque tile, inlined from the same SVG so
// the two can never drift. The <style> block inside favicon.svg is stripped:
// its prefers-color-scheme rule would resolve against the RENDERING machine's
// OS theme, which would make the committed PNG depend on whose laptop ran the
// build. Colours are pinned to the light-theme accent instead.
const markSvg = readFileSync(join(ROOT, "assets/favicon.svg"), "utf8")
  .replace(/<style>[\s\S]*?<\/style>/, "")
  .replace(/class="brace"/g, 'fill="none" stroke="#2563eb" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"')
  .replace(/class="block"/g, 'fill="#2563eb"');

const touchHtml = join(tmpdir(), `maud-ui-touch-${process.pid}.html`);
writeFileSync(
  touchHtml,
  `<style>html,body{margin:0;padding:0}body{width:${TOUCH_SIZE}px;height:${TOUCH_SIZE}px;background:#fff;display:flex;align-items:center;justify-content:center}svg{width:120px;height:120px}</style>${markSvg}`,
);
try {
  console.log(`[build-og] rendering ${TOUCH_SIZE}x${TOUCH_SIZE} apple-touch-icon`);
  await shoot(`file://${touchHtml}`, TOUCH_DEST, TOUCH_SIZE, TOUCH_SIZE);
  console.log(`[build-og] wrote assets/apple-touch-icon.png (${statSync(TOUCH_DEST).size} B)`);
} finally {
  rmSync(touchHtml, { force: true });
}
