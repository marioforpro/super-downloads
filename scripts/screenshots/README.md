# Landing screenshots

`capture.cjs` renders the app's real frontend (`src/`) in WebKit — the engine behind the app's WKWebView — with a fake Tauri API and staged downloads (`mock.js`), and writes `web/public/shots/*.webp`. Re-run it whenever the app UI changes so the landing never shows an old app.

```bash
npm i --no-save playwright@1.59 && npx playwright install webkit   # once
brew install webp                                                   # once (cwebp)
node scripts/screenshots/capture.cjs
```

- **Scenes** (`?scene=` in `mock.js`): `list`, `batch`, `settings`, `menu`, `onboarding`, `empty`. Add one by adding a list in `mock.js` and an entry in `SCENES`.
- **Thumbnails** are Blender Studio open movies (CC BY 4.0), downloaded from YouTube into `.cache/` (gitignored). The landing footer carries the credit — keep it if you keep the films.
- **Engine version** shown in Settings is hardcoded in `mock.js` (`get_ytdlp_version`); keep it equal to the bundled yt-dlp.
- **`web/public/og.png`** (1200×630 share preview) is a capture of the landing hero, not of the app. Re-take it after hero changes: build the site (`cd web && npm run build`), serve `web/dist`, screenshot the hero at 1200×630 with the nav hidden and the brand placed above the headline.
