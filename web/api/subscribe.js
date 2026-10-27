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
  const consentText = typeof body.consentText === 'string' ? body.consentText.trim().slice(0, 300) : '';
  if (body.consent !== true || !consentText) {
    return send(res, 400, { ok: false, error: 'consent_required' });
  }

  const token = process.env.AIRTABLE_TOKEN;
  const base = process.env.AIRTABLE_BASE_ID;
  const table = process.env.AIRTABLE_SD_USERS_TABLE;
  if (!token || !base || !table) return send(res, 503, { ok: false, error: 'not_configured' });

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

  try {
    const r = await fetch(`https://api.airtable.com/v0/${encodeURIComponent(base)}/${encodeURIComponent(table)}`, {
      method: 'POST',
      headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' },
      body: JSON.stringify({ records: [{ fields }], typecast: true }),
      signal: AbortSignal.timeout(8000),
    });
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
