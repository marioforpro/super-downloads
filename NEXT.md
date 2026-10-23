# Siguiente sesión — guía para retomar

> **NEXT breadcrumb** (rol M4, `docs/protocols/DOC-CONTRACT.md`): guía humana corta. Lo trackeable vive en `ROADMAP.md` (plan 2026-10-23: R-SD-005 → R-SD-009). Historia anterior (LemonSqueezy, v1.1–v1.5): `docs/archive/NEXT-history-2026.md`.

> **NEXT (2026-10-23):** v1.5.4 publicada. **Demo v8 en la landing** (R-SD-011): música original de Suno, desenfoque real, cambios del founder; ficheros versionados `web/public/demo/*-v8.*` (la caché servía la v5). Landing: sin la timeline del hero, barra y botones con filo LED + glow, ventana del hero con borde de luz. **Redes: no publicar todavía** (founder). Taller de la demo en STUDIO `clients/super-downloads/demo-film/` (másters 16:9 y 9:16 en `motion/*/renders/`). Siguiente paso: que el founder vea la landing en vivo; «Watch demo» en Settings sale con la próxima versión de la app. Firma Apple: diciembre (R-SD-009).

## Dónde estamos (2026-10-23)

- **App gratis, v1.5.4 publicada** (2026-10-23: fiabilidad + pulido; la v1.5.3 del mismo día trajo «Report a problem» y la bienvenida rediseñada; la v1.5.2 del mismo día trajo motor 2026.10.21 y checksum en el auto-update). Fase: uso propio + amigos testeando, sin anuncio público (`docs/DECISIONS.md` 2026-10-03).
- **113 descargas de DMG** en total (GitHub API). La más bajada: v1.2.0 (60).
- **Health check diario:** 7/7 PASS. El DEGRADED (motor viejo) de 2026-10-22 queda resuelto con la v1.5.2 (motor empaquetado 2026.10.21).
- **Landing nueva en producción** (R-SD-005), pulida tras revisión de diseño (visual, UX, copy, accesibilidad): descarga directa según chip, guía de instalación tras el clic, móvil «Send this page to your Mac», FAQ ampliado, WCAG AA.

## Próximos pasos, en orden

1. ~~Desplegar la landing~~ — **en producción 2026-10-23**. Pendiente: pegar `superdownloads.app` en WhatsApp/X para ver la preview.
2. ~~v1.5.2 de mantenimiento~~ — **publicada 2026-10-23** (E2E 9/9). Pendiente founder: actualizar la app instalada con el banner «Update». E2E antes: `cd src-tauri && cargo test e2e_downloads_live -- --ignored --nocapture`. Release: `load-secrets && ./scripts/make-release.sh`.
3. ~~Motor que se actualiza solo~~ — **ya existía** desde v1.2.0 (refresco semanal en segundo plano). Lo que faltaba era verificar el checksum: hecho, va en la v1.5.2 (R-SD-007).
4. ~~«Report a problem» dentro de la app~~ — **publicado en la v1.5.3** (2026-10-23; E2E 9/9, UI 30/30). Probado por el founder: abre Apple Mail con el correo listo. Los reportes de amigos llegan a support@ — de ahí salen los siguientes arreglos.
5. **Firma + notarización de Apple** (R-SD-009): **decidido — sí, en diciembre (Navidad, objetivo 2026-12-20).** Tu paso: alta en Apple Developer (99 USD/año, `marioforpro@gmail.com`) + certificado Developer ID. El resto lo hago yo. App Store no (norma 5.2.3).
6. ~~v1.5.4~~ (R-SD-010) — **publicada 2026-10-23** (E2E 9/9, UI 39/39). Release: `source ~/.secrets && ./scripts/make-release.sh` → `--no-build --publish` (procedimiento completo: `docs/OPS.md` § Release pipeline).

## Regenerar screenshots de la landing

Cuando cambie la UI de la app: ver `scripts/screenshots/README.md` (renderiza el frontend real en WebKit con datos de ejemplo y escribe `web/public/shots/*.webp`).

## Bugs menores conocidos (QA 2026-10-03, ninguno bloquea)

1. ~~Un cambio de estado cerraba un menú ⋯ abierto en otra fila~~ → arreglado en v1.5.4.
2. ~~Sin watchdog si la red se cuelga~~ → v1.5.4: 90 s sin datos = error «connection stalled» con Retry.
3. ~~Prompt oculto del Llavero sin timeout~~ → v1.5.4: el mismo límite de 90 s.
4. ~~Al abrir Settings el foco no entraba en el panel~~ → arreglado en v1.5.4.
5. ~~Las filas nuevas no tenían animación de entrada~~ → v1.5.4.
6. ~~`url.contains("x.com")` casaba con dropbox.com~~ → arreglado en v1.5.2.
7. ~~Sin verificación SHA256 del motor auto-actualizado~~ → arreglado en v1.5.2.
8. ~~URLs de prueba del fallback de Instagram muertas~~ → cambiadas al reel vivo (2026-10-23). **Hallazgo:** el fallback propio ya no resuelve ni ese reel (dice «unavailable or deleted» mientras yt-dlp lo baja bien). Está dormido — solo entra si yt-dlp falla con el HTTP 400 — así que hoy no afecta. Si Instagram vuelve a romperse, primer paso: poner al día sus endpoints o quitarlo.
9. Intel: la primera ejecución del motor tras instalar tarda ~10 s (Rosetta; el calentamiento en segundo plano lo absorbe).

## Claves y secretos

| Archivo | Qué es | ¿En git? |
|---|---|---|
| `src-tauri/.tauri-update-key` | Clave privada del updater (contraseña en `~/.secrets`, `load-secrets`) | NO |
| `src-tauri/.tauri-update-key.pub` | Clave pública del updater | Sí |
