# ROADMAP — Super Downloads

---
scope: project:SUPER-DOWNLOADS
prefix: SD
lifecycle: pre-launch
current_phase: Phase 4 — Billing (resumed 2026-05-06; v1.1.0 published, LemonSqueezy E2E verification next)
next_milestone: LemonSqueezy E2E verification (checkout · payment · license · activation · limits · LAUNCH30)
updated: 2026-05-06
---

> Forward-motion view for Super Downloads at LifeOS level.
> Schema contract: `docs/protocols/ROADMAP-SCHEMA.md`.
> Phase 0–3 history preserved below and in git. Day-to-day code-level detail (per-phase checkbox lists) is NOT tracked here any more — that was task-manager territory. This file tracks launch-level forward motion only.

## Roadmap

- **R-SD-001** · P0 · active · Submit Super Downloads to LemonSqueezy for approval
  - progress: v1.1.0 published 2026-05-06 (3/5 checklist items shipped — DNS verified · fresh DMGs from HEAD `cfbc320` · GitHub Release reachable anonymously after repo flipped public). Pre-publish caught a stale-artifact false invariant (DMGs in `dist/` predated LS-URL fix `cba5d29` by 7 weeks); resolved by `scripts/check-release-artifacts.sh` build-provenance gate.
  - next: LemonSqueezy E2E verification (six gates per `docs/NEXT-SESSION.md` step 4: checkout loads · test-mode payment · license generated · in-app activation · free/pro limit behavior · LAUNCH30 promo). On pass, product is commercially operational; demo recording (step 5) follows.
  - target: re-baseline at LS-verify session start
  - source: docs/LAUNCH.md, docs/NEXT-SESSION.md, docs/OPS.md
  - moved: 2026-05-06
- **R-SD-002** · P1 · next · Publish launch announcement (Reddit r/macapps + HN Show HN)
  - next: draft copy once R-SD-001 clears + LemonSqueezy checkout URL is live
  - depends_on: R-SD-001
  - source: docs/MARKETING.md
  - moved: 2026-04-23

## Shipped (last 30d)

_(no shipped roadmap items this window — R-SD-001 is paused by founder direction)_

## Backlog (unranked)

- OG image asset (screenshot or custom design)
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
- **Phase 3: Landing Page** — COMPLETE (live at `superdownloads.app` and `www.superdownloads.app` via Vercel; PostHog integrated).
- **Phase 4: Billing** — PAUSED → tracked as **R-SD-001** above.
- **Phase 5: Pre-Launch** — NOT STARTED → rolled into **R-SD-001** (demo video is the gating asset) and backlog (screenshots, copy).
- **Phase 6: Launch** — NOT STARTED → tracked as **R-SD-002** and backlog (directory submits).
- **Phase 7: Post-Launch** — future work, not in scope until R-SD-001 clears.
- **Phase 8: Browser Extension** — future, backlog.

Full phase-level detail (with per-task checkboxes) can be retrieved with `git show HEAD~:01_Projects/SUPER-DOWNLOADS/ROADMAP.md` until a formal `PROGRESS.md` is created.
