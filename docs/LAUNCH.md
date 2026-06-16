# Launch Checklist — Super Downloads

> Everything needed before going public. Items are ordered by dependency.
> **This is the single source of truth for launch gates.** `LAUNCH-PLAN.md` is launch-day execution only (channels, copy, metrics).

> **Status (reconciled 2026-06-16):** PARKED 2026-06-15 — product shipped (v1.1.0 + v1.1.1), commercially un-launched.
> Gates 1–4 infra is largely DONE (shipped via v1.1.0/v1.1.1). The **one remaining launch blocker is the LemonSqueezy E2E verification** (Gate 4, marked ← OPEN GATE below; 6 checks in `NEXT-SESSION.md` step 4). Reopen on real signal, then run that gate.

---

## Gate 1: Product Ready

- [x] App icon profesional (no Tauri defaults)
- [x] Lógica freemium implementada y testeada (5 downloads/day free)
- [x] First-run onboarding funcional
- [x] Versión visible en la app
- [ ] About screen con info de contacto
- [x] Error handling pulido (mensajes user-friendly)
- [x] Build limpio sin warnings (verificado 2026-06-16: `cargo fmt/clippy -D warnings/test` ✅)
- [ ] Testado en macOS 13+ (Ventura, Sonoma, Sequoia)

## Gate 2: Distribution Ready

- [x] Code signing decidido e implementado — workaround Ed25519 documentado (no Apple signing)
- [x] Sistema de updates funcional — Tauri updater + one-click in-app auto-update (v1.1.1)
- [x] DMGs finales generados (Apple Silicon + Intel)
- [x] SHA256 checksums para los DMGs (en CHANGELOG.md)
- [ ] Instrucciones de instalación claras (Gatekeeper / unsigned)

## Gate 3: Web Ready

- [x] Dominio registrado y configurado (superdownloads.app + www, vía Vercel)
- [x] Landing page live
- [x] Página de pricing visible
- [x] Botones de descarga funcionales (→ GitHub Releases latest)
- [ ] Terms of Service publicados
- [ ] Privacy Policy publicada
- [ ] Disclaimer de uso responsable
- [ ] OG image + meta tags (en backlog ROADMAP)
- [x] Analytics web instalado (PostHog)

## Gate 4: Billing Ready

- [x] LemonSqueezy cuenta creada y producto configurado (UUID wired `src/main.js:99`)
- [x] Producto "Super Downloads Pro" — €29 lifetime
- [x] License key generation habilitado (3 activaciones por key)
- [ ] Promo code LAUNCH30 creado (30% off) — verificar en LS
- [ ] **Flujo de compra funcional (web → checkout → license key → app activation)** ← OPEN GATE
- [x] License validation implementada en la app (LemonSqueezy API)
- [ ] **Prueba de compra end-to-end (test mode)** ← OPEN GATE (el único blocker de lanzamiento)
- [ ] Email de confirmación de compra (lo gestiona LemonSqueezy — verificar en E2E)
- [x] Proceso de restauración de licencia en la app (activate/validate/deactivate)

## Gate 5: Communication Ready

- [ ] Canal de soporte definido (email mínimo)
- [ ] FAQ con preguntas comunes (instalación, Gatekeeper, plataformas)
- [ ] Al menos 1 red social configurada
- [ ] Copy de lanzamiento preparado
- [ ] Screenshots de la app (3-5 capturas)
- [ ] Vídeo/GIF demo (opcional pero recomendado)

## Gate 6: Launch

- [ ] Todos los gates anteriores completados
- [ ] 3-5 beta testers han probado la versión final
- [ ] Bugs críticos resueltos
- [ ] Publicar landing
- [ ] Anunciar en redes
- [ ] Monitorizar primeras horas

---

## Post-Launch Immediate (primera semana)

- [ ] Revisar analytics (web + app si hay)
- [ ] Responder a todo el feedback
- [ ] Hotfix si hay bugs críticos
- [ ] Documentar aprendizajes
- [ ] Evaluar siguientes pasos
