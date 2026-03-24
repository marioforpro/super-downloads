# Super Downloads - Architecture

## Overview
macOS desktop app for downloading videos from multiple platforms. Built with Tauri 2.x (Rust backend + vanilla JS frontend). All downloads are optimized for Premiere Pro compatibility (H.264/AAC/MP4).

## Tech Stack
- **Backend**: Rust + Tauri 2.x framework
- **Frontend**: Vanilla JS/HTML/CSS (no frameworks)
- **Bundled Binaries**: yt-dlp, ffmpeg, ffprobe (for both aarch64 and x86_64)
- **Platform**: macOS only (Apple Silicon + Intel)

## Directory Structure
```
SUPER-DOWNLOADS-v1.0/
├── src/                        # Frontend
│   ├── main.js                 # App logic, state, events, UI
│   ├── index.html              # HTML structure
│   └── styles.css              # Styling with dark/light themes
├── src-tauri/                  # Rust backend
│   ├── src/
│   │   ├── lib.rs              # Core: downloads, window mgmt, binaries
│   │   └── main.rs             # Entry point (delegates to lib.rs)
│   ├── tauri.conf.json         # App config, window, bundling
│   ├── Cargo.toml              # Rust dependencies
│   ├── binaries/               # Bundled yt-dlp, ffmpeg, ffprobe
│   ├── icons/                  # App icons
│   └── capabilities/           # Tauri permissions
├── dist/                       # Built DMG packages
├── docs/                       # Documentation
├── create-dmg.sh               # DMG creation script
└── package.json                # Node deps (Tauri CLI)
```

## Communication Flow
```
Frontend (JS) ──invoke──> Backend (Rust) ──spawn──> yt-dlp process
                                              │
Frontend (JS) <──emit─── Backend (Rust) <──stdout── yt-dlp output
```

**Tauri Commands** (frontend calls backend):
- `download_video` - Start a download
- `cancel_download` - Cancel active download
- `resize_window_height` - Adjust window height
- `set_min_window_height` - Set minimum height
- `pick_folder` - Open folder picker dialog
- `reveal_in_finder` - Open file in Finder
- `read_clipboard_text` - Read clipboard
- `save/load_download_history` - Persist downloads
- `cache_thumbnail_for_download` / `delete_cached_thumbnail` - Thumbnail cache

**Events** (backend emits to frontend):
- `download-started` - Title, thumbnail available
- `download-progress` - Percent, speed, ETA, status
- `download-finished` - File path, metadata
- `download-error` - Error message
- `download-cancelled` - Cancellation confirmed

## Download Lifecycle
```
queued → starting → downloading → [converting] → completed
                                                → error
                                                → cancelled
```

1. User pastes URL, clicks Download
2. Frontend validates URL, creates download entry, invokes `download_video`
3. Backend extracts metadata via `yt-dlp --dump-json`
4. Backend starts download with format selection (H.264 preferred)
5. Progress updates streamed via events
6. If VP9/AV1 content: auto-converts to H.264 via VideoToolbox
7. Completed file path emitted to frontend

## Supported Platforms
YouTube, TikTok, X/Twitter, Vimeo, Instagram, Facebook, LinkedIn

## Key Design Decisions
- **No cookies by default**: Avoids macOS keychain prompts. Auto-retries with cookies on auth errors (age-restricted, private content).
- **H.264 VideoToolbox**: Hardware-accelerated encoding for Premiere Pro compatibility.
- **Fixed window width**: Window auto-resizes height only (480px logical width).
- **Bundled binaries**: App is self-contained, no external dependencies needed.
- **Settings in localStorage**: Simple persistence, no backend settings store.
