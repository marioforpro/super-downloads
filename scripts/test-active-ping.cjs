// Behaviour test for the anonymous daily "active" ping (src/main.js; payload in
// src-tauri/src/active_ping.rs). Runs src/ in WebKit with a fake Tauri API and a fake clock.
// Setup once: npm i --no-save playwright@1.59 && npx playwright install webkit
// Run:        node scripts/test-active-ping.cjs

const ROOT = require("path").resolve(__dirname, "..");
const { webkit } = require("playwright");
const http = require('http'), fs = require('fs'), path = require('path'), os = require('os');
const STAGE = fs.mkdtempSync(path.join(os.tmpdir(), 'sd-ping-'));
for (const f of ['index.html', 'main.js', 'styles.css']) fs.copyFileSync(path.join(ROOT, 'src', f), path.join(STAGE, f));
// The fake backend behaves like send_active_ping: it keeps a valid id and makes a new one otherwise.
// window.__pingDelay (ms) keeps a ping in flight.
fs.writeFileSync(path.join(STAGE, 'fake.js'), `
  window.__calls = []; window.__pingArgs = []; window.__opened = []; let n = 0;
  const invoke = async (cmd, args) => { window.__calls.push(cmd);
    if (cmd === 'send_active_ping') { window.__pingArgs.push(args);
      if (window.__pingDelay) await new Promise(r => setTimeout(r, window.__pingDelay));
      return (args && args.installId) || '00000000-0000-4000-8000-' + String(++n + Date.now() % 1000000).padStart(12, '0'); }
    if (cmd === 'load_download_history') return '[]';
    if (cmd === 'get_ytdlp_version') return { version: 'x', source: 'bundled' };
    return null; };
  window.__TAURI__ = { core: { invoke, convertFileSrc: p => p }, event: { listen: async () => () => {} }, app: { getVersion: async () => '1.5.5' },
    opener: { openUrl: async (u) => { window.__opened.push(u); } }, window: { getCurrentWindow: () => ({ onFocusChanged: async () => () => {}, setFocus: async () => {} }) },
    webviewWindow: { getCurrentWebviewWindow: () => ({ onDragDropEvent: async () => () => {} }) } };`);
