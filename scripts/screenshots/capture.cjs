// Regenerate the landing screenshots (web/public/shots/*.webp) from the real app frontend.
//
// Renders src/ in WebKit (the engine behind the app's WKWebView) with a fake Tauri API
// (mock.js) and staged downloads. Thumbnails are Blender Studio open movies (CC BY 4.0),
// credited in the landing footer.
//
// Setup once:  npm i --no-save playwright@1.59 && npx playwright install webkit
// Run:         node scripts/screenshots/capture.cjs       (needs cwebp: brew install webp)
const http = require("http");
const fs = require("fs");
const os = require("os");
const path = require("path");
const { execFileSync } = require("child_process");
const { webkit } = require("playwright");

const ROOT = path.resolve(__dirname, "../..");
const OUT = path.join(ROOT, "web/public/shots");
const CACHE = path.join(__dirname, ".cache");
const STAGE = fs.mkdtempSync(path.join(os.tmpdir(), "sd-shots-"));

// YouTube IDs of the Blender open movies used as thumbnails (see mock.js).
const THUMBS = ["_cMxraX_5RE", "UXqq0ZvbOnk", "WhWc3b3KhnY", "PVGeM40dABA", "u9lj-c29dxI", "Y-rmzh0PI3c", "mN0zPOpADL4", "pKmSdY56VtY"];

// Scenes used on the landing; `act` drives the UI before the capture.
// `fit` returns the y (CSS px) where the window should end, so crops land on a
// row boundary instead of cutting through one.
const rowEnd = (n, pad = 10) => (pg) =>
  pg.evaluate(([n, pad]) => {
    const rows = document.querySelectorAll(".download-item");
    const r = rows[Math.min(n, rows.length) - 1].getBoundingClientRect();
    return Math.ceil(r.bottom + pad);
  }, [n, pad]);
const SCENES = [
  { scene: "list", theme: "dark", h: 520, fit: rowEnd(4) },
  { scene: "convert", theme: "light", h: 400, fit: rowEnd(2) },
  { scene: "empty", theme: "dark", h: 250, clip: "https://youtu.be/WhWc3b3KhnY" },
  { scene: "batch", theme: "dark", h: 420, act: (pg) => pg.click("#batch-toggle"),
    fit: async (pg) => (await rowEnd(3, 0)(pg)) + 60 },
  { scene: "settings", theme: "light", h: 520, act: (pg) => pg.click("#settings-toggle"),
    fit: (pg) => pg.evaluate(() => Math.ceil(document.querySelector("#engine-section").closest(".settings-group").getBoundingClientRect().bottom + 18)) },
  { scene: "menu", theme: "dark", h: 420, act: (pg) => pg.click('.download-item-menu-btn[data-download-id="d3"]') },
  { scene: "onboarding", theme: "light", h: 640 },
];

function stage() {
  for (const f of ["index.html", "main.js", "styles.css"]) fs.copyFileSync(path.join(ROOT, "src", f), path.join(STAGE, f));
  fs.copyFileSync(path.join(__dirname, "mock.js"), path.join(STAGE, "mock.js"));
  const html = path.join(STAGE, "index.html");
  fs.writeFileSync(html, fs.readFileSync(html, "utf8").replace('<script type="module" src="main.js">', '<script src="mock.js"></script>\n    <script type="module" src="main.js">'));
  fs.mkdirSync(path.join(STAGE, "thumbs"));
  fs.mkdirSync(CACHE, { recursive: true });
  for (const id of THUMBS) {
    const cached = path.join(CACHE, `${id}.jpg`);
    if (!fs.existsSync(cached)) execFileSync("curl", ["-sfL", "-o", cached, `https://i.ytimg.com/vi/${id}/maxresdefault.jpg`]);
    execFileSync("sips", ["-s", "format", "jpeg", "-Z", "480", cached, "--out", path.join(STAGE, "thumbs", `${id}.jpg`)], { stdio: "ignore" });
  }
}

const TYPES = { ".html": "text/html", ".js": "text/javascript", ".css": "text/css", ".jpg": "image/jpeg" };
const server = http.createServer((req, res) => {
  const p = path.join(STAGE, decodeURIComponent(new URL(req.url, "http://x").pathname));
  fs.readFile(p, (err, buf) => {
    if (err) { res.writeHead(404); return res.end(); }
    res.writeHead(200, { "Content-Type": TYPES[path.extname(p)] || "application/octet-stream" });
    res.end(buf);
  });
});

(async () => {
  stage();
  await new Promise((r) => server.listen(0, r));
  const port = server.address().port;
  const browser = await webkit.launch();
  fs.mkdirSync(OUT, { recursive: true });
  for (const s of SCENES) {
    const ctx = await browser.newContext({ viewport: { width: 560, height: s.h }, deviceScaleFactor: 2, colorScheme: s.theme });
    const pg = await ctx.newPage();
    pg.on("pageerror", (e) => console.warn(`[${s.scene}-${s.theme}]`, e.message));
    const clip = s.clip ? `&clip=${encodeURIComponent(s.clip)}` : "";
    await pg.goto(`http://localhost:${port}/index.html?theme=${s.theme}&scene=${s.scene}${clip}`);
    await pg.waitForTimeout(900);
    if (s.act) { await s.act(pg); await pg.waitForTimeout(700); }
    if (s.fit) {
      await pg.setViewportSize({ width: 560, height: await s.fit(pg) });
      await pg.waitForTimeout(300);
    }
    const png = path.join(STAGE, `${s.scene}-${s.theme}.png`);
    await pg.screenshot({ path: png });
    execFileSync("cwebp", ["-quiet", "-q", "88", png, "-o", path.join(OUT, `${s.scene}-${s.theme}.webp`)]);
    // 1× variant for srcset (the landing shows windows at ≤ 600 CSS px).
    execFileSync("cwebp", ["-quiet", "-q", "86", "-resize", "560", "0", png, "-o", path.join(OUT, `${s.scene}-${s.theme}-560.webp`)]);
    console.log("✓", `${s.scene}-${s.theme}.webp`);
    await ctx.close();
  }
  await browser.close();
  server.close();
  fs.rmSync(STAGE, { recursive: true, force: true });
})();
