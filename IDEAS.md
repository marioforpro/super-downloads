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

- 2026-04-23 | Batch URL paste / queue for Super Downloads (multi-URL ingestion at once) [size: M] [confidence: 3]
  - What: Paste multiple URLs at once and download as a managed queue.
  - Why: Single-URL flow is the #1 friction reported by power users.
  - Where: SUPER-DOWNLOADS / app UX
- 2026-04-23 | Browser extension companion — detect videos on page → push to Super Downloads desktop [size: M] [confidence: 3]
  - What: Browser extension detects videos on-page and pushes them to the desktop app.
  - Why: Closes the gap between discovery (browser) and download (app).
  - Where: SUPER-DOWNLOADS / extension
- 2026-04-23 | Super Downloads Windows / Linux port [size: M] [confidence: 3]
  - What: Port the Tauri app to Windows and Linux from the macOS-only base.
  - Why: Mac-only caps TAM; Tauri makes the port cheap if scoped right.
  - Where: SUPER-DOWNLOADS / platform expansion
- 2026-04-23 | Super Downloads analytics telemetry opt-in pipeline [size: M] [confidence: 3]
  - What: Opt-in telemetry to learn which platforms / formats / failures matter.
  - Why: Without telemetry, roadmap priorities are guesses.
  - Where: SUPER-DOWNLOADS / analytics
