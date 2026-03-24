# App Audit — Super Downloads v1.1.0

> Auditoría completa de diseño, UX, funcionalidad y calidad técnica.
> Fecha: 2026-03-24. Basada en revisión de todo el código fuente (4,726 líneas).

---

## 1. Lo que funciona muy bien

Estas cosas están bien implementadas y no necesitan cambios significativos:

- **Download engine**: Sólido. yt-dlp + ffmpeg bien integrados, fallbacks de binarios, retry con cookies.
- **Progress tracking**: Smoothed speed, ETA calculation, conversion progress simulation — sofisticado.
- **Thumbnail system**: Caching local de thumbnails, fallback SVG, cleanup on remove.
- **Window auto-resize**: Se adapta al contenido dinámicamente.
- **Error handling**: Mensajes específicos por plataforma (TikTok, Facebook, LinkedIn, Vimeo, YouTube age-restricted).
- **Keyboard shortcuts**: Cmd+V paste, Cmd+K clear, Enter download, Escape close.
- **Clipboard auto-add**: Monitorización periódica con auto-start de descargas.
- **Code architecture**: Clean separation frontend/backend, event-based communication.

---

## 2. Problemas de Diseño / UI

### P1 — No hay estado vacío inspirador
**Actual:** "No active downloads" — texto gris plano.
**Problema:** Primera impresión fría. No comunica qué hacer ni qué puede hacer la app.
**Propuesta:** Un empty state con el icono de la app, el tagline, y un hint ("Paste a video link to get started" o similar). Mucho más acogedor y profesional.

### P2 — Settings panel como overlay en 100% de los casos
**Actual:** `syncSettingsLayoutMode()` siempre aplica `settings-overlay` y nunca `settings-compact`.
**Problema:** El breakpoint de 620px nunca se alcanza porque la ventana tiene `maxWidth: 600`. El modo compacto existe en CSS pero nunca se usa.
**Propuesta:** Eliminar la lógica muerta de `settings-compact` o ajustar el breakpoint para que funcione dentro de 480px.

### P3 — El download list no se persiste (intencional pero confuso)
**Actual:** `scheduleSaveDownloads()` y `saveDownloads()` son no-ops intencionales. El README dice "Download list persists across restarts" pero el código lo contradice.
**Problema:** Discrepancia entre documentación y comportamiento real. Los usuarios pierden el historial al cerrar la app.
**Propuesta:** Decidir: ¿persistir o no? Si no, actualizar README. Si sí, activar la persistencia (las funciones `save/load_download_history` ya existen en Rust).

### P4 — Settings guide modal básico
**Actual:** Modal con texto plano explicando 3 settings.
**Problema:** Funcional pero visualmente básico. No hay iconos, no hay formato visual.
**Propuesta:** Mejorar con iconos inline y mejor separación visual. No es crítico.

### P5 — No hay feedback visual al copiar link
**Actual:** `copy-link` copia al clipboard pero no muestra ninguna confirmación visual.
**Problema:** El usuario no sabe si se copió. Patrón UX roto.
**Propuesta:** Toast/notification temporal "Link copied" (1-2 segundos).

### P6 — Alerts nativos para errores
**Actual:** `alert()` para errores de URL, folder picker fallback, cancel failures.
**Problema:** Los alerts nativos del navegador se ven feos e interrumpen el flujo. No encajan con el diseño premium de la app.
**Propuesta:** Reemplazar con inline messages o toast notifications dentro de la app.

### P7 — Confirm dialog nativo para bulk downloads y clear list
**Actual:** `window.confirm()` para playlists y clear all.
**Problema:** Mismo que P6 — se ve feo, rompe la experiencia nativa.
**Propuesta:** Modal custom in-app que siga el estilo de la app.

### P8 — Versión en el settings panel no se actualiza dinámicamente
**Actual:** Versión hardcodeada en HTML ("v1.1.0").
**Problema:** Hay que actualizarla manualmente en cada release.
**Propuesta:** Leer la versión desde `tauri.conf.json` via Tauri API (`app.getVersion()`).

---

## 3. Problemas de UX / Funcionalidad

### U1 — No hay drag & drop
**Problema:** El usuario no puede arrastrar un link desde el navegador a la app. Solo copy-paste.
**Propuesta:** Añadir drop zone en el input o en toda la ventana. Feature diferenciadora.

### U2 — No hay notificaciones de sistema
**Problema:** Cuando una descarga termina, no hay notificación nativa de macOS. Si el usuario está en otra app, no se entera.
**Propuesta:** Notificación nativa al completar descarga (Tauri notification plugin).

