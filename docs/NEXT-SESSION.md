# Siguiente Sesion — Guia de Acciones para el Founder

> Abre este archivo → graba el demo → responde a Vishnu. Sin pensar.

---

## ⏸ PAUSED — 2026-04-27

> Founder switched focus to other work. Do NOT auto-resume R-SD-001 in next Mission Control. Resume only when founder explicitly says so.

### Resume entry point
Open this file. The checklist below is the resume plan — pick up at step 3.

### Checklist progress at pause

- [x] **1 · DNS verified for `superdownloads.app`** — verified 2026-04-28 with `dig superdownloads.app +short` returning `216.198.79.1`.
- [x] **2 · DMGs present locally** — verified 2026-04-28 in `dist/`: `SuperDownloads_v1.1_AppleSilicon.dmg` and `SuperDownloads_v1.1_Intel.dmg`.
- [ ] **3 · GitHub Release v1.1.0** — does not exist. Upload the existing local DMGs with release asset names the landing page expects:
  - `dist/SuperDownloads_v1.1_AppleSilicon.dmg#Super-Downloads_aarch64.dmg`
  - `dist/SuperDownloads_v1.1_Intel.dmg#Super-Downloads_x64.dmg`
- [ ] **4 · Verify LemonSqueezy product** — UUID `21db1cfb-37f8-4371-8085-b5e30f89645f` (already wired in both `src/main.js:99` and `web/src/pages/index.astro:184`). Confirm: product exists, **test mode active** (required for Scene 2 recording), €29 one-time, license key generation enabled (3 activations max), `LAUNCH30` promo code present, checkout URL loads.
- [ ] **5 · Record demo + reply to Vishnu** — per shot list below. Last step. Do not start until 1–4 are green.

### State changes already shipped (do NOT redo on resume)

- ✅ `src/main.js:99` — in-app `LEMONSQUEEZY_CHECKOUT_URL` wired to real product UUID (commit `cba5d29` in SUPER-DOWNLOADS repo, 2026-04-27). Was placeholder `/checkout/buy/TODO` before.
- ✅ Demo execution plan persisted (this file) — script intact below.
- ✅ Vishnu reply template ready (below).

### What's NOT pending (verified 2026-04-28)

- `superdownloads.app` DNS — resolves to Vercel (`216.198.79.1`).
- `www.superdownloads.app` DNS — resolves correctly through Vercel CNAME.
- DMGs — present locally in `dist/`.
- LemonSqueezy URL on landing page — already wired to real product UUID.

---

## 🎬 EXECUTE — R-SD-001 demo video (target 2026-04-30)

### Prerequisite (do first)

- [ ] Confirm LemonSqueezy product is in **test mode** (checkout functional, no real charge). If live-only, flip to test mode before recording.

### Shot list — 60–75s total

**Scene 1 · App UX (0:00–0:20) — 20s**
- `0:00` Open Super Downloads (clean empty state)
- `0:04` Paste YouTube URL into input
- `0:08` Click Download → progress bar fills
- `0:15` File appears in Finder → drag into Premiere Pro timeline

**Scene 2 · Fulfillment (0:20–1:00) — 40s** ← what LS actually cares about
- `0:20` In-app: click "Get Pro" (€29)
- `0:23` LemonSqueezy checkout opens (test mode) → fill card → confirm
- `0:32` Purchase success screen
- `0:35` Switch to Mail → LemonSqueezy license-key email (show key)
- `0:42` Back to Super Downloads → Settings → paste license key → Activate
- `0:52` "Pro · unlimited downloads" state visible
- `0:58` Quick close

**Scene 3 · Outro (1:00–1:10) — 10s**
- `1:00` Logo + superdownloads.app
- `1:05` End

### Upload

- [ ] Export MP4 → upload to YouTube (unlisted) or Loom
- [ ] Copy share URL

### Reply to Vishnu — thread `{#HS:3271250731-516790#}`

```
Hola Vishnu,

Adjunto los dos items que pediste:

1) Demo video (60-75s): [PASTE YOUTUBE URL]
   Muestra: descarga de un vídeo, integración con Premiere,
   checkout en LemonSqueezy y flow de activación de licencia.

2) Perfiles para verificación:
   - LinkedIn:  https://linkedin.com/in/mariomonteirotv
   - X:         https://x.com/mariomonteiro
   - Portfolio: https://mariomonteiro.tv

Quedamos a la espera del approval para activar la store en producción.

Gracias,
Mario
```

- [ ] Send reply
- [ ] Mark R-SD-001 as unblocked in `ROADMAP.md` once LS confirms approval

### Launch-order reminder

DMGs + GitHub Release can proceed in parallel — they don't depend on LS approval. Do NOT publicly announce launch until: (1) LS approved, (2) real €29 purchase verified end-to-end.

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

### 1. Verify DNS — Bare domain `superdownloads.app`

`www.superdownloads.app` works and `superdownloads.app` currently resolves to Vercel. Only re-open Hostinger if this check fails.

Verify:
```bash
dig superdownloads.app +short
# Should return: 216.198.79.1
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
