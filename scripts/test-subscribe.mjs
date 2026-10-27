// Offline test of the website news sign-up: the /api/subscribe handler (fetch mocked)
// and the form on the built site (Playwright WebKit over web/dist, /api/subscribe routed).
// Run: (cd web && npm run build) && node scripts/test-subscribe.mjs
import fs from 'node:fs';
import http from 'node:http';
import path from 'node:path';
import { createRequire } from 'node:module';
import subscribe from '../web/api/subscribe.js';

let pass = 0;
let fail = 0;
const ok = (cond, name) => {
  if (cond) pass++;
  else {
    fail++;
    console.error('FAIL', name);
  }
};

// ---- API ----
let calls = [];
let airtableOk = true;
globalThis.fetch = async (url, opts = {}) => {
  calls.push({ url: String(url), method: opts.method || 'GET', body: opts.body ? JSON.parse(opts.body) : null });
  return { ok: airtableOk, status: airtableOk ? 200 : 422, json: async () => ({ records: [{ id: 'rec1' }] }) };
};
const res = () => ({ statusCode: 0, headers: {}, body: '', setHeader(k, v) { this.headers[k] = v; }, end(b) { this.body = b || ''; } });
const call = async (req) => { const r = res(); await subscribe(req, r); return r; };
const TEXT = 'Email me about new versions and news from Super Downloads.';
const good = { email: 'ed@example.com', consent: true, consentText: TEXT, place: 'faq' };

process.env.AIRTABLE_TOKEN = 't';
process.env.AIRTABLE_BASE_ID = 'appX';
process.env.AIRTABLE_SD_USERS_TABLE = 'tblX';

ok((await call({ method: 'OPTIONS' })).statusCode === 204, 'OPTIONS 204');
ok((await call({ method: 'GET' })).statusCode === 405, 'GET 405');
ok((await call({ method: 'POST', body: 'nope{' })).statusCode === 400, 'bad JSON 400');
ok((await call({ method: 'POST', body: { ...good, email: 'nope' } })).statusCode === 400, 'bad email 400');
calls = [];
ok((await call({ method: 'POST', body: { ...good, consent: false } })).statusCode === 400 && calls.length === 0, 'unticked box → 400, nothing stored');
ok((await call({ method: 'POST', body: { ...good, consentText: '  ' } })).statusCode === 400 && calls.length === 0, 'no consent wording → 400, nothing stored');
ok((await call({ method: 'POST', body: { ...good, website: 'http://spam' } })).statusCode === 200 && calls.length === 0, 'honeypot → quiet 200, nothing stored');

calls = [];
const r1 = await call({ method: 'POST', body: good });
const f = calls[0]?.body?.records?.[0]?.fields || {};
ok(r1.statusCode === 200 && calls.length === 1 && calls[0].method === 'POST', 'good sign-up stored once');
ok(f.Source === 'web' && f.Email === 'ed@example.com' && f.Instance === 'web:faq', 'Source web, email, place');
ok(f['Marketing Opt-in'] === true && f['Marketing Consent Text'] === TEXT && !!f['Marketing Consent At'], 'consent wording + time stored');
ok(!('Activated At' in f), 'not counted as an activation');
ok(!/other Super (apps|tools)/i.test(f['Marketing Consent Text']), 'web consent does not cover other Super apps');

calls = [];
await call({ method: 'POST', body: { ...good, place: 'elsewhere' } });
ok(calls[0]?.body?.records?.[0]?.fields?.Instance === 'web:web', 'unknown place normalised');

airtableOk = false;
ok((await call({ method: 'POST', body: good })).statusCode === 502, 'Airtable error → 502');
airtableOk = true;
delete process.env.AIRTABLE_TOKEN;
ok((await call({ method: 'POST', body: good })).statusCode === 503, 'missing env → 503');

// ---- Form on the built site ----
const DIST = path.resolve('web/dist');
let webkit;
try { webkit = createRequire(import.meta.url)(path.resolve('node_modules/playwright')).webkit; } catch {}
if (!webkit || !fs.existsSync(path.join(DIST, 'faq/index.html'))) {
  console.log('SKIP form checks (needs playwright + web/dist; see header)');
} else {
  const TYPES = { '.html': 'text/html', '.js': 'text/javascript', '.css': 'text/css', '.svg': 'image/svg+xml', '.woff2': 'font/woff2', '.webp': 'image/webp' };
  const srv = http.createServer((q, r) => {
    let p = path.join(DIST, decodeURIComponent(new URL(q.url, 'http://x').pathname));
    if (fs.existsSync(p) && fs.statSync(p).isDirectory()) p = path.join(p, 'index.html');
    if (!fs.existsSync(p)) { r.writeHead(404); return r.end(); }
    r.writeHead(200, { 'Content-Type': TYPES[path.extname(p)] || 'application/octet-stream' });
    fs.createReadStream(p).pipe(r);
  });
  await new Promise((r) => srv.listen(0, r));
  const port = srv.address().port;
  const browser = await webkit.launch();
  const page = await browser.newPage({ viewport: { width: 390, height: 844 } });
  const posted = [];
  await page.route('**/api/subscribe', async (route) => {
    posted.push(JSON.parse(route.request().postData()));
    await route.fulfill({ status: 200, contentType: 'application/json', body: '{"ok":true}' });
  });
  await page.goto(`http://localhost:${port}/faq`);
  const form = page.locator('form[data-subscribe][data-place="faq"]');
  const statusText = () => form.locator('.sub-status').textContent();
  ok(!(await form.locator('input[name="consent"]').isChecked()), 'box starts unticked');
  await form.locator('button[type="submit"]').click();
  ok(/valid email/.test(await statusText()) && posted.length === 0, 'empty email → message, nothing sent');
  await form.locator('input[name="email"]').fill('ed@example.com');
  await form.locator('button[type="submit"]').click();
  ok(/Tick the box/.test(await statusText()) && posted.length === 0, 'unticked → message, nothing sent');
  await form.locator('input[name="consent"]').check();
  await form.locator('button[type="submit"]').click();
  await page.waitForTimeout(300);
  ok(posted.length === 1 && posted[0].consent === true && posted[0].place === 'faq', 'ticked → one POST with consent');
  ok(posted[0]?.consentText?.startsWith('Email me about new versions'), 'the stored wording is the visible label');
  ok(/on the list/.test(await statusText()), 'success message');
  ok(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth), '/faq no horizontal overflow at 390');

  // Home: the hero form is only for visitors who can't install (not a Mac).
  const mac = await browser.newPage({ viewport: { width: 1440, height: 900 }, userAgent: 'Mozilla/5.0 (Macintosh; Intel Mac OS X 14_0) AppleWebKit/605.1.15 Version/17.0 Safari/605.1.15' });
  await mac.goto(`http://localhost:${port}/`);
  ok(await mac.locator('#mobile-sub').isHidden(), 'Mac visitor: no hero form');
  ok(await mac.locator('form[data-place="download"]').count() === 1, 'download section has the form');
  const phone = await browser.newPage({ viewport: { width: 390, height: 844 }, userAgent: 'Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) AppleWebKit/605.1.15 Version/17.0 Mobile/15E148 Safari/604.1' });
  await phone.goto(`http://localhost:${port}/`);
  ok(await phone.locator('#mobile-sub').isVisible(), 'phone visitor: hero form shown');
  ok(await phone.evaluate(() => document.documentElement.scrollWidth <= innerWidth), 'home no horizontal overflow at 390');
  await browser.close();
  srv.close();
}

console.log(`${pass} passed, ${fail} failed`);
process.exit(fail ? 1 : 0);
