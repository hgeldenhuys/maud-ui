import { build } from "esbuild";
import { mkdirSync } from "node:fs";

mkdirSync("dist", { recursive: true });

await build({
  entryPoints: ["js/maud-ui.ts"],
  bundle: true,
  format: "iife",
  globalName: "MaudUIBundle",
  platform: "browser",
  target: "es2020",
  minify: true,
  outfile: "dist/maud-ui.js",
});

console.log("✓ dist/maud-ui.js built");
