# IDEAS — SUPER-DOWNLOADS

> Append only. One line per idea. Canonical line format:
> `- YYYY-MM-DD | <text> [marker] [marker]`
>
> State markers:
> - `[high-signal]`        — pre-priority flag (cap: 3 active per file)
> - `[parked]`             — kept, waiting for signal
> - `~~struck~~`           — killed (audit trail, never removed)
> - `[promoted→R-SD-NNN]` — linked to roadmap item (one-way pointer)
> - `[duplicates→IDEA-X]`  — manual dup marker
>
> Idea IDs in this file are emitted as `IDEA-SD-NNNN` by the reconstruction engine.

- 2026-04-23 | Batch URL paste / queue for Super Downloads (multi-URL ingestion at once) [size: M] [confidence: 3] [id: 0015] [presence: high]
  - What: Paste multiple URLs at once and download as a managed queue.
  - Why: Single-URL flow is the #1 friction reported by power users.
  - Where: SUPER-DOWNLOADS / app UX
- 2026-04-23 | Browser extension companion — detect videos on page → push to Super Downloads desktop [size: M] [confidence: 3] [id: 0019] [presence: high]
  - What: Browser extension detects videos on-page and pushes them to the desktop app.
  - Why: Closes the gap between discovery (browser) and download (app).
  - Where: SUPER-DOWNLOADS / extension
- 2026-04-23 | Super Downloads Windows / Linux port [size: M] [confidence: 3] [id: 0023] [presence: high]
  - What: Port the Tauri app to Windows and Linux from the macOS-only base.
  - Why: Mac-only caps TAM; Tauri makes the port cheap if scoped right.
  - Where: SUPER-DOWNLOADS / platform expansion
- 2026-04-23 | Super Downloads analytics telemetry opt-in pipeline [size: M] [confidence: 3] [id: 0027] [presence: high]
  - What: Opt-in telemetry to learn which platforms / formats / failures matter.
  - Why: Without telemetry, roadmap priorities are guesses.
  - Where: SUPER-DOWNLOADS / analytics
- 2026-05-31 | Platform-health monitor — daily automated check that all 7 platforms still extract, alert on failure [size: M] [confidence: 4] [presence: high] [shipped→launchd 2026-05-31]
  - What: Scheduled (launchd) daily run of `scripts/platform-health-check.sh`; macOS notification only on FAIL (silent on all-pass). Detects extractor breakage AND silent quality degradation (the 360p fallback) before users hit it.
  - Why: yt-dlp breaks per-platform constantly; today we only learn when a user complains. A daily green/red signal turns reactive firefighting into proactive patch releases.
  - Where: SUPER-DOWNLOADS / reliability ops (protocol live in docs/PLATFORM-HEALTH.md + script shipped)
- 2026-05-31 | One-click in-app auto-update UX — "Update" button → app relaunches itself (Claude Code / Codex desktop style) [size: M] [confidence: 4] [presence: high] [shipped→v1.1.1]
  - What: When a new release exists, surface an "Update" button in-app; one click downloads, installs, and relaunches with zero user steps. Tauri updater infra (pubkey + endpoints) is already configured in tauri.conf.json — the gap is the frontend check/prompt/relaunch flow (no updater UI in src/main.js yet).
  - Why: Frictionless updates are how we ship yt-dlp/extractor fixes fast (ties directly to platform-health). Users on stale builds = the 360p bug recurring forever. Bar is set by Claude Code / Codex desktop: click Update, done.
  - Where: SUPER-DOWNLOADS / app UX + release pipeline
- 2026-05-31 | Runtime yt-dlp self-update — refresh the extractor without an app release [size: M] [confidence: 3] [presence: high]
  - What: On launch / weekly, download the latest yt-dlp into a writable app-support dir (signed .app bundle is read-only, so can't self-update in place) and prefer that over the bundled binary when newer. Optionally surface "extractor updated" silently. ffmpeg/ffprobe change rarely and don't break per-platform — keep those release-bundled only.
  - Why: yt-dlp breaks per-platform on a days-to-weeks cadence; the app-release cycle is too slow to keep up (this is the 360p bug's structural cause). Decouples extractor freshness from app version so YouTube breakage is fixed in hours, not a release cycle. This is the "automatic, no conflicts for the user" mechanism the founder asked about (2026-05-31) — note `find_ytdlp()` already prefers external yt-dlp, so a managed app-support copy fits the existing resolution order.
  - Where: SUPER-DOWNLOADS / Rust backend (binary resolution + updater)
