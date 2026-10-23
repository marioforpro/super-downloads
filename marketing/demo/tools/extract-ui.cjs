// Extract the REAL app UI for the demo: renders src/ in WebKit (the engine of the
// app's WKWebView) with a fake Tauri API and staged rows, then writes
//   assets/app/app.css    — the app's own CSS, every selector scoped under .sd-app
//   assets/app/parts.json — HTML of each state the film shows
// Re-run after any app UI change:
//   NODE_PATH=$HOME/.npm-global/lib/node_modules node marketing/demo/tools/extract-ui.cjs
const http = require("http");
const fs = require("fs");
const os = require("os");
const path = require("path");
const { webkit } = require("playwright");

const ROOT = path.resolve(__dirname, "../../..");
const OUT = path.resolve(__dirname, "../assets/app");
const STAGE = fs.mkdtempSync(path.join(os.tmpdir(), "sd-demo-ui-"));

const MOCK = `(() => {
  localStorage.clear();
  localStorage.setItem("hasSeenOnboarding", "true");
  localStorage.setItem("activationEmail", "you@studio.com");
  localStorage.setItem("activationSynced", "1");
  localStorage.setItem("appSettings", JSON.stringify({ theme: "dark", keepHistory: true, videoQuality: "best" }));
  const t = (id) => "assets/img/" + id + ".jpg";
  const base = { conversionProgress: 0, speed: "", eta: "", completed: false, error: null, filePath: "", outputFormat: "mp4" };
  const spring = { ...base, url: "https://www.youtube.com/watch?v=WhWc3b3KhnY", title: "Spring — Blender Open Movie",
    thumbnail: t("WhWc3b3KhnY"), duration: "7:44", size: "301.4 MB", format: "MP4", resolution: "1080p", fps: "24fps" };
  const rows = [
    { ...spring, id: "s-dl", progress: 0, status: "downloading", speed: "18.42MiB/s", eta: "ETA 0:16" },
    { ...spring, id: "s-cv", progress: 100, conversionProgress: 0, status: "converting", eta: "ETA 0:06" },
    { ...spring, id: "s-ok", progress: 100, status: "completed", completed: true, filePath: "/Users/you/Downloads/Spring.mp4" },
    { ...base, id: "b1", url: "https://youtu.be/UXqq0ZvbOnk", title: "CHARGE — Blender Open Movie", thumbnail: t("UXqq0ZvbOnk"),
      progress: 0, status: "downloading", speed: "21.88MiB/s", eta: "ETA 0:09", duration: "4:23", size: "412.6 MB", format: "MP4", resolution: "2160p", fps: "24fps" },
    { ...base, id: "b2", url: "https://youtu.be/_cMxraX_5RE", title: "Sprite Fright — Blender Open Movie", thumbnail: t("_cMxraX_5RE"),
      progress: 0, status: "downloading", speed: "16.03MiB/s", eta: "ETA 0:21", duration: "10:30", size: "688.1 MB", format: "MP4", resolution: "1080p", fps: "24fps" },
    { ...base, id: "b3", url: "https://youtu.be/u9lj-c29dxI", title: "WING IT! — Blender Open Movie", thumbnail: t("u9lj-c29dxI"),
      progress: 0, status: "downloading", speed: "12.70MiB/s", eta: "ETA 0:07", duration: "3:58", size: "162.9 MB", format: "MP4", resolution: "1080p", fps: "24fps" },
  ];
  // Completed versions of the three Batch rows.
  for (const r of rows.slice(3)) rows.push({ ...r, id: r.id + "ok", progress: 100, status: "completed", completed: true, filePath: "/Users/you/Downloads/x.mp4" });
  window.__TAURI__ = {
    core: { convertFileSrc: (p) => p, invoke: async (cmd) => {
      if (cmd === "load_download_history") return location.search.includes("empty") ? "[]" : JSON.stringify(rows);
      if (cmd === "get_ytdlp_version") return { version: "2026.10.21", source: "bundled" };
      return null; } },
    event: { listen: async () => () => {} },
    app: { getVersion: async () => "1.5.4" },
    opener: { openUrl: async () => {}, openPath: async () => {} },
  };
})();`;

