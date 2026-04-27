# OPS.md — Super Downloads

> Operational rules for Super Downloads.
> Claude must read this before executing any operational task for this project.
> This file is authoritative — overrides generic assumptions.

---

## Critical Rules

**Always:**
- Run `npm run tauri build` before any release
- Run `npm run check` after code changes
- Optimize all downloads for Premiere Pro (H.264/AAC/MP4)

**Never:**
- Push binaries (DMGs, yt-dlp, ffmpeg) to git — they're in `.gitignore`
- Commit anything from `src-tauri/binaries/`

## Deployment

- **App**: Tauri v2 — build with `npm run tauri build`. Produces DMGs in `src-tauri/target/release/bundle/dmg/`.
- **Landing page**: `cd web && npm run build`. Deployed on Vercel (auto-deploy on push to main).

## Dev

- **Dev mode**: `npm run tauri dev`
- **Landing dev**: `cd web && npm run dev`
- **Build check**: `npm run tauri build`

## Key Integrations

- **Billing**: LemonSqueezy — €29 one-time, 3 activations. Checkout URL wired in `main.js` + `index.astro`. No API key required.
- **Analytics**: PostHog (landing page only). No key required (client-side snippet).
- **DNS**: Hostinger — superdownloads.app. A record → Vercel. No key required.

## Binaries

- `src-tauri/binaries/` contains yt-dlp, ffmpeg, ffprobe (architecture-specific)
- Bundled into the app at build time by Tauri

## Constraints

- Code signing deferred — Tauri uses own Ed25519 signing
- Freemium: 5 downloads/day free, unlimited Pro
