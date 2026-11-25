// Offline test of the abuse guards in web/api (fetch mocked by a small fake Airtable):
// one welcome email per address, activation dedupe, server-side consent wording,
// rate limits, Airtable timeout, and unsubscribe following `offset` pagination.
// Run: node scripts/test-abuse.mjs
import activate from '../web/api/activate.js';
import subscribe from '../web/api/subscribe.js';
import unsubscribe from '../web/api/unsubscribe.js';
import { unsubToken } from '../web/api/_unsub.js';

let pass = 0;
let fail = 0;
const ok = (cond, name) => {
  if (cond) pass++;
  else {
    fail++;
    console.error('FAIL', name);
  }
};

let rows = [];
let pageCap = 100;
let calls = [];
let nextId = 1;
globalThis.fetch = async (url, opts = {}) => {
  url = String(url);
  const method = opts.method || 'GET';
  const body = opts.body ? JSON.parse(opts.body) : null;
  calls.push({ url, method, body, signal: opts.signal });
  if (url.includes('resend')) return { ok: true, status: 200, json: async () => ({ id: 'em' }) };
  if (!url.includes('airtable')) return { ok: false, status: 404 };
  if (method === 'GET') {
    const u = new URL(url);
    const m = /='(.*)'\)?$/.exec(u.searchParams.get('filterByFormula') || '');
    const want = m ? m[1].replace(/\\'/g, "'") : '';
    const hits = rows.filter((r) => String(r.fields.Email || '').trim().toLowerCase() === want);
    const size = Math.min(Number(u.searchParams.get('pageSize')) || 100, pageCap);
    const start = Number(u.searchParams.get('offset')) || 0;
    const page = hits.slice(start, start + size);
    const more = start + size < hits.length;
    return { ok: true, status: 200, json: async () => ({ records: page, ...(more ? { offset: String(start + size) } : {}) }) };
  }
  if (method === 'POST') {
    const made = body.records.map((r) => ({ id: `rec${nextId++}`, fields: { ...r.fields } }));
    rows.push(...made);
    return { ok: true, status: 200, json: async () => ({ records: made.map((r) => ({ id: r.id })) }) };
  }
  if (method === 'PATCH') {
    for (const r of body.records) Object.assign(rows.find((x) => x.id === r.id).fields, r.fields);
    return { ok: true, status: 200, json: async () => ({ records: [] }) };
  }
  return { ok: false, status: 405 };
};
const res = () => ({ statusCode: 0, headers: {}, body: '', setHeader(k, v) { this.headers[k] = v; }, end(b) { this.body = b || ''; } });
const run = async (h, req) => { calls = []; const r = res(); await h(req, r); return r; };
const mails = () => calls.filter((c) => c.url.includes('resend'));
const inserts = () => calls.filter((c) => c.url.includes('airtable') && c.method === 'POST');

process.env.AIRTABLE_TOKEN = 't';
process.env.AIRTABLE_BASE_ID = 'appX';
process.env.AIRTABLE_SD_USERS_TABLE = 'tblX';
process.env.SD_WELCOME_EMAIL = 'on';
process.env.RESEND_API_KEY = 're_test';
process.env.SD_UNSUB_SECRET = 'test-secret';
const APP_TEXT = 'Email me about updates and other Super apps';
const WEB_TEXT = 'Email me about new versions and news from Super Downloads. A few emails a year; every one has a one-click unsubscribe.';
const base = { appVersion: '1.5.9', arch: 'aarch64', osVersion: '15.1', instance: 'Mac', source: 'app' };
const act = (body, headers = {}) => run(activate, { method: 'POST', body: { ...base, ...body }, headers });

// Welcome email at most once per address, activation deduped
let r = await act({ email: 'once@example.com' });
ok(r.statusCode === 200 && mails().length === 1, 'first activation → one email');
r = await act({ email: 'once@example.com' });
ok(r.statusCode === 200 && mails().length === 0 && inserts().length === 0, 'repeat activation → no second email, no second row');
r = await act({ email: 'ONCE@Example.com' });
ok(mails().length === 0 && inserts().length === 0, 'dedupe ignores case');
ok(rows.filter((x) => x.fields.Email.toLowerCase() === 'once@example.com').length === 1, 'one row for the address');

