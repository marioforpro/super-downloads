// Live checks of superdownloads.app against production (read-only: nothing is written,
// /api/activate only gets requests it rejects). Run: node scripts/test-live-site.mjs
const SITE = process.env.SD_SITE || 'https://superdownloads.app';
const REPO = 'marioforpro/super-downloads';
let pass = 0, fail = 0;
const ok = (c, m) => { console.log((c ? 'PASS ' : 'FAIL ') + m); c ? pass++ : fail++; };
const get = (u, o) => fetch(u, { redirect: 'follow', ...o });
const meta = (html, prop) => html.match(new RegExp(`<meta (?:property|name)="${prop}" content="([^"]*)"`))?.[1];

// Pages
const pages = {};
for (const p of ['/', '/changelog', '/privacy', '/terms', '/copyright', '/press']) {
  const r = await get(SITE + p);
  pages[p] = await r.text();
  ok(r.status === 200, `${p} 200`);
}
const home = pages['/'];

// Security headers
const h = (await get(SITE + '/')).headers;
for (const k of ['content-security-policy', 'x-content-type-options', 'x-frame-options', 'referrer-policy', 'strict-transport-security'])
  ok(!!h.get(k), `header ${k}`);
ok(/us\.i\.posthog\.com/.test(h.get('content-security-policy') || ''), 'CSP allows PostHog ingestion');

// OG / share card
const og = meta(home, 'og:image');
ok(!!og && meta(home, 'og:title') && meta(home, 'og:description') && meta(home, 'twitter:card') === 'summary_large_image', 'OG + Twitter tags present');
const ogr = await get(og);
const ogLen = Number(ogr.headers.get('content-length') || (await ogr.arrayBuffer()).byteLength);
ok(ogr.status === 200 && /image\/(jpeg|png)/.test(ogr.headers.get('content-type')) && ogLen < 300000, `share card 200, image, ${Math.round(ogLen / 1024)} KB < 300 KB`);
ok(/noindex/.test(pages['/press']) && !/noindex/.test(home), 'press is noindex, home is not');

// Release: landing version = GitHub latest; DMGs + updater reachable; hashes match
const rel = await (await get(`https://api.github.com/repos/${REPO}/releases/latest`, { headers: { 'User-Agent': 'sd-live-test' } })).json();
const version = String(rel.tag_name).replace(/^v/, '');
ok(home.includes(`v${version}`) && home.includes(`"softwareVersion":"${version}"`), `landing shows v${version} (text + JSON-LD)`);
for (const a of ['Super-Downloads_aarch64.dmg', 'Super-Downloads_x64.dmg', 'latest.json']) {
  const r = await get(`https://github.com/${REPO}/releases/latest/download/${a}`, { method: 'HEAD' });
  ok(r.status === 200, `${a} downloadable`);
  const d = rel.assets.find((x) => x.name === a)?.digest?.replace('sha256:', '');
  if (a.endsWith('.dmg')) ok(!!d && home.includes(d), `${a} SHA-256 on the page matches GitHub`);
}
const latest = await (await get(`https://github.com/${REPO}/releases/latest/download/latest.json`)).json();
ok(latest.version === version, `updater latest.json = ${latest.version}`);

// Activation endpoint (rejections only)
const A = SITE + '/api/activate';
ok((await get(A, { method: 'OPTIONS' })).status === 204, 'activate OPTIONS 204');
ok((await get(A)).status === 405, 'activate GET 405');
ok((await get(A, { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: '{"email":"nope"}' })).status === 400, 'activate bad email 400');

// Copy guards
const text = Object.values(pages).join(' ');
ok(!/\bbeta\b/i.test(text.replace(/<[^>]+>/g, ' ')), 'no «beta» on public pages');
ok(!/founder/i.test(text.replace(/<[^>]+>/g, ' ')), 'no «founder» on public pages');
ok((home.match(/utm_campaign=family/g) || []).length === 3, 'family footer links 3 siblings');
ok(/id="known-issues"/.test(pages['/changelog']), 'changelog has known issues');

console.log(`${pass} passed, ${fail} failed`);
process.exit(fail ? 1 : 0);
