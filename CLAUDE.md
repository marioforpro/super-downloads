# CLAUDE.md — Super Downloads

> Read automatically by Claude Code on session start.

## First Action

Read `docs/NEXT-SESSION.md` for founder action items, then `ROADMAP.md` for project state.

## Project

Super Downloads — macOS desktop app for downloading videos from multiple platforms.
Stack: Tauri 2.x (Rust backend + Vanilla JS frontend). Bundled yt-dlp + ffmpeg + ffprobe.
Landing: Astro v6 in `web/` subfolder. Deploy on Vercel.
Domain: superdownloads.app (Hostinger). Email: support@superdownloads.app.

## Current State (after Session 3, 2026-03-26)

- **Phase 0-1:** COMPLETE (foundation + app polish)
- **Phase 2:** COMPLETE (freemium + onboarding + auto-updater configured)
- **Phase 3:** Near complete (landing deployed on Vercel, DNS configured, awaiting propagation)
- **Phase 4+:** Not started (billing, pre-launch, launch)
- **GitHub:** `marioforpro/super-downloads` (private)
- **Vercel:** `super-downloads.vercel.app` (live, auto-deploy on push)
- **Domain:** `superdownloads.app` — DNS pointing to Vercel, propagation pending
- **Analytics:** PostHog integrated in landing
- **Founder actions pending:** See `docs/NEXT-SESSION.md`

## Critical Context

- **Brand is independent** — Super Downloads is NOT related to Super Prompts. Different product, different brand.
- **Freemium model** — 5 downloads/day free, unlimited Pro. €29 one-time lifetime. LemonSqueezy.
- **Premiere Pro focused** — All downloads optimized for H.264/AAC/MP4 editing compatibility.
- **Code signing deferred** — Tauri updater uses own Ed25519 signing (not Apple).
- **Build check** — After code changes: `npm run check`
- **Dev mode** — `npm run tauri dev`
- **Landing dev** — `cd web && npm run dev`

## Commit Style

`feat:` · `fix:` · `docs:` · `refactor:` · `chore:`

## Doc Ownership

| What | File |
|------|------|
| Next actions for founder | `docs/NEXT-SESSION.md` |
| Roadmap & phases | `ROADMAP.md` |
| Progress log | `PROGRESS.md` |
| Decisions | `docs/DECISIONS.md` |
| App UX audit | `docs/APP-AUDIT.md` |
| Product audit & gaps | `docs/DIAGNOSTIC.md` |
| Architecture | `docs/ARCHITECTURE.md` |
| Development guide | `docs/DEVELOPMENT.md` |
| Brand guidelines | `docs/BRAND.md` |
| Launch checklist | `docs/LAUNCH.md` |
| Marketing plan | `docs/MARKETING.md` |