const html = path.join(STAGE, 'index.html');
fs.writeFileSync(html, fs.readFileSync(html, 'utf8').replace('<script type="module" src="main.js">', '<script src="fake.js"></script>\n    <script type="module" src="main.js">'));
const server = http.createServer((q, r) => fs.readFile(path.join(STAGE, new URL(q.url, 'http://x').pathname), (e, b) => { if (e) { r.writeHead(404); return r.end(); } r.writeHead(200, { 'Content-Type': q.url.endsWith('.js') ? 'text/javascript' : q.url.endsWith('.css') ? 'text/css' : 'text/html' }); r.end(b); }));
const pings = pg => pg.evaluate(() => window.__calls.filter(c => c === 'send_active_ping').length);
const lastSentId = pg => pg.evaluate(() => window.__pingArgs.at(-1)?.installId ?? null);
const ls = (pg, k) => pg.evaluate(k => localStorage.getItem(k), k);
const setLs = (pg, kv) => pg.evaluate(kv => Object.entries(kv).forEach(([k, v]) => localStorage.setItem(k, v)), kv);
const daysAgo = (pg, d) => pg.evaluate(d => { const t = new Date(Date.now() - d * 86400000); return `${t.getFullYear()}-${String(t.getMonth() + 1).padStart(2, '0')}-${String(t.getDate()).padStart(2, '0')}`; }, d);
const text = pg => pg.evaluate(() => document.body.innerText);
// Nothing about the ping is shown when the app opens (docs/DECISIONS.md 2026-10-26).
const NOTICE = /anonymous|usage stats|ping/i;
const switchTo = (pg, on) => pg.evaluate(on => { const c = document.querySelector('#usage-stats-enabled'); c.checked = on; c.dispatchEvent(new Event('change')); }, on);
const existingInstall = async (b) => {
  const ctx = await b.newContext(); const pg = await ctx.newPage();
  pg.on('pageerror', e => console.log('pageerror', e.message));
  await pg.clock.install();
  await pg.goto(url); await setLs(pg, { hasSeenOnboarding: 'true', activationEmail: 'a@b.co', activationSynced: '1' });
  await pg.reload(); await pg.clock.runFor(1000);
  return pg;
};
let url;
let fail = 0; const ok = (c, m) => { console.log((c ? 'PASS ' : 'FAIL ') + m); if (!c) fail++; };
(async () => {
  await new Promise(r => server.listen(0, r)); url = `http://localhost:${server.address().port}/index.html`;
  const b = await webkit.launch();

  // A: an install that saw the welcome screen before this version
  const pg = await existingInstall(b);
  ok(!NOTICE.test(await text(pg)) && !await pg.$('.toast'), 'existing install: no notice on open');
  ok(await pg.evaluate(() => document.querySelector('#usage-stats-enabled').checked) === true, 'on by default');
  ok(await pings(pg) === 0, 'no ping in the first seconds');
  await pg.clock.runFor(15000);
  ok(await pings(pg) === 1 && await lastSentId(pg) === null, 'one ping after 15 s, with no id yet (the backend makes one)');
  const id1 = await ls(pg, 'installId');
  ok(/^[0-9a-f-]{36}$/.test(id1 || ''), 'install id stored');
  ok(await ls(pg, 'installIdCreated') === await daysAgo(pg, 0), 'id creation day stored');
  await pg.clock.runFor(3 * 3600 * 1000 + 1000);
  ok(await pings(pg) === 1, 'no second ping the same day');
  await pg.reload(); await pg.clock.runFor(17000);
  ok(await pings(pg) === 0 && !NOTICE.test(await text(pg)), 'relaunch same day: no ping, no notice');

  // B: the id is kept for a year, then renewed
  await setLs(pg, { lastActivePingDay: '2000-01-01', installIdCreated: await daysAgo(pg, 100) });
  await pg.reload(); await pg.clock.runFor(17000);
  ok(await pings(pg) === 1 && await lastSentId(pg) === id1 && await ls(pg, 'installId') === id1, '100-day-old id: kept');
  await setLs(pg, { lastActivePingDay: '2000-01-01', installIdCreated: await daysAgo(pg, 366) });
  await pg.reload(); await pg.clock.runFor(17000);
  const id2 = await ls(pg, 'installId');
  ok(await pings(pg) === 1 && await lastSentId(pg) === null && id2 && id2 !== id1, '366-day-old id: renewed');
  ok(await ls(pg, 'installIdCreated') === await daysAgo(pg, 0), 'renewed id: creation day reset');
  await pg.evaluate(() => localStorage.removeItem('installIdCreated'));
  await setLs(pg, { lastActivePingDay: '2000-01-01' });
  await pg.reload(); await pg.clock.runFor(17000);
  ok(await lastSentId(pg) === null, 'id without a creation day: renewed');

  // C: off
  await setLs(pg, { lastActivePingDay: '2000-01-01' });
  await pg.reload(); await pg.evaluate(() => { window.__pingDelay = 5000; });
  await pg.clock.runFor(16000);
  ok(await pings(pg) === 1, 'ping in flight');
  await switchTo(pg, false); await pg.clock.runFor(6000);
  ok(await ls(pg, 'installId') === null && await ls(pg, 'installIdCreated') === null && await ls(pg, 'lastActivePingDay') === null,
    'turned off mid-ping: the id is forgotten and not written back');
  await setLs(pg, { lastActivePingDay: '2000-01-01' });
  await pg.clock.runFor(3 * 3600 * 1000 + 1000);
  ok(await pings(pg) === 1, 'off: no ping on a new day');
  await pg.reload(); await pg.clock.runFor(17000);
  ok(await pg.evaluate(() => document.querySelector('#usage-stats-enabled').checked) === false && await pings(pg) === 0, 'off survives a relaunch');

  // D: Settings shows what is sent, links to /privacy, and off before the first ping sends nothing
  const p3 = await existingInstall(b);
  await p3.click('#settings-toggle'); await p3.clock.runFor(500);
  ok(await p3.isVisible('#usage-stats-hint') && /app version, Mac type and a random number/.test(await p3.textContent('#usage-stats-hint')),
    'Settings: the switch says what is sent');
  await p3.click('#usage-stats-privacy-link');
  ok(await p3.evaluate(() => window.__opened.at(-1)) === 'https://superdownloads.app/privacy', 'Settings: Privacy link opens /privacy');
  await switchTo(p3, false); await p3.clock.runFor(20000);
  ok(await pings(p3) === 0, 'turned off before the first ping: nothing sent');
  await p3.reload(); await p3.clock.runFor(17000);
  ok(await pings(p3) === 0, 'off: no ping after relaunch');

  // E: brand-new install
  const ctx2 = await b.newContext(); const p2 = await ctx2.newPage(); await p2.clock.install();
  await p2.goto(url); await p2.clock.runFor(17000);
  ok(await p2.isVisible('.onboarding-overlay') && !NOTICE.test(await text(p2)), 'new install: the welcome screen says nothing about it');
  ok(await pings(p2) === 0, 'new install: no ping while the welcome screen is open');
  await p2.fill('.activation-email-input', 'new@user.co'); await p2.click('.onboarding-start-btn'); await p2.clock.runFor(500);
  ok(await pings(p2) === 1 && !NOTICE.test(await text(p2)) && !await p2.$('.toast'), 'new install: one ping, no notice after Get Started');

  await b.close(); server.close(); fs.rmSync(STAGE, { recursive: true, force: true });
  console.log(fail ? `${fail} FAILED` : 'ALL PASS'); process.exit(fail ? 1 : 0);
})();
