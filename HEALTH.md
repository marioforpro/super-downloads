---
name: Super Downloads
slug: SUPER-DOWNLOADS
emoji: "🔵"
type: product
status: pre-launch
lifecycle: pre-launch
blocked: false
blocker_reason: null
resolution_factor: 1.0
validity_factor: 1.0
next_milestone: Relaunch hardening — v1.2.0 reliability release + legal repositioning live (reopened 2026-07-16)
last_updated: 2026-07-16
stack: Tauri v2, Rust, vanilla JS, Astro
repo: marioforpro/super-downloads
---

# Super Downloads — Health Card

## Status
- **State**: REOPENED 2026-07-16 (founder recommit) — relaunch hardening active: download reliability (v1.2.0) + legal repositioning + payments resilience. Plan: `docs/superpowers/specs/2026-07-16-relaunch-hardening-design.md`. Was PARKED 2026-06-15 (product shipped, commercially un-launched).
- **Version**: v1.1.1 (shipped 2026-05-31 — yt-dlp 360p fix + one-click in-app auto-update)
- **Phase**: Relaunch hardening (R-SD-004, order B → A → C). LemonSqueezy E2E verification = Track C2 (closes R-SD-001)
- **Build**: macOS DMGs (Apple Silicon + Intel)
- **Monitoring**: daily platform health-check via launchd (shipped 2026-05-31)

## What It Does
macOS desktop app for downloading media. Tauri v2 with Rust backend, vanilla JS frontend. Astro-based landing page.

## Revenue Model
- Freemium: 5 downloads/day free; Pro is €29 one-time lifetime with up to 3 devices

## Key Decisions Pending
- LemonSqueezy E2E verification — six gates per `docs/NEXT-SESSION.md` step 4 (closes R-SD-001)
- Re-baseline launch target at LS-verify session start

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
- Resumed 2026-05-06; v1.1.1 + auto-updater + daily launchd health monitor shipped 2026-05-31. `docs/NEXT-SESSION.md` step 4 is the live checklist (LemonSqueezy E2E).
