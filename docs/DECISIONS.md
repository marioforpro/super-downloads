# Decisions — Super Downloads

> Source of truth for important product and strategic decisions.

---

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
