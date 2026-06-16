# Launch Plan: Super Downloads

> First product launch in the Super ecosystem.
> Based on Launch-Playbook.md template.
> **Launch-day execution only** (channels · copy · metrics). For launch readiness gates, `docs/LAUNCH.md` is the single source of truth.
> **Status: PARKED 2026-06-15** — reopen on real signal. v1.1.0 + v1.1.1 shipped; one open gate = LemonSqueezy E2E (see LAUNCH.md Gate 4).

---

## Product Summary

- **What**: macOS desktop app for media downloads (yt-dlp + ffmpeg)
- **Price**: €29 one-time, 3 device activations
- **Billing**: LemonSqueezy (wired, checkout URL in app + landing)
- **Landing**: superdownloads.app (Astro, deployed via Vercel)
- **Repo**: marioforpro/super-downloads

---

## Pre-Launch — Status

- [x] App built (Tauri v2, vanilla JS frontend)
- [x] Landing page live
- [x] LemonSqueezy product created (€29, 3 activations)
- [x] Checkout URL wired in main.js + index.astro
- [x] DNS bare domain fixed (A record → 216.198.79.1)
- [x] **Build DMGs** (Intel + Apple Silicon)
- [x] **Create GitHub Release** — v1.1.0 (2026-05-06) + v1.1.1 (2026-05-31), DMGs + SHA256 attached
- [ ] **Create LAUNCH30 promo code** in LemonSqueezy (30% off) — verify in LS
- [ ] Launch copy written
- [ ] Screenshots / demo GIF ready

---

## Launch Day Plan

- [ ] Reddit: r/macapps, r/commandline, r/selfhosted
- [ ] Hacker News: Show HN post
- [ ] X/Twitter: @marioforpro announcement
- [ ] LinkedIn: Post to professional network
- [ ] mariomonteiro.tv: Add to portfolio / projects section

**Launch copy draft** (to refine):
> Super Downloads — a native macOS app for downloading media from YouTube, Vimeo, and 1000+ sites. One-time purchase, no subscription. Built with Tauri.

---

## Post-Launch Tracking

| Metric | Target (Month 1) | Source |
|--------|-------------------|--------|
| Revenue | €100+ | LemonSqueezy |
| Downloads | 50+ | GitHub Release stats |
| Landing page views | 500+ | Vercel Analytics |
| Reddit upvotes | — | Manual check |
| Feedback signals | 5+ | Signals.md |

---

## Post-Launch Actions

- [ ] Monitor Signals.md daily (week 1)
- [ ] Respond to all feedback within 24h
- [ ] Identify top bug / feature request
- [ ] Revenue update → `02_Finance/Income-Analysis.md`
- [ ] Decide: iterate on Downloads or shift focus to Super Prompts PH launch

---

## Related

- `01_Projects/SUPER-DOWNLOADS/HEALTH.md` — Project health
- `08_Growth/Channels.md` — Distribution channels
- `08_Growth/Signals.md` — Feedback capture
- `02_Finance/Income-Analysis.md` — Revenue tracking
