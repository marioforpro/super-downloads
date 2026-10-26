// Shared by activate.js and unsubscribe.js (files starting with _ are not routes).
// Unsubscribe links carry the email and an HMAC of it, so nobody can unsubscribe
// someone else by guessing. Secret: SD_UNSUB_SECRET (Vercel env). Without it no
// link can be made, and emails that would need one (the Super Guides P.S.) are
// not sent with it.
import { createHmac, timingSafeEqual } from 'node:crypto';

export const SITE = 'https://superdownloads.app';

const norm = (email) => String(email || '').trim().toLowerCase();

export function unsubToken(email) {
  const secret = process.env.SD_UNSUB_SECRET;
  if (!secret || !norm(email)) return null;
  return createHmac('sha256', secret).update(norm(email)).digest('hex').slice(0, 32);
}

export function unsubUrl(email) {
  const t = unsubToken(email);
  return t ? `${SITE}/api/unsubscribe?e=${encodeURIComponent(norm(email))}&t=${t}` : null;
}

export function validToken(email, token) {
  const want = unsubToken(email);
  if (!want || typeof token !== 'string' || token.length !== want.length) return false;
  return timingSafeEqual(Buffer.from(want), Buffer.from(token));
}
