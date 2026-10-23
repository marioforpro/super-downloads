# Siguiente sesión — guía para retomar

> **NEXT breadcrumb** (rol M4, `docs/protocols/DOC-CONTRACT.md`): guía humana corta. Lo trackeable vive en `ROADMAP.md` (plan 2026-10-23: R-SD-005 → R-SD-009). Historia anterior (LemonSqueezy, v1.1–v1.5): `docs/archive/NEXT-history-2026.md`.

> **NEXT (2026-10-23):** v1.5.3 publicada («Report a problem» + bienvenida nueva). Primer paso founder: actualizar la app con el banner «Update» y probar ⋯ › Report en una fila fallida (+ pegar `superdownloads.app` en WhatsApp/X para ver la preview). Decisiones abiertas: firma/notarización (R-SD-009) · qué pasa al acabar «Free for a limited time» · frase «Made by Mario Monteiro». Después: arreglar lo que reporten los amigos.

## Dónde estamos (2026-10-23)

- **App gratis, v1.5.3 publicada** (2026-10-23: «Report a problem» dentro de la app + pantalla de bienvenida rediseñada; la v1.5.2 del mismo día trajo motor 2026.10.21 y checksum en el auto-update). Fase: uso propio + amigos testeando, sin anuncio público (`docs/DECISIONS.md` 2026-10-03).
- **113 descargas de DMG** en total (GitHub API). La más bajada: v1.2.0 (60).
- **Health check diario:** 7/7 PASS. El DEGRADED (motor viejo) de 2026-10-22 queda resuelto con la v1.5.2 (motor empaquetado 2026.10.21).
- **Landing nueva en producción** (R-SD-005), pulida tras revisión de diseño (visual, UX, copy, accesibilidad): descarga directa según chip, guía de instalación tras el clic, móvil «Send this page to your Mac», FAQ ampliado, WCAG AA.

## Próximos pasos, en orden

1. ~~Desplegar la landing~~ — **en producción 2026-10-23**. Pendiente: pegar `superdownloads.app` en WhatsApp/X para ver la preview.
2. ~~v1.5.2 de mantenimiento~~ — **publicada 2026-10-23** (E2E 9/9). Pendiente founder: actualizar la app instalada con el banner «Update». E2E antes: `cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture`. Release: `load-secrets && ./scripts/make-release.sh`.
3. ~~Motor que se actualiza solo~~ — **ya existía** desde v1.2.0 (refresco semanal en segundo plano). Lo que faltaba era verificar el checksum: hecho, va en la v1.5.2 (R-SD-007).
4. ~~«Report a problem» dentro de la app~~ — **publicado en la v1.5.3** (2026-10-23; E2E 9/9, UI 30/30). Pendiente founder: probar ⋯ › Report una vez en la app instalada. Los reportes de amigos llegan a support@ — de ahí salen los siguientes arreglos.
5. **Decidir firma + notarización de Apple** (R-SD-009): el disparador acordado (>100 descargas) ya se cumplió. 99 USD/año.

## Regenerar screenshots de la landing

Cuando cambie la UI de la app: ver `scripts/screenshots/README.md` (renderiza el frontend real en WebKit con datos de ejemplo y escribe `web/public/shots/*.webp`).

## Bugs menores conocidos (QA 2026-10-03, ninguno bloquea)

1. Un cambio de estado re-renderiza toda la lista → cierra un menú ⋯ abierto en otra fila.
2. Sin watchdog si la red se cuelga sin error (yt-dlp puede esperar ~8 min con `--retries 15`).
3. Un prompt oculto del Llavero (cookies) puede dejar el reintento esperando — cancelable, sin timeout.
4. Al abrir Settings el foco no entra en el panel.
5. Las filas nuevas no tienen animación de entrada.
6. ~~`url.contains("x.com")` casaba con dropbox.com~~ → arreglado en v1.5.2.
7. ~~Sin verificación SHA256 del motor auto-actualizado~~ → arreglado en v1.5.2.
8. Las URLs de prueba del fallback de Instagram están muertas (`--ig-fallback` da WARN).
9. Intel: la primera ejecución del motor tras instalar tarda ~10 s (Rosetta; el calentamiento en segundo plano lo absorbe).

## Claves y secretos

| Archivo | Qué es | ¿En git? |
|---|---|---|
| `src-tauri/.tauri-update-key` | Clave privada del updater (contraseña en `~/.secrets`, `load-secrets`) | NO |
| `src-tauri/.tauri-update-key.pub` | Clave pública del updater | Sí |
