# CLAUDE.md — Super Downloads

> Read automatically by Claude Code on session start.

## First Action

Read `NEXT.md` for founder action items, then `ROADMAP.md` for project state.

## Project

Super Downloads — macOS desktop app for downloading videos from multiple platforms.
Stack: Tauri 2.x (Rust backend + Vanilla JS frontend). Bundled yt-dlp + ffmpeg + ffprobe.
Landing: Astro v6 in `web/` subfolder. Deploy on Vercel.
Domain: superdownloads.app (Hostinger). Email: support@superdownloads.app.

## Current State (updated 2026-10-03)

- **State:** FREE app, live (v1.3.0 public). v1.4.0 committed and QA'd, not yet built/published.
- **Active plan:** ship v1.4.0 (onedir engine, launch-freeze fix, redesigned Settings, QA pass), then the launch announcement (R-SD-002). Payments track closed 2026-10-03.
- **Releases shipped:** v1.1.0 (2026-05-06) · v1.1.1 (2026-05-31) · v1.2.0 (2026-07-16) · v1.3.0 (2026-10-03, free mode + email activation). Pre-release E2E: `cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture`.
- **Phase 0-3:** COMPLETE (foundation, app polish, freemium + onboarding + auto-updater, www + bare domain via Vercel)
- **Phase 4 (Billing):** CLOSED 2026-10-03 — no payments provider (no LemonSqueezy account); the app is free with email activation. See `docs/DECISIONS.md` 2026-10-03.
- **GitHub:** `marioforpro/super-downloads` (public — Releases reachable anonymously)
- **Vercel:** `superdownloads.vercel.app` (live, auto-deploy on push)
- **Domain:** `superdownloads.app` + `www.superdownloads.app` working
- **Analytics:** PostHog integrated in landing
- **Billing:** none. License/checkout code is dormant behind `FREE_MODE` in `src/main.js`
- **Founder actions pending:** ship v1.4.0 (`./scripts/make-release.sh`), then the launch announcement (R-SD-002). See `NEXT.md`

## Critical Context

- **Brand is independent** — Super Downloads is NOT related to Super Prompts. Different product, different brand.
- **Free** — no download limit; email activation (→ Airtable `SD · Users`). No payments provider.
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
| Next actions for founder | `NEXT.md` |
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
