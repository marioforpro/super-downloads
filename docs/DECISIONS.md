# Decisions — Super Downloads

> Source of truth for important product and strategic decisions.

---

## 2026-10-04 — Download flow: copy & come back by default, Batch mode for many
**Context:** Founder wanted fewer steps: «cuando tengo la app abierta y hago copy... el link ya debería aparecer»; paste should start the download. The clipboard auto-add already existed but sat in Settings and reset OFF every launch, so in practice it was never used. Merging everything into one always-on clipboard watcher was considered and rejected: every YouTube link copied for any other reason (WhatsApp, notes) would become a download.
**Decision (founder):** Two modes, modelled on 4K Video Downloader's Paste Link + Smart Mode:
- **Default (always on, no setting):** when the app comes to the front with a new video link on the clipboard, it is prefilled and Download is highlighted — Enter downloads. Never auto-starts (a link copied 20 min ago must not download just because the app was opened). Pasting a video link (field or Cmd+V anywhere) downloads immediately.
- **Batch mode** (the old auto-add, renamed): a toggle in the top bar (⌘B), not in Settings. While on, every copied video link downloads on its own, even with the app in the background; the ribbon counts them, duplicates are skipped, and what was on the clipboard when it was turned on is ignored. Still OFF on each launch (2026-03-24 decision kept for this mode).
**Why:** Copy → come back → Enter is one action and never surprises; bulk work gets a conscious, visible, session-only mode.

## 2026-10-03 — No public launch for now: private use + friends testing
**Context:** v1.4.0 published; the next roadmap step was the public announcement (R-SD-002, r/macapps + Show HN).
**Decision (founder):** «no quiero poner ningún anuncio, primero es para mí y mis amigos y vamos testeando, ya veremos si la promociono». No announcement. The app is used by Mario and shared with friends as a test phase; promotion is decided later.
**Consequence:** R-SD-002 → on hold. `docs/ANNOUNCEMENT-DRAFT.md` kept up to date (v1.4.0 facts) but marked paused. The public site and GitHub releases stay as they are (they already serve the free app).

