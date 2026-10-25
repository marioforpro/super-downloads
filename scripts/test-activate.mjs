// Offline test of web/api/activate.js: consent rules and the welcome email flag.
// Run: node scripts/test-activate.mjs  (no network: fetch is mocked)
import handler, { welcomeEmail } from '../web/api/activate.js';

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
globalThis.fetch = async (url, opts = {}) => {
  calls.push({ url, method: opts.method, body: opts.body ? JSON.parse(opts.body) : null });
  if (url.includes('airtable')) return { ok: true, status: 200, json: async () => ({ records: [{ id: 'recTEST' }] }) };
  if (url.includes('resend')) return { ok: true, status: 200, json: async () => ({ id: 'em_1' }) };
  return { ok: false, status: 404 };
};

async function call(body, method = 'POST') {
  calls = [];
  const res = { statusCode: 0, headers: {}, body: '', setHeader(k, v) { this.headers[k] = v; }, end(b) { this.body = b || ''; } };
  await handler({ method, body }, res);
  return res;
}
const created = () => calls.find((c) => c.url.includes('airtable') && c.method === 'POST')?.body.records[0].fields;

process.env.AIRTABLE_TOKEN = 't';
process.env.AIRTABLE_BASE_ID = 'appX';
process.env.AIRTABLE_SD_USERS_TABLE = 'tblX';
const base = { email: 'a@b.co', appVersion: '1.5.6', arch: 'aarch64', osVersion: '15.1', instance: 'Mac', source: 'app' };
const TEXT = 'Email me about app updates and other Super tools (optional)';

// Methods and validation
ok((await call(null, 'OPTIONS')).statusCode === 204, 'OPTIONS 204');
ok((await call(null, 'GET')).statusCode === 405, 'GET 405');
ok((await call({ email: 'nope' })).statusCode === 400, 'bad email 400');

// Old builds (pre-ticked box, no text) → never consent
let r = await call({ ...base, appVersion: '1.5.5', marketingOptIn: true });
ok(r.statusCode === 200, 'old build 200');
ok(created()['Marketing Opt-in'] === false, 'old build opt-in stored false');
ok(!('Marketing Consent At' in created()), 'old build no consent date');

// New build, box ticked → consent with date + text
r = await call({ ...base, marketingOptIn: true, marketingConsentText: TEXT });
ok(created()['Marketing Opt-in'] === true, 'ticked → opt-in true');
ok(created()['Marketing Consent Text'] === TEXT, 'consent text stored');
ok(!Number.isNaN(Date.parse(created()['Marketing Consent At'])), 'consent date stored');

// New build, box left unticked → no consent
r = await call({ ...base, marketingOptIn: false, marketingConsentText: TEXT });
ok(created()['Marketing Opt-in'] === false, 'unticked → false');

// Welcome email: off by default
delete process.env.SD_WELCOME_EMAIL;
process.env.RESEND_API_KEY = 're_test';
await call(base);
ok(!calls.some((c) => c.url.includes('resend')), 'flag off → no email');

// Flag on → one email, then Welcome Sent At patched
process.env.SD_WELCOME_EMAIL = 'on';
r = await call(base);
const mail = calls.find((c) => c.url.includes('resend'));
ok(r.statusCode === 200 && mail && mail.body.to[0] === 'a@b.co', 'flag on → email sent');
ok(mail && mail.body.reply_to === 'support@superdownloads.app', 'reply-to support');
ok(calls.some((c) => c.method === 'PATCH' && c.body.records[0].fields['Welcome Sent At']), 'Welcome Sent At patched');

// Landing source or missing version → no email
await call({ ...base, source: 'landing' });
ok(!calls.some((c) => c.url.includes('resend')), 'landing → no email');

// Resend down → activation still 200
globalThis.fetch = async (url) => (url.includes('resend') ? Promise.reject(new Error('down')) : { ok: true, status: 200, json: async () => ({ records: [{ id: 'r' }] }) });
r = await call(base);
ok(r.statusCode === 200, 'resend failure never fails activation');

// Copy rules: brand voice, no founder, no exclamation marks
const w = welcomeEmail();
ok(w.text.includes('The Super Downloads team') && !/!/.test(w.text), 'welcome copy: signed by the team, no exclamations');

console.log(`${pass} passed, ${fail} failed`);
process.exit(fail ? 1 : 0);