### U3 — No hay forma de abrir el archivo directamente
**Problema:** "Show in Finder" existe, pero no hay "Open file" para reproducir el vídeo directamente.
**Propuesta:** Añadir "Open" al context menu que abra el archivo con el player por defecto.

### U4 — Download location path muestra "~/Downloads" literal
**Problema:** El path no se expande visualmente. No está claro si es la carpeta real.
**Propuesta:** Mostrar path expandido o al menos indicar que `~` = home directory.

### U5 — No hay indicación de espacio en disco
**Problema:** Si el disco está lleno, la descarga falla sin aviso previo.
**Propuesta:** Mostrar espacio disponible en el directorio de descarga (nice-to-have, no crítico).

### U6 — Audio-only y auto-add siempre se resetean al abrir la app
**Actual:** Hardcoded a `false` en `loadSettings()`.
**Problema:** El usuario tiene que reactivar cada vez que abre la app.
**Propuesta:** Decidir si esto es intencional (seguridad) o un bug. Si intencional, documentar. Si no, persistir el estado.

### U7 — No hay batch download de URLs
**Problema:** Solo se puede pegar una URL a la vez.
**Propuesta:** Permitir pegar múltiples URLs separadas por nueva línea. La app las encola automáticamente. Quick win de alto impacto.

### U8 — No hay dark/light mode automático (follow system)
**Actual:** Toggle manual en settings.
**Propuesta:** Añadir opción "System" que siga la preferencia de macOS (`prefers-color-scheme`).

---

## 4. Problemas Técnicos

### T1 — CSP permite `img-src https:`
**Actual:** `img-src 'self' https: data:` — demasiado permisivo para imágenes.
**Propuesta:** Restringir a dominios específicos de thumbnails si es posible (YouTube, Vimeo, etc.). Bajo riesgo actual.

### T2 — Download history persistence desactivada
**Actual:** Las funciones `save/load_download_history` existen en Rust pero el frontend las ignora.
**Problema:** Feature muerta. Si se decide persistir, solo hay que conectar el frontend.

### T3 — Muchos console.log en producción
**Actual:** Logs de debug por toda la base de código.
**Propuesta:** Limpiar o condicionar a modo dev antes del lanzamiento.

### T4 — escapeHtml() crea un DOM element cada vez
**Actual:** `escapeHtml` crea un `div` temporal para escapar texto.
**Problema:** Ineficiente en loops largos de renderizado. No es un problema real con <100 items pero es un code smell.
**Propuesta:** Usar una función de escape basada en string replace.

---

## 5. Mejoras de Producto Propuestas

Ordenadas por impacto vs esfuerzo:

| # | Mejora | Impacto | Esfuerzo | Prioridad |
|---|--------|---------|----------|-----------|
| 1 | Empty state inspirador | Alto (primera impresión) | Bajo | **Must** |
| 2 | Toast notifications (reemplazar alerts) | Alto (profesionalismo) | Medio | **Must** |
| 3 | Copy link feedback visual | Alto (UX básica) | Bajo | **Must** |
| 4 | "Open file" en context menu | Medio (utilidad) | Bajo | **Must** |
| 5 | Versión dinámica desde Tauri API | Bajo (mantenimiento) | Bajo | **Must** |
| 6 | Decidir y resolver persistencia del historial | Alto (expectativa de usuario) | Bajo (ya implementado en Rust) | **Must** |
| 7 | Decidir y resolver reset de audio-only/auto-add | Medio (UX) | Bajo | **Should** |
| 8 | Drag & drop de URLs | Alto (diferenciación) | Medio | **Should** |
| 9 | Notificaciones nativas macOS | Alto (workflow) | Medio | **Should** |
| 10 | System theme (follow macOS) | Medio (polish) | Bajo | **Should** |
| 11 | Batch URL paste | Alto (power users) | Medio | **Could** |
| 12 | Limpiar console.logs | Bajo (calidad) | Bajo | **Could** |
| 13 | Custom modal (reemplazar confirm()) | Medio (polish) | Medio | **Could** |
| 14 | Eliminar lógica settings-compact muerta | Bajo (limpieza) | Bajo | **Could** |

---

## 6. Resumen

**Estado general:** La app es funcional y bien construida. El core download engine es sólido. Los problemas son principalmente de **polish** — cosas que hacen la diferencia entre "herramienta que funciona" y "producto que se siente profesional".

**Trabajo estimado para polish completo:**
- **Must-haves (6 items):** ~1-2 sesiones de trabajo
- **Should-haves (4 items):** ~1-2 sesiones adicionales
- **Could-haves (4 items):** ~1 sesión adicional

**No hay nada fundamentalmente roto.** El camino de "herramienta" a "producto" es más corto de lo que parece.
