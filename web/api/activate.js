// POST /api/activate — email activation intake for the Super Downloads app.
// Vercel Node serverless function (ESM, `export default (req, res)`), no deps.
// Secrets and Airtable IDs come from env only: AIRTABLE_TOKEN,
// AIRTABLE_BASE_ID, AIRTABLE_SD_USERS_TABLE.

const EMAIL_RE = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
const ARCHS = ['aarch64', 'x86_64', 'unknown'];

const clamp = (v, n = 100) => (typeof v === 'string' ? v.trim().slice(0, n) : '');

function send(res, status, body) {
  res.statusCode = status;
  res.setHeader('Content-Type', 'application/json');
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

  const email = typeof body.email === 'string' ? body.email.trim() : '';
  if (!email || email.length > 254 || !EMAIL_RE.test(email)) {
    return send(res, 400, { ok: false, error: 'invalid_email' });
  }

  const token = process.env.AIRTABLE_TOKEN;
  const base = process.env.AIRTABLE_BASE_ID;
  const table = process.env.AIRTABLE_SD_USERS_TABLE;
  if (!token || !base || !table) {
    return send(res, 503, { ok: false, error: 'not_configured' });
  }

  const arch = ARCHS.includes(body.arch) ? body.arch : 'unknown';
  const fields = {
    Email: email,
    'Activated At': new Date().toISOString(),
    'App Version': clamp(body.appVersion),
    Arch: arch,
    'OS Version': clamp(body.osVersion),
    Instance: clamp(body.instance),
    Source: body.source === 'landing' ? 'landing' : 'app',
    'Marketing Opt-in': body.marketingOptIn === true,
  };

  try {
    const r = await fetch(
      `https://api.airtable.com/v0/${encodeURIComponent(base)}/${encodeURIComponent(table)}`,
      {
        method: 'POST',
        headers: {
          Authorization: `Bearer ${token}`,
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({ records: [{ fields }], typecast: true }),
      }
    );
    if (!r.ok) {
      console.error('[activate] Airtable status', r.status);
      return send(res, 502, { ok: false, error: 'upstream' });
    }
    return send(res, 200, { ok: true });
  } catch {
    console.error('[activate] Airtable request failed');
    return send(res, 502, { ok: false, error: 'upstream' });
  }
}
