# ROADMAP — Super Downloads

---
scope: project:SUPER-DOWNLOADS
prefix: SD
lifecycle: pre-launch
current_phase: Phase 4 — Billing (resumed 2026-05-06; v1.1.0 published, LemonSqueezy E2E verification next)
next_milestone: LemonSqueezy E2E verification (checkout · payment · license · activation · limits · LAUNCH30)
updated: 2026-05-31
---

> Forward-motion view for Super Downloads at LifeOS level.
> Schema contract: `docs/protocols/ROADMAP-SCHEMA.md`.
> Phase 0–3 history preserved below and in git. Day-to-day code-level detail (per-phase checkbox lists) is NOT tracked here any more — that was task-manager territory. This file tracks launch-level forward motion only.

## Roadmap

- **R-SD-001** · P0 · active · Submit Super Downloads to LemonSqueezy for approval
  - progress: v1.1.0 published 2026-05-06 (3/5 checklist items shipped — DNS verified · fresh DMGs from HEAD `cfbc320` · GitHub Release reachable anonymously after repo flipped public). Pre-publish caught a stale-artifact false invariant (DMGs in `dist/` predated LS-URL fix `cba5d29` by 7 weeks); resolved by `scripts/check-release-artifacts.sh` build-provenance gate.
  - next: LemonSqueezy E2E verification (six gates per `docs/NEXT-SESSION.md` step 4: checkout loads · test-mode payment · license generated · in-app activation · free/pro limit behavior · LAUNCH30 promo). On pass, product is commercially operational; demo recording (step 5) follows.
  - target: re-baseline at LS-verify session start
  - source: docs/LAUNCH.md, docs/NEXT-SESSION.md, docs/OPS.md
  - moved: 2026-05-06
- **R-SD-002** · P1 · next · Publish launch announcement (Reddit r/macapps + HN Show HN)
  - next: draft copy once R-SD-001 clears + LemonSqueezy checkout URL is live
  - depends_on: R-SD-001
  - source: docs/MARKETING.md
  - moved: 2026-04-23
- **R-SD-003** · P0 · next · Download reliability — fix silent quality degradation (4K/1080p videos downloading at 360p) and failing downloads
  - problem: founder testing (2026-05-31) found YouTube videos with 4K/2K/1080p available downloading at 360p, plus some videos not downloading at all. Root cause confirmed: bundled yt-dlp was `2026.01.29` (~4mo stale) — a known-4K YouTube probe returned only heights `[27,45,90,180,360]`. Stale yt-dlp loses access to high-res DASH formats and silently falls back to 360p; Facebook extraction failed with "Confirm you are on the latest version using yt-dlp -U".
  - progress (2026-05-31): bundled yt-dlp refreshed `2026.01.29` → `2026.03.17`; YouTube probe now returns 2160p (4K) — local fix verified via `scripts/platform-health-check.sh`. NOT yet shipped to users (they still run the old bundle until a release goes out).
  - next: (1) ✅ refresh bundled binaries — done; (2) ✅ re-run health check — YT 4K confirmed; (3) **ship a patch release** so users get the fresh binary (gated on one-click auto-update UX to be frictionless). Secondary: `find_ytdlp()` prefers system yt-dlp over bundled — review so end users always get the fresh bundle. Open: Facebook still FAILs on latest stable yt-dlp (real degradation, not staleness).
  - validation: `scripts/platform-health-check.sh` (asserts HD platforms ≥720p; exits non-zero on FAIL)
  - target: pre-launch blocker — degraded quality is invisible to users (no error), erodes trust on day one
  - source: docs/PLATFORM-HEALTH.md
  - moved: 2026-05-31

## Shipped (last 30d)

_(no shipped roadmap items this window — R-SD-001 is paused by founder direction)_

## Backlog (unranked)

