# Progress — Super Downloads

> Implementation log. One entry per session.

---

## Session 1 — 2026-03-24 — Project Init & Strategic Audit

**What happened:**
- Full audit of Super Downloads current state
- Studied Super Prompts as reference for transferable patterns
- Created complete documentation structure:
  - CLAUDE.md (session bootstrap)
  - ROADMAP.md (6 phases + parallel track)
  - BRAND.md (brand operating system)
  - DECISIONS.md (4 decisions logged)
  - DIAGNOSTIC.md (full product audit)
  - LAUNCH.md (launch checklist with 6 gates)
  - MARKETING.md (positioning + channels + calendar)
  - PROGRESS.md (this file)
- Identified 5 critical gaps, 4 important gaps, 3 desirable gaps
- Defined freemium model as commercial strategy
- Recommended Astro for landing page
- Recommended "The video downloader built for editors" as positioning
- Saved memory context for future sessions

**Key decisions made (round 1):**
- Freemium model confirmed
- Brand independence from Super Prompts confirmed
- Documentation structure adopted from Super Prompts patterns

**Key decisions made (round 2 — same session):**
- Tagline locked: "The video downloader built for editors."
- Freemium: 5 downloads/day free, unlimited Pro
- Blue identity confirmed (existing icon palette)
- Code signing deferred (Tauri updater works independently — no blocker)
- Astro confirmed for landing (founder delegated)
- LemonSqueezy recommended for payments (founder delegated)
- Git repo initialized
- Cargo.toml author fixed
- CHANGELOG.md created
- .gitignore already existed (good)

**Research completed:**
- Domain availability: 8+ domains available. Top pick: superdownloads.app
- Tauri v2 updater: Uses own Ed25519 signing, NOT Apple signing. Auto-updates work without $99/year.
- Payment platforms: LemonSqueezy recommended (built-in license keys, MoR, indie-friendly)
- Naming conflicts: SuperDownloads.com.br (Brazilian portal) exists but different category

**Pending founder decisions:**
- Domain selection (top 3 options presented in DECISIONS.md)
- LemonSqueezy confirmation
- Pricing for Pro tier ($/month, $/year)

**Quick wins completed:**
- [x] Fix Cargo.toml author ("you" → "Mario Monteiro")
- [x] Init git repo
- [x] Create CHANGELOG.md
- [x] Add CSP in tauri.conf.json (was null — now restricts to self + https images)
- [x] Add version label in UI (v1.1.0 in settings panel)

**All Phase 0 decisions now closed:**
- Domain: superdownloads.app
- Payment: LemonSqueezy (confirmed)
- Pricing: €29 one-time lifetime, launch promo LAUNCH30 (30% off)
- Competitive research: 4K Download analyzed (Personal €30.25, Pro €48.40)

**Phase 0 remaining:**
- [ ] Register domain superdownloads.app
- [ ] Initial git commit

**Next:** Phase 1 (product polish: freemium logic, updater, onboarding) → Phase 2 (Astro landing page) → Phase 3 (LemonSqueezy billing).
