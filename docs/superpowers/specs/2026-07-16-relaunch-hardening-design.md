# Relaunch Hardening — Design (2026-07-16)

> Approved by founder 2026-07-16. Unparks the project (founder recommit).
> Three tracks: A · Download reliability + health protocol · B · Legal repositioning · C · Payments/licensing resilience.

## Diagnosis (evidence-based, tested live 2026-07-16)

**Downloads** (probe matrix: bundled yt-dlp `2026.03.17` vs system `2026.07.04`, ± Chrome cookies):

- YouTube / TikTok / Vimeo: PASS on both binaries. YouTube is fragile though — intermittent
  anti-bot blocks ("confirm you're not a bot", SABR/403) which the app's retry logic does not
  recognize (`is_auth_required_error` too narrow, `src-tauri/src/lib.rs:36-47`).
- Instagram: broken. Requires login; the app has **no auth path for Instagram** (cookie retry
  list only covers YouTube/Vimeo/Facebook, `lib.rs:1294-1298`). Upstream extractor also flaky
  even with cookies (yt-dlp #13626, #16311).
- Facebook: broken upstream ("Cannot parse data") on ALL yt-dlp versions without TLS
  impersonation. Known mitigation: `--impersonate chrome` (curl_cffi is included in the
  official yt-dlp_macos binary). The app never passes `--impersonate` (yt-dlp #15161).
- X / LinkedIn: health-check probe URLs are dead → not actually monitored.
- Shipped users still run bundled `2026.03.17`: the runtime yt-dlp self-update
  (implemented 2026-06-16) was **never released**.
- Daily health check runs without cookies → auth platforms (IG/FB/LinkedIn/X) only ever WARN;
  real monitoring covers only YouTube/Vimeo/TikTok. Metadata-only (no download probe).

**Legal** (internal audit + external research, primary sources):

- Privacy Policy claims "no analytics / no telemetry" while the landing runs PostHog (US host,
  no consent banner, undisclosed). Founder is in Spain/EU → most concrete exposure, cheapest fix.
- Public README advertises "age-restricted/private content" retry — worst possible phrasing for
  a payment-processor risk review.
- Missing: EULA, copyright/abuse policy, responsible-use disclaimer on the home page
  (required by BRAND.md §8, never implemented), ToS acceptance in app onboarding.
- The tool itself is lawful (youtube-dl/EFF precedent; no DRM circumvention for standard
  YouTube). Spanish private-copy exception (art. 31.3 TRLPI) does NOT cleanly cover YouTube
  downloads → sales pitch must be "content you own / are licensed / CC / authorized", never
  "it's legal to download anything". Downie & 4K Video Downloader name YouTube openly — naming
  platforms is defensible; celebrating downloading other people's paid content is not.

**Payments** (verified 2026-07-16):

- LemonSqueezy does not ban downloaders by name, but inherits Stripe's discretionary
  "facilitates IP infringement" clause and can suspend without notice. Highest-risk moment:
  the 2026 forced migration to Stripe Managed Payments. If suspended, the license validation
  API likely dies with the store → already-sold keys would stop activating.
- Paddle explicitly prohibits "streaming downloaders" → permanently ruled out.
- Competitor routes (verified via checkout source): 4K Video Downloader → direct Stripe with
  own German entity; Downie → Stripe Payment Links + Setapp; PullTube → FastSpring.

## Decisions (founder, 2026-07-16)

1. **Execution order**: Track B urgent items first (GDPR + README), then reliability release
   v1.2.0, then robustness + health protocol v2, then payments mitigations.
2. **Tiered platform honesty**: landing + app distinguish stable platforms (YouTube, TikTok,
   Vimeo) from "requires your browser login / best-effort" (Instagram, Facebook, LinkedIn, X).
   Rationale: broken downloads → refunds/chargebacks → processor review; honesty is the cheapest
   insurance.
3. **Payments**: stay on LemonSqueezy short-term with 4 mitigations (periodic license/customer
   export, local activation cache in-app so an LS outage/ban never bricks Pro users, written
   product description to LS support for an on-record OK, chargeback monitoring). Re-evaluate
   Stripe-direct / FastSpring / Setapp only on real sales signal.
4. **GDPR**: PostHog stays but in cookieless/anonymous mode (memory persistence, no person
   profiles); Privacy Policy rewritten to disclose it honestly. No consent banner needed.

## Track A — Reliability + health protocol

**A1 · Release v1.2.0** (ship what exists + quick wins):
- Re-bundle fresh yt-dlp (≥ 2026.07.04) for both arches.
- Ship the runtime yt-dlp self-update (already in code, unreleased). Add version-comparison
  guard (stale managed copy must never beat a fresher bundled one after an app update).
- Add `--impersonate chrome` for Facebook (and as global fallback retry), guarded by a
  capability check (`yt-dlp --list-impersonate-targets`).
- YouTube hardening: appropriate `--extractor-args youtube:player_client=...`.

**A2 · Robustness layer**:
- Widen `is_auth_required_error`: bot-check, 429/rate-limit, IG/TikTok login patterns.
- Include Instagram + TikTok in the cookies retry; detect the user's default browser
  (Safari/Firefox/Brave/Arc/Edge) instead of hardcoding Chrome.
- Format fallback (downgrade chain) when the format selector fails.
- Global download concurrency cap (queue) to avoid rate-limit storms.
- Process-level timeout for hung yt-dlp runs; libx264 software fallback if VideoToolbox fails.
- End-user error messages (remove "brew upgrade yt-dlp" advice from a bundled app).

**A3 · Health protocol v2 ("operational certification")**:
- Fix dead probe URLs (X, LinkedIn); add URL-rot self-check.
- Daily run WITH cookies so auth platforms are genuinely monitored.
- Add one real-download probe (not metadata-only) at least weekly.
- Report includes managed/bundled yt-dlp versions; escalation runbook in PLATFORM-HEALTH.md.
- Definition of "app certified operational": all Tier-1 platforms PASS + yt-dlp ≤ 4 weeks old.

## Track B — Legal + sales repositioning

**B1 · GDPR (urgent)**: PostHog cookieless config in `web/src/layouts/Layout.astro`; rewrite
`web/src/pages/privacy.astro` (disclose PostHog anonymous analytics, LemonSqueezy, yt-dlp;
GDPR bases; controller identity/contact).

**B2 · Copy sanitation (urgent)**: README ("age-restricted/private content" line), Cargo.toml
placeholder description, "1000+ sites" claim in LAUNCH-PLAN.md.

**B3 · Legal surfaces**: EULA (rights representation, personal use, DRM exclusion, indemnity,
Spain/EU jurisdiction), copyright/abuse policy page with contact, responsible-use disclaimer on
the home page, ToS acceptance in app onboarding (done in Track A code block to avoid conflicts).

**B4 · Sales pitch reframing (Downie pattern)**: keep naming platforms; frame everything as
"save video for your offline editing workflow — your content, licensed, CC or authorized".
The Premiere Pro angle IS the legal positioning. Tiered platform honesty per Decision 2.

## Track C — Payments/licensing resilience

- C1: the 4 LS mitigations (Decision 3). License export: documented manual procedure + script if
  LS API allows. Local activation cache: app code (Track A block).
- C2: run the LS E2E verification (6 gates in NEXT-SESSION.md step 4) — founder-in-the-loop.
- C3 (deferred until sales signal): Stripe-direct vs FastSpring decision; Setapp submission.

## Non-goals

- No migration off LemonSqueezy now. No Windows/Linux. No new features beyond reliability.
- No full DRM-grade license hardening (separate backlog item stays in backlog).

## Risks

- Instagram may remain flaky even after cookies+impersonation (upstream) → tiered honesty
  covers the product promise; health protocol catches regressions.
- `--impersonate` depends on the bundled binary including curl_cffi → verify in A1 before
  shipping; fall back gracefully if unsupported.
- Release requires updater signing key (password in `~/.secrets`) — release ops per MEMORY.md.
