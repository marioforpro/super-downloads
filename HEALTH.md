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
next_milestone: paused — resume R-SD-001 from docs/NEXT-SESSION.md only when founder explicitly says so
last_updated: 2026-04-28
stack: Tauri v2, Rust, vanilla JS, Astro
repo: marioforpro/super-downloads
---

# Super Downloads — Health Card

## Status
- **Version**: v1.1 (DMGs ready)
- **Phase**: Pre-launch — app built, landing page ready
- **Build**: macOS DMGs (Apple Silicon + Intel)

## What It Does
macOS desktop app for downloading media. Tauri v2 with Rust backend, vanilla JS frontend. Astro-based landing page.

## Revenue Model
- Freemium: 5 downloads/day free; Pro is €29 one-time lifetime with up to 3 devices

## Key Decisions Pending
- Founder decision to resume R-SD-001
- Re-baseline launch target once work resumes

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
- As of 2026-04-27 this project is paused, not an active blocker. `docs/NEXT-SESSION.md` is the resume checklist.
