// Behaviour test for the «What's new» card after an in-app update (src/main.js).
// Runs src/ in WebKit with a fake Tauri API whose app version the test sets.
// Setup once: npm i --no-save playwright@1.59 && npx playwright install webkit
// Run:        node scripts/test-whats-new.cjs

const ROOT = require('path').resolve(__dirname, '..');
const { webkit } = require(require('path').join(ROOT, 'node_modules/playwright'));
const http = require('http'), fs = require('fs'), path = require('path'), os = require('os');
const STAGE = fs.mkdtempSync(path.join(os.tmpdir(), 'sd-whatsnew-'));
for (const f of ['index.html', 'main.js', 'styles.css']) fs.copyFileSync(path.join(ROOT, 'src', f), path.join(STAGE, f));
fs.writeFileSync(path.join(STAGE, 'fake.js'), `
  window.__calls = []; window.__opened = [];
  const ver = localStorage.getItem('__testVersion') || '1.5.7';
  const invoke = async (cmd) => { window.__calls.push(cmd);
    if (cmd === 'check_for_update') return { version: '1.5.9', notes: '• Faster 4K conversion.\\n• Batch skips duplicates.' };
    if (cmd === 'install_update') return null;
    if (cmd === 'send_active_ping') return '00000000-0000-4000-8000-000000000001';
    if (cmd === 'load_download_history') return '[]';
    if (cmd === 'get_ytdlp_version') return { version: 'x', source: 'bundled' };
    return null; };
  window.__TAURI__ = { core: { invoke, convertFileSrc: p => p }, event: { listen: async () => () => {} }, app: { getVersion: async () => ver },
    opener: { openUrl: async (u) => { window.__opened.push(u); } }, window: { getCurrentWindow: () => ({ onFocusChanged: async () => () => {}, setFocus: async () => {} }) },
    webviewWindow: { getCurrentWebviewWindow: () => ({ onDragDropEvent: async () => () => {} }) } };`);
const html = path.join(STAGE, 'index.html');
fs.writeFileSync(html, fs.readFileSync(html, 'utf8').replace('<script type="module" src="main.js">', '<script src="fake.js"></script>\n    <script type="module" src="main.js">'));
const server = http.createServer((q, r) => fs.readFile(path.join(STAGE, new URL(q.url, 'http://x').pathname), (e, b) => { if (e) { r.writeHead(404); return r.end(); } r.writeHead(200, { 'Content-Type': q.url.endsWith('.js') ? 'text/javascript' : q.url.endsWith('.css') ? 'text/css' : 'text/html' }); r.end(b); }));

let fail = 0; const ok = (c, m) => { console.log((c ? 'PASS ' : 'FAIL ') + m); if (!c) fail++; };
const ls = (pg, k) => pg.evaluate((k) => localStorage.getItem(k), k);
const setLs = (pg, kv) => pg.evaluate((kv) => Object.entries(kv).forEach(([k, v]) => localStorage.setItem(k, v)), kv);
const card = (pg) => pg.$('.toast.whats-new');

(async () => {
  await new Promise((r) => server.listen(0, r));
  const url = `http://localhost:${server.address().port}/index.html`;
  const b = await webkit.launch();
  const ctx = await b.newContext(); const pg = await ctx.newPage();
  pg.on('pageerror', (e) => console.log('pageerror', e.message));
  await pg.goto(url);
  await setLs(pg, { hasSeenOnboarding: 'true', activationEmail: 'a@b.co', activationSynced: '1', __testVersion: '1.5.7' });
  await pg.reload(); await pg.waitForTimeout(1800);
  ok(!(await card(pg)), 'no card on a normal launch');

  // Update on offer (the app checks 1.5 s after launch) → clicking Update stores what is being installed.
  ok(!(await pg.evaluate(() => document.getElementById('update-banner').hidden)), 'update banner shows the new version');
  await pg.click('#update-banner-btn'); await pg.waitForTimeout(200);
  const saved = JSON.parse((await ls(pg, 'whatsNew')) || 'null');
  ok(saved?.version === '1.5.9' && /Faster 4K/.test(saved.notes), 'Update click stores the version and its notes');

  // Relaunch on an older version (install failed): no card, kept for later.
  await pg.reload(); await pg.waitForTimeout(1800);
  ok(!(await card(pg)) && (await ls(pg, 'whatsNew')) !== null, 'still on the old version: no card, notes kept');

  // Relaunch on the new version: the card, once.
  await setLs(pg, { __testVersion: '1.5.9' });
  await pg.reload(); await pg.waitForTimeout(1800);
  const c = await card(pg);
  ok(!!c && /Updated to 1\.5\.9/.test(await c.textContent()), 'new version: card says what it updated to');
  if (process.env.SHOT) await pg.screenshot({ path: process.env.SHOT });
  ok((await pg.$$eval('.toast.whats-new li', (l) => l.map((x) => x.textContent))).join('|') === 'Faster 4K conversion.|Batch skips duplicates.', 'card lists the notes, bullets stripped');
  ok((await ls(pg, 'whatsNew')) === null, 'shown once: notes cleared');
  await pg.click('[data-whats-new="notes"]');
  ok((await pg.evaluate(() => window.__opened.at(-1))) === 'https://superdownloads.app/changelog#v1.5.9', '«All release notes» opens /changelog#v1.5.9');
  await pg.reload(); await pg.waitForTimeout(1800);
  ok(!(await card(pg)), 'next launch: no card');

  // Notes from an older update when a newer version is already installed: dropped silently.
  await setLs(pg, { whatsNew: JSON.stringify({ version: '1.5.8', notes: '• Old' }) });
  await pg.reload(); await pg.waitForTimeout(1800);
  ok(!(await card(pg)) && (await ls(pg, 'whatsNew')) === null, 'stale notes (older than the app): dropped, no card');

  // Text is inserted as text, never as HTML.
  await setLs(pg, { whatsNew: JSON.stringify({ version: '1.5.9', notes: '<img src=x onerror="window.__xss=1">' }) });
  await pg.reload(); await pg.waitForTimeout(1800);
  ok((await pg.evaluate(() => window.__xss)) === undefined && /<img/.test(await (await card(pg)).textContent()), 'notes are shown as text, not HTML');

  await b.close(); server.close(); fs.rmSync(STAGE, { recursive: true, force: true });
  console.log(fail ? `${fail} FAILED` : 'ALL PASS'); process.exit(fail ? 1 : 0);
})();
