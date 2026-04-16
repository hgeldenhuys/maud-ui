import { build } from "esbuild";
import { readFileSync, writeFileSync, readdirSync, mkdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { execSync } from "node:child_process";

mkdirSync("dist", { recursive: true });

// Read base init script from .bak file
const base = readFileSync("dist/maud-ui.js.bak", "utf8");

// Collect all behavior files from dist/behaviors/
const behaviorDir = "dist/behaviors";
const behaviors = readdirSync(behaviorDir)
  .filter(f => f.endsWith(".js"))
  .sort()
  .map(f => {
    const content = readFileSync(join(behaviorDir, f), "utf8");
    return `// ============ ${f} ============\n${content}`;
  })
  .join("\n\n");

// Combine base + behaviors
const combined = `${base}\n\n// ============ Component Behaviors ============\n\n${behaviors}`;

// Write unminified version for dev
writeFileSync("dist/maud-ui.js", combined);

// Minify with esbuild
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

// Report sizes
const fullStats = statSync("dist/maud-ui.js");
const minStats = statSync("dist/maud-ui.min.js");

const fullSize = fullStats.size;
const minSize = minStats.size;

// Gzip size
const gzipSize = Buffer.byteLength(
  execSync("gzip -c dist/maud-ui.min.js", { encoding: "utf8" })
);

console.log("\n✓ Build complete:");
console.log(`  dist/maud-ui.js:       ${fullSize.toLocaleString()} bytes (unminified)`);
console.log(`  dist/maud-ui.min.js:   ${minSize.toLocaleString()} bytes (minified)`);
console.log(`  dist/maud-ui.min.js:   ${gzipSize.toLocaleString()} bytes (gzipped)`);
console.log(`\n  Compression: ${((1 - minSize / fullSize) * 100).toFixed(1)}% minified, ${((1 - gzipSize / fullSize) * 100).toFixed(1)}% gzipped`);
