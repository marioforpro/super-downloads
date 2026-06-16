# ROADMAP — Super Downloads

---
scope: project:SUPER-DOWNLOADS
prefix: SD
lifecycle: pre-launch
current_phase: Phase 4 — Billing · PARKED 2026-06-15 (v1.1.1 shipped; commercially un-launched; reopen on real signal)
next_milestone: Parked — reopen on real signal (user pull / press / founder recommit); then LemonSqueezy E2E verification
updated: 2026-06-16
---

> Forward-motion view for Super Downloads at LifeOS level.
> Schema contract: `docs/protocols/ROADMAP-SCHEMA.md`.
> Phase 0–3 history preserved below and in git. Day-to-day code-level detail (per-phase checkbox lists) is NOT tracked here any more — that was task-manager territory. This file tracks launch-level forward motion only.

## Roadmap

- **R-SD-001** · P0 · paused · Submit Super Downloads to LemonSqueezy for approval
  - paused: 2026-06-15 (founder direction, Weekly Loop) — blocked ~7 weeks with no external pull; formally parked. Reopen only on real signal (user pull / press / explicit founder recommit). Resolves the active↔paused header conflict flagged in the 2026-05-31 audit.
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

## Shipped (last 30d)

- **R-SD-003** · download reliability (360p fix) — SHIPPED in **v1.1.1** (2026-05-31). Root cause: bundled yt-dlp was `2026.01.29` (~4mo stale) → YouTube fell back to 360p. Refreshed to `2026.03.17` (YouTube restored to 4K), verified via `scripts/platform-health-check.sh`. Released to users. Verified: `find_ytdlp()` already prefers the bundled binary first (same as ffmpeg/ffprobe), so users always get the fresh bundled yt-dlp. Open: Facebook extraction degraded on stable yt-dlp (upstream); runtime yt-dlp self-update (implemented in code 2026-06-16, Unreleased — see Backlog) decouples extractor freshness from app releases.
- **Platform-health monitor (automated)** — SHIPPED 2026-05-31. `launchd` agent runs `scripts/platform-health-notify.sh` daily at 10:00; notifies only when a platform's status *changes* (no daily spam). See `docs/PLATFORM-HEALTH.md`.
- **One-click auto-update** — SHIPPED in **v1.1.1** (2026-05-31). Tauri updater + static `latest.json` on GitHub Releases; in-app "Update" banner → download/install/relaunch. Release pipeline: `scripts/make-release.sh`. Signing key rotated (password in `~/.secrets`). First updater-capable build; future releases self-update for users on v1.1.1+.

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
- **Runtime yt-dlp self-update** — ✅ IMPLEMENTED IN CODE 2026-06-16 (Unreleased, see CHANGELOG). `src-tauri/src/lib.rs`: managed copy in `~/Library/Application Support/com.supermac.super-downloads/bin/`, preferred over bundled in `find_ytdlp()`; weekly non-blocking `.setup()` auto-update (atomic download → chmod → verify `--version` → rename); manual `update_ytdlp` command. Compiles clean (`npm run check:rust` ✅). **Pending to close:** (1) ship in the next signed build/release; (2) runtime E2E verify of the network/download path on a real run (untested while parked); (3) optional frontend "Update extractor" button wired to `update_ytdlp`; (4) consider a version-comparison guard so a stale managed copy never beats a fresher bundled one right after an app update.
- **Cockpit state wiring** — after the 2026-06-16 frontmatter reconciliation (ROADMAP + HEALTH → parked), regenerate + publish `state.super-downloads.json` via the LifeOS engine (`/lifeos-publish`) so the cockpit stops rendering SD as an active Phase-4 project. The doc edits are the inputs; the shard regen is the engine-facing surface. (cross-department: 00_System/LIFEOS-COCKPIT + Vercel)
- **Reconcile stale product audits** — `docs/APP-AUDIT.md` (14 items) + `docs/DIAGNOSTIC.md` are frozen at 2026-03-24/v1.1.0; many flagged items shipped across Phases 0–3 + v1.1.1. Add a "Reconciled" banner, strike shipped items, and lift the still-open debt into this backlog: history-persistence README contradiction, hardcoded version (P8), stray `console.log`s (T3), landing CSP `img-src`.
- **Release-ops hardening (dept)** — (a) backfill local annotated git tags for v1.1.0/v1.1.1 (none exist; `git show <tag>` history hints don't resolve) and add a tag-push step to `scripts/make-release.sh`; (b) lift the release pipeline (`make-release.sh`, required `latest.json` asset, `check-release-artifacts.sh` provenance gate, Ed25519 key rotation) from MEMORY.md into `docs/OPS.md` (the authoritative ops doc per Doc Ownership).

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
