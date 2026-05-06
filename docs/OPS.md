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
- Run `./scripts/check-release-artifacts.sh dist/*.dmg` before any `gh release create` — never trust file presence alone, always verify build provenance

**Never:**
- Push binaries (DMGs, yt-dlp, ffmpeg) to git — they're in `.gitignore`
- Commit anything from `src-tauri/binaries/`

## Deployment

- **App**: Tauri v2 — build with `npm run tauri build`. Produces DMGs in `src-tauri/target/<triple>/release/bundle/dmg/`.
- **Landing page**: `cd web && npm run build`. Deployed on Vercel (auto-deploy on push to main).
- **Cross-arch builds**: Homebrew `rust` ships only the host stdlib. To build the non-host target, prepend the rustup toolchain to PATH: `PATH="$HOME/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH" npm run tauri build -- --target x86_64-apple-darwin --bundles dmg`.

## Release verification

> **Why:** Session 161 (2026-05-06) caught a false invariant before publish — `dist/` contained DMGs whose mtime was 7 weeks older than the LS-URL fix commit (`cba5d29`, 2026-04-27). The 2026-04-28 pre-flight had verified file presence, not freshness. "Artifact exists" ≠ "artifact valid"; provenance is what proves a release. Rule below codifies that lesson.

Tauri compresses embedded JS assets (Brotli), so a built DMG cannot be `strings`-grepped for the LemonSqueezy URL. `scripts/check-release-artifacts.sh` proves provenance instead:

1. **Source gate** — git tree clean for release-critical paths · `src/main.js` contains the LS product UUID · `cba5d29` (URL-fix commit) reachable from HEAD.
2. **Artifact gate** — each DMG exists and its mtime is ≥ the latest commit touching release-critical paths.

Run before every `gh release create`. Failure means the artifact was built from stale source — rebuild before publishing.

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
