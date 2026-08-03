# Super Downloads

macOS video download app built with Tauri 2.x, made for editors. Downloads from YouTube, TikTok, and Vimeo, plus Instagram, Facebook, X/Twitter, and LinkedIn (these platforms typically require your browser's login session and are supported on a best-effort basis). All videos are optimized for Premiere Pro compatibility (H.264/AAC/MP4).

Super Downloads is intended for downloading content you own, have licensed, or are otherwise authorized to download.

## Features

- Paste any supported video URL and download
- Queue-style list with live progress, speed, and metadata
- Auto-converts to H.264 for Premiere Pro compatibility
- Can use your browser's login session for platforms that require signing in (your session stays local; passwords are never seen or stored)
- Dark and light themes
- Auto-resize window, clipboard auto-add, MP3-only mode
- Optional download history — the list persists across restarts when enabled in Settings (off by default)

## Quick Start

```bash
npm install
npm run tauri dev
```

## Build

```bash
# Apple Silicon
npm run tauri build -- --target aarch64-apple-darwin --bundles app

# Intel
npm run tauri build -- --target x86_64-apple-darwin --bundles app

# Create DMGs
./create-dmg.sh
```

## Third-party components

Super Downloads bundles three third-party tools: **yt-dlp** (The Unlicense), **ffmpeg**, and **ffprobe** (GPL-family — the vendored builds differ in license mode by CPU architecture; see notices for the verified breakdown). See [THIRD-PARTY-NOTICES.md](THIRD-PARTY-NOTICES.md) for verified build/license details, upstream pointers, and known gaps.

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - Project structure, tech stack, design decisions
- [Development](docs/DEVELOPMENT.md) - Setup, building, adding platforms, debugging
