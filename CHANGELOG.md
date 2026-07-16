# Changelog — Super Downloads

All notable changes to this project will be documented in this file.

---

## [1.2.0] — 2026-07-16 (released)

GitHub Release: https://github.com/marioforpro/super-downloads/releases/tag/v1.2.0
Published 2026-07-16 on founder instruction; anonymous download URLs verified
HTTP 200; updater endpoint serves `latest.json` v1.2.0 (both arches).
SHA-256:
- `Super-Downloads_aarch64.dmg` — `619c91c884d7a9000a3c7b4d75d48f4a99918623b43571b3b76e44248d4598d2`
- `Super-Downloads_x64.dmg`     — `7c2b961a1272ebf99b9e3de3bb12485ccf2abd7b0dafb123f8241830c4f32f05`

Relaunch hardening release — download reliability (Track A of
`docs/superpowers/specs/2026-07-16-relaunch-hardening-design.md`).

### Added
- **Runtime yt-dlp self-update** (implemented 2026-06-16, first shipped here) —
  managed copy in `~/Library/Application Support/com.supermac.super-downloads/bin/`,
  preferred over the bundled binary; weekly non-blocking background refresh
  (atomic: download → chmod → verify `--version` → rename), manual `update_ytdlp`
  command. Decouples extractor freshness from the app release cycle.
- **Version guard** — a stale managed copy is pruned at startup when the bundled
  binary is fresher (post-app-update safety).
- **Browser impersonation** — `--impersonate chrome` (curl_cffi) on Facebook
  metadata/download/retry, plus a generic retry for fingerprint-blocked
  extraction ("Cannot parse data", "Unable to extract", 403). Fixes Facebook
  (verified live 2026-07-16: PASS 720p vs FAIL without).
- **Wider auth retry** — bot-check walls ("confirm you're not a bot"),
  429/rate-limits and Instagram's "empty media response" now trigger the
  cookie retry; Instagram, TikTok and X/Twitter added to the retry platforms.
- **Default-browser cookies** — `--cookies-from-browser` now targets the user's
  default browser (LaunchServices detection: chrome/safari/firefox/brave/edge/
  vivaldi/chromium; falls back to chrome).
- **Settings → Downloader engine** — shows the active engine version + source
  (bundled/self-updated) with an "Update engine" button.
- **Onboarding terms acceptance** — ToS/Privacy links + acceptance recorded
  (`termsAcceptedAt`).

### Changed
- Bundled yt-dlp refreshed `2026.03.17` → `2026.07.04` (universal, curl_cffi).
- End-user error messages: removed `brew`/`pip` advice (inapplicable in a
  bundled app); honest Instagram best-effort message; browser-agnostic wording.

---

## [1.1.1] — 2026-05-31 (released)

GitHub Release: https://github.com/marioforpro/super-downloads/releases/tag/v1.1.1
SHA-256:
- `Super-Downloads_aarch64.dmg` — `dd3a39f2023291f36668819b0dbe39798a5b544222ef6a3273c8294a5c849fb0`
- `Super-Downloads_x64.dmg`     — `75320c71adbce834e021c5e0bd14199cd08fdae19a1fcd798f96b6698f4208fd`

### Fixed
- **YouTube downloads capped at 360p** — bundled `yt-dlp` was 4 months stale
  (`2026.01.29`), which lost access to high-resolution DASH formats and silently
  fell back to 360p on 4K/1080p videos. Refreshed bundled `yt-dlp` → `2026.03.17`;
  YouTube now downloads up to 4K again. Also fixes assorted "video won't download"
  failures caused by outdated extractors.

### Added
- **One-click auto-update** — when a new release exists, an "Update" banner
  appears in-app; one click downloads, installs, and relaunches automatically
  (no re-download). Powered by the Tauri updater plugin with a static
  `latest.json` manifest published on GitHub Releases. Release pipeline scripted
  in `scripts/make-release.sh`.
- `scripts/platform-health-check.sh` + `docs/PLATFORM-HEALTH.md` — daily smoke
  test across the 7 supported platforms to catch extractor breakage before users do.

---

## [1.1.0] — 2026-05-06 (released)

GitHub Release: https://github.com/marioforpro/super-downloads/releases/tag/v1.1.0
SHA-256:
- `Super-Downloads_aarch64.dmg` — `ea0df1b2edee5161e8bb177ed93d4a29edeeaa6e2430974f06a374c8f86d4419`
- `Super-Downloads_x64.dmg`     — `768beac78921be8a6e75d6ebab2534d6c31a64ea3a67ab56c2a589ef839e4216`

### Fixed at release
- In-app "Get Pro" button URL — wired to real LemonSqueezy product UUID `21db1cfb-37f8-4371-8085-b5e30f89645f` (commit `cba5d29`, 2026-04-27). Pre-existing local DMGs predated this fix and were rebuilt before publish.

### Added (during 1.1.0 development cycle)
- Video quality selection (Best / 1080p / 720p)
- MP3-only mode for audio extraction
- Clipboard auto-add (monitors clipboard for video URLs)
- Light theme option
- Settings info guide
- Auto-resize window based on download list
- Clear all downloads button
- Download history persists across app restarts
- Thumbnail caching for completed downloads

### Improved
- H.264 VideoToolbox hardware-accelerated encoding
- Better error messages for common failures
- Auto-retry with browser cookies for age-restricted content

### Supported Platforms
- YouTube, TikTok, X/Twitter, Vimeo, Instagram, Facebook, LinkedIn

---

## [1.0.0] — 2026-02-XX

### Initial Release
- Download videos from YouTube, TikTok, X/Twitter, Vimeo, Instagram, Facebook, LinkedIn
- Queue-style download list with live progress
- Auto-conversion to H.264/AAC/MP4 for Premiere Pro compatibility
- Dark theme
- Apple Silicon + Intel builds
- DMG distribution
