// Renders the press-kit logos into web/public/press/:
//   sd-icon-1024.png, sd-icon-512.png              (app icon, works on any background)
//   sd-logo-on-light.png, sd-logo-on-dark.png      (icon + «Super Downloads» in Geist, transparent)
// Run: node scripts/press/render-logos.cjs   (re-run if icon.svg or the wordmark changes)
const fs = require("fs");
const path = require("path");
const { webkit } = require("playwright");

const ROOT = path.resolve(__dirname, "../..");
const OUT = path.join(ROOT, "web/public/press");
const ICON = fs.readFileSync(path.join(ROOT, "web/public/icon.svg"), "utf8");
const FONT = path.join(ROOT, "web/node_modules/@fontsource-variable/geist/files/geist-latin-wght-normal.woff2");
const fontData = fs.readFileSync(FONT).toString("base64");

const page = (body) => `<!doctype html><html><head><style>
@font-face { font-family: "Geist"; src: url(data:font/woff2;base64,${fontData}) format("woff2"); font-weight: 100 900; }
html, body { margin: 0; background: transparent; }
.lockup { display: inline-flex; align-items: center; gap: 28px; padding: 24px; font: 600 88px/1 "Geist", sans-serif; letter-spacing: -0.03em; }
.lockup svg { width: 120px; height: 120px; }
.icon { display: inline-block; } .icon svg { display: block; }
</style></head><body>${body}</body></html>`;

(async () => {
  fs.mkdirSync(OUT, { recursive: true });
  fs.writeFileSync(path.join(OUT, "sd-icon.svg"), ICON, { mode: 0o644 });
  const browser = await webkit.launch();
  const ctx = await browser.newContext({ deviceScaleFactor: 2 });
  const p = await ctx.newPage();

  for (const size of [1024, 512]) {
    const svg = ICON.replace(/<svg /, `<svg width="${size / 2}" height="${size / 2}" `); // ×2 device scale
    await p.setContent(page(`<div class="icon">${svg}</div>`));
    await p.locator(".icon").screenshot({ path: path.join(OUT, `sd-icon-${size}.png`), omitBackground: true });
  }
  for (const [name, color] of [["on-light", "#0a163f"], ["on-dark", "#ffffff"]]) {
    await p.setContent(page(`<div class="lockup" style="color:${color}">${ICON}<span>Super Downloads</span></div>`));
    await p.evaluate(() => document.fonts.ready);
    await p.locator(".lockup").screenshot({ path: path.join(OUT, `sd-logo-${name}.png`), omitBackground: true });
  }
  await browser.close();
  console.log("press logos →", path.relative(ROOT, OUT));
})();
