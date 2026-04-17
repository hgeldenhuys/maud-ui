// Static file server for the pre-generated maud-ui gallery at public/.
// Run with `bun server.ts`. Listens on $PORT (default 3000).
//
// The showcase binary at `examples/showcase.rs` generates the HTML; we run
// it once at build time, curl every route into public/, and serve the
// resulting static tree here. Kapable's bun-server framework invokes this
// file via `bun run start`.

import { file } from "bun";
import { join, resolve } from "node:path";
import { existsSync, statSync } from "node:fs";

const PUBLIC_DIR = resolve(import.meta.dir, "public");
// Kapable's bun-server framework sets PORT on the container; fall back to
// 3000 for local runs. BUN_PORT / APP_PORT are checked as backups in case
// the framework evolves.
const PORT = Number(
  process.env.PORT ?? process.env.BUN_PORT ?? process.env.APP_PORT ?? 3000,
);

function resolvePath(pathname: string): string | null {
  // Strip leading slash and normalize
  const clean = pathname.replace(/^\/+/, "").replace(/\.\./g, "");
  const base = join(PUBLIC_DIR, clean);

  // Ensure we never escape PUBLIC_DIR
  if (!base.startsWith(PUBLIC_DIR)) return null;

  // Direct file hit (e.g., /css/maud-ui.min.css)
  if (existsSync(base) && statSync(base).isFile()) return base;

  // Directory → index.html
  const indexed = existsSync(base) && statSync(base).isDirectory()
    ? join(base, "index.html")
    : `${base}/index.html`;
  if (existsSync(indexed) && statSync(indexed).isFile()) return indexed;

  // Try <path>.html at the parent (supports clean URLs for flat exports)
  const flat = `${base}.html`;
  if (existsSync(flat) && statSync(flat).isFile()) return flat;

  return null;
}

const MIME: Record<string, string> = {
  ".html": "text/html; charset=utf-8",
  ".css":  "text/css; charset=utf-8",
  ".js":   "application/javascript; charset=utf-8",
  ".mjs":  "application/javascript; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".svg":  "image/svg+xml",
  ".png":  "image/png",
  ".jpg":  "image/jpeg",
  ".jpeg": "image/jpeg",
  ".webp": "image/webp",
  ".ico":  "image/x-icon",
  ".woff2":"font/woff2",
  ".woff": "font/woff",
  ".txt":  "text/plain; charset=utf-8",
};

function contentType(path: string): string {
  const idx = path.lastIndexOf(".");
  if (idx < 0) return "application/octet-stream";
  return MIME[path.slice(idx).toLowerCase()] ?? "application/octet-stream";
}

console.log(`maud-ui gallery serving ${PUBLIC_DIR} on :${PORT}`);

Bun.serve({
  port: PORT,
  hostname: "0.0.0.0",
  async fetch(req) {
    const url = new URL(req.url);

    // Kapable health probe — respond 200 cheaply so deploy succeeds.
    if (url.pathname === "/health" || url.pathname === "/_health") {
      return new Response("ok", {
        status: 200,
        headers: { "content-type": "text/plain; charset=utf-8" },
      });
    }

    const filepath = resolvePath(url.pathname);

    if (!filepath) {
      return new Response("Not found", { status: 404 });
    }

    const f = file(filepath);
    return new Response(f, {
      headers: {
        "content-type": contentType(filepath),
        // We overwrite /css/maud-ui.css and /js/maud-ui.js in place on
        // every redeploy (they're NOT content-hashed), so `immutable`
        // would strand visitors on stale CSS forever. Short TTL +
        // must-revalidate lets Bun's automatic ETag handling serve 304s
        // when nothing changed, while still picking up redeploys
        // quickly. HTML: even shorter since content changes most often.
        "cache-control": filepath.endsWith(".html")
          ? "public, max-age=60, must-revalidate"
          : "public, max-age=300, must-revalidate",
      },
    });
  },
});
