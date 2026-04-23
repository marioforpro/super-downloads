# ROADMAP — Super Downloads

---
scope: project:SUPER-DOWNLOADS
prefix: SD
lifecycle: pre-launch
current_phase: Phase 4 — Billing (blocked on Mario)
next_milestone: launch on LemonSqueezy
updated: 2026-04-23
---

> Forward-motion view for Super Downloads at LifeOS level.
> Schema contract: `docs/protocols/ROADMAP-SCHEMA.md`.
> Phase 0–3 history preserved below and in git. Day-to-day code-level detail (per-phase checkbox lists) is NOT tracked here any more — that was task-manager territory. This file tracks launch-level forward motion only.

## Roadmap

- **R-SD-001** · P0 · blocked · Submit Super Downloads to LemonSqueezy for approval
  - blocker: demo video not recorded + account setup pending Mario (since 2026-03-28)
  - next: record 30–60s screen capture per `docs/LAUNCH.md`, then create LemonSqueezy product (€29 lifetime) + LAUNCH30 promo + license-key issuance rules
  - target: 2026-04-30
  - source: docs/LAUNCH.md, docs/NEXT-SESSION.md
  - moved: 2026-03-28
- **R-SD-002** · P1 · next · Publish launch announcement (Reddit r/macapps + HN Show HN)
  - next: draft copy once R-SD-001 clears + LemonSqueezy checkout URL is live
  - depends_on: R-SD-001
  - source: docs/MARKETING.md
  - moved: 2026-04-23

## Shipped (last 30d)

_(no shipped roadmap items this window — open items all blocked on R-SD-001)_

## Backlog (unranked)

- OG image asset (screenshot or custom design)
- DNS propagation + HTTPS verified on bare `superdownloads.app` domain
- Screenshots / assets / demo GIF for launch day polish (distinct from the demo video under R-SD-001)
- Batch URL paste feature
- Analytics telemetry opt-in (from Phase 2 deferred list)
- Windows/Linux expansion
- Browser extension (was Phase 8 in prior roadmap — legitimately future work)
- MacUpdate / AlternativeTo directory submissions
- Product Hunt launch (timing TBD)

## Prior Phases — history (one-line summary)

Earlier phase-oriented roadmap (Phases 0–8) is preserved in git history of this file. High-level status at 2026-04-23:

- **Phase 0: Foundation** — COMPLETE (branding, domain, payment choice, docs).
- **Phase 1: App Polish** — COMPLETE (empty state, toasts, drag-and-drop, native notifications, theme).
- **Phase 2: Product Infrastructure** — COMPLETE (freemium counter, auto-updater, onboarding, license UI).
- **Phase 3: Landing Page** — NEAR COMPLETE (live at `www.superdownloads.app` via Vercel; PostHog integrated; bare-domain DNS pending).
- **Phase 4: Billing** — IN PROGRESS → tracked as **R-SD-001** above.
- **Phase 5: Pre-Launch** — NOT STARTED → rolled into **R-SD-001** (demo video is the gating asset) and backlog (screenshots, copy).
- **Phase 6: Launch** — NOT STARTED → tracked as **R-SD-002** and backlog (directory submits).
- **Phase 7: Post-Launch** — future work, not in scope until R-SD-001 clears.
- **Phase 8: Browser Extension** — future, backlog.

Full phase-level detail (with per-task checkboxes) can be retrieved with `git show HEAD~:01_Projects/SUPER-DOWNLOADS/ROADMAP.md` until a formal `PROGRESS.md` is created.
