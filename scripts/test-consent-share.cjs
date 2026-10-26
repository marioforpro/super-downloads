// Behaviour test for the 1.5.6 activation consent box and the one-time share notice
// (src/main.js). Runs src/ in WebKit with a fake Tauri API, like test-active-ping.cjs.
// Run: node scripts/test-consent-share.cjs

const ROOT = require("path").resolve(__dirname, "..");
const { webkit } = require("playwright");
const http = require("http"), fs = require("fs"), path = require("path"), os = require("os");
const STAGE = fs.mkdtempSync(path.join(os.tmpdir(), "sd-consent-"));
for (const f of ["index.html", "main.js", "styles.css"]) fs.copyFileSync(path.join(ROOT, "src", f), path.join(STAGE, f));
// Fake backend: records invoke calls; event.listen keeps the handlers so the test can fire events.
fs.writeFileSync(path.join(STAGE, "fake.js"), `
  window.__calls = []; window.__handlers = {};
  const invoke = async (cmd, args) => { window.__calls.push({ cmd, args });
    if (cmd === 'load_download_history') return '[]';
    if (cmd === 'get_ytdlp_version') return { version: 'x', source: 'bundled' };
    if (cmd === 'send_active_ping') return '00000000-0000-4000-8000-000000000001';
    return null; };
  window.__TAURI__ = { core: { invoke, convertFileSrc: p => p },
    event: { listen: async (name, fn) => { window.__handlers[name] = fn; return () => {}; } },
    app: { getVersion: async () => '1.5.6' }, opener: { openUrl: async () => {} },
    window: { getCurrentWindow: () => ({ onFocusChanged: async () => () => {}, setFocus: async () => {} }) },
    webviewWindow: { getCurrentWebviewWindow: () => ({ onDragDropEvent: async () => () => {} }) } };
  window.__finish = (id) => window.__handlers['download-finished']({ payload: [id, 'Clip ' + id, '/tmp/' + id + '.mp4', '', '', 'mp4', '1080p', '', ''] });`);
const html = path.join(STAGE, "index.html");
fs.writeFileSync(html, fs.readFileSync(html, "utf8").replace('<script type="module" src="main.js">', '<script src="fake.js"></script>\n    <script type="module" src="main.js">'));
const server = http.createServer((q, r) => fs.readFile(path.join(STAGE, new URL(q.url, "http://x").pathname), (e, b) => { if (e) { r.writeHead(404); return r.end(); } r.writeHead(200, { "Content-Type": q.url.endsWith(".js") ? "text/javascript" : q.url.endsWith(".css") ? "text/css" : "text/html" }); r.end(b); }));

let fail = 0;
const ok = (c, m) => { console.log((c ? "PASS " : "FAIL ") + m); if (!c) fail++; };
const activations = (pg) => pg.evaluate(() => window.__calls.filter((c) => c.cmd === "register_activation").map((c) => c.args));

(async () => {
  await new Promise((r) => server.listen(0, r));
  const url = `http://localhost:${server.address().port}/index.html`;
  const b = await webkit.launch();

  // A: fresh install → welcome screen with the consent box unticked
  for (const tick of [false, true]) {
    const ctx = await b.newContext({ permissions: [] });
    const pg = await ctx.newPage();
    pg.on("pageerror", (e) => console.log("pageerror", e.message));
    await pg.goto(url);
    await pg.waitForSelector(".activation-optin-checkbox");
    if (!tick) {
      ok(await pg.isChecked(".activation-optin-checkbox") === false, "consent box starts unticked");
      ok(/Email me about updates and other Super apps/.test(await pg.innerText(".activation-optin")), "consent box wording");
    }
    await pg.fill(".activation-email-input", "a@b.co");
    if (tick) await pg.check(".activation-optin-checkbox");
    await pg.click(".onboarding-start-btn");
    await pg.waitForTimeout(400);
    const a = (await activations(pg)).at(-1);
    if (tick) ok(a && a.marketingOptIn === true && a.marketingConsentText === "Email me about updates and other Super apps", "ticked → opt-in with its wording");
    else ok(a && a.marketingOptIn === false && a.marketingConsentText === "", "unticked → no opt-in, no wording");
    await ctx.close();
  }

  // B: share notice after the 10th finished download, once
  const ctx = await b.newContext();
  const pg = await ctx.newPage();
  pg.on("pageerror", (e) => console.log("pageerror", e.message));
  await pg.goto(url);
  await pg.evaluate(() => { localStorage.setItem("hasSeenOnboarding", "true"); localStorage.setItem("activationEmail", "a@b.co"); localStorage.setItem("activationSynced", "1"); });
  await pg.reload();
  await pg.waitForFunction(() => window.__handlers["download-finished"]);
  await pg.evaluate(() => { document.hasFocus = () => true; });
  for (let i = 1; i <= 9; i++) await pg.evaluate((i) => window.__finish("d" + i), i);
  ok(await pg.locator(".share-nudge").count() === 0, "no notice before the 10th download");
  await pg.evaluate(() => window.__finish("d10"));
  ok(await pg.locator(".share-nudge").count() === 1, "notice after the 10th download");
  ok(/Send them the link/.test(await pg.innerText(".share-nudge")), "notice copy");
  await pg.click('.share-nudge [data-share="no"]');
  await pg.waitForTimeout(400);
  ok(await pg.locator(".share-nudge").count() === 0, "No thanks closes it");
  await pg.evaluate(() => window.__finish("d11"));
  ok(await pg.locator(".share-nudge").count() === 0, "never shown twice");

  // C: not while the window is in the background (shows at a later finish instead)
  await pg.evaluate(() => { localStorage.removeItem("shareNudgeShown"); document.hasFocus = () => false; window.__finish("d12"); });
  ok(await pg.locator(".share-nudge").count() === 0, "not shown with the window in the background");
  await pg.evaluate(() => { document.hasFocus = () => true; window.__finish("d13"); });
  ok(await pg.locator(".share-nudge").count() === 1, "shown at the next finish in front");

  await b.close();
  server.close();
  console.log(fail ? `${fail} FAILED` : "ALL PASS");
  process.exit(fail ? 1 : 0);
})();
