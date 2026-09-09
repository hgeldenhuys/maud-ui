// 0.9 CSS is authored under static/. Runtime overrides compose with read-only legacy behaviors.
import { build } from "esbuild";
import { readFileSync, readdirSync, mkdirSync, existsSync } from "node:fs";
const check = process.argv.includes("--check");
if (!check) mkdirSync("static", { recursive: true });
async function emit(options) {
  const result = await build({ ...options, write: !check });
  if (check && !Buffer.from(result.outputFiles[0].contents).equals(readFileSync(options.outfile))) {
    throw new Error(`${options.outfile} is stale; run node examples/build-assets.mjs`);
  }
}
const legacyBehaviors = readdirSync("dist/behaviors").filter(n => n.endsWith(".js")).sort();
const js = [
  readFileSync("dist/maud-ui.js.bak", "utf8"),
  ...legacyBehaviors.map(n => readFileSync(existsSync(`static/behaviors/${n}`) ? `static/behaviors/${n}` : `dist/behaviors/${n}`, "utf8")),
  ...readdirSync("static/behaviors").filter(n => n.endsWith(".js") && !legacyBehaviors.includes(n)).sort().map(n => readFileSync(`static/behaviors/${n}`, "utf8")),
].join("\n");
for (const minify of [false, true]) {
  const suffix = minify ? ".min" : "";
  await emit({ stdin: { contents: js, loader: "js" }, minify, target: "es2020", outfile: `static/maud-ui${suffix}.js` });
  await emit({ stdin: { contents: '@import "./static/styles/maud-ui.css";', resolveDir: process.cwd(), loader: "css" }, bundle: true, minify, outfile: `static/maud-ui${suffix}.css` });
}
console.log(`${check ? "Verified" : "Built"} static/maud-ui{,.min}.{css,js}`);