function stage() {
  for (const f of ["index.html", "main.js", "styles.css"]) fs.copyFileSync(path.join(ROOT, "src", f), path.join(STAGE, f));
  fs.writeFileSync(path.join(STAGE, "mock.js"), MOCK);
  // Thumbnails at the same root-relative path the composition uses.
  fs.cpSync(path.resolve(__dirname, "../assets/img"), path.join(STAGE, "assets/img"), { recursive: true });
  const html = path.join(STAGE, "index.html");
  fs.writeFileSync(html, fs.readFileSync(html, "utf8").replace('<script type="module" src="main.js">', '<script src="mock.js"></script>\n    <script type="module" src="main.js">'));
}

const TYPES = { ".html": "text/html", ".js": "text/javascript", ".css": "text/css", ".jpg": "image/jpeg", ".svg": "image/svg+xml" };
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
  const browser = await webkit.launch();
  const page = await (await browser.newContext({ viewport: { width: 480, height: 640 }, colorScheme: "dark" })).newPage();
  page.on("pageerror", (e) => console.warn("pageerror:", e.message));
  await page.goto(`http://localhost:${server.address().port}/index.html`);
  await page.waitForTimeout(900);

  // The app's CSS, scoped: :root/html/body become the .sd-app wrapper.
  const css = await page.evaluate(() => {
    const scope = (sel) => sel.split(",").map((s) => {
      s = s.trim();
      if (/^(:root|html|body)$/.test(s)) return ".sd-app";
      if (/^(html|body)[.:\[\s>]/.test(s)) return s.replace(/^(html|body)/, ".sd-app");
      return ".sd-app " + s;
    }).join(", ");
    const walk = (rules) => [...rules].map((r) => {
      if (r.type === CSSRule.STYLE_RULE) return `${scope(r.selectorText)} { ${r.style.cssText} }`;
      if (r.type === CSSRule.MEDIA_RULE) return /reduced-motion/.test(r.conditionText) ? "" : `@media ${r.conditionText} { ${walk(r.cssRules)} }`;
      return r.cssText; // @keyframes, @font-face
    }).join("\n");
    return [...document.styleSheets].map((s) => walk(s.cssRules)).join("\n");
  });

  const parts = await page.evaluate(() => {
    const row = (id) => document.querySelector(`.download-item[data-download-id="${id}"]`).outerHTML;
    const out = {};
    out.empty = document.querySelector(".top-bar").outerHTML;
    const input = document.querySelector("#url-input");
    input.value = "https://www.youtube.com/watch?v=WhWc3b3KhnY";
    document.querySelector("#download-btn").classList.add("is-ready");
    out.topReady = document.querySelector(".top-bar").outerHTML;
    for (const id of ["s-dl", "s-cv", "s-ok", "b1", "b2", "b3", "b1ok", "b2ok", "b3ok"]) out[id] = row(id);
    return out;
  });
  // Batch mode on: the real toggle + ribbon.
  await page.evaluate(() => { document.querySelector("#url-input").value = ""; document.querySelector("#download-btn").classList.remove("is-ready"); });
  await page.click("#batch-toggle");
  await page.waitForTimeout(400);
  Object.assign(parts, await page.evaluate(() => ({
    topBatch: document.querySelector(".top-bar").outerHTML,
    ribbon: document.querySelector("#auto-add-ribbon").outerHTML,
  })));

  // The empty list, exactly as a fresh app shows it.
  await page.goto(`http://localhost:${server.address().port}/index.html?empty=1`);
  await page.waitForTimeout(700);
  parts.emptyList = await page.evaluate(() => document.querySelector("#download-list").innerHTML.trim());

  fs.mkdirSync(OUT, { recursive: true });
  fs.writeFileSync(path.join(OUT, "app.css"), `/* Generated by tools/extract-ui.cjs from src/styles.css — do not edit */\n${css}\n`);
  fs.writeFileSync(path.join(OUT, "parts.json"), JSON.stringify(parts, null, 2));
  console.log("app.css", css.length, "bytes; parts:", Object.keys(parts).join(", "));
  await browser.close();
  server.close();
  fs.rmSync(STAGE, { recursive: true, force: true });
})();
