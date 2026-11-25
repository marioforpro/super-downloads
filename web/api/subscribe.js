// POST /api/subscribe — news sign-up from the website, without installing the app.
// Same table as activations (Airtable `SD · Users`), told apart by Source = "web"
// and an empty «Activated At». Env: AIRTABLE_TOKEN, AIRTABLE_BASE_ID,
// AIRTABLE_SD_USERS_TABLE (missing → 503).
//
// The whole point of this form is marketing email, so it only stores anything
// when the person ticked the (unticked) box: the request must carry consent=true
// and the exact wording of that box, which is stored with the time. Every email
// sent to this list carries the one-click unsubscribe (/api/unsubscribe).
// Nothing is emailed from here: no confirmation email yet (would need Resend).

import { airtableConfig, airtableFetch, consentWording, findByEmail, rateLimited } from './_shared.js';

const EMAIL_RE = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const PLACES = ['hero', 'download', 'faq'];

function send(res, status, body) {
  res.statusCode = status;
  res.setHeader('Content-Type', 'application/json');
  res.setHeader('Cache-Control', 'no-store');
  res.end(JSON.stringify(body));
}

function parseBody(req) {
  const b = req.body;
  if (b && typeof b === 'object' && !Buffer.isBuffer(b)) return b;
  if (typeof b === 'string' || Buffer.isBuffer(b)) {
    try {
      return JSON.parse(b.toString());
    } catch {
      return null;
    }
  }
  return null;
}

export default async function handler(req, res) {
  if (req.method === 'OPTIONS') {
    res.statusCode = 204;
    res.setHeader('Allow', 'POST, OPTIONS');
    return res.end();
  }
  if (req.method !== 'POST') {
    res.setHeader('Allow', 'POST, OPTIONS');
    return send(res, 405, { ok: false, error: 'method_not_allowed' });
  }

  const body = parseBody(req);
  if (!body) return send(res, 400, { ok: false, error: 'invalid_json' });

  // Honeypot: a hidden field people never fill. Bots get a quiet success.
  if (typeof body.website === 'string' && body.website.trim() !== '') return send(res, 200, { ok: true });

  const email = typeof body.email === 'string' ? body.email.trim() : '';
  if (!email || email.length > 254 || !EMAIL_RE.test(email)) {
    return send(res, 400, { ok: false, error: 'invalid_email' });
  }
  const consentText = consentWording(body.consentText, 'web');
  if (body.consent !== true || !consentText) {
    return send(res, 400, { ok: false, error: 'consent_required' });
  }

  const cfg = airtableConfig();
  if (!cfg) return send(res, 503, { ok: false, error: 'not_configured' });
  if (rateLimited(req, email, 'subscribe')) {
    res.setHeader('Retry-After', '3600');
    return send(res, 429, { ok: false, error: 'rate_limited' });
  }

  try {
    // Nothing nobody confirmed can undo an unsubscribe or pile up rows: an address
    // that already opted in, or that withdrew, gets a quiet success and no new row.
    const existing = await findByEmail(cfg, email, ['Email', 'Marketing Opt-in', 'Marketing Withdrawn At']);
    if (existing.some((r) => r.fields?.['Marketing Opt-in'] === true || r.fields?.['Marketing Withdrawn At'])) {
      return send(res, 200, { ok: true });
    }

    const now = new Date().toISOString();
    const place = PLACES.includes(body.place) ? body.place : 'web';
    const fields = {
      Email: email,
      Source: 'web',
      Instance: `web:${place}`,
      'Marketing Opt-in': true,
      'Marketing Consent At': now,
      'Marketing Consent Text': consentText,
    };
    const r = await airtableFetch(cfg, { method: 'POST', payload: { records: [{ fields }], typecast: true } });
    if (!r.ok) {
      console.error('[subscribe] Airtable status', r.status);
      return send(res, 502, { ok: false, error: 'upstream' });
    }
  } catch {
    console.error('[subscribe] Airtable request failed');
    return send(res, 502, { ok: false, error: 'upstream' });
  }
  return send(res, 200, { ok: true });
}
