---
name: Super Downloads
type: product
status: pre-launch
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
- Freemium (planned)

## Key Decisions Pending
- Launch date and distribution strategy
- Pricing tiers for premium features

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
- No Vercel deployment (native macOS app)
