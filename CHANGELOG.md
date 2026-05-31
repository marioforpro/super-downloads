# Changelog — Super Downloads

All notable changes to this project will be documented in this file.

---

## [1.1.1] — 2026-05-31 (pending release)

### Fixed
- **YouTube downloads capped at 360p** — bundled `yt-dlp` was 4 months stale
  (`2026.01.29`), which lost access to high-resolution DASH formats and silently
  fell back to 360p on 4K/1080p videos. Refreshed bundled `yt-dlp` → `2026.03.17`;
  YouTube now downloads up to 4K again. Also fixes assorted "video won't download"
  failures caused by outdated extractors.

### Added
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
