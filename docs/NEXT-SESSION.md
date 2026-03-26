# Siguiente Sesion — Guia de Acciones para el Founder

> Lee esto al empezar la proxima sesion de trabajo.
> Estas son las tareas que necesitan TU accion directa antes de seguir avanzando con Claude.

---

## Estado actual del proyecto

```
Phase 0: Foundation        — COMPLETE
Phase 1: App Polish        — COMPLETE
Phase 2: Product Infra     — COMPLETE
Phase 3: Landing Page      — NEAR COMPLETE (www works, bare domain DNS pending)
Phase 4: Billing           — IN PROGRESS (app-side done, needs LemonSqueezy account)
Phase 5: Pre-Launch        — NOT STARTED
Phase 6: Launch            — NOT STARTED
```

**Lo que se hizo en Session 4 (2026-03-26):**
- LemonSqueezy license validation integrated in Rust backend (activate/validate/deactivate)
- License UI in settings panel (activate key, deactivate, upgrade link)
- Landing page download buttons wired to GitHub Releases URLs
- Landing page "Get Pro" button wired to LemonSqueezy checkout (placeholder URL)
- All checks pass: frontend + fmt + clippy + tests

---

## TAREAS QUE NECESITAS HACER TU

### 1. Fix DNS — Bare domain `superdownloads.app`

`www.superdownloads.app` works, but `superdownloads.app` (bare domain) does not resolve.

**In Hostinger hPanel:**
1. Go to Domain → DNS Zone for `superdownloads.app`
2. Add or edit the **A record** for `@` → `76.76.21.21`
3. Make sure there's no conflicting A record (parking IP)
4. Wait 5-30 min for propagation

Verify:
```bash
dig superdownloads.app +short
# Should return: 76.76.21.21
```

---

### 2. Create LemonSqueezy account + product

This is **required** to complete Phase 4.

1. Go to [lemonsqueezy.com](https://lemonsqueezy.com) and create account
2. Complete onboarding (tax info, bank account)
3. Create a **Store** (e.g., "Super Downloads")
4. Create product: **"Super Downloads Pro"**
   - Price: **€29** one-time
   - License key: **enabled**, 3 activations max
5. Create promo code: **LAUNCH30** (30% off)
6. Note down these values:
   - **Checkout URL** (from the product page → Share → Direct link)

---

### 3. Update checkout URL in code

Once you have the LemonSqueezy checkout URL, replace the placeholder in two places:

**File 1:** `src/main.js` (line ~100)
```
const LEMONSQUEEZY_CHECKOUT_URL = "https://superdownloads.lemonsqueezy.com/checkout/buy/TODO";
```
Replace `TODO` with your actual product checkout path.

**File 2:** `web/src/pages/index.astro` (pricing section)
```
href="https://superdownloads.lemonsqueezy.com/checkout/buy/TODO"
```
Same replacement.

---

### 4. Create GitHub Release + Upload DMGs

Build the DMGs and create a GitHub release so download buttons work:

```bash
cd /Users/supermac/Desktop/DEV/SUPER-DOWNLOADS

# Apple Silicon
npm run tauri build -- --target aarch64-apple-darwin --bundles dmg

# Intel
npm run tauri build -- --target x86_64-apple-darwin --bundles dmg
```

Then create a release on GitHub:
```bash
# Find the DMG files
find src-tauri/target -name "*.dmg" 2>/dev/null

# Create release and upload (adjust filenames)
gh release create v1.1.0 --title "Super Downloads v1.1.0" --notes "Initial release" \
  path/to/Super-Downloads_aarch64.dmg \
  path/to/Super-Downloads_x64.dmg
```

The landing page download buttons already point to:
- `https://github.com/marioforpro/super-downloads/releases/latest/download/Super-Downloads_aarch64.dmg`
- `https://github.com/marioforpro/super-downloads/releases/latest/download/Super-Downloads_x64.dmg`

Make sure the uploaded filenames match exactly.

---

## QUE HACER CON CLAUDE EN LA SIGUIENTE SESION

1. **Verify DNS** — Confirm bare domain works
2. **Plug in LemonSqueezy URL** — Quick find/replace once you have it
3. **Test license flow** — Activate a test license in the app
4. **OG image** — Create social sharing image for landing
5. **Pre-launch prep** (Phase 5) — Screenshots, beta testing, copy

---

## ARCHIVOS IMPORTANTES

| Archivo | Para que |
|---------|----------|
| `ROADMAP.md` | Estado de cada fase |
| `PROGRESS.md` | Log detallado de lo hecho |
| `docs/DECISIONS.md` | Todas las decisiones tomadas |
| `docs/LAUNCH.md` | Checklist de lanzamiento |

## CLAVES Y SECRETOS

| Archivo | Que es | En git? |
|---------|--------|---------|
| `src-tauri/.tauri-update-key` | Clave privada del updater | NO (.gitignore) |
| `src-tauri/.tauri-update-key.pub` | Clave publica del updater | SI (committed) |
| `.env` (futuro) | API keys de LemonSqueezy | NO |
