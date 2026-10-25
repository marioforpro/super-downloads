# SUPER-DOWNLOADS — Context

## Purpose
macOS video downloader for editors (H.264/MP4 ready for Premiere/Resolve/Final Cut). FREE app, live — v1.5.4 (2026-10-23); private use + friends testing, no public announcement yet. No payments provider (Track C closed 2026-10-03, `docs/DECISIONS.md`). Work plan 2026-10-23 in `ROADMAP.md` (R-SD-005…009). [Superseded: Track C2 (six Lemon Squeezy E2E gates) was the next move since the Big Sur gate cleared (founder decision SD-F-002=(a), 2026-08-03). *(Status line said "Paused pre-launch" until 2026-08-03 — stale through the 2026-06-15 park and the 2026-07-16 reopen; corrected per SD-F-003/PROJ-F-014.)*]

## Belongs here
- Tauri app code (`src/` frontend, `src-tauri/` Rust backend)
- Astro landing page (`web/`)
- Bundled binaries (`src-tauri/binaries/` — yt-dlp, ffmpeg)
- Project ideas + roadmap

## Does NOT belong here
- Cross-platform port ideas if scoped beyond macOS — start in `01_Projects/IDEAS.md`
- Growth channel strategy surfaces (`08_Growth/`) — but note: the launch announcement itself is **in-project** (payments track closed 2026-10-03) (R-SD-002 and R-SD-004 Track C2 live in this `ROADMAP.md`; ownership corrected 2026-08-03 per SD-F-003 — this file previously routed "distribution strategy" wholesale to `08_Growth/` against the ledger)

## Tech stack
- Tauri v2 (Rust backend, vanilla JS frontend)
- Astro for landing page
- Bundled: yt-dlp, ffmpeg (large binaries — must stay in `.gitignore`)

## Key files
- `CLAUDE.md` — project agent context
- `DESIGN.md` — design reference (web + app + STUDIO pieces): tokens, type, components, motion, copy
- `HEALTH.md` — health card (reopened status + last activity)
- `ROADMAP.md` — relaunch checklist
- `PROGRESS.md` — historical pre-launch progress notes
- `CHANGELOG.md` — project-level change log
- `create-dmg.sh` — build script

## Local idea bank
- Path: `01_Projects/SUPER-DOWNLOADS/IDEAS.md` (Phase 2)
- ID prefix: `IDEA-SD-NNNN`

## Cockpit surface
- `/departments/projects/super-downloads` (Phase 3)
- Primary signal: status (paused/blocked) + days since last commit

## Common workflows
- Work follows the ROADMAP plan (2026-10-23): landing refresh → v1.5.2 maintenance → engine self-refresh → tester feedback → signing decision
- Landing screenshots: regenerate with `node scripts/screenshots/capture.cjs` after any app UI change (`scripts/screenshots/README.md`)
- Build: `./create-dmg.sh` from project root — hold released 2026-08-16: SD-F-009 resolved (a) deferral — the FREE build ships unsigned (xattr / «Open Anyway» instructions on landing+FAQ); revisit signing at >100 downloads or install complaints

## Known risks / staleness
- Bundled binaries (`src-tauri/binaries/`) are large; never commit
- iCloud sync produces duplicate artefacts (`node_modules 2`, `web/dist/* 2.*`, `.nosync` variants). 2026-08-03: the empty `node_modules 2` dir and four `web/dist/* 2.*` files were relocated to `_archive/icloud-duplicates-2026-08-03/` (never-delete: archived, not destroyed; `_archive/` is gitignored). `node_modules.nosync` remains in place.

## Pointers (do NOT auto-load)
- Launch plan / checklist (in-project): `docs/LAUNCH-PLAN.md`, `docs/LAUNCH.md`
- Growth channel strategy: `08_Growth/CONTEXT.md`
- Pricing decisions: `10_Decisions/CONTEXT.md`
