// /api/unsubscribe — withdraw marketing consent (emails about updates and other
// Super apps) in one click. Linked from every email that carries marketing
// content, plus the List-Unsubscribe header (RFC 8058).
//   GET  ?e=&t=  → a page with one button (link scanners must not unsubscribe)
//   POST ?e=&t=  → sets «Marketing Opt-in» false and «Marketing Withdrawn At» on
//                  every SD · Users record with that email (the mail client's
//                  one-click POST lands here too)
// Env: AIRTABLE_TOKEN, AIRTABLE_BASE_ID, AIRTABLE_SD_USERS_TABLE, SD_UNSUB_SECRET.
import { validToken } from './_unsub.js';

const esc = (s) => String(s).replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);

function page(res, status, title, body) {
  res.statusCode = status;
  res.setHeader('Content-Type', 'text/html; charset=utf-8');
  res.setHeader('Cache-Control', 'no-store');
  res.setHeader('X-Robots-Tag', 'noindex');
  res.end(`<!doctype html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>${esc(title)} · Super Downloads</title>
<style>body{margin:0;background:#f4f7fc;color:#0f172a;font:16px/1.6 -apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif}main{max-width:480px;margin:12vh auto;padding:0 16px}h1{font-size:22px;letter-spacing:-.02em}p{color:#475569}button{font:inherit;font-weight:600;padding:10px 18px;border:0;border-radius:8px;background:#0b6bcb;color:#fff;cursor:pointer}a{color:#0b6bcb}</style></head>
<body><main><h1>${esc(title)}</h1>${body}<p><a href="/">superdownloads.app</a></p></main></body></html>`);
}

function query(req) {
  const u = new URL(req.url || '/', 'https://superdownloads.app');
  return { email: (u.searchParams.get('e') || '').trim().toLowerCase(), token: u.searchParams.get('t') || '' };
}

export default async function handler(req, res) {
  const { email, token } = query(req);
  if (req.method !== 'GET' && req.method !== 'POST') {
    res.setHeader('Allow', 'GET, POST');
    return page(res, 405, 'Not allowed', '');
  }
  if (!email || !validToken(email, token)) {
    return page(res, 400, 'This link doesn’t work', '<p>It may be incomplete. Reply to any of our emails with «unsubscribe» and we’ll take you off the list.</p>');
  }

  if (req.method === 'GET') {
    const action = `/api/unsubscribe?e=${encodeURIComponent(email)}&t=${encodeURIComponent(token)}`;
    return page(res, 200, 'Unsubscribe',
      `<p>Stop emails about updates and other Super apps to <strong>${esc(email)}</strong>? The app keeps working, and we’ll still write if something affects how you use it (a security fix, for example).</p>
<form method="post" action="${esc(action)}"><button type="submit">Unsubscribe</button></form>`);
  }

  const tokenEnv = process.env.AIRTABLE_TOKEN;
  const base = process.env.AIRTABLE_BASE_ID;
  const table = process.env.AIRTABLE_SD_USERS_TABLE;
  if (!tokenEnv || !base || !table) return page(res, 503, 'Try again later', '<p>We couldn’t reach the list. Reply to any of our emails with «unsubscribe».</p>');

  const url = `https://api.airtable.com/v0/${encodeURIComponent(base)}/${encodeURIComponent(table)}`;
  const headers = { Authorization: `Bearer ${tokenEnv}`, 'Content-Type': 'application/json' };
  try {
    const formula = `LOWER(TRIM({Email}))='${email.replace(/'/g, "\\'")}'`;
    const found = await fetch(`${url}?filterByFormula=${encodeURIComponent(formula)}&fields%5B%5D=Email&pageSize=100`, { headers });
    if (!found.ok) throw new Error(`find ${found.status}`);
    const ids = ((await found.json()).records || []).map((r) => r.id);
    const now = new Date().toISOString();
    for (let i = 0; i < ids.length; i += 10) {
      const r = await fetch(url, {
        method: 'PATCH',
        headers,
        body: JSON.stringify({ records: ids.slice(i, i + 10).map((id) => ({ id, fields: { 'Marketing Opt-in': false, 'Marketing Withdrawn At': now } })) }),
      });
      if (!r.ok) throw new Error(`patch ${r.status}`);
    }
  } catch (e) {
    console.error('[unsubscribe]', e.message);
    return page(res, 502, 'Try again later', '<p>We couldn’t reach the list. Reply to any of our emails with «unsubscribe».</p>');
  }
  return page(res, 200, 'You’re unsubscribed', '<p>No more emails about updates or other Super apps. The app keeps working as before.</p>');
}
