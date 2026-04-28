# SUPER-DOWNLOADS — Context

## Purpose
macOS desktop app for media downloads. Paused pre-launch — resume only on explicit founder request.

## Belongs here
- Tauri app code (`src/` frontend, `src-tauri/` Rust backend)
- Astro landing page (`web/`)
- Bundled binaries (`src-tauri/binaries/` — yt-dlp, ffmpeg)
- Project ideas + roadmap

## Does NOT belong here
- Distribution strategy (LemonSqueezy / store submissions) → `08_Growth/`
- Cross-platform port ideas if scoped beyond macOS — start in `01_Projects/IDEAS.md`

## Tech stack
- Tauri v2 (Rust backend, vanilla JS frontend)
- Astro for landing page
- Bundled: yt-dlp, ffmpeg (large binaries — must stay in `.gitignore`)

## Key files
- `CLAUDE.md` — project agent context
- `HEALTH.md` — paused status + last activity
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
- Resume: only on explicit founder request (default Claude behaviour: do not work here without prompt)
- Build: `./create-dmg.sh` from project root

## Known risks / staleness
- Bundled binaries (`src-tauri/binaries/`) are large; never commit
- Multiple `node_modules` folders in tree (`node_modules`, `node_modules 2`, `node_modules.nosync`) — iCloud sync artefact

## Pointers (do NOT auto-load)
- Distribution / launch plan: `08_Growth/CONTEXT.md`
- Pricing decisions: `10_Decisions/CONTEXT.md`
