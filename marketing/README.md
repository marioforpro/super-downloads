# Marketing — Super Downloads demo film

«Links become clips» — a 24 s demo built in HyperFrames from the **real app UI**. Story and shots: `demo/STORYLINES.md`, `demo/SPEC.md`.

| Project | Format | Use |
|---|---|---|
| `demo/` | 1920×1080 | landing `#demo`, X, YouTube |
| `demo-vertical/` | 1080×1920 | Instagram Reels/Stories, TikTok (shares `demo/assets` + `demo/compositions` by symlink) |

## After an app UI change

```bash
NODE_PATH=$HOME/.npm-global/lib/node_modules node marketing/demo/tools/extract-ui.cjs   # real HTML/CSS of each state
node marketing/demo/tools/build-app-scene.mjs                                           # app.html + v-app.html from src/app.tpl.html
```

Never edit `compositions/app.html` / `v-app.html` by hand — edit `demo/src/app.tpl.html` (layouts for both formats live in `build-app-scene.mjs`).

## Render

```bash
cd marketing/demo && npx hyperframes render -q delivery -o renders/demo.mp4
cd marketing/demo-vertical && npx hyperframes render -q delivery -o renders/demo_vertical.mp4
```

Run snapshots/renders with `env -u GEMINI_API_KEY -u GOOGLE_API_KEY` (otherwise `snapshot` calls Gemini, paid). `renders/` is git-ignored.
QA: `python3 ../STUDIO/method/studio finish motion-qa <render> --comp <project>` (one-frame jumps, dead stops; the platform roll and the footage's own motion are intended).

## Landing files

`web/public/demo/demo-1080.{mp4,webm}` + `demo-poster.webp`, encoded from the 16:9 master:

```bash
ffmpeg -i renders/demo.mp4 -c:v libx264 -preset slow -crf 25 -pix_fmt yuv420p -movflags +faststart -c:a aac -b:a 96k web/public/demo/demo-1080.mp4
ffmpeg -i renders/demo.mp4 -c:v libvpx-vp9 -crf 36 -b:v 0 -row-mt 1 -c:a libopus -b:a 80k web/public/demo/demo-1080.webm
```

## Credits

Film: «Spring», «CHARGE», «Sprite Fright», «WING IT!» © Blender Foundation, CC BY 4.0 (credited on the end card and the landing footer). SFX: Pixabay licence (`demo/assets/sfx/CREDITS.md`). Music: none yet — the film is cut on a 120 BPM grid so any 120 BPM track drops in.
