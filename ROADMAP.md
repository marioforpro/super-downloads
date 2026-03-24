# Roadmap — Super Downloads

> Plan de ejecución por fases. Cada fase tiene criterios de completado claros.

---

## Phase 0: Foundation — COMPLETE

> Objetivo: Base documental, branding, y quick wins técnicos. Todo lo necesario antes de construir.

**Criterio de completado:** Identidad visual definida, docs estructurados, repo inicializado, quick wins técnicos resueltos.

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
- [x] Initial git commit con estado actual

---

## Phase 1: Product Polish — NOT STARTED

> Objetivo: Preparar la app para usuarios reales. Cerrar gaps de UX y añadir lógica freemium.

**Criterio de completado:** App lista para que un desconocido la descargue y la use sin ayuda.

- [ ] First-run onboarding / welcome screen
- [ ] Lógica freemium implementada (límites en free tier)
- [ ] Versión visible en la UI
- [ ] About / info screen
- [ ] Changelog visible
- [ ] Mejorar mensajes de error (user-friendly)
- [ ] Code signing + notarización (si se decide en Phase 0)
- [ ] Sistema de auto-update (Tauri updater plugin)
- [ ] Analytics básico (telemetría anónima opt-in)

**Requiere input del founder:** Definición exacta de límites free/pro, copy de onboarding.

---

## Phase 2: Landing Page — NOT STARTED

> Objetivo: Presencia web pública. Lugar donde enviar tráfico. Canal de descarga.

**Criterio de completado:** Web live con descarga directa, info del producto, y pricing.

**Stack recomendado:** Astro (sitio estático, deploy en Vercel)

- [ ] Diseño de landing page (hero, features, pricing, download, FAQ)
- [ ] Desarrollo de la landing
- [ ] Página de pricing (free vs pro)
- [ ] Botones de descarga (Apple Silicon + Intel)
- [ ] Legal: Terms of Service + Privacy Policy
- [ ] SEO básico (meta tags, OG image, sitemap)
- [ ] Analytics web (PostHog o similar)
- [ ] Favicon + assets

**Requiere input del founder:** Copy de producto, screenshots finales, dominio.

---

## Phase 3: Billing — NOT STARTED

> Objetivo: Monetización funcionando. El usuario puede pagar por Pro.

**Criterio de completado:** Un usuario puede comprar la licencia Pro y desbloquear features.

- [ ] Sistema de licencias (clave de activación o cuenta)
- [ ] Integración con Stripe (o Gumroad/Paddle/LemonSqueezy)
- [ ] Flujo de compra desde la app
- [ ] Flujo de compra desde la web
- [ ] Verificación de licencia en la app
- [ ] Restauración de licencia
- [ ] Email de confirmación de compra

**Requiere input del founder:** Plataforma de pago preferida, pricing exacto, flujo de activación.

---

## Phase 4: Pre-Launch — NOT STARTED

> Objetivo: Todo preparado para anunciar públicamente.

**Criterio de completado:** Lista de launch completada, todos los assets listos, canales preparados.

- [ ] Screenshots profesionales de la app
- [ ] Vídeo demo / GIF animado
- [ ] Assets para redes sociales
- [ ] Copy de lanzamiento (post para X, Instagram, etc.)
- [ ] Canal de soporte definido (email mínimo)
- [ ] FAQ / documentación de usuario
- [ ] Testeo con 3-5 beta testers
- [ ] Última ronda de bug fixing

**Requiere input del founder:** Revisión de materiales, aprobación de copy.

---

## Phase 5: Launch — NOT STARTED

> Objetivo: Producto público. Primeros usuarios reales.

- [ ] Publicar en web
- [ ] Anunciar en redes sociales
- [ ] Publicar en Product Hunt (cuando tenga sentido)
- [ ] Submits a directorios de apps Mac
- [ ] Submits a directorios de herramientas
- [ ] Monitorizar feedback y bugs

---

## Phase 6: Post-Launch — FUTURE

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
