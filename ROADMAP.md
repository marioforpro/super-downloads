# Roadmap — Super Downloads

> Plan de ejecución por fases. Cada fase tiene criterios de completado claros.

---

## Phase 0: Foundation — COMPLETE

> Objetivo: Base documental, branding, y quick wins técnicos. Todo lo necesario antes de construir.

- [x] Inicializar git repo
- [x] Definir identidad visual (icono confirmado, paleta azul definida)
- [x] Crear BRAND.md con reglas de marca
- [x] Fix Cargo.toml author
- [x] Dominio decidido: `superdownloads.app`
- [x] Definir estructura freemium (5/día free, ilimitado pro, €29 lifetime)
- [x] Decidir code signing (diferido, Tauri updater no lo requiere)
- [x] Decidir plataforma de pago (LemonSqueezy)
- [x] Crear documentación completa (DIAGNOSTIC, DECISIONS, LAUNCH, MARKETING, CHANGELOG)
- [x] Fix CSP en tauri.conf.json
- [x] Añadir versión visible en la UI (settings panel)
- [x] Registrar dominio superdownloads.app (Hostinger)
- [x] Registrar email support@superdownloads.app
- [x] Initial git commit con estado actual

---

## Phase 1: App Polish — COMPLETE

> Objetivo: Convertir la herramienta en un producto pulido. Diseño, UX, y funcionalidad.
> Referencia: `docs/APP-AUDIT.md` para detalle completo de cada item.

**Criterio de completado:** La app se siente profesional. Un desconocido puede usarla sin ayuda.

### Must-haves (bloquean el lanzamiento)

- [x] **Empty state inspirador** — Icono + hint + plataformas soportadas
- [x] **Toast notifications** — Reemplazados todos los `alert()` con toasts in-app
- [x] **Copy link feedback** — Toast "Link copied" al copiar
- [x] **"Open file" en context menu** — Abre vídeo con player por defecto
- [x] **Versión dinámica** — Lee versión desde Tauri API (`app.getVersion()`)
- [x] **Persistencia del historial** — Toggle "Keep download history" en settings (OFF por defecto)
- [x] **Audio-only/auto-add reset** — Confirmado intencional, documentado

### Should-haves (mejoran significativamente la experiencia)

- [x] **Drag & drop de URLs** — Drop zone en toda la ventana, soporte multi-URL
- [x] **Notificaciones nativas macOS** — Notificación al terminar (solo cuando app no tiene foco)
- [x] **System theme** — Selector Dark/Light/System con listener para cambios de OS
- [x] **Limpiar console.logs** — Eliminados 12 debug logs, mantenidos warn/error

### Could-haves (polish extra)

- [x] **Custom confirm modal** — Reemplazados ambos `window.confirm()` con modal in-app
- [x] **Eliminar settings-compact** — ~180 líneas de CSS muerto eliminadas
- [ ] Batch URL paste (múltiples URLs de una vez) — deferred to post-launch

**Requiere input del founder:** Decisión sobre persistencia del historial, decisión sobre reset de settings.

---

## Phase 2: Product Infrastructure — IN PROGRESS

> Objetivo: Añadir la lógica de negocio y sistemas de distribución sobre el producto pulido.

**Criterio de completado:** App con freemium funcional, auto-updater, y onboarding.

- [x] Lógica freemium (contador de 5 descargas/día, persistente, reset midnight)
- [x] UI de límite alcanzado (toast + counter en top bar)
- [x] Download counter visible (X/5 badge, color-coded)
- [x] Pro user detection (license key in localStorage)
- [ ] Sistema de auto-update (Tauri updater plugin — keys need interactive generation)
- [x] First-run onboarding / welcome screen (icon, features, free limit note)
- [x] About screen (version, email, website — click version label)
- [ ] Analytics básico (telemetría anónima opt-in) — deferred to post-launch

---

## Phase 3: Landing Page — IN PROGRESS

> Objetivo: Presencia web pública. Lugar donde enviar tráfico. Canal de descarga.

**Criterio de completado:** Web live con descarga directa, info del producto, y pricing.

**Stack:** Astro v6 (sitio estático en `web/`, deploy en Vercel)

- [x] Diseño de landing page (hero, features, pricing, download, FAQ)
- [x] Desarrollo de la landing (index.astro, responsive)
- [x] Página de pricing integrada en landing (free vs pro, €29 lifetime, LAUNCH30)
- [x] Sección de descarga (Apple Silicon + Intel, instrucciones de instalación)
- [x] Legal: Terms of Service + Privacy Policy (pages completas)
- [x] SEO básico (meta tags, OG tags, description)
- [x] Favicon SVG (icono de la app)
- [ ] Conectar URLs de descarga a DMGs reales
- [ ] OG image (screenshot o diseño)
- [ ] Configurar Vercel deployment
- [ ] Analytics web (PostHog o Plausible)
- [ ] Apuntar dominio superdownloads.app a Vercel

---

## Phase 4: Billing — NOT STARTED

> Objetivo: Monetización funcionando. El usuario puede pagar por Pro.

**Criterio de completado:** Un usuario puede comprar la licencia Pro y desbloquear features.

- [ ] LemonSqueezy: cuenta, producto "Super Downloads Pro" (€29 lifetime)
- [ ] License key generation (3 activaciones por key)
- [ ] Promo code LAUNCH30 (30% off)
- [ ] License validation en la app (LemonSqueezy API)
- [ ] Flujo: web checkout → license key → app activation
- [ ] Restauración de licencia
- [ ] Email de confirmación de compra

---

## Phase 5: Pre-Launch — NOT STARTED

> Objetivo: Todo preparado para anunciar públicamente.

**Criterio de completado:** Launch checklist completada (ver `docs/LAUNCH.md`).

- [ ] Screenshots profesionales de la app (3-5 capturas)
- [ ] Vídeo demo / GIF animado
- [ ] Assets para redes sociales
- [ ] Copy de lanzamiento
- [ ] Canal de soporte (support@superdownloads.app)
- [ ] FAQ / documentación de usuario
- [ ] Testeo con 3-5 beta testers
- [ ] Última ronda de bug fixing

---

## Phase 6: Launch — NOT STARTED

> Objetivo: Producto público. Primeros usuarios reales.

- [ ] Publicar web en superdownloads.app
- [ ] Anunciar en redes sociales
- [ ] Submits a directorios de apps Mac (MacUpdate, AlternativeTo, etc.)
- [ ] Publicar en Product Hunt (cuando tenga sentido)
- [ ] Monitorizar feedback y bugs

---

## Phase 7: Post-Launch — FUTURE

> Iteración basada en datos reales.

- [ ] Análisis de métricas (descargas, activaciones, conversiones)
- [ ] Iterar basado en feedback
- [ ] Más plataformas de descarga
- [ ] Features avanzadas (batch download, subtítulos, etc.)
- [ ] Posible expansión a Windows/Linux

---

## Parallel Track: Social Presence

> Se puede iniciar en cualquier momento. No bloquea ninguna fase.

- [ ] Cuentas creadas (X, Instagram como mínimo)
- [ ] Bio y avatar configurados
- [ ] Primeros posts de producto
- [ ] Estrategia de contenido definida
