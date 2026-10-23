# Marketing — Super Downloads

The demo film («Links become clips», 16:9 + 9:16) is produced in **STUDIO**, not here. Production workshops live in STUDIO; only approved finals come back to this repo.

- **Workshop:** `/Users/supermac/Desktop/DEV/01_Projects/STUDIO/clients/super-downloads/demo-film/`
  - `motion/demo/` is the 16:9 HyperFrames project and the tools.
  - `motion/demo-vertical/` is the 9:16 version.
  - `motion/demo/README` explains how to rebuild, render (shutter motion blur), mix and do QA.
- **Finals delivered here:** `web/public/demo/demo-1080.{mp4,webm}` + `demo-poster.webp` (the landing `#demo` section). The job's `deliver_to` is `web/public/demo/`.
- **The film's UI is this app's real UI:** the workshop's `tools/extract-ui.cjs` renders `src/` from this repo. Re-run it after any UI change.
