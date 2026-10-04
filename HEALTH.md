---
name: Super Downloads
slug: SUPER-DOWNLOADS
emoji: "🔵"
type: product
status: active
lifecycle: active
blocked: false
blocker_reason: null
resolution_factor: 1.0
validity_factor: 1.0
next_milestone: Relaunch hardening — v1.2.0 reliability release + legal repositioning live (reopened 2026-07-16)
last_updated: 2026-10-04
validated_end_to_end: 2026-10-03  # free-mode path: founder smoke test of the v1.3.0 DMG (activation card + YouTube download) + superdownloads.app/api/activate → Airtable SD · Users verified (200 + row; 400 on bad email). LemonSqueezy Track C2 stays deferred with free mode.
stack: Tauri v2, Rust, vanilla JS, Astro
repo: marioforpro/super-downloads
---

# Super Downloads — Health Card

## Status
- **State**: FREE app, live — v1.5.0 published 2026-10-04 (release URLs 200, updater serves 1.5.0 for both arches). No payments provider.
- **Version**: v1.3.0 (published 2026-10-03 — FREE mode: no daily limit, license UI hidden, email activation → Airtable `SD · Users`; landing «Free for a limited time» + unsigned-install FAQ; includes the LinkedIn and Vimeo fixes). Previous: v1.2.0 (published 2026-07-16 — relaunch hardening Track A: download reliability, wider auth retry, engine UI, bundled yt-dlp 2026.07.04). Previous: v1.1.1 (2026-05-31 — yt-dlp 360p fix + one-click in-app auto-update)
- **Phase**: Private use + friends testing. Public announcement (R-SD-002) on hold by founder decision 2026-10-03. Payments (Track C, LemonSqueezy) CLOSED 2026-10-03 — no account, app is free (`docs/DECISIONS.md`).
- **Build**: macOS DMGs (Apple Silicon + Intel)
- **Monitoring**: daily platform health-check via launchd (shipped 2026-05-31). Lifecycle truth: the agent was dead 2026-07-02 → 2026-08-03 (plist sat renamed `.plist.disabled`; zero automated runs; one manual run 2026-07-16). Reloaded 2026-08-03 (audit Wave D, no kickstart) — first scheduled run expected 2026-08-04 10:00; registered in `00_System/AUTOMATIONS.md`.

## What It Does
macOS desktop app for downloading media. Tauri v2 with Rust backend, vanilla JS frontend. Astro-based landing page.

## Revenue Model
- Free (no limit), email activation. No payments provider since 2026-10-03; monetization, if ever, starts from scratch.

## Key Decisions Pending
- Signing/notarization (SD-F-009) — RESOLVED 2026-08-16 (a) deferral kept for the FREE launch: ship unsigned + xattr/«Open Anyway» instructions; revisit at >100 downloads or install complaints. Build hold released.

## LifeOS Integration
- **Domain**: 01_Projects (Product)
- **CLAUDE.md**: `./CLAUDE.md` (project-specific, comprehensive)
- **Parent CLAUDE.md**: `../../CLAUDE.md` (system-level context)
- **Docs**: `./docs/` (architecture, brand, launch, marketing, decisions)

## Key Files
- Project CLAUDE.md: `./CLAUDE.md`
- Binaries: `src-tauri/binaries/` (yt-dlp + ffmpeg, in .gitignore)

## Notes
- Bundles yt-dlp/ffmpeg binaries — large files, must stay gitignored
- Landing page deploys on Vercel; native app release ships through DMG/GitHub Release
- Resumed 2026-05-06; v1.1.1 + auto-updater + daily launchd health monitor shipped 2026-05-31. `NEXT.md` is the live resume guide.
