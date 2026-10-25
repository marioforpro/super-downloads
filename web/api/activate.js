// POST /api/activate — email activation intake for the Super Downloads app.
// Vercel Node serverless function (ESM, `export default (req, res)`), no deps.
// Secrets and Airtable IDs come from env only: AIRTABLE_TOKEN,
// AIRTABLE_BASE_ID, AIRTABLE_SD_USERS_TABLE.
//
// Marketing consent: only counts when the app sends the exact wording of the
// box the person ticked (`marketingConsentText`). App 1.5.6+ shows that box
// unticked and sends the text; 1.5.5 and earlier showed it pre-ticked, which
// is not valid consent (GDPR), so those opt-ins are stored as false.
//
// Welcome email (transactional, via Resend): off until SD_WELCOME_EMAIL=on and
// RESEND_API_KEY are set and superdownloads.app is verified in Resend.
// Optional SD_WELCOME_FROM (default "Super Downloads <support@superdownloads.app>").

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

export function welcomeEmail() {
  const subject = 'Super Downloads is ready';
  const text = [
    'Super Downloads is activated. Three things worth knowing:',
    '',
    '1. Copy a video link in your browser, switch to the app and press Return. The link is already there.',
    '2. Press Cmd+B for Batch: every video link you copy downloads on its own, even with the app in the background.',
    '3. Every file is an H.264 MP4 that opens in Premiere Pro, DaVinci Resolve and Final Cut.',
    '',
    'If a site stops working, the app updates its download engine every week on its own. To check right away: Settings, then Update next to Engine.',
    '',
    'Questions or a link that fails? Reply to this email.',
    '',
    'The Super Downloads team',
    'https://superdownloads.app',
    '',
    'You get this one email because you activated the app with this address.',
  ].join('\n');
  const html = `<!doctype html><html><body style="margin:0;padding:0;background:#f4f7fc">
<div style="max-width:520px;margin:0 auto;padding:32px 24px;font:15px/1.6 -apple-system,BlinkMacSystemFont,'Segoe UI',sans-serif;color:#0f172a">
<p style="margin:0 0 20px"><img src="https://superdownloads.app/og/sd-email-icon.png" width="40" height="40" alt="" style="border-radius:9px;display:block"></p>
<h1 style="margin:0 0 16px;font-size:22px;line-height:1.25;letter-spacing:-0.02em">Super Downloads is ready.</h1>
<p style="margin:0 0 16px;color:#475569">Three things worth knowing:</p>
<ol style="margin:0 0 20px;padding-left:20px;color:#475569">
<li style="margin-bottom:8px"><strong style="color:#0f172a">Copy a link, switch, press Return.</strong> The link you copied is already in the field.</li>
<li style="margin-bottom:8px"><strong style="color:#0f172a">⌘B turns on Batch.</strong> Every video link you copy downloads on its own, even with the app in the background.</li>
<li><strong style="color:#0f172a">Every file is an H.264 MP4</strong> that opens in Premiere Pro, DaVinci Resolve and Final Cut.</li>
</ol>
<p style="margin:0 0 16px;color:#475569">If a site stops working, the app updates its download engine every week on its own. To check right away: Settings › <strong style="color:#0f172a">Update</strong> next to Engine.</p>
<p style="margin:0 0 24px;color:#475569">Questions, or a link that fails? Reply to this email.</p>
<p style="margin:0;color:#0f172a">The Super Downloads team<br><a href="https://superdownloads.app/?utm_source=welcome&amp;utm_medium=email" style="color:#0b6bcb">superdownloads.app</a></p>
<p style="margin:28px 0 0;font-size:12px;color:#5b6b80">You get this one email because you activated Super Downloads with this address.</p>
</div></body></html>`;
  return { subject, text, html };
}

async function sendWelcome(email) {
  const key = process.env.RESEND_API_KEY;
  if (process.env.SD_WELCOME_EMAIL !== 'on' || !key) return false;
  const { subject, text, html } = welcomeEmail();
  try {
    const r = await fetch('https://api.resend.com/emails', {
      method: 'POST',
      headers: { Authorization: `Bearer ${key}`, 'Content-Type': 'application/json' },
      body: JSON.stringify({
        from: process.env.SD_WELCOME_FROM || 'Super Downloads <support@superdownloads.app>',
        to: [email],
        reply_to: 'support@superdownloads.app',
        subject,
        text,
        html,
      }),
      signal: AbortSignal.timeout(5000),
    });
    if (!r.ok) console.error('[activate] Resend status', r.status);
    return r.ok;
  } catch {
    console.error('[activate] Resend request failed');
    return false;
  }
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

  const now = new Date().toISOString();
  const arch = ARCHS.includes(body.arch) ? body.arch : 'unknown';
  const consentText = clamp(body.marketingConsentText, 300);
  const consent = body.marketingOptIn === true && consentText !== '';
  const fields = {
    Email: email,
    'Activated At': now,
    'App Version': clamp(body.appVersion),
    Arch: arch,
    'OS Version': clamp(body.osVersion),
    Instance: clamp(body.instance),
    Source: body.source === 'landing' ? 'landing' : 'app',
    'Marketing Opt-in': consent,
  };
  if (consent) {
    fields['Marketing Consent At'] = now;
    fields['Marketing Consent Text'] = consentText;
  }

  const airtable = (method, path, payload) =>
    fetch(`https://api.airtable.com/v0/${encodeURIComponent(base)}/${encodeURIComponent(table)}${path}`, {
      method,
      headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' },
      body: JSON.stringify(payload),
    });

  let recordId = null;
  try {
    const r = await airtable('POST', '', { records: [{ fields }], typecast: true });
    if (!r.ok) {
      console.error('[activate] Airtable status', r.status);
      return send(res, 502, { ok: false, error: 'upstream' });
    }
    recordId = (await r.json().catch(() => null))?.records?.[0]?.id ?? null;
  } catch {
    console.error('[activate] Airtable request failed');
    return send(res, 502, { ok: false, error: 'upstream' });
  }

  // The welcome email never blocks or fails an activation. Only real app
  // activations get it (the landing never activates), once per sync.
  if (fields.Source === 'app' && fields['App Version'] && (await sendWelcome(email)) && recordId) {
    try {
      await airtable('PATCH', '', { records: [{ id: recordId, fields: { 'Welcome Sent At': new Date().toISOString() } }] });
    } catch {}
  }
  return send(res, 200, { ok: true });
}
