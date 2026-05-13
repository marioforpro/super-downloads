# SECURITY NOTES — Super Downloads

> Open security observations for the freemium / license layer.
> Findings here are technical-debt items, not active incidents — the app is pre-launch and not yet commercially distributed.
> Source of truth for "should we harden X?" decisions before public launch.

---

## 2026-05-13 — License gate is trivially bypassable (Session 201)

### Finding

Founder needed to use the app past the 5/day free cap without a real LemonSqueezy license (Lemon checkout still pending). Bypassed the freemium gate in ~60s, no code change, no rebuild.

### Attack vector

The Pro check in [`src/main.js:105-108`](../src/main.js) is purely local:

```js
function isProUser() {
  const key = localStorage.getItem(LICENSE_KEY_STORAGE);
  return key && key.trim().length > 0;
}
```

There is **no re-validation on app launch**. The Lemon Squeezy API (`activate_license`, `validate_license`, `deactivate_license` in [`src-tauri/src/lib.rs:1981-2095`](../src-tauri/src/lib.rs)) is only hit during the in-app activation button click. Once `proLicenseKey` exists in `localStorage`, the user is Pro forever, regardless of whether the key is real, expired, deactivated, or revoked.

LocalStorage in the WKWebView Tauri stack is a plain SQLite file at:

```
~/Library/WebKit/com.supermac.super-downloads/WebsiteData/Default/<random>/<random>/LocalStorage/localstorage.sqlite3
```

Schema is `ItemTable(key TEXT UNIQUE, value BLOB)` with values stored as UTF-16LE blobs.

The full bypass:

```bash
DB=~/Library/WebKit/com.supermac.super-downloads/.../LocalStorage/localstorage.sqlite3
KEY=$(python3 -c 'print("ANYTHING".encode("utf-16-le").hex())')
sqlite3 "$DB" "INSERT OR REPLACE INTO ItemTable VALUES ('proLicenseKey', x'${KEY}');"
```

That's it. App opens → counter shows "Pro" → unlimited downloads.

### Severity assessment

- **Skill barrier:** Low. Anyone comfortable with the macOS terminal + 1 grep can reproduce. Tauri/WKWebView localStorage paths are well-documented online.
- **Reach:** macOS-only, requires local filesystem access (so no remote exploit — needs the binary installed).
- **Economic blast radius:** Each crack costs €29 of forgone revenue. At pre-launch volume: negligible. At 1K+ paying users: still small relative to engineering cost of hardening (see "Right-sized hardening" below).
- **Reputation risk:** Higher. A YouTube/Reddit "how to crack Super Downloads in 60 seconds" video would be embarrassing for the brand, especially given the audience overlap with technical users (Premiere Pro editors, Mac power users).

### Right-sized hardening (proposals — pick one, do NOT stack everything)

For a €29 one-time consumer app, full DRM is overkill and a tarpit. Three tiers, increasing cost:

**Tier 1 — Periodic revalidation (low effort, kills the trivial inject)**
- On app launch, if `proLicenseKey` exists, call `validate_license` against Lemon. If invalid/revoked → wipe local state + downgrade to Free.
- Cache last successful validation timestamp (signed with bundled HMAC secret); allow offline grace period (e.g. 7 days) so users without internet don't get locked out mid-trip.
- Why this works: SQLite inject of a fake key now fails the next time the app boots online. The bar moves from "1 SQLite write" to "intercept network + forge signed response" — same skill class as cracking any Lemon-protected app, no longer trivial.
- Cost: ~1 day. Risk: connectivity-edge UX (handle grace period carefully).

**Tier 2 — Server-side download metering (medium effort)**
- Move the 5/day counter from `localStorage` to a backend the app pings on each download. Stateless local app, server enforces.
- Why this works: no local state to crack. But requires running a backend + auth + abuse handling. Massive scope creep for the current product.
- Cost: ~1–2 weeks + ongoing ops. **Not recommended** unless the product moves to subscription.

**Tier 3 — Native macOS Keychain + code-signed integrity (high effort)**
- Store license in macOS Keychain (requires user approval prompt, harder to extract).
- Sign + verify the Tauri binary; refuse to run if tampered with.
- Why this works: raises the bar to "skilled reverse engineer with codesign tooling". Still ultimately crackable (every app is), but no longer something a curious user does on a Saturday.
- Cost: ~3–5 days. Risk: Keychain UX, Apple Developer Program friction.

### Recommendation for pre-launch

**Tier 1 only.** It eliminates the trivial-inject path (which is the only one a non-technical pirate will ever find) and costs ~1 day. Tiers 2–3 are diminishing returns at €29/copy and should wait for clear evidence of meaningful piracy at scale.

### What NOT to do

- **Obfuscate the JS bundle.** It's a Tauri webview, source is trivially recoverable. Obfuscation only hurts our own debugging.
- **Rely on hostile UI** (popups, nag screens, "phone home" theatrics). Hurts legit Pro users more than pirates.
- **Build a custom DRM scheme.** Use Lemon's `validate_license` — that's what we're paying them for.

### Tracked in

- `ROADMAP.md` Backlog → "License hardening / anti-crack security review"
- This file is the authoritative threat model. Update when hardening lands or when new attack vectors are found.

---

## Founder license stop-gap (also Session 201)

Three keys were injected directly into the localStorage SQLite file on the founder's Mac, as a 60s workaround until a real Lemon comp license can be issued:

```
proLicenseKey      = FOUNDER-MARIO-001
proLicenseName     = Mario Monteiro
proLicenseInstance = founder-local
```

**Action item:** Once Lemon Squeezy product is live + checkout E2E-verified (R-SD-001), issue a real comp license through the Lemon dashboard for `supermariomonteiro@gmail.com`, activate it through the in-app flow (which will overwrite these keys), and confirm `proLicenseKey` now contains the real Lemon key + `proLicenseInstance` is a real Lemon instance ID.

Tracked in `ROADMAP.md` Backlog → "Founder comp license via LemonSqueezy".

If Tier 1 hardening (periodic revalidation) ships **before** the real comp license is issued, this founder stop-gap will start failing — the fake key won't pass `validate_license`. Order the work so the real comp license lands first, OR add a temporary founder-bypass flag in code that gets removed when the real key is in place.

---

## Related files

- [`src/main.js`](../src/main.js) — `isProUser()`, `canDownload()`, freemium counter, license UI
- [`src-tauri/src/lib.rs`](../src-tauri/src/lib.rs) — Lemon Squeezy API calls (activate/validate/deactivate)
- [`docs/ARCHITECTURE.md`](ARCHITECTURE.md) — overall app architecture
- [`docs/DECISIONS.md`](DECISIONS.md) — historical product decisions including freemium model
