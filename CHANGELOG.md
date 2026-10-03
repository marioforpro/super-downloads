# Changelog — Super Downloads

All notable changes to this project will be documented in this file.

---

## [1.4.0] — 2026-10-03 (released)

GitHub Release: https://github.com/marioforpro/super-downloads/releases/tag/v1.4.0 — anonymous DMG URLs 200; updater `latest.json` serves 1.4.0 (aarch64 + x86_64). Supersedes the unpublished 1.3.1.

Faster downloads — every yt-dlp call used to cost ~7s before any work started.

### Changed
- **Engine switched to yt-dlp's onedir build** (`yt-dlp_macos.zip`, bundled as `Contents/Resources/yt-dlp-engine/`). The single-file build unpacked itself into a fresh temp dir on every run and macOS scanned those files each time: ~7s per call, and a download makes several (impersonation probe, metadata, download). Onedir is scanned once, then starts in ~0.2s (measured).
- Engine self-update installs the onedir zip into `bin/engine-<tag>/` (ditto extract → `--version` check → atomic rename); skips the download when that tag is already installed (it re-downloaded weekly before). Older engines are removed at the next launch, never mid-download.
- At launch, a background task cleans up the v1.2–1.3 single-file engine and interrupted downloads, and warms the active engine so its one-time macOS scan never lands on the user's first download.
- **No needless re-encode at 4K.** "Best Available" re-encoded every 1440p+ video to H.264, even when the source already offered H.264 at that resolution (Vimeo 4K). Now it converts only when the top resolution exists solely as VP9/AV1 (YouTube 4K) — faster and lossless otherwise.
- **Settings redesigned.** A full-window view replaces the 272px side drawer (which hid ~200px of settings below the fold): grouped rows in the macOS System Settings style, label left / control right, segmented controls for Format (Video MP4 | Audio MP3), Quality and Theme, a folder chip with middle-truncated path, Account and Engine in one compact group. Everything fits a 480×400 window without scrolling. Cmd+, toggles it; Esc closes the guide first, then Settings. The closed panel is no longer reachable with Tab. Light theme: the off-state switch is visible again (it was white on white).
- **Pre-release QA pass (E2E on the real `download_video`, 7 platforms + cancel + 4K):**
  - Format choice: highest resolution within the quality cap, H.264 preferred at equal resolution (`-S res,vcodec:h264,acodec:m4a`). After the download the file's real codec is checked (ffprobe) and only non-H.264 video is re-encoded (VideoToolbox) — with **real progress** from ffmpeg instead of a timer. Fixes TikTok 1080p arriving as HEVC while the UI said "converting", and H.264 sources being re-encoded for nothing.
  - Progress never moves backwards: video/audio streams are mapped onto one bar (≈85/15) and jitter from concurrent fragments is clamped. "Finishing…" shows during the merge, the bar pulses where no % exists, speed/ETA always show.
  - `--no-playlist` everywhere: a `watch?v=…&list=…` link downloaded the whole playlist (361 entries) into one file.
  - Cancel: every process runs in its own process group and cancel kills the group (ffmpeg kept encoding after cancel); cancel before the download starts is honoured; a cancelled row never comes back; partial files are cleaned up.
  - Retries (cookies/impersonation) and the Instagram fallback stream progress instead of a frozen bar until done.
  - No panics on error text with accents/emoji (char-safe truncation); output reads survive non-UTF-8; file names capped at 150 chars ("File name too long"); two downloads of the same title get distinct paths.
  - Engine update: one at a time, with network timeouts.
  - UI: plain-language errors (offline, sign-in, private, rate-limited, disk full, DRM), clamped to 3 lines; Enter no longer starts a download from a focused button; double-click and duplicate URL guarded; retry keeps MP3 as MP3; Open File opens the file; fade-ins actually animate; menus near the bottom open upwards; "Reduce motion" respected.
  - Vimeo: three-step flow — video page anonymously → embed player anonymously → video page with the browser session (owners who restrict embedding, which used to fail with HTTP 401). Videos whose owner enabled Vimeo DRM (opt-in per team/video — e.g. Vimeo's own account) are never circumvented; the app says "This video is DRM-protected by the site".
- Health check: probes the onedir engine; the YouTube download probe now merges video+audio like the app (YouTube no longer serves single-file formats, which made the old probe report a false pipeline FAIL).

---

## [1.3.1] — 2026-10-03

### Fixed
- **Launch freeze (~7s).** The app froze right after opening: the Settings engine label called `get_ytdlp_version`, a sync command (runs on the main thread), and the standalone yt-dlp takes ~7s to answer `--version` (it unpacks itself and macOS scans the fresh files on every run). Now async + `spawn_blocking`. The startup stale-engine prune (two more `--version` calls, ~14s when a self-updated engine exists) moved off `setup()` into a background task.
- Weekly engine self-update is stamped only on success — quitting the app mid-update no longer skips it for a week.

### Changed
- Activation: the email opt-in is ticked by default, copy "Email me about updates and new products". Privacy policy updated (legitimate interest / LSSI 21.2, opt-out at activation and in every email).

---

## [1.3.0] — 2026-10-03

Free mode — the whole app is free for a limited time, activated with an email.

### Changed
- **No daily limit.** The 5-downloads/day freemium cap and the Pro/license UI are hidden behind `FREE_MODE` (LemonSqueezy code kept intact, Track C deferred).
- Landing: "Free for a limited time", single free plan, install FAQ for the not-yet-notarized build ("Open Anyway" / `xattr`), privacy policy covers the activation email.

### Added
- **Email activation** — asked once (onboarding for new users, a one-time card for existing ones; Pro licence holders can skip). Stored locally first; sent to `superdownloads.app/api/activate` → Airtable in the background and retried on launch. Downloads are never blocked by a server failure.
- `web/api/activate.js` — Vercel function (env: `AIRTABLE_TOKEN`, `AIRTABLE_BASE_ID`, `AIRTABLE_SD_USERS_TABLE`).

### Fixed
- LinkedIn: try logged-out first, browser cookies only as retry (`99768b6`).
- Vimeo: match yt-dlp 2026.08+ login-wall text in the embed retry (`c587dbb`).

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
