import { build } from "esbuild";
import { readFileSync, writeFileSync, readdirSync, mkdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { gzipSync } from "node:zlib";

mkdirSync("dist", { recursive: true });

// ============================================================
// JS bundle — base init + per-component behaviors
// ============================================================

const base = readFileSync("dist/maud-ui.js.bak", "utf8");

const behaviorDir = "dist/behaviors";
const behaviors = readdirSync(behaviorDir)
  .filter((f) => f.endsWith(".js"))
  .sort()
  .map((f) => {
    const content = readFileSync(join(behaviorDir, f), "utf8");
    return `// ============ ${f} ============\n${content}`;
  })
  .join("\n\n");

const combined = `${base}\n\n// ============ Component Behaviors ============\n\n${behaviors}`;

writeFileSync("dist/maud-ui.js", combined);

await build({
  stdin: {
    contents: combined,
    loader: "js",
  },
  bundle: false,
  minify: true,
  target: "es2020",
  platform: "browser",
  outfile: "dist/maud-ui.min.js",
});

// ============================================================
// CSS bundle — resolve @imports from css/maud-ui.css
// ============================================================

await build({
  entryPoints: ["css/maud-ui.css"],
  bundle: true,
  minify: false,
  outfile: "dist/maud-ui.css",
  loader: { ".css": "css" },
});

await build({
  entryPoints: ["css/maud-ui.css"],
  bundle: true,
  minify: true,
  outfile: "dist/maud-ui.min.css",
  loader: { ".css": "css" },
});

// ============================================================
// Size report
// ============================================================

const gzipSize = (path) => gzipSync(readFileSync(path)).length;

const files = [
  ["dist/maud-ui.js", "JS  unminified"],
  ["dist/maud-ui.min.js", "JS  minified  "],
  ["dist/maud-ui.css", "CSS unminified"],
  ["dist/maud-ui.min.css", "CSS minified  "],
];

console.log("\n✓ Build complete:\n");
for (const [path, label] of files) {
  const raw = statSync(path).size;
  const gz = gzipSize(path);
  console.log(
    `  ${label}  ${path.padEnd(22)}  ${raw.toLocaleString().padStart(8)} B  (${gz.toLocaleString()} B gzipped)`
  );
}
