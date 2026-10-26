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

## Release pipeline

1. **Tests** — `npm run check`, then the live E2E on the real `download_video`: `cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture` (9 platform cases; the metadata-only health check once said Vimeo PASS while every download failed).
2. **Version + CHANGELOG** — bump `package.json`, `src-tauri/tauri.conf.json`, `src-tauri/Cargo.toml` (+ `cargo update -p super-downloads --offline` for `Cargo.lock`); write the `## [x.y.z]` CHANGELOG section — it becomes the GitHub release notes — and add a plain-language entry at the top of `web/src/data/releases.js` (the landing's `/changelog`).
3. **Commit + push first** — the provenance gate compares DMG mtimes with the last release-critical commit, and the tag must point at a commit already on `origin/main`.
4. **Build** — `source ~/.secrets && ./scripts/make-release.sh` builds both arches, signs the updater artifacts (Ed25519 key `src-tauri/.tauri-update-key`, password `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` in `~/.secrets`; public key in `tauri.conf.json` = the `.pub` file verbatim) and writes `dist/latest.json`. **Apple signing + notarization** switch on by themselves when `APPLE_SIGNING_IDENTITY` and the notary key are in `~/.secrets` (dormant until the January enrollment, SD-F-017): `docs/SIGNING.md`; rehearsal without an account: `./scripts/make-release.sh --sign-dry-run`.
5. **Publish** — `./scripts/make-release.sh --no-build --publish`: runs the provenance gate, tags HEAD (`--target`), uses the CHANGELOG section as notes, uploads both DMGs + both `.app.tar.gz` + `latest.json`. The in-app updater polls `releases/latest/download/latest.json`, so `latest.json` must always be an asset. Auto mode cannot publish: the founder runs this line.
6. **After** — check the DMG + updater URLs return 200 and `latest.json` shows the new version; set the fallback `release` in `web/src/pages/index.astro` and push (Vercel rebuilds the landing, which reads version + sizes from the GitHub API).

## Release verification

> **Why:** Session 161 (2026-05-06) caught a false invariant before publish — `dist/` contained DMGs whose mtime was 7 weeks older than the LS-URL fix commit (`cba5d29`, 2026-04-27). The 2026-04-28 pre-flight had verified file presence, not freshness. "Artifact exists" ≠ "artifact valid"; provenance is what proves a release. Rule below codifies that lesson.

Tauri compresses embedded JS assets (Brotli), so a built DMG cannot be `strings`-grepped for a source marker (the dormant checkout UUID is used as one). `scripts/check-release-artifacts.sh` proves provenance instead:

1. **Source gate** — git tree clean for release-critical paths · `src/main.js` contains the LS product UUID · `cba5d29` (URL-fix commit) reachable from HEAD.
2. **Artifact gate** — each DMG exists and its mtime is ≥ the latest commit touching release-critical paths.

Run before every `gh release create`. Failure means the artifact was built from stale source — rebuild before publishing.

## Dev

- **Dev mode**: `npm run tauri dev`
- **Landing dev**: `cd web && npm run dev`
- **Build check**: `npm run tauri build`

## Key Integrations

- **Billing**: none — the app is free (no payments provider since 2026-10-03). Dormant checkout/license code stays behind `FREE_MODE` (`main.js`) and `SHOW_PRO=false` (`index.astro`).
- **Analytics**: PostHog (landing page only). No key required (client-side snippet).
- **DNS**: Hostinger — superdownloads.app. A record → Vercel. No key required.

## Binaries

- `src-tauri/binaries/` contains yt-dlp, ffmpeg, ffprobe (architecture-specific)
- Bundled into the app at build time by Tauri

## Constraints

- Code signing deferred — Tauri uses own Ed25519 signing
- Free: no download limit, email activation (no payments provider since 2026-10-03)
