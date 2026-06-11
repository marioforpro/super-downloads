# Repo-Intel References — SUPER-DOWNLOADS

> **Type:** pattern-steal reference (NOT a dependency). Resolved via `/repo-intel-resolve` 2026-06-11. Canonical: `~/Vault/13_Sources/Repos/averygan-reclip.md`.

## averygan/reclip — EXTRACT (competing-product teardown)
A competing yt-dlp-wrapper download app — useful as a teardown of how a minimal version of SUPER-DOWNLOADS is built.
- **external-binary-preference resolution** — reclip just calls a bare `yt-dlp` on `$PATH`; that's the *minimal form* of SD's own `find_ytdlp()` external-binary resolution. Confirms SD's approach (resolve the binary at runtime, prefer a known path, fall back to `$PATH`) is the standard shape. **runtime-ytdlp-resolution** is the durable pattern.
- **in-memory job table + poll** — a uuid-keyed dict + worker thread + status-poll endpoint = the minimal legible form of background-job delegation. Reference for any SD background-download/queue surface that doesn't need a full job queue.
- **competing-product teardown** — reclip's feature set is a checklist to position SD against (what a bare wrapper does vs SD's value-add).

## History
- 2026-06-11 — created via `/repo-intel-resolve` (EXTRACT). runtime-ytdlp-resolution + in-memory job-table patterns captured as a competing-product teardown reference. Not a dependency, not installed.
