// Offline test of the Super Guides P.S. gate (web/api/activate.js) and the
// one-click unsubscribe (web/api/unsubscribe.js). No network: fetch is mocked.
// Run: node scripts/test-unsubscribe.mjs
import activate, { welcomeEmail, OTHER_APPS_RE } from '../web/api/activate.js';
import unsubscribe from '../web/api/unsubscribe.js';
import { unsubToken, unsubUrl, validToken } from '../web/api/_unsub.js';

let pass = 0;
let fail = 0;
const ok = (cond, name) => {
  if (cond) pass++;
  else {
    fail++;
    console.error('FAIL', name);
  }
};

let calls = [];
let airtableRecords = [{ id: 'rec1' }, { id: 'rec2' }];
globalThis.fetch = async (url, opts = {}) => {
  calls.push({ url: String(url), method: opts.method || 'GET', body: opts.body ? JSON.parse(opts.body) : null });
  if (String(url).includes('airtable')) return { ok: true, status: 200, json: async () => ({ records: airtableRecords }) };
  if (String(url).includes('resend')) return { ok: true, status: 200, json: async () => ({ id: 'em_1' }) };
  return { ok: false, status: 404 };
};
const res = () => ({ statusCode: 0, headers: {}, body: '', setHeader(k, v) { this.headers[k] = v; }, end(b) { this.body = b || ''; } });

process.env.AIRTABLE_TOKEN = 't';
process.env.AIRTABLE_BASE_ID = 'appX';
process.env.AIRTABLE_SD_USERS_TABLE = 'tblX';
process.env.SD_WELCOME_EMAIL = 'on';
process.env.RESEND_API_KEY = 're_test';
const APP_TEXT = 'Email me about updates and other Super apps'; // src/main.js MARKETING_CONSENT_TEXT
const base = { email: 'Ed@Example.com', appVersion: '1.5.6', arch: 'aarch64', osVersion: '15.1', instance: 'Mac', source: 'app' };
const sent = () => calls.find((c) => c.url.includes('resend'))?.body;
async function activateWith(extra) {
  calls = [];
  const r = res();
  await activate({ method: 'POST', body: { ...base, ...extra } }, r);
  return sent();
}

// Tokens
delete process.env.SD_UNSUB_SECRET;
ok(unsubToken('a@b.co') === null && unsubUrl('a@b.co') === null, 'no secret → no link');
process.env.SD_UNSUB_SECRET = 'test-secret';
const t = unsubToken('ed@example.com');
ok(/^[0-9a-f]{32}$/.test(t), 'token is 32 hex');
ok(unsubToken('  ED@example.com ') === t, 'token ignores case and spaces');
ok(validToken('ed@example.com', t) && !validToken('other@example.com', t) && !validToken('ed@example.com', 'x'), 'token checks');
ok(OTHER_APPS_RE.test(APP_TEXT), 'the app box wording counts as consent to other Super apps');

// P.S. gate
let m = await activateWith({ marketingOptIn: true, marketingConsentText: APP_TEXT });
ok(m && m.text.includes('Super Guides') && m.html.includes('Super Guides'), 'consent → P.S.');
ok(m.text.includes('/api/unsubscribe?e=ed%40example.com&t=' + t) && m.html.includes('Unsubscribe'), 'P.S. email has the unsubscribe link');
ok(m.headers?.['List-Unsubscribe']?.includes('/api/unsubscribe') && m.headers['List-Unsubscribe-Post'] === 'List-Unsubscribe=One-Click', 'one-click headers');
m = await activateWith({ marketingOptIn: false, marketingConsentText: '' });
ok(m && !m.text.includes('Super Guides') && !m.headers, 'no consent → no P.S., no list headers');
m = await activateWith({ marketingOptIn: true });
ok(m && !m.text.includes('Super Guides'), 'old pre-ticked opt-in (no text) → no P.S.');
m = await activateWith({ marketingOptIn: true, marketingConsentText: 'Send me updates' });
ok(m && !m.text.includes('Super Guides'), 'consent text without other apps → no P.S.');
delete process.env.SD_UNSUB_SECRET;
m = await activateWith({ marketingOptIn: true, marketingConsentText: APP_TEXT });
ok(m && !m.text.includes('Super Guides'), 'no unsubscribe secret → no P.S.');
process.env.SD_UNSUB_SECRET = 'test-secret';
ok(!welcomeEmail().text.includes('P.S.'), 'default welcome email has no P.S.');

// Unsubscribe endpoint
const link = (e, tok) => `/api/unsubscribe?e=${encodeURIComponent(e)}&t=${tok}`;
calls = [];
let r = res();
await unsubscribe({ method: 'GET', url: link('ed@example.com', t) }, r);
ok(r.statusCode === 200 && r.body.includes('<form method="post"') && calls.length === 0, 'GET shows a button and changes nothing');
r = res();
await unsubscribe({ method: 'GET', url: link('ed@example.com', 'bad') }, r);
ok(r.statusCode === 400, 'bad token 400');
calls = [];
r = res();
await unsubscribe({ method: 'POST', url: link('ed@example.com', t) }, r);
const patch = calls.find((c) => c.method === 'PATCH');
ok(r.statusCode === 200 && r.body.includes('unsubscribed'), 'POST unsubscribes');
ok(decodeURIComponent(calls[0].url).includes("LOWER(TRIM({Email}))='ed@example.com'"), 'finds every record with that email');
ok(patch?.body.records.length === 2 && patch.body.records.every((x) => x.fields['Marketing Opt-in'] === false && x.fields['Marketing Withdrawn At']), 'all records opted out, with date');
airtableRecords = Array.from({ length: 12 }, (_, i) => ({ id: `rec${i}` }));
calls = [];
await unsubscribe({ method: 'POST', url: link('ed@example.com', t) }, res());
ok(calls.filter((c) => c.method === 'PATCH').length === 2, 'patches in batches of 10');
r = res();
await unsubscribe({ method: 'DELETE', url: link('ed@example.com', t) }, r);
ok(r.statusCode === 405, 'other methods 405');

console.log(`${pass} passed, ${fail} failed`);
process.exit(fail ? 1 : 0);