## 2026-10-03 — "Best" always means the highest resolution available (4K included)
**Context:** v1.4.0 QA measured YouTube 4K (VP9/AV1 only) → H.264 conversion at ~1x real time on an M4 Pro (VideoToolbox; hardware decode, HEVC and two parallel sessions don't help — 4K frame throughput is the limit; a 1080p downscale runs ~3.9x). Option offered: cap "Best" at 1080p and add an explicit "4K" choice.
**Decision (founder):** «siempre best» — "Best" keeps delivering the highest resolution available, converted to H.264 when needed. 1080p/720p stay as the fast choices.
**Consequence:** no quality-menu change. The long conversion is made honest instead: real ffmpeg progress + time remaining in the row ("Converting to H.264 for editing… 42% · ETA 5:10").

## 2026-10-03 — No payments provider: the app is free, Track C closed
**Context:** Founder, 2026-10-03: «yo no tengo cuenta de LemonSqueezy y la app ahora es free». The docs still described a LemonSqueezy account, a wired €29 checkout and six E2E gates as the next move.
**Decision:** There is no payments provider. Super Downloads is free with email activation. Track C (LemonSqueezy mitigations + C2 gates, R-SD-004) is **closed**, not deferred; the 2026-07-16 "stay on LemonSqueezy" and the 2026-05-13 license-hardening decisions are superseded. If monetization ever returns, the provider is chosen from scratch (Paddle stays ruled out for downloaders).
**Consequence:** Terms (no 5/day tier, no Pro, no refund policy → "currently free") and Privacy (no purchase/LemonSqueezy section) updated for v1.4.0. The license/checkout code stays dormant behind `FREE_MODE` (not removed — no live cost); `scripts/check-release-artifacts.sh` still checks the checkout UUID as a provenance marker only. Legal positioning is unchanged and does not depend on a payments provider: no DRM circumvention (Terms §4, EU/ES law).

## 2026-08-10 — Launch goes FREE until further notice (email activation)
**Context:** Founder capture session 2026-08-10. The six LemonSqueezy C2 gates llevan semanas sin ejecutarse y el tema licencias/legal estaba frenando el lanzamiento a cero movimiento. Founder: «ponerlo Free For Limited Time, así me olvido del tema de las licencias por ahora y todo el mundo puede tener la app funcionando, y no tendremos problemas legales… incluso usarlo como marketing».
**Decisions (4, founder-approved):**
1. **App entera GRATIS hasta nuevo aviso** — sin fecha de fin comprometida públicamente ("Free for a limited time" como framing de marketing, sin relojes por usuario).
2. **Activación por email** — descarga libre, activación captura el email. La lista de usuarios ES el activo: el día que se active el cobro hay audiencia a la que hablarle.
3. **Objetivo de esta fase: tráfico web, descargas y usuarios** — no revenue.
4. **Track C (LemonSqueezy/pagos) SE DIFIERE** — las seis gates C2 dejan de bloquear el lanzamiento; se ejecutan cuando se active el cobro. Las 4 mitigaciones LS del 2026-07-16 siguen documentadas y vigentes para ese momento.
**Consequence:** R-SD-002 (announcement) se re-ancla al free launch (ya no espera el checkout LS). El copy freemium actual (5/día + €29 Pro) se sustituye por copy de modo free. Implementación: `00_System/TASKS.md § SUPER-DOWNLOADS`.

## 2026-07-16 — Relaunch hardening decisions
**Context:** Founder recommitted 2026-07-16 after the 2026-06-15 park. Live probe matrix + legal/payments audit (evidence in `docs/superpowers/specs/2026-07-16-relaunch-hardening-design.md`, the source of truth for this plan) surfaced: Instagram/Facebook broken, shipped builds running a stale bundled yt-dlp, legally toxic public copy, undisclosed PostHog analytics, and LemonSqueezy discretionary-suspension risk.
**Decisions (4, founder-approved):**
1. **Execution order B → A → C.** Track B urgent items first (GDPR + copy sanitation), then Track A reliability release v1.2.0 + robustness + health protocol v2, then Track C payments mitigations.
2. **Tiered platform honesty.** Landing + app distinguish stable platforms (YouTube, TikTok, Vimeo) from "requires your browser login / best-effort" (Instagram, Facebook, LinkedIn, X). **Why:** broken downloads → refunds/chargebacks → payment-processor review; honesty is the cheapest insurance.
3. **Stay on LemonSqueezy with 4 mitigations** (instead of migrating): (a) periodic license/customer export, (b) local activation cache in-app so an LS outage/ban never bricks Pro users, (c) written product description to LS support for an on-record OK, (d) chargeback monitoring. Re-evaluate Stripe-direct / FastSpring / Setapp only on real sales signal. Paddle permanently ruled out (prohibits "streaming downloaders").
4. **PostHog stays, cookieless/anonymous mode, no consent banner.** Memory persistence, no person profiles; Privacy Policy rewritten to disclose it honestly.
**Consequence:** Project unparked (R-SD-001 back to active; new R-SD-004 tracks the hardening). All sales copy reframed as "content you own / licensed / CC / authorized" — the Premiere Pro editing angle IS the legal positioning.

## 2026-05-13 — License gate hardened to Tier 1 (planned, not implemented) — Session 201
**Context:** Founder hit 5/day cap with no real Lemon license yet (Lemon checkout still pending). Bypassed gate in 60s via raw SQLite localStorage inject. Discovery: `isProUser()` is local-only, no revalidation on launch.
**Decision:** Two follow-up items added to ROADMAP.md Backlog:
  1. Issue real founder comp license via Lemon dashboard once product is live (replaces the local stop-gap).
  2. Implement Tier 1 hardening (periodic Lemon revalidation + signed offline grace cache). NOT Tier 2 (server-side metering) or Tier 3 (Keychain + signed integrity) — diminishing returns at €29/copy pre-launch.
**Why:** €29 one-time consumer app does not warrant full DRM. Tier 1 kills the trivial inject path (90%+ of realistic pirates) at ~1 day cost; higher tiers wait for evidence of meaningful piracy at scale. Detailed threat model + tier comparison in `docs/SECURITY-NOTES.md`.
**Consequence:** Pre-launch hardening item, not launch-blocker. Sequence: real founder comp license MUST land before Tier 1 ships, otherwise the founder stop-gap key fails revalidation.

---

## 2026-03-24 — Commercial model: Freemium
**Context:** Defining how to monetize Super Downloads before building any payment infrastructure.
**Decision:** Freemium model — free tier with limits, paid Pro tier for full features.
**Why:** Allows maximum distribution (free download) while enabling revenue from power users. Mirrors proven model for macOS utilities.
**Consequence:** Need to define exact free/pro boundaries before implementing. Billing infrastructure required.

**PENDING:** Exact tier definitions (what limits? what unlocks?). Options to explore:
- Download limit per day (e.g., 3 free / unlimited pro)
- Quality limit (e.g., 720p free / 4K pro)
- Platform limit (e.g., YouTube-only free / all platforms pro)
- Feature limit (e.g., no MP3 mode, no auto-add in free)

**RESOLVED (annotated 2026-08-03, MA-SD-8):** tiers were defined and shipped long ago — **5 downloads/day free · Pro unlimited, €29 one-time lifetime, up to 3 devices** (live in the app's freemium gate since Phase 2; see `HEALTH.md` Revenue Model). This PENDING block is kept as the historical option list.

---

## 2026-03-24 — Brand independence from Super Prompts
**Context:** Both products share the "Super" prefix but are different products for different audiences.
**Decision:** Super Downloads operates as a fully independent brand. No shared visual identity, no cross-promotion (for now).
**Why:** Different product categories, different audiences, different value propositions. Mixing brands would confuse both.
**Consequence:** Separate branding, separate channels, separate web presence. Umbrella brand may come later.

---

## 2026-03-24 — Code signing: deferred (updater NOT blocked)
**Context:** App is currently unsigned. macOS Gatekeeper creates friction for new users.
**Decision:** Defer Apple Developer ID ($99/year) until project is consolidated.
**Why:** Founder prefers to validate the product first before committing recurring costs.
**Key finding:** Tauri v2 updater uses its OWN Ed25519 key pair for update verification — this is INDEPENDENT from Apple code signing. We CAN ship auto-updates without paying $99/year.
**Trade-off:**
- Auto-updates: WORK without Apple signing (Tauri handles its own crypto)
- Gatekeeper warnings: Users will still see "app is damaged" warnings on first install
- Workaround: Include clear install instructions + `xattr -dr com.apple.quarantine` command
**Revisit:** When product has enough traction to justify $99/year. Signs: >100 downloads, or user complaints about install friction become significant.

---

## 2026-03-24 — Payment platform: LemonSqueezy (recommended)
**Context:** Need a payment platform for the Pro tier with license key management.
**Decision:** RECOMMENDED LemonSqueezy.
**Why:**
- Built-in license key generation and validation (perfect for desktop apps)
- Merchant of Record — handles all global taxes automatically
- 5% + $0.50 per transaction (same as Paddle)
- Instant setup, no approval process (Paddle requires days of review)
- Simple REST API for license validation from the Tauri app
- Owned by Stripe since 2024 (stable, not going anywhere)
- Purpose-built for indie/solo developers
**Alternative:** Paddle (more mature API, better for complex billing, but slower onboarding and overkill for our needs).
**Status:** CONFIRMED by founder 2026-03-24.

---

## 2026-03-24 — Web stack for landing: Astro
**Context:** Need a landing page for Super Downloads. Evaluated options.
**Decision:** Astro (static site on Vercel).
**Why:**
- Landing page is static content + download links. No dynamic features needed.
- Astro outputs zero JS by default = fastest possible load.
- Deploys to Vercel (familiar infrastructure).
- Can add React islands if interactivity needed later.
- Lighter and simpler than Next.js for this use case.
- Excellent SEO out of the box.
**Status:** CONFIRMED by founder delegation ("lo que tú recomiendes").

---

## 2026-03-24 — Domain selection: pending
**Context:** Need a domain for the Super Downloads landing page.
**Research results (2026-03-24):**

| Domain | Status | Notes |
|--------|--------|-------|
| superdownloads.com | TAKEN | Registered since 2004 (GoDaddy). Brazilian download portal. |
| super-downloads.com | TAKEN | Registered Dec 2025 (Alibaba). Likely squatter. |
| **superdownloads.app** | **AVAILABLE** | Best option. `.app` is perfect for macOS app. Google-backed. ~$14/yr. |
| **superdownloads.pro** | **AVAILABLE** | Matches superprompts.pro pattern. Professional feel. |
| superdownloads.dev | AVAILABLE | Good, but `.dev` suggests developer tool, not user app. |
| superdownloads.io | AVAILABLE | Common for startups but no particular advantage. |
| superdownloads.co | AVAILABLE | Clean but can be confused with .com. |
| superdownloads.tools | AVAILABLE | Descriptive but unusual TLD. |
| superdownloads.download | AVAILABLE | Too on-the-nose, might look spammy. |
| getsuperdownloads.com | AVAILABLE | .com fallback with "get" prefix. |
| superdownloadsapp.com | AVAILABLE | .com fallback, longer. |

**Top 3 recommendations:**
1. **superdownloads.app** — Perfect TLD for a macOS application. Clean, memorable, modern. Requires HTTPS (standard anyway).
2. **superdownloads.pro** — Matches Super Prompts domain pattern (superprompts.pro). Creates implicit consistency for potential future brand family.
3. **getsuperdownloads.com** — .com fallback. "get" prefix is common for app landing pages.

**Naming conflict note:** "SuperDownloads" (superdownloads.com.br) is a well-known Brazilian software download portal. Different category (directory vs tool), mainly Portuguese market. Not a blocking conflict but worth being aware of.

**Decision:** `superdownloads.app` — confirmed by founder 2026-03-24. Registered on Hostinger.

---

## 2026-03-24 — Freemium tier definition + pricing
**Context:** Need to define what the free tier includes vs Pro, and pricing.
**Decision:** Free tier = 5 downloads/day, 1 device. Pro = unlimited downloads, 3 devices. **€29 one-time lifetime license.**
**Why:**
- One-time payment matches market expectations (4K Download, Downie, etc. all use lifetime licensing)
- €29 is competitive with 4K Download Personal (€30.25) and in the market sweet spot
- No subscription fatigue for a local utility with no server costs
- All features available in both tiers — only limit is downloads/day and device count
**Implementation notes:**
- Counter resets daily (midnight local time)
- Need persistent counter in app data directory (not localStorage — that's frontend only)
- Pro unlocks via LemonSqueezy license key
- License validation: call LemonSqueezy API on activation, periodic revalidation
- Device limit: 3 activations per license key (LemonSqueezy handles this)
- Launch promo: LAUNCH30 (30% off = ~€20)
**Pricing comparison:**
- 4K Download Lite: €18.15/year (subscription)
- 4K Download Personal: €30.25 lifetime
- 4K Download Pro: €48.40 lifetime
- **Super Downloads Pro: €29 lifetime** ← competitive, clean price point

---

## 2026-03-24 — Tagline confirmed
**Context:** Needed a tagline that captures the product's unique positioning.
**Decision:** "The video downloader built for editors."
**Why:** Differentiates from generic downloaders. Speaks directly to target audience. Highlights the H.264/Premiere Pro optimization.
**Status:** LOCKED. Use across all platforms and materials.

---

## 2026-03-24 — Visual identity direction: Blue
**Context:** Defining the color direction for Super Downloads brand.
**Decision:** Blue-based identity. Professional, technological, clean.
**Reference:** Current app icon — dark navy background (#162b73 → #0a163f gradient) with light blue arrow (#d5ebff → #30a7ff gradient).
**Why:** Founder likes the existing icon and blue theme. It communicates trust, professionalism, and technology.
**Status:** Icon is confirmed. Web/marketing assets should extend this palette.

---

## 2026-03-24 — Support email registered
**Context:** Need a contact channel for the product.
**Decision:** support@superdownloads.app (hosted on Hostinger).

---

## 2026-03-24 — App needs design/UX review before business logic
**Context:** Founder clarified that the app is NOT finished. It needs a design pass, functionality review, and UX improvements before adding freemium logic or preparing for launch.
**Decision:** Insert a dedicated "App Review & Polish" phase BEFORE freemium/billing infrastructure.
**Why:** Business logic (freemium limits, updater, onboarding) should be built on top of a polished product, not a work-in-progress. Changing UX after adding freemium logic creates rework.
**Consequence:** Roadmap restructured. New Phase 1 = App Polish (design, UX, features). Freemium/infra moves to Phase 2.

---

## 2026-03-24 — Download history: clear on launch by default, user toggle in settings
**Context:** App currently always starts with empty download list. Need to decide if history persists.
**Decision:** Default = clear on launch (current behavior). Add toggle in settings: "Keep download history" (OFF by default). When ON, history persists across restarts.
**Why:** Founder prefers clean start, but users should have the choice. The Rust backend already has save/load functions ready.

---

## 2026-03-24 — Audio-only and Auto-add reset on launch: intentional
**Context:** Both settings reset to OFF every time the app opens.
**Decision:** This is intentional. Keep current behavior. No change needed.
**Why:** Safety defaults. Auto-add watching clipboard is a privacy-sensitive feature. Audio-only changes output format unexpectedly. Both should require conscious activation each session.

---

## 2026-03-24 — Project documentation structure
**Context:** Setting up Super Downloads for structured development with AI collaboration.
**Decision:** Adopt documentation patterns from Super Prompts, adapted for a desktop app context.
**Why:** Proven system that works well for AI-assisted development. Saves time by reusing what works.
**Files created:** CLAUDE.md, ROADMAP.md, BRAND.md, DECISIONS.md, DIAGNOSTIC.md, LAUNCH.md, MARKETING.md, PROGRESS.md

---

---

## 2026-08-16 — Code signing: deferral re-affirmed for the FREE launch (SD-F-009)
**Context:** Launch pivoted to FREE until further notice (§2026-08-10). Founder decision SD-F-009 asked whether Developer ID + notarization ($99/year) becomes a prerequisite of the free build. Evidence at decision time: 45 DMG downloads total across v1.1.0→v1.2.0 (revisit threshold from §2026-03-24 was >100); macOS 15 no longer offers right-click → Open for un-notarized apps (System Settings → Privacy & Security → "Open Anyway"; fully unsigned bundles show "damaged" and need `xattr -dr com.apple.quarantine`).
**Decision:** (a) keep the deferral. The FREE build ships unsigned; landing + FAQ carry explicit install instructions for both paths.
**Consequence:** the "no build/release/signing until SD-F-009" hold (CONTEXT/HEALTH/ROADMAP) is released; install instructions are part of the free-mode task (`00_System/TASKS.md`).
**Revisit:** unchanged — >100 downloads or install-friction complaints. Decided OOO 2026-08-16 (`10_Decisions/PENDING.md`).
