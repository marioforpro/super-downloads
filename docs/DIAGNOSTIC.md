# Diagnóstico — Super Downloads v1.1.0

> Auditoría completa del estado actual del producto. Fecha: 2026-03-24.

---

## 1. Estado Actual del Producto

### Lo que YA funciona (y funciona bien)

| Área | Estado | Detalle |
|------|--------|---------|
| Core download engine | Completo | yt-dlp + ffmpeg integrados, formato H.264/AAC/MP4 |
| Plataformas soportadas | 7 plataformas | YouTube, TikTok, X/Twitter, Vimeo, Instagram, Facebook, LinkedIn |
| Cola de descargas | Completo | Lista con progreso en tiempo real, velocidad, ETA |
| Thumbnails | Completo | Muestra preview del vídeo con metadata |
| Conversión H.264 | Completo | VideoToolbox (hardware acceleration) para compatibilidad Premiere Pro |
| Auto-retry con cookies | Completo | Contenido privado/age-restricted se reintenta automáticamente |
| Temas dark/light | Completo | CSS variables, toggle en settings |
| Modo MP3 | Completo | Extracción de audio solamente |
| Auto-add clipboard | Completo | Monitoriza clipboard para URLs de vídeo |
| Persistencia | Completo | Download history sobrevive a reinicios |
| Calidad configurable | Completo | Best / 1080p / 720p |
| Carpeta de destino | Completo | Selector de carpeta nativo |
| Build dual-arch | Completo | Apple Silicon + Intel DMGs |

**Veredicto:** La funcionalidad core está SÓLIDA. La app funciona, es útil, y resuelve un problema real.

### Nivel de Madurez

```
[██████████░░░░░░░░░░] ~50% — Producto funcional, 0% go-to-market
```

- **Producto/técnico:** ~85% listo para un v1 público
- **Go-to-market:** ~5% (solo existe un README técnico)
- **Branding:** ~10% (nombre + icono básico, sin sistema visual)
- **Marketing:** 0%
- **Web/landing:** 0%
- **Distribución:** ~30% (DMGs creados, pero sin firma ni canal)

---

## 2. Lo que FALTA para lanzar

### Bloqueo Crítico (sin esto NO se lanza)

| # | Gap | Impacto | Esfuerzo |
|---|-----|---------|----------|
| 1 | **Landing page** | No hay donde enviar a nadie. Sin web = sin producto público. | Alto |
| 2 | **Sistema de updates** | Los usuarios no pueden actualizar. Se quedan en v1.1 para siempre. | Alto |
| 3 | **Lógica freemium** | No hay distinción free/pro. Sin límites = sin monetización. | Alto |
| 4 | **Sistema de pago** | No hay forma de pagar. Sin checkout = sin revenue. | Alto |
| 5 | **Branding propio** | Iconos genéricos de Tauri. Sin identidad visual reconocible. | Medio |

### Importante (debería estar para v1)

| # | Gap | Impacto | Esfuerzo |
|---|-----|---------|----------|
| 6 | **Code signing** | Gatekeeper bloquea la app. UX de instalación muy friction. | Medio (requiere Apple Dev $99/año) |
| 7 | **Onboarding / first-run** | El usuario abre la app y no sabe qué hacer. No hay tutorial. | Bajo |
| 8 | **Error reporting** | Si la app crashea, no nos enteramos. | Bajo |
| 9 | **Analytics** | No sabemos cuántos usuarios hay ni cómo usan la app. | Medio |
| 10 | **Changelog / release notes** | Usuarios no saben qué cambió entre versiones. | Bajo |

### Deseable (puede esperar a v1.1+)

| # | Gap | Impacto | Esfuerzo |
|---|-----|---------|----------|
| 11 | Social media presence | Sin canales de comunicación con usuarios | Bajo |
| 12 | SEO de la landing | Tráfico orgánico a medio plazo | Medio |
| 13 | Soporte / FAQ | Auto-servicio para problemas comunes | Bajo |
| 14 | Blog / content | Autoridad y SEO | Alto |

---

## 3. Riesgos Identificados

### Riesgo Alto

1. **Legalidad de yt-dlp**: Descargar vídeos de plataformas puede tener implicaciones legales en algunos países. YouTube ToS lo prohíbe explícitamente. Esto NO bloquea el lanzamiento (hay decenas de apps similares), pero conviene:
   - No mencionar "YouTube" en la landing de forma prominente
   - Posicionar como "media downloader" genérico
   - Incluir disclaimer de uso responsable
   - Investigar precedentes legales

2. **Dependencia de yt-dlp**: Si yt-dlp deja de funcionar con alguna plataforma, la app se rompe. Mitigación: sistema de updates que incluya binarios actualizados.

3. **App sin firmar**: macOS cada vez restringe más las apps sin firmar. Cada versión de macOS añade más friction. A medio plazo, esto puede ser un dealbreaker.

### Riesgo Medio

4. **Competencia abundante**: Hay muchos video downloaders. Diferenciación necesaria.
5. **Sin telemetría**: No sabemos nada de los usuarios. Volar a ciegas post-lanzamiento.

---

## 4. Quick Wins (alto impacto, bajo esfuerzo)

| # | Quick Win | Tiempo estimado |
|---|-----------|----------------|
| 1 | Arreglar `authors` en Cargo.toml ("you" → nombre real) | 5 min |
| 2 | Añadir CSP básico en tauri.conf.json (security) | 15 min |
| 3 | Crear un icono de app profesional / distintivo | 1-2h |
| 4 | First-run welcome screen con instrucciones básicas | 2-3h |
| 5 | Añadir versión visible en la UI (footer o about) | 30 min |
| 6 | Crear un CHANGELOG.md | 30 min |
| 7 | Inicializar git repo | 10 min |
| 8 | Placeholder favicon + OG image para la landing | 1h |

---

## 5. Diferenciación Potencial

Lo que hace Super Downloads DIFERENTE de otros downloaders:

1. **Optimización para editores de vídeo** — H.264/Premiere Pro ready. Ningún otro downloader se posiciona así.
2. **Nativo macOS** — No es Electron, no es web. Tauri = ligero y rápido.
3. **Multi-plataforma en una sola app** — YouTube + TikTok + X + Vimeo + Instagram + Facebook + LinkedIn.
4. **Auto-add clipboard** — Workflow de zero-friction para editores.
5. **Hardware encoding** — VideoToolbox para conversiones instantáneas.

**Ángulo de posicionamiento recomendado:** "The video downloader built for editors."

---

## 6. Comparación con Super Prompts (madurez)

| Dimensión | Super Prompts | Super Downloads |
|-----------|---------------|-----------------|
| Producto core | Completo | Completo |
| Web/landing | Live | No existe |
| Branding | Sistema completo (BRAND.md) | Solo nombre |
| Documentación | 20+ docs estructurados | 3 docs técnicos |
| Analytics | PostHog + 12 eventos | Nada |
| Marketing | OS completo + automación | Nada |
| Billing | Stripe live | Nada |
| Roadmap | Fases claras, tracking | No existe |
| Social | X + Instagram activos | No existe |
| Legal | Terms + Privacy live | No existe |
| Distribución | Vercel (web) | DMG sin firmar |
| Updates | Vercel auto-deploy | Manual |
| Soporte | Help Scout + KB | No existe |

**Conclusión:** Super Downloads tiene el core técnico, pero le falta TODO el go-to-market que Super Prompts ya tiene. La buena noticia: ya sabes exactamente cómo construirlo.
