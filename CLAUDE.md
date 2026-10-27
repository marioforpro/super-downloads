# CLAUDE.md — Super Downloads

> Read automatically by Claude Code on session start.

## First Action

Read `NEXT.md` for founder action items, then `ROADMAP.md` for project state.

## Project

Super Downloads — macOS desktop app for downloading videos from multiple platforms.
Stack: Tauri 2.x (Rust backend + Vanilla JS frontend). Bundled yt-dlp + ffmpeg + ffprobe.
Landing: Astro v6 in `web/` subfolder. Deploy on Vercel.
Domain: superdownloads.app (Hostinger). Email: support@superdownloads.app.

## Current State (updated 2026-10-26, v1.5.7 published)

- **State:** FREE app, live — **v1.5.7 published 2026-10-26** (GitHub Release + updater; DMGs 97/110 MB): no in-app usage notice (users are told on the website — hero + download section — and /privacy#usage-stats; switch in Settings), welcome card «Free · unlimited downloads». Demo film v10 live at /#demo and /press. Price line «Free», no limited time (SD-F-016). Anonymous daily `app_active` ping, on by default within the CNIL/AEPD audience-measurement exemption (privacy §2; test `node scripts/test-active-ping.cjs`).
- **Active plan:** private use + friends testing (no public launch for now — `docs/DECISIONS.md` 2026-10-03). Fix what testers find; run the live E2E before each release. Payments track closed; go-to-market plan for the big launch Tue 2027-01-26 (after signing): the private GTM plan (LifeOS `08_Growth/super-downloads/GTM-2026-10-25.md`, kept out of this public repo) (R-SD-012).
- **Releases shipped:** v1.1.0 (2026-05-06) · v1.1.1 (2026-05-31) · v1.2.0 (2026-07-16) · v1.3.0 (2026-10-03, free mode + email activation) · v1.4.0 (2026-10-03, onedir engine, launch-freeze fix, Settings redesign, QA pass, Vimeo 3-step) · v1.5.0 (2026-10-04, copy & come back, paste-to-download, Batch mode) · v1.5.1 (2026-10-20, Vimeo domain-restricted embeds via bundled yt-dlp plugin `src-tauri/ytdlp-plugins/`) · v1.5.2 (2026-10-23, engine 2026.10.21, checksum-verified engine self-update, X-host fix) · v1.5.3 (2026-10-23, in-app «Report a problem» → prefilled email + clipboard copy, redesigned welcome screen) · v1.5.4 (2026-10-23, 90-s stall watchdog, stable row menu, Settings focus, real macOS minimums 12 arm64 / 10.15 Intel) · v1.5.5 (2026-10-25, Vimeo hidden pages, Instagram backup route, «Copy. Paste. Done.», Watch demo, anonymous daily active ping) · v1.5.6 (2026-10-26, unticked marketing consent, share notice, Tauri 2.12) · v1.5.7 (2026-10-26, usage ping told on the website instead of in-app, «Free» welcome card). Pre-release E2E: `cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture`.
- **Phase 0-3:** COMPLETE (foundation, app polish, freemium + onboarding + auto-updater, www + bare domain via Vercel)
- **Phase 4 (Billing):** CLOSED 2026-10-03 — no payments provider (no LemonSqueezy account); the app is free with email activation. See `docs/DECISIONS.md` 2026-10-03.
- **GitHub:** `marioforpro/super-downloads` (public — Releases reachable anonymously)
- **Vercel:** `superdownloads.vercel.app` (live, auto-deploy on push)
- **Domain:** `superdownloads.app` + `www.superdownloads.app` working
- **Analytics:** PostHog integrated in landing
- **Billing:** none. License/checkout code is dormant behind `FREE_MODE` in `src/main.js`
- **Landing:** redesigned 2026-10-23 (real app screenshots, editing-timeline hero, `og.png`, Geist) — live. Screenshots regenerate via `scripts/screenshots/capture.cjs`.
- **Health:** 7/7 platforms PASS; bundled engine 2026.10.21 (v1.5.6). Installed apps self-update the engine weekly (since v1.2.0).
- **Founder actions pending:** update the installed app via the banner; Apple Developer enrollment by 2027-01-05 (R-SD-009, SD-F-017; steps in `docs/SIGNING.md`); welcome email switch-on (Resend + Vercel env). See `NEXT.md`

## Critical Context

- **Brand is independent** — Super Downloads is NOT related to Super Prompts. Different product, different brand.
- **Free** — no download limit; email activation (→ Airtable `SD · Users`). No payments provider.
- **Premiere Pro focused** — All downloads optimized for H.264/AAC/MP4 editing compatibility.
- **Code signing deferred** — Tauri updater uses own Ed25519 signing (not Apple). Apple Developer ID pipeline is ready but dormant until the January enrollment (`docs/SIGNING.md`, `make-release.sh --sign-dry-run`). The site never says «signed»/«notarized» until a notarized build is published.
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
| Docs index (what is current) | `docs/README.md` |
| Platform health protocol | `docs/PLATFORM-HEALTH.md` |
| Support (channels, FAQ, macros) | `docs/support/` |
| Architecture | `docs/ARCHITECTURE.md` |
| Development guide | `docs/DEVELOPMENT.md` |
| Brand guidelines | `docs/BRAND.md` |
| Design system (tokens, type, components, motion, copy patterns) | `DESIGN.md` |
| Launch checklist | `docs/LAUNCH.md` |
| Go-to-market plan | private, LifeOS `08_Growth/super-downloads/` |
