// Tell IndexNow (Bing, and through it ChatGPT/Copilot search; Yandex, Seznam, Naver) about every URL in the LIVE
// sitemap. Runs after each production deploy (.github/workflows/indexnow.yml) or by hand: node web/scripts/indexnow.mjs [--dry].
// The key is public by design: it must be served at https://superdownloads.app/<key>.txt (web/public/).
const KEY = '2f66e840d002f1a6421e99f8456ed04d';
const ORIGIN = 'https://superdownloads.app';
const dry = process.argv.includes('--dry');

const keyFile = await fetch(`${ORIGIN}/${KEY}.txt`).then((r) => (r.ok ? r.text() : ''));
if (keyFile.trim() !== KEY) { console.error(`key file not live at ${ORIGIN}/${KEY}.txt — deploy first`); process.exit(1); }
const sitemap = await fetch(`${ORIGIN}/sitemap.xml`).then((r) => r.text());
const urlList = [...sitemap.matchAll(/<loc>([^<]+)<\/loc>/g)].map((m) => m[1]);
if (!urlList.length) { console.error('no URLs in the live sitemap'); process.exit(1); }
const body = { host: new URL(ORIGIN).host, key: KEY, keyLocation: `${ORIGIN}/${KEY}.txt`, urlList };
if (dry) { console.log(JSON.stringify(body, null, 2)); process.exit(0); }
const r = await fetch('https://api.indexnow.org/indexnow', { method: 'POST', headers: { 'Content-Type': 'application/json; charset=utf-8' }, body: JSON.stringify(body) });
console.log(`indexnow ${r.status} · ${urlList.length} urls`);
process.exit(r.status === 200 || r.status === 202 ? 0 : 1);
