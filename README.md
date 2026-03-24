# Super Downloads

macOS video download app built with Tauri 2.x. Downloads from YouTube, TikTok, X/Twitter, Vimeo, Instagram, Facebook, and LinkedIn. All videos are optimized for Premiere Pro compatibility (H.264/AAC/MP4).

## Features

- Paste any supported video URL and download
- Queue-style list with live progress, speed, and metadata
- Auto-converts to H.264 for Premiere Pro compatibility
- Auto-retries with browser cookies for age-restricted/private content
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
