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
- Download list persists across restarts

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

## Documentation

- [Architecture](docs/ARCHITECTURE.md) - Project structure, tech stack, design decisions
- [Development](docs/DEVELOPMENT.md) - Setup, building, adding platforms, debugging
