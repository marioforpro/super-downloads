# Siguiente Sesion — Guia de Acciones para el Founder

> **REOPENED 2026-07-16** — recommit explícito del founder. Plan activo: relaunch hardening (reliability + legal + payments), spec en `superpowers/specs/2026-07-16-relaunch-hardening-design.md`, orden de ejecución B → A → C. Estuvo PARKED 2026-06-15 (producto enviado v1.1.1, sin lanzar comercialmente). La verificación E2E de LemonSqueezy (paso 4 abajo) sigue vigente como **Track C2** del plan.

---

## ✅ v1.1.0 SHIPPED — 2026-05-06 (Session 161)

GitHub Release: https://github.com/marioforpro/super-downloads/releases/tag/v1.1.0
Anonymous download URLs verified HTTP 200 (`releases/latest/download/Super-Downloads_<arch>.dmg`).
Repo flipped private → public 2026-05-06; landing-page download buttons now reach end users.

### Resume entry point
Open this file. The checklist below is the resume plan — pick up at step 4.

### Checklist progress

- [x] **1 · DNS verified for `superdownloads.app`** — `dig` returns `216.198.79.1`.
- [x] **2 · DMGs present locally** — original 2026-04-28 check verified file presence only; the DMGs in `dist/` predated the LS-URL fix `cba5d29` by 7 weeks (build mtime 2026-03-05 vs fix 2026-04-27). False invariant caught Session 161 pre-publish; stale DMGs quarantined to `dist/_stale/`. Now enforced via `scripts/check-release-artifacts.sh` (build-provenance gate, see `docs/OPS.md` §Release verification).
- [x] **3 · GitHub Release v1.1.0** — published 2026-05-06 from HEAD `cfbc320` (post `cba5d29`). Both assets uploaded:
  - `Super-Downloads_aarch64.dmg` — sha256 `ea0df1b2edee5161e8bb177ed93d4a29edeeaa6e2430974f06a374c8f86d4419`
  - `Super-Downloads_x64.dmg`     — sha256 `768beac78921be8a6e75d6ebab2534d6c31a64ea3a67ab56c2a589ef839e4216`
- [ ] **4 · LemonSqueezy E2E verification** — strict end-to-end. Product UUID `21db1cfb-37f8-4371-8085-b5e30f89645f` (wired in `src/main.js:99` + `web/src/pages/index.astro:184`). All six gates must pass before R-SD-001 closes:
  1. Checkout URL loads in browser (test mode active)
  2. Test-mode payment succeeds with a Stripe test card (€29)
  3. License key generated and emailed by LemonSqueezy
  4. App accepts the license key in Settings → Activate (RPC round-trip succeeds against `api.lemonsqueezy.com/v1/licenses/activate`)
  5. Free-tier limit (5 downloads/day) enforced before activation; unlimited downloads after activation
  6. `LAUNCH30` promo code applies a 30% discount at checkout
  At step-4 close: product is **commercially operational**, even before the demo/video layer.
- [ ] **5 · Record demo + reply to Vishnu** — per shot list below. Do not start until step 4 is fully green.

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
