# BORRADOR — no publicar sin OK de Mario (OOO-auto 2026-10-02)

SUPER-DOWNLOADS · announcement copy (R-SD-002). Hechos sólo de README.md, CONTEXT.md, docs/DECISIONS.md (2026-08-10, 2026-08-16), ROADMAP.md, docs/LAUNCH-PLAN.md, docs/ARCHITECTURE.md, THIRD-PARTY-NOTICES (vía README), web/src/pages/index.astro. Nada se ha publicado.

Ángulo (ROADMAP R-SD-002 nota 2026-08-10): "Free for a limited time" ES parte del pitch. Sin fecha de fin comprometida.

---

## 1. r/macapps

**Título, opción A**
Super Downloads: a free (for a limited time) macOS video downloader for editors, outputs Premiere-ready H.264/MP4 [I'm the dev]

**Título, opción B**
I built a native macOS app that saves videos as edit-ready H.264/MP4 for Premiere Pro. Free for now, feedback wanted

**Cuerpo (~230 palabras)**

Hi r/macapps, I'm Mario, the developer of Super Downloads. Disclosure: it's my own app. It's free for now ("free for a limited time", no end date set, and I'll say so here before anything changes).

**What it does**
- Paste a video URL, it joins a queue with live progress and speed.
- Works out of the box with YouTube, TikTok and Vimeo.
- Instagram, Facebook, X/Twitter and LinkedIn work best-effort and can use your browser's login session. The session stays local; the app never sees or stores your passwords.
- Everything is converted to H.264/AAC/MP4 so it drops into Premiere Pro without re-encoding.
- Also: MP3-only mode, clipboard auto-add, dark/light theme, optional download history (off by default).

**What it doesn't do**
- It's meant for content you own, have licensed, or are authorized to download. It doesn't circumvent DRM. [VERIFICAR: "no DRM circumvention" aparece en el borrador del email a LemonSqueezy (NEXT.md), no en el README/landing; confirmar antes de afirmarlo]
- Instagram/Facebook/X/LinkedIn can break when those sites change. I'd rather say that up front.
- macOS only (Apple Silicon and Intel).

**Heads-up:** the build is unsigned (no Apple Developer ID yet). On macOS 15 use System Settings > Privacy & Security > "Open Anyway", or run `xattr -dr com.apple.quarantine "/Applications/Super Downloads.app"`.

Activation captures your email. [VERIFICAR: flujo de activación por email aún no implementado según 00_System/TASKS.md; decidir cómo se describe]

Link: superdownloads.app [VERIFICAR: la landing aún muestra el plan 5/día + €29 Pro]

What would make this useful for your workflow? Bugs and platform requests welcome.

---

## 2. Show HN

**Título, opción A**
Show HN: Super Downloads – macOS video downloader for editors (Tauri, Rust, yt-dlp)

**Título, opción B**
Show HN: A native Mac app that saves web video as Premiere-ready H.264/MP4

**Primer comentario (~190 palabras)**

Hi HN, I'm the author. Super Downloads is a macOS app (Apple Silicon and Intel) that saves video for offline editing and converts it to H.264/AAC/MP4 so Premiere Pro opens it directly.

Stack: Tauri 2.x, a Rust backend and a vanilla JS frontend with no framework. The backend spawns bundled yt-dlp and ffmpeg/ffprobe processes, parses yt-dlp's stdout for progress, and emits events to the UI. The landing page is Astro on Vercel.

Things I learned that might interest you:
- Platform extractors break constantly. I run a daily platform health check against real URLs, and it caught Vimeo failing for ~13 days when yt-dlp's anonymous OAuth bootstrap was revoked upstream (yt-dlp #17271). The app now retries via player.vimeo.com.
- Instagram anonymous endpoints are unreliable. I added an endpoint-format fallback inspired by instaloader. I couldn't verify its real-world hit rate, so that tier stays "best-effort". [VERIFICAR: re-comprobar con `scripts/platform-health-check.sh --ig-fallback` desde red residencial antes de publicar]
- Third-party licensing: yt-dlp is The Unlicense; ffmpeg/ffprobe builds are GPL-family and differ by CPU architecture (see THIRD-PARTY-NOTICES.md in the repo).

It's free for now, unsigned, and meant for content you own or are authorized to download. Repo: github.com/marioforpro/super-downloads [VERIFICAR: es público; NEXT.md dice "flipped private → public 2026-05-06", pero confirmar licencia y que el código fuente está abierto, no sólo las releases]. Feedback welcome.

---

## 3. Checklist antes de publicar

- [ ] Build en modo FREE distribuida (gate de licencia y límite 5/día quitados, activación por email, emails capturados en lista consultable). Fuente: 00_System/TASKS.md § SUPER-DOWNLOADS "Done when"; ROADMAP.md R-SD-002 nota 2026-08-10. Estado hoy: sin completar; landing/web index.astro sigue con "5 downloads per day" + "€29 Pro".
- [ ] Landing actualizada a copy free-mode (retirar pricing €29 de la superficie). Fuente: docs/DECISIONS.md §2026-08-10 Consequence; ROADMAP.md R-SD-002.
- [ ] Instrucciones de instalación para app sin firmar en landing + FAQ (xattr / "Abrir igualmente"). Fuente: docs/DECISIONS.md SD-F-009 (2026-08-16); docs/LAUNCH.md Gate 2 (casilla abierta). Parte ya en index.astro (~l.215).
- [ ] Release con las correcciones de main (Vimeo retry, IG fallback, fix LinkedIn 99768b6) publicada; hoy sólo v1.2.0 está pública. Fuente: NEXT.md (2026-09-27), ROADMAP.md R-SD-004 nota 2026-08-17.
- [ ] Screenshots (3-5) y GIF/vídeo demo. Fuente: docs/LAUNCH.md Gate 5; docs/LAUNCH-PLAN.md Pre-Launch.
- [ ] Canal de soporte y FAQ (instalación, Gatekeeper, plataformas). Fuente: docs/LAUNCH.md Gate 5.
- [ ] 3-5 beta testers con la versión final; bugs críticos resueltos. Fuente: docs/LAUNCH.md Gate 6.
- [ ] Probado en macOS 13+ y About con contacto. Fuente: docs/LAUNCH.md Gate 1. [VERIFICAR: tauri.conf.json dice minimumSystemVersion 10.13 y la landing "macOS 10.13+"; no está probado en la práctica]
- [ ] Disclaimer de uso responsable + Privacy/Terms revisados. Fuente: docs/LAUNCH.md Gate 3 (parcialmente hecho en Track B, ROADMAP.md R-SD-004).
- [ ] Normas de r/macapps (flair/disclosure de autor, apps gratis) revisadas el día. Fuente: no documentadas en el repo [VERIFICAR].
- [ ] Gate de lanzamiento comercial (seis gates LemonSqueezy C2) NO es requisito: diferido. Fuente: docs/DECISIONS.md §2026-08-10.
- [ ] OK explícito de Mario (propose, founder approves). Fuente: CLAUDE.md raíz.

Nota: docs/LAUNCH-PLAN.md y su borrador siguen en modo de pago (€29, LAUNCH30) y está desfasado frente a la decisión free; no usar ese copy.
