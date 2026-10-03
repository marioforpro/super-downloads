# BORRADOR EN PAUSA — no publicar (decisión founder 2026-10-03)

> **Estado:** R-SD-002 **en pausa**. Mario usa la app él mismo y con amigos para testearla; la promoción pública se decide más adelante (`docs/DECISIONS.md` 2026-10-03). Este copy se mantiene al día para ese momento — no es un plan activo.
>
> Actualizado 2026-10-03 contra la **v1.4.0** publicada. Hechos verificados en la sesión de QA de ese día (E2E real en 7 plataformas).

---

## 1. r/macapps

**Título, opción A**
Super Downloads: a free macOS video downloader for editors — outputs Premiere-ready H.264/MP4 [I'm the dev]

**Título, opción B**
I built a native Mac app that saves web video as edit-ready H.264/MP4 for Premiere Pro. Free, feedback wanted

**Cuerpo (~240 palabras)**

Hi r/macapps, I'm Mario, the developer of Super Downloads. Disclosure: it's my own app. It's free for now (no end date set, and I'll say so here before anything changes).

**What it does**
- Paste a video URL, it joins a queue with live progress, speed and time remaining.
- YouTube, TikTok and Vimeo work out of the box. X/Twitter, Instagram, Facebook and LinkedIn work too and can use your browser's login session when a post needs it — the session stays on your Mac.
- Every file ends up as H.264/AAC/MP4, so it drops straight into Premiere Pro. "Best" keeps the highest resolution (4K included) and converts on your Mac's hardware encoder with a real progress bar; 1080p/720p are the fast options.
- Also: MP3-only mode, clipboard auto-add, light/dark theme, optional download history.

**What it doesn't do**
- It's for content you own, have licensed, or are authorized to download. It never circumvents DRM — Vimeo videos whose owner turned on Vimeo's DRM can't be downloaded, and the app says so.
- Instagram/Facebook/X/LinkedIn can break when those sites change. I'd rather say that up front.
- macOS only (Apple Silicon and Intel).

**Heads-up:** the build isn't signed with an Apple Developer ID yet. On first launch use System Settings → Privacy & Security → "Open Anyway", or run `xattr -dr com.apple.quarantine "/Applications/Super Downloads.app"`.

Activation asks for an email once (you can opt out of product emails). No account, no daily limit.

Link: superdownloads.app

What would make this useful for your workflow? Bugs and platform requests welcome.

---

## 2. Show HN

**Título, opción A**
Show HN: Super Downloads – macOS video downloader for editors (Tauri, Rust, yt-dlp)

**Título, opción B**
Show HN: A native Mac app that saves web video as Premiere-ready H.264/MP4

**Primer comentario (~230 palabras)**

Hi HN, I'm the author. Super Downloads is a macOS app (Apple Silicon and Intel) that saves video for offline editing and guarantees H.264/AAC/MP4, so Premiere Pro opens it directly.

Stack: Tauri 2, a Rust backend and a vanilla JS frontend with no framework. The backend runs bundled yt-dlp and ffmpeg, maps yt-dlp's per-stream progress onto one bar, and emits events to the UI. The landing is Astro on Vercel.

Things I learned that might interest you:
- The single-file macOS build of yt-dlp unpacks itself into a fresh temp dir on every run, and macOS scans those new files each time: ~7 s per call, before any work. Switching to the "onedir" build made it ~0.2 s.
- Pick the format by resolution first and prefer H.264 only at equal resolution; then check the real codec of the downloaded file with ffprobe and convert only what isn't H.264. TikTok's 1080p is HEVC; YouTube's 4K is VP9/AV1. 4K→H.264 on VideoToolbox runs about real time on an M4 Pro — hardware decode or parallel sessions don't help.
- Vimeo DRM is opt-in per owner. My first test videos happened to be Vimeo's own (DRM on), which nearly convinced me "all of Vimeo" was encrypted.
- A metadata-only health check said "Vimeo PASS" while every download failed. Now there's a live end-to-end test on the real download command before each release.

Free for now, unsigned, for content you own or are authorized to download. Feedback welcome.

---

## 3. Checklist antes de publicar (cuando se decida)

- [x] Build en modo FREE publicada (sin límite diario, activación por email → Airtable). v1.3.0+, hoy v1.4.0.
- [x] Landing con copy free-mode y FAQ de instalación sin firmar.
- [x] Terms/Privacy al día (sin Pro, sin LemonSqueezy, opt-in de emails, "no DRM circumvention" en Terms §4).
- [ ] Unas semanas de uso propio + amigos sin bugs críticos (fase actual).
- [ ] Screenshots (3-5) y GIF/vídeo demo con la v1.4.0 (Settings nuevo).
- [ ] Canal de soporte decidido (support@superdownloads.app ya existe).
- [ ] Probado en un macOS antiguo (la app declara 10.13+; nunca probado por debajo del actual).
- [ ] Instagram: re-verificar el fallback sin login con `scripts/platform-health-check.sh --ig-fallback` (sus URLs de prueba estaban muertas el 2026-10-03).
- [ ] Repo público **sin licencia** (código visible, no open source): decidir si se menciona el repo y con qué licencia antes de enlazarlo en HN.
- [ ] Normas de r/macapps (flair/disclosure de autor) revisadas el día.
- [ ] OK explícito de Mario.

Nota: `docs/LAUNCH-PLAN.md` sigue en modo de pago (€29, LAUNCH30) y está desfasado; no usar ese copy.
