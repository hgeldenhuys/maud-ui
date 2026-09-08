// 0.8 assets are self-contained under static/. Legacy dist/ remains a 0.7 snapshot.
import { build } from "esbuild";
import { readFileSync, readdirSync, mkdirSync } from "node:fs";
mkdirSync("static", { recursive: true });
const js = [
  readFileSync("dist/maud-ui.js.bak", "utf8"),
  ...readdirSync("dist/behaviors").filter(n => n.endsWith(".js")).sort().map(n => readFileSync(`dist/behaviors/${n}`, "utf8")),
  readFileSync("static/behaviors/curation.js", "utf8"),
].join("\n");
for (const minify of [false, true]) {
  const suffix = minify ? ".min" : "";
  await build({ stdin: { contents: js, loader: "js" }, minify, target: "es2020", outfile: `static/maud-ui${suffix}.js` });
  await build({ stdin: { contents: '@import "./css/maud-ui.css";\n@import "./static/styles/curation.css";', resolveDir: process.cwd(), loader: "css" }, bundle: true, minify, outfile: `static/maud-ui${suffix}.css` });
}
console.log("Built static/maud-ui{,.min}.{css,js}");
