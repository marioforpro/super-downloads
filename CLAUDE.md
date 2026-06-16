# CLAUDE.md — Super Downloads

> Read automatically by Claude Code on session start.

## First Action

Read `docs/NEXT-SESSION.md` for founder action items, then `ROADMAP.md` for project state.

## Project

Super Downloads — macOS desktop app for downloading videos from multiple platforms.
Stack: Tauri 2.x (Rust backend + Vanilla JS frontend). Bundled yt-dlp + ffmpeg + ffprobe.
Landing: Astro v6 in `web/` subfolder. Deploy on Vercel.
Domain: superdownloads.app (Hostinger). Email: support@superdownloads.app.

## Current State (updated 2026-06-16)

- **State:** PARKED 2026-06-15 — product shipped, commercially un-launched. Reopen only on real signal (user pull / press / explicit founder recommit).
- **Releases shipped:** v1.1.0 (2026-05-06) + v1.1.1 (2026-05-31). v1.1.1 = 360p fix (refreshed bundled yt-dlp) + one-click in-app auto-update + daily launchd platform-health monitor.
- **Phase 0-3:** COMPLETE (foundation, app polish, freemium + onboarding + auto-updater, www + bare domain via Vercel)
- **Phase 4 (Billing):** resumed 2026-05-06, re-parked 2026-06-15. One open gate when reopened → LemonSqueezy E2E verification (never run; 6 checks in `docs/NEXT-SESSION.md` step 4).
- **GitHub:** `marioforpro/super-downloads` (public — Releases reachable anonymously)
- **Vercel:** `superdownloads.vercel.app` (live, auto-deploy on push)
- **Domain:** `superdownloads.app` + `www.superdownloads.app` working
- **Analytics:** PostHog integrated in landing
- **Billing:** LemonSqueezy checkout wired to real product UUID (`src/main.js:99`); E2E verification not yet run
- **Founder actions pending:** Project is parked; resume only on explicit founder request. See `docs/NEXT-SESSION.md`

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
| Platform health protocol | `docs/PLATFORM-HEALTH.md` |
| Product audit & gaps | `docs/DIAGNOSTIC.md` |
| Architecture | `docs/ARCHITECTURE.md` |
| Development guide | `docs/DEVELOPMENT.md` |
| Brand guidelines | `docs/BRAND.md` |
| Launch checklist | `docs/LAUNCH.md` |
| Marketing plan | `docs/MARKETING.md` |
