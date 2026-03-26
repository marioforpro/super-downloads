# Siguiente Sesión — Guía de Acciones para el Founder

> Lee esto al empezar la próxima sesión de trabajo.
> Estas son las tareas que necesitan TU acción directa antes de seguir avanzando con Claude.

---

## Estado actual del proyecto

```
Phase 0: Foundation        — COMPLETE
Phase 1: App Polish        — COMPLETE
Phase 2: Product Infra     — COMPLETE (auto-updater done)
Phase 3: Landing Page      — NEAR COMPLETE (deployed, DNS propagating)
Phase 4: Billing           — NOT STARTED
Phase 5: Pre-Launch        — NOT STARTED
Phase 6: Launch            — NOT STARTED
```

**Lo que se hizo en Session 3 (2026-03-26):**
- Auto-updater: Ed25519 keys generated, tauri-plugin-updater configured
- GitHub repo created (`marioforpro/super-downloads`, private)
- Landing deployed to Vercel → `super-downloads.vercel.app`
- Domain `superdownloads.app` + `www` added to Vercel
- Hostinger: email verification completed, DNS configured, nameservers switched
- PostHog analytics integrated in landing
- OG/Twitter image meta tags added
- Clippy fix, cargo fmt, all checks pass

---

## TAREAS QUE NECESITAS HACER TÚ

### 1. Verificar DNS propagación

Espera unas horas y luego comprueba:
```bash
dig superdownloads.app +short
# Debe devolver: 76.76.21.21
```

O simplemente visita `https://superdownloads.app` en el navegador. Si ves tu landing, el DNS propagó.

Si después de 24h no funciona, contactar a Hostinger soporte — los nameservers se cambiaron correctamente pero la propagación puede requerir verificación adicional.

---

### 2. Probar la app (si no lo has hecho)

```bash
cd /Users/supermac/Desktop/DEV/SUPER-DOWNLOADS
npm run tauri dev
```

**Cosas que probar:**
- [ ] Empty state, onboarding, download counter
- [ ] Descargar un vídeo de YouTube, TikTok, Instagram
- [ ] Drag & drop un link desde el navegador
- [ ] Theme switching (Dark/Light/System)
- [ ] About screen (click versión en settings)

---

### 3. Crear cuenta en LemonSqueezy

1. Ve a [lemonsqueezy.com](https://lemonsqueezy.com) y crea una cuenta
2. Completa onboarding (datos fiscales, cuenta bancaria)
3. Crea un Store
4. Crea producto **"Super Downloads Pro"** (€29, one-time, license key, 3 activaciones)
5. Crea promo code **LAUNCH30** (30% off)
6. Anota: **API Key**, **Store ID**, **Product ID**, **Checkout URL**

---

### 4. Build nuevos DMGs

```bash
cd /Users/supermac/Desktop/DEV/SUPER-DOWNLOADS

# Apple Silicon
npm run tauri build -- --target aarch64-apple-darwin --bundles app

# Intel
npm run tauri build -- --target x86_64-apple-darwin --bundles app

# Crear DMGs
./create-dmg.sh
```

---

## QUÉ HACER CON CLAUDE EN LA SIGUIENTE SESIÓN

Con las tareas de arriba completadas:

1. **Confirmar DNS** — Verificar que `superdownloads.app` sirve la landing
2. **LemonSqueezy integration** (Phase 4) — License validation en la app, checkout URL en landing
3. **Actualizar landing** con URLs reales (DMGs + checkout)
4. **Build DMGs con updater** — Los nuevos DMGs incluirán el auto-updater
5. **OG image** — Crear imagen para social sharing
6. **Pre-launch prep** (Phase 5) — Screenshots, beta testing, copy

---

## TEMAS ESTRATÉGICOS PENDIENTES

### Platform Reliability & Monitoring
- Health checks automáticos para detectar cuando yt-dlp se rompe en cada plataforma
- Auto-update de yt-dlp sin requerir update de toda la app
- Comunicación clara al usuario cuando una plataforma está down
- **Decisión pendiente**: ¿dónde corren los health checks? ¿Script simple en launchd o servicio cloud?

### UI Polish — Inspiración Transmission
- Auto-resize de ventana según número de descargas
- Diseño más clean/compacto de la lista de descargas
- **Prerrequisito**: Probar la app actual y anotar qué te molesta visualmente

### Browser Extension (Phase 8 — Future)
- Extensión que envía URLs directamente a la app via deep link
- No prioritario hasta post-launch

---

## ARCHIVOS IMPORTANTES

| Archivo | Para qué |
|---------|----------|
| `ROADMAP.md` | Estado de cada fase |
| `PROGRESS.md` | Log detallado de lo hecho |
| `docs/DECISIONS.md` | Todas las decisiones tomadas |
| `docs/LAUNCH.md` | Checklist de lanzamiento |
| `docs/APP-AUDIT.md` | Auditoría de UX/diseño |
| `docs/BRAND.md` | Reglas de marca |

## CLAVES Y SECRETOS

| Archivo | Qué es | En git? |
|---------|--------|---------|
| `src-tauri/.tauri-update-key` | Clave privada del updater | NO (.gitignore) |
| `src-tauri/.tauri-update-key.pub` | Clave pública del updater | SÍ (committed) |
| `.env` (futuro) | API keys de LemonSqueezy | NO |
