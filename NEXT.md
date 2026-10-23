# Siguiente sesión — guía para retomar

> **NEXT breadcrumb** (rol M4, `docs/protocols/DOC-CONTRACT.md`): guía humana corta. Lo trackeable vive en `ROADMAP.md` (plan 2026-10-23: R-SD-005 → R-SD-009). Historia anterior (LemonSqueezy, v1.1–v1.5): `docs/archive/NEXT-history-2026.md`.

## Dónde estamos (2026-10-23)

- **App gratis, v1.5.1 publicada** (2026-10-20). Fase: uso propio + amigos testeando, sin anuncio público (`docs/DECISIONS.md` 2026-10-03).
- **113 descargas de DMG** en total (GitHub API). La más bajada: v1.2.0 (60).
- **Health check diario:** 7/7 plataformas PASS, pero veredicto **DEGRADED (motor viejo)** desde 2026-10-22 — yt-dlp empaquetado `2026.08.19` (64 días), upstream `2026.10.21`.
- **Landing nueva construida en local, sin desplegar** (R-SD-005): screenshots reales de la app, hero con timeline de edición, `og.png` (antes faltaba), tipografía Geist.

## Próximos pasos, en orden

1. **Desplegar la landing** (founder, 2 min): revisar en local `cd web && npm run dev` → OK → commit + push (Vercel despliega solo) → pegar `superdownloads.app` en WhatsApp/X para ver la preview.
2. **v1.5.2 de mantenimiento** (R-SD-006): motor yt-dlp al día + fix `x.com`/dropbox + copy del estado vacío. E2E antes: `cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture`. Release: `load-secrets && ./scripts/make-release.sh`.
3. **Motor que se actualiza solo** con verificación SHA256 (R-SD-007) — elimina el «motor viejo» para siempre.
4. **«Report a problem» dentro de la app** (R-SD-008) — que los amigos puedan reportar con un clic, con versión y error incluidos.
5. **Decidir firma + notarización de Apple** (R-SD-009): el disparador acordado (>100 descargas) ya se cumplió. 99 USD/año.

## Regenerar screenshots de la landing

Cuando cambie la UI de la app: ver `scripts/screenshots/README.md` (renderiza el frontend real en WebKit con datos de ejemplo y escribe `web/public/shots/*.webp`).

## Bugs menores conocidos (QA 2026-10-03, ninguno bloquea)

1. Un cambio de estado re-renderiza toda la lista → cierra un menú ⋯ abierto en otra fila.
2. Sin watchdog si la red se cuelga sin error (yt-dlp puede esperar ~8 min con `--retries 15`).
3. Un prompt oculto del Llavero (cookies) puede dejar el reintento esperando — cancelable, sin timeout.
4. Al abrir Settings el foco no entra en el panel.
5. Las filas nuevas no tienen animación de entrada.
6. `url.contains("x.com")` también casa con dropbox.com → **en R-SD-006**.
7. Sin verificación SHA256 del motor auto-actualizado → **en R-SD-007**.
8. Las URLs de prueba del fallback de Instagram están muertas (`--ig-fallback` da WARN).
9. Intel: la primera ejecución del motor tras instalar tarda ~10 s (Rosetta; el calentamiento en segundo plano lo absorbe).

## Claves y secretos

| Archivo | Qué es | ¿En git? |
|---|---|---|
| `src-tauri/.tauri-update-key` | Clave privada del updater (contraseña en `~/.secrets`, `load-secrets`) | NO |
| `src-tauri/.tauri-update-key.pub` | Clave pública del updater | Sí |
