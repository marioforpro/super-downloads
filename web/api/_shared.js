// Shared by activate.js, subscribe.js and unsubscribe.js (files starting with _
// are not routes): server-side consent wording, Airtable helpers with a timeout
// and pagination, and a small rate limiter.

// Exact wording of each consent box, keyed by version. Consent is decided from
// this copy, never from text the client sends: the client's text only counts if
// it equals one of these, and the stored wording is always this copy.
// Keep in sync with src/main.js (MARKETING_CONSENT_TEXT) and
// web/src/components/SubscribeForm.astro (CONSENT); add a new version, don't
// edit an old one, when a box changes.
export const CONSENT_TEXTS = {
  'app-1': 'Email me about updates and other Super apps',
  'web-1': 'Email me about new versions and news from Super Downloads. A few emails a year; every one has a one-click unsubscribe.',
};

/** The server copy of the wording the client says it showed, or '' if unknown. */
export function consentWording(text, channel) {
  const t = typeof text === 'string' ? text.trim() : '';
  if (!t) return '';
  for (const [version, wording] of Object.entries(CONSENT_TEXTS)) {
    if (version.startsWith(`${channel}-`) && wording === t) return wording;
  }
  return '';
}

export const AIRTABLE_TIMEOUT_MS = 8000;

export function airtableConfig() {
  const token = process.env.AIRTABLE_TOKEN;
  const base = process.env.AIRTABLE_BASE_ID;
  const table = process.env.AIRTABLE_SD_USERS_TABLE;
  if (!token || !base || !table) return null;
  return {
    url: `https://api.airtable.com/v0/${encodeURIComponent(base)}/${encodeURIComponent(table)}`,
    headers: { Authorization: `Bearer ${token}`, 'Content-Type': 'application/json' },
  };
}

/** fetch against Airtable with a timeout (a slow upstream must not hang the function). */
export function airtableFetch(cfg, { method = 'GET', query = '', payload } = {}) {
  return fetch(cfg.url + query, {
    method,
    headers: cfg.headers,
    ...(payload ? { body: JSON.stringify(payload) } : {}),
    signal: AbortSignal.timeout(AIRTABLE_TIMEOUT_MS),
  });
}

/** Every record whose Email matches (case-insensitive), following `offset` pages. Throws on upstream errors. */
export async function findByEmail(cfg, email, fields = ['Email']) {
  const norm = String(email).trim().toLowerCase().replace(/\\/g, '\\\\').replace(/'/g, "\\'");
  const formula = `LOWER(TRIM({Email}))='${norm}'`;
  const fieldQs = fields.map((f) => `&fields%5B%5D=${encodeURIComponent(f)}`).join('');
  const out = [];
  let offset = '';
  for (let page = 0; page < 100; page++) {
    const r = await airtableFetch(cfg, {
      query: `?filterByFormula=${encodeURIComponent(formula)}${fieldQs}&pageSize=100${offset ? `&offset=${encodeURIComponent(offset)}` : ''}`,
    });
    if (!r.ok) throw new Error(`find ${r.status}`);
    const j = await r.json();
    out.push(...(j.records || []));
    if (!j.offset) return out;
    offset = j.offset;
  }
  return out;
}

// Best-effort limiter: counters live in the memory of one serverless instance,
// so it slows bursts but is not a global guarantee. There is no IP column in
// the Airtable table (and the schema is not changed), so no cross-instance
// per-IP count is possible; per-address abuse is also bounded by the
// look-up-before-insert checks in the handlers.
const hits = new Map();
export const LIMITS = { ip: 30, email: 5, windowMs: 60 * 60 * 1000 };

function hit(key, max, now) {
  const fresh = (hits.get(key) || []).filter((t) => now - t < LIMITS.windowMs);
  if (fresh.length >= max) {
    hits.set(key, fresh);
    return false;
  }
  fresh.push(now);
  hits.set(key, fresh);
  return true;
}

export function clientIp(req) {
  const xff = req.headers?.['x-forwarded-for'];
  const v = Array.isArray(xff) ? xff[0] : xff;
  return typeof v === 'string' ? v.split(',')[0].trim() : '';
}

/** True when this request is over the per-IP or per-address limit (and counts it otherwise). */
export function rateLimited(req, email, scope = 'a') {
  const now = Date.now();
  if (hits.size > 5000) {
    for (const [k, v] of hits) if (!v.some((t) => now - t < LIMITS.windowMs)) hits.delete(k);
  }
  const ip = clientIp(req);
  if (ip && !hit(`${scope}:ip:${ip}`, LIMITS.ip, now)) return true;
  return !hit(`${scope}:em:${String(email).trim().toLowerCase()}`, LIMITS.email, now);
}

/** Test helper: forget all rate-limit counters. */
export function resetRateLimits() {
  hits.clear();
}