- OG image asset (screenshot or custom design)
- Screenshots / assets / demo GIF for launch day polish (distinct from the demo video under R-SD-001)
- Batch URL paste feature
- Analytics telemetry opt-in (from Phase 2 deferred list)
- Windows/Linux expansion
- Browser extension (was Phase 8 in prior roadmap — legitimately future work)
- MacUpdate / AlternativeTo directory submissions
- Product Hunt launch (timing TBD)
- **Founder comp license via LemonSqueezy** — issue real comp license through Lemon dashboard (Product → Licenses → Issue License Key, comp/free) for `supermariomonteiro@gmail.com`, replace local `FOUNDER-MARIO-001` stop-gap (Session 201 inject, see `docs/SECURITY-NOTES.md`). Validates the full activation flow end-to-end with a real Lemon-tracked record. Depends on: Lemon product live (linked to R-SD-001 Phase 4 LS-verify).
- **License hardening / anti-crack security review** — current freemium gate is bypassable in 60s via raw SQLite localStorage inject (`proLicenseKey` set → `isProUser()` returns true forever, no re-validation). Documented attack vector in `docs/SECURITY-NOTES.md`. Scope: pick + implement a hardening level appropriate for €29 one-time consumer app (recommended: periodic Lemon revalidation + signed cache, NOT full DRM). Pre-launch hardening item, not launch-blocker.
- **Platform-health monitor (automated)** — schedule `scripts/platform-health-check.sh` via launchd to run daily; macOS notification only on FAIL (silent on all-pass). Protocol + manual runbook already live in `docs/PLATFORM-HEALTH.md`; this item is the automation layer. Turns extractor breakage from reactive (user complains) into proactive (patch before launch-day reviews). Ties to R-SD-003. (IDEAS.md, 2026-05-31)
- **One-click in-app auto-update UX** — surface an "Update" button when a new release exists; one click downloads + installs + relaunches (Claude Code / Codex desktop style). Tauri updater infra (pubkey + endpoints) already configured in `tauri.conf.json`; gap is the frontend check/prompt/relaunch flow — no updater UI in `src/main.js`. Critical delivery path for yt-dlp/extractor fixes (R-SD-003): without easy updates, users stay on stale builds and the 360p bug recurs forever. (IDEAS.md, 2026-05-31)
- **Runtime yt-dlp self-update** — refresh the extractor into a writable app-support dir without shipping an app release; prefer it over the bundled binary when newer. Decouples extractor freshness from app version (yt-dlp breaks weekly; releases are slower). The "automatic, no user conflicts" mechanism — `find_ytdlp()` already prefers external yt-dlp so it fits the existing resolution order. ffmpeg stays release-bundled (rarely breaks). (IDEAS.md, 2026-05-31)

## Prior Phases — history (one-line summary)

Earlier phase-oriented roadmap (Phases 0–8) is preserved in git history of this file. High-level status at 2026-04-23:

- **Phase 0: Foundation** — COMPLETE (branding, domain, payment choice, docs).
- **Phase 1: App Polish** — COMPLETE (empty state, toasts, drag-and-drop, native notifications, theme).
- **Phase 2: Product Infrastructure** — COMPLETE (freemium counter, auto-updater, onboarding, license UI).
- **Phase 3: Landing Page** — COMPLETE (live at `superdownloads.app` and `www.superdownloads.app` via Vercel; PostHog integrated).
- **Phase 4: Billing** — PAUSED → tracked as **R-SD-001** above.
- **Phase 5: Pre-Launch** — NOT STARTED → rolled into **R-SD-001** (demo video is the gating asset) and backlog (screenshots, copy).
- **Phase 6: Launch** — NOT STARTED → tracked as **R-SD-002** and backlog (directory submits).
- **Phase 7: Post-Launch** — future work, not in scope until R-SD-001 clears.
- **Phase 8: Browser Extension** — future, backlog.

Full phase-level detail (with per-task checkboxes) can be retrieved with `git show HEAD~:01_Projects/SUPER-DOWNLOADS/ROADMAP.md` until a formal `PROGRESS.md` is created.