// Address already on the list (web sign-up) → activation never mails it
rows.push({ id: 'recW', fields: { Email: 'web@example.com', Source: 'web', 'Marketing Opt-in': true } });
r = await act({ email: 'web@example.com' });
ok(r.statusCode === 200 && mails().length === 0, 'existing row → no welcome email');

// Consent decided from the server copy, never from client text
r = await act({ email: 'forged@example.com', marketingOptIn: true, marketingConsentText: 'news about other Super apps' });
let f = rows.find((x) => x.fields.Email === 'forged@example.com').fields;
ok(f['Marketing Opt-in'] === false && !('Marketing Consent Text' in f), 'invented wording → no consent stored');
ok(mails().length === 1 && !mails()[0].body.text.includes('Super Guides'), 'invented wording → no P.S.');
r = await act({ email: 'real@example.com', marketingOptIn: true, marketingConsentText: APP_TEXT });
f = rows.find((x) => x.fields.Email === 'real@example.com').fields;
ok(f['Marketing Opt-in'] === true && f['Marketing Consent Text'] === APP_TEXT, 'known wording → consent stored with server copy');
ok(mails()[0]?.body.text.includes('Super Guides'), 'known wording → P.S.');

// Airtable calls carry a timeout signal
ok(calls.filter((c) => c.url.includes('airtable')).every((c) => c.signal), 'every Airtable fetch has a timeout signal');

// Rate limits
let last;
for (let i = 0; i < 40; i++) last = await act({ email: `bulk${i}@example.com` }, { 'x-forwarded-for': '203.0.113.9' });
ok(last.statusCode === 429, 'per-IP limit → 429');
for (let i = 0; i < 8; i++) last = await act({ email: 'same@example.com' }, { 'x-forwarded-for': `198.51.100.${i}` });
ok(last.statusCode === 429, 'per-email limit → 429');

// Subscribe
const sub = (body, headers = {}) => run(subscribe, { method: 'POST', body: { consent: true, consentText: WEB_TEXT, place: 'faq', ...body }, headers });
r = await sub({ email: 'n1@example.com', consentText: 'I want emails about other Super apps' });
ok(r.statusCode === 400 && inserts().length === 0, 'subscribe: invented wording → 400');
r = await sub({ email: 'n1@example.com' });
ok(r.statusCode === 200 && inserts().length === 1, 'subscribe: known wording stored');
r = await sub({ email: 'n1@example.com' });
ok(r.statusCode === 200 && inserts().length === 0, 'subscribe: already subscribed → no second row');
rows.push({ id: 'recG', fields: { Email: 'gone@example.com', Source: 'web', 'Marketing Opt-in': false, 'Marketing Withdrawn At': '2026-11-01T00:00:00Z' } });
r = await sub({ email: 'gone@example.com' });
ok(r.statusCode === 200 && inserts().length === 0 && rows.find((x) => x.id === 'recG').fields['Marketing Opt-in'] === false, 'subscribe: withdrawn address is not re-subscribed');
for (let i = 0; i < 40; i++) last = await sub({ email: `s${i}@example.com` }, { 'x-forwarded-for': '203.0.113.77' });
ok(last.statusCode === 429, 'subscribe: per-IP limit → 429');

// Unsubscribe follows pagination
pageCap = 2;
for (let i = 0; i < 5; i++) rows.push({ id: `recU${i}`, fields: { Email: 'multi@example.com', 'Marketing Opt-in': true } });
const t = unsubToken('multi@example.com');
r = await run(unsubscribe, { method: 'POST', url: `/api/unsubscribe?e=multi%40example.com&t=${t}` });
ok(r.statusCode === 200, 'unsubscribe 200');
ok(rows.filter((x) => x.fields.Email === 'multi@example.com').every((x) => x.fields['Marketing Opt-in'] === false), 'unsubscribe patches every page of rows');

console.log(`${pass} passed, ${fail} failed`);
process.exit(fail ? 1 : 0);
