# SUPER-DOWNLOADS — Context

## Purpose
macOS desktop app for media downloads. REOPENED 2026-07-16 (founder recommit) — relaunch hardening (R-SD-004) active; v1.2.0 published 2026-07-16. Commercially un-launched: Track C2 (six Lemon Squeezy E2E gates) is the next move since the Big Sur gate cleared (founder decision SD-F-002=(a), 2026-08-03). *(Status line said "Paused pre-launch" until 2026-08-03 — stale through the 2026-06-15 park and the 2026-07-16 reopen; corrected per SD-F-003/PROJ-F-014.)*

## Belongs here
- Tauri app code (`src/` frontend, `src-tauri/` Rust backend)
- Astro landing page (`web/`)
- Bundled binaries (`src-tauri/binaries/` — yt-dlp, ffmpeg)
- Project ideas + roadmap

## Does NOT belong here
- Cross-platform port ideas if scoped beyond macOS — start in `01_Projects/IDEAS.md`
- Growth channel strategy surfaces (`08_Growth/`) — but note: the launch announcement + LemonSqueezy execution itself is **in-project** (R-SD-002 and R-SD-004 Track C2 live in this `ROADMAP.md`; ownership corrected 2026-08-03 per SD-F-003 — this file previously routed "distribution strategy" wholesale to `08_Growth/` against the ledger)

## Tech stack
- Tauri v2 (Rust backend, vanilla JS frontend)
- Astro for landing page
- Bundled: yt-dlp, ffmpeg (large binaries — must stay in `.gitignore`)

## Key files
- `CLAUDE.md` — project agent context
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
- Work proceeds under R-SD-004 (reopened 2026-07-16; the old "resume only on explicit founder request" rule ended with the founder recommit)
- Build: `./create-dmg.sh` from project root — **hold:** no build/release/signing until SD-F-009 (signing/notarization) is answered by the founder

## Known risks / staleness
- Bundled binaries (`src-tauri/binaries/`) are large; never commit
- Multiple `node_modules` folders in tree (`node_modules`, `node_modules 2`, `node_modules.nosync`) — iCloud sync artefact

## Pointers (do NOT auto-load)
- Launch plan / checklist (in-project): `docs/LAUNCH-PLAN.md`, `docs/LAUNCH.md`
- Growth channel strategy: `08_Growth/CONTEXT.md`
- Pricing decisions: `10_Decisions/CONTEXT.md`
