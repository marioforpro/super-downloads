// Behaviour test for the anonymous daily "active" ping (src/main.js; payload in
// src-tauri/src/active_ping.rs). Runs src/ in WebKit with a fake Tauri API and a fake clock.
// Setup once: npm i --no-save playwright@1.59 && npx playwright install webkit
// Run:        node scripts/test-active-ping.cjs

const ROOT = require("path").resolve(__dirname, "..");
const { webkit } = require("playwright");
const http = require('http'), fs = require('fs'), path = require('path'), os = require('os');
const STAGE = fs.mkdtempSync(path.join(os.tmpdir(), 'sd-ping-'));
for (const f of ['index.html', 'main.js', 'styles.css']) fs.copyFileSync(path.join(ROOT, 'src', f), path.join(STAGE, f));
fs.writeFileSync(path.join(STAGE, 'fake.js'), `
  window.__calls = [];
  const invoke = async (cmd, args) => { window.__calls.push(cmd);
    if (cmd === 'send_active_ping') return (args && args.installId) || '3f2b8c1e-9a4d-4e6f-8b7a-1c2d3e4f5a6b';
    if (cmd === 'load_download_history') return '[]';
    if (cmd === 'get_ytdlp_version') return { version: 'x', source: 'bundled' };
    return null; };
  window.__TAURI__ = { core: { invoke, convertFileSrc: p => p }, event: { listen: async () => () => {} }, app: { getVersion: async () => '1.5.5' },
    opener: { openUrl: async () => {} }, window: { getCurrentWindow: () => ({ onFocusChanged: async () => () => {}, setFocus: async () => {} }) },
    webviewWindow: { getCurrentWebviewWindow: () => ({ onDragDropEvent: async () => () => {} }) } };`);
const html = path.join(STAGE, 'index.html');
fs.writeFileSync(html, fs.readFileSync(html, 'utf8').replace('<script type="module" src="main.js">', '<script src="fake.js"></script>\n    <script type="module" src="main.js">'));
const server = http.createServer((q, r) => fs.readFile(path.join(STAGE, new URL(q.url, 'http://x').pathname), (e, b) => { if (e) { r.writeHead(404); return r.end(); } r.writeHead(200, { 'Content-Type': q.url.endsWith('.js') ? 'text/javascript' : q.url.endsWith('.css') ? 'text/css' : 'text/html' }); r.end(b); }));
const pings = pg => pg.evaluate(() => window.__calls.filter(c => c === 'send_active_ping').length);
const ls = (pg, k) => pg.evaluate(k => localStorage.getItem(k), k);
let fail = 0; const ok = (c, m) => { console.log((c ? 'PASS ' : 'FAIL ') + m); if (!c) fail++; };
(async () => {
  await new Promise(r => server.listen(0, r)); const url = `http://localhost:${server.address().port}/index.html`;
  const b = await webkit.launch();
  // A–C: an install that saw the welcome screen before this version
  const ctx = await b.newContext(); const pg = await ctx.newPage();
  pg.on('pageerror', e => console.log('pageerror', e.message));
  await pg.clock.install();
  await pg.goto(url); await pg.evaluate(() => { localStorage.setItem('hasSeenOnboarding', 'true'); localStorage.setItem('activationEmail', 'a@b.co'); localStorage.setItem('activationSynced', '1'); });
  await pg.reload(); await pg.clock.runFor(1000);
  ok(/one anonymous ping a day/.test(await pg.evaluate(() => document.body.innerText)), 'existing install: one-time notice shown');
  ok(await pings(pg) === 0, 'no ping before 15 s');
  await pg.clock.runFor(16000);
  ok(await pings(pg) === 1, 'one ping after start');
  ok(/^[0-9a-f-]{36}$/.test(await ls(pg, 'installId') || ''), 'install id stored');
  ok(!!await ls(pg, 'lastActivePingDay'), 'day stored');
  await pg.clock.runFor(3 * 3600 * 1000 + 1000);
  ok(await pings(pg) === 1, 'no second ping the same day');
  await pg.reload(); await pg.clock.runFor(17000);
  ok(await pings(pg) === 0 && !/one anonymous ping a day/.test(await pg.evaluate(() => document.body.innerText)), 'relaunch same day: no ping, no notice');
  await pg.evaluate(() => { const c = document.querySelector('#usage-stats-enabled'); c.checked = false; c.dispatchEvent(new Event('change')); });
  ok(await ls(pg, 'installId') === null, 'turning it off forgets the install id');
  await pg.evaluate(() => localStorage.setItem('lastActivePingDay', '2000-01-01'));
  await pg.clock.runFor(3 * 3600 * 1000 + 1000);
  ok(await pings(pg) === 0, 'off: no ping on a new day');
  await pg.reload(); await pg.clock.runFor(17000);
  ok(await pg.evaluate(() => document.querySelector('#usage-stats-enabled').checked) === false && await pings(pg) === 0, 'off survives a relaunch');
  // D: brand-new install
  const ctx2 = await b.newContext(); const p2 = await ctx2.newPage(); await p2.clock.install();
  await p2.goto(url); await p2.clock.runFor(17000);
  ok(await pings(p2) === 0, 'new install: no ping while the welcome screen is open');
  await p2.fill('.activation-email-input', 'new@user.co'); await p2.click('.onboarding-start-btn'); await p2.clock.runFor(500);
  ok(await pings(p2) === 1, 'new install: ping right after Get Started');
  ok(!/now sends one anonymous ping/.test(await p2.evaluate(() => document.body.innerText)), 'new install: no extra notice (welcome line told them)');
  await b.close(); server.close(); fs.rmSync(STAGE, { recursive: true, force: true });
  console.log(fail ? `${fail} FAILED` : 'ALL PASS'); process.exit(fail ? 1 : 0);
})();
