# Siguiente Sesión — Guía de Acciones para el Founder

> Lee esto al empezar la próxima sesión de trabajo.
> Estas son las tareas que necesitan TU acción directa antes de seguir avanzando con Claude.

---

## Estado actual del proyecto

```
Phase 0: Foundation        — COMPLETE
Phase 1: App Polish        — COMPLETE (13/14 items)
Phase 2: Product Infra     — NEAR COMPLETE (falta auto-updater)
Phase 3: Landing Page      — IN PROGRESS (código hecho, falta deploy)
Phase 4: Billing           — NOT STARTED
Phase 5: Pre-Launch        — NOT STARTED
Phase 6: Launch            — NOT STARTED
```

**Lo que se hizo en Session 1 (2026-03-24):**
- 8 commits, ~7,000 líneas de código
- Auditoría completa + plan estratégico + documentación
- App polish: toast system, drag & drop, notificaciones macOS, system theme, freemium, onboarding, about screen
- Landing page Astro: hero, features, pricing, download, FAQ, legal
- Investigación: dominios, Tauri updater, LemonSqueezy, 4K Download pricing

---

## TAREAS QUE NECESITAS HACER TÚ

### 1. Probar la app con todos los cambios

```bash
cd /Users/supermac/Desktop/DEV/SUPER-DOWNLOADS
npm run tauri dev
```

**Cosas que probar:**
- [ ] Empty state (abrir app sin descargas)
- [ ] Onboarding (borrar localStorage para verlo otra vez, o abrir en perfil limpio)
- [ ] Descargar un vídeo — verificar que el toast funciona
- [ ] Verificar el counter de descargas (X/5 en el top bar)
- [ ] Copiar un link — verificar toast "Link copied"
- [ ] Click derecho en descarga completada → "Open File"
- [ ] Drag & drop un link desde el navegador
- [ ] Cambiar theme a System y verificar que sigue macOS
- [ ] Click en la versión (settings) → About screen
- [ ] Probar "Keep download history" toggle

**Anotar**: cualquier cosa que no te guste, bugs, o mejoras que quieras.

---

### 2. Generar las claves del auto-updater

El auto-updater de Tauri necesita un par de claves (Ed25519). La generación es interactiva (pide contraseña).

```bash
cd /Users/supermac/Desktop/DEV/SUPER-DOWNLOADS
npx tauri signer generate -w src-tauri/.tauri-update-key
```

- Te pedirá una contraseña — puedes dejarla vacía (Enter) o poner una.
- Genera dos archivos: la clave privada (`.tauri-update-key`) y la pública (`.tauri-update-key.pub`).
- **IMPORTANTE**: Añadir `.tauri-update-key` al `.gitignore` (es secreta).
- La clave pública se usa en `tauri.conf.json` para verificar updates.

---

### 3. Build nuevos DMGs

Con todos los cambios de la app, necesitas builds frescos:

```bash
cd /Users/supermac/Desktop/DEV/SUPER-DOWNLOADS

# Apple Silicon
npm run tauri build -- --target aarch64-apple-darwin --bundles app

# Intel
npm run tauri build -- --target x86_64-apple-darwin --bundles app

# Crear DMGs
./create-dmg.sh
```

Los DMGs se generan en `dist/`. Estos son los que se enlazarán en la landing page.

---

### 4. Deploy de la landing page en Vercel

**Opción A: Desde la web de Vercel**
1. Ve a [vercel.com](https://vercel.com) y logéate
2. Import the git repository (o conecta el directorio)
3. **Root Directory:** `web` (importante — no la raíz del proyecto)
4. **Framework Preset:** Astro (Vercel lo detecta automáticamente)
5. Deploy

**Opción B: Desde el CLI**
```bash
cd /Users/supermac/Desktop/DEV/SUPER-DOWNLOADS/web
npx vercel
# Seguir los prompts, seleccionar "web" como root
```

---

### 5. Apuntar el dominio a Vercel

En **Hostinger** (donde registraste superdownloads.app):

1. Ve a la configuración DNS de superdownloads.app
2. Añade un registro CNAME:
   - **Nombre:** `@` (o vacío para el dominio raíz)
   - **Valor:** `cname.vercel-dns.com`
3. O usa los registros A que Vercel te indica en el dashboard del proyecto

Luego en **Vercel**:
1. Ve a Settings → Domains del proyecto
2. Añade `superdownloads.app`
3. Vercel verificará el DNS automáticamente

---

### 6. Crear cuenta en LemonSqueezy

1. Ve a [lemonsqueezy.com](https://lemonsqueezy.com) y crea una cuenta
2. Completa el onboarding (datos fiscales, cuenta bancaria para payouts)
3. Crea un Store
4. Crea un Producto:
   - **Nombre:** Super Downloads Pro
   - **Precio:** €29 (one-time)
   - **Tipo:** License key
   - **Activaciones por key:** 3
   - **Variante:** Lifetime
5. Crea un Promotion Code:
   - **Código:** LAUNCH30
   - **Descuento:** 30%
   - **Tipo:** First payment
6. Anota:
   - **API Key** (Settings → API)
   - **Store ID**
   - **Product ID**
   - **Checkout URL** (para enlazar desde la landing y la app)

---

### 7. Actualizar URLs en la landing

Una vez tengas los DMGs y el checkout de LemonSqueezy, necesitarás actualizar en `web/src/pages/index.astro`:

- Los `href="#"` de los botones de descarga → URLs reales de los DMGs
- El botón "Get Pro" → URL de checkout de LemonSqueezy

Esto lo puedes hacer tú o pedirle a Claude en la siguiente sesión.

---

## QUÉ HACER CON CLAUDE EN LA SIGUIENTE SESIÓN

Una vez hayas completado las tareas de arriba, la siguiente sesión con Claude debería:

1. **Integrar LemonSqueezy** en la app (license validation, activation flow)
2. **Configurar Tauri updater** con las claves generadas
3. **Actualizar landing** con URLs reales (DMGs + checkout)
4. **Tomar screenshots** de la app para OG image y marketing
5. **Preparar pre-launch** (assets, beta testing, copy de lanzamiento)

---

## ARCHIVOS IMPORTANTES

| Archivo | Para qué |
|---------|----------|
| `ROADMAP.md` | Estado de cada fase |
| `PROGRESS.md` | Log detallado de lo hecho |
| `docs/DECISIONS.md` | Todas las decisiones tomadas |
| `docs/LAUNCH.md` | Checklist de lanzamiento (6 gates) |
| `docs/APP-AUDIT.md` | Auditoría de UX/diseño |
| `docs/BRAND.md` | Reglas de marca |
| `docs/MARKETING.md` | Plan de marketing |
| `CHANGELOG.md` | Historial de versiones |
