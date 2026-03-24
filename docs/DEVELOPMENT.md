# Super Downloads - Development Guide

## Prerequisites
- **Rust**: Install via [rustup](https://rustup.rs/)
- **Node.js**: v18+ (for Tauri CLI)
- **Xcode Command Line Tools**: `xcode-select --install`
- **Cross-compilation targets** (for building both architectures):
  ```
  rustup target add aarch64-apple-darwin
  rustup target add x86_64-apple-darwin
  ```

## Setup
```bash
npm install
```

## Development
```bash
npm run tauri dev
```
This starts the app in dev mode with hot reload for the frontend.

## Building for Production

### Apple Silicon (M1/M2/M3/M4):
```bash
npm run tauri build -- --target aarch64-apple-darwin --bundles app
```

### Intel:
```bash
npm run tauri build -- --target x86_64-apple-darwin --bundles app
```

### Create DMGs:
```bash
./create-dmg.sh
```
Output: `dist/SuperDownloads_v1.1_AppleSilicon.dmg` and `dist/SuperDownloads_v1.1_Intel.dmg`

## Code Quality Checks
```bash
npm run check          # Run all checks
npm run check:frontend # JS syntax check
npm run check:rust     # cargo fmt + clippy + test
```

## Key Files
| File | What it does |
|------|-------------|
| `src/main.js` | Frontend logic: state, events, rendering, settings |
| `src-tauri/src/lib.rs` | Backend: yt-dlp execution, progress, window mgmt |
| `src-tauri/tauri.conf.json` | Window config, bundling, security |
| `src/styles.css` | All styling with CSS variables for theming |

## Adding a New Platform
1. **`src/main.js`**: Add domain(s) to `videoDomains` array in `isVideoURL()`
2. **`src-tauri/src/lib.rs`**: Add platform detection flag (`is_newplatform = url.contains(...)`)
3. **`src-tauri/src/lib.rs`**: Add platform-specific yt-dlp args if needed (cookies, user-agent)
4. **`src/main.js`**: Add friendly error message in `getFriendlyErrorMessage()`
5. **`src/index.html`**: Update placeholder text if appropriate

## Updating Bundled Binaries
The app bundles yt-dlp, ffmpeg, and ffprobe for both architectures in `src-tauri/binaries/`.

To update yt-dlp:
```bash
# Download latest for both architectures
curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos -o src-tauri/binaries/yt-dlp-aarch64-apple-darwin
curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp_macos -o src-tauri/binaries/yt-dlp-x86_64-apple-darwin
chmod +x src-tauri/binaries/yt-dlp-*
```

## Debugging
- Backend logs go to stderr: `eprintln!()` statements throughout lib.rs
- Run `npm run tauri dev` and check terminal output for yt-dlp commands and errors
- Frontend console: Cmd+Option+I in dev mode

## Version Bump Checklist
1. `src-tauri/tauri.conf.json` → `version`
2. `package.json` → `version`
3. `src-tauri/Cargo.toml` → `version`
4. `create-dmg.sh` → DMG filenames
