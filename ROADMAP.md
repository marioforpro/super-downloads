# ROADMAP — Super Downloads

---
scope: project:SUPER-DOWNLOADS
prefix: SD
lifecycle: pre-launch
current_phase: Relaunch hardening (reliability + legal + payments) — reopened 2026-07-16 on founder recommit
next_milestone: v1.2.0 reliability release + legal repositioning live
updated: 2026-08-03
---

> Forward-motion view for Super Downloads at LifeOS level.
> Schema contract: `docs/protocols/ROADMAP-SCHEMA.md`.
> Phase 0–3 history preserved below and in git. Day-to-day code-level detail (per-phase checkbox lists) is NOT tracked here any more — that was task-manager territory. This file tracks launch-level forward motion only.

## Roadmap

- **R-SD-001** · P1 · dropped · Submit Super Downloads to LemonSqueezy for approval
  - dropped: 2026-07-21
  - dropped_reason: merged into R-SD-004 — the six LemonSqueezy gates are Track C2 of R-SD-004, which is the single live launch track. Kept for audit trail; do not reopen.
  - reprioritized: 2026-07-16 (P0 → P1) — subsumed into R-SD-004 Track C2 (LemonSqueezy E2E verification), which per HEALTH.md runs after Track B (legal, shipped) and Track A (v1.2.0 reliability, code complete); R-SD-004 is the current top-priority item, not this one.
  - reopened: 2026-07-16 (founder recommit — relaunch hardening, see docs/superpowers/specs/2026-07-16-relaunch-hardening-design.md)
  - paused: 2026-06-15 (founder direction, Weekly Loop) — blocked ~7 weeks with no external pull; formally parked. Reopen only on real signal (user pull / press / explicit founder recommit). Resolves the active↔paused header conflict flagged in the 2026-05-31 audit.
  - progress: v1.1.0 published 2026-05-06 (3/5 checklist items shipped — DNS verified · fresh DMGs from HEAD `cfbc320` · GitHub Release reachable anonymously after repo flipped public). Pre-publish caught a stale-artifact false invariant (DMGs in `dist/` predated LS-URL fix `cba5d29` by 7 weeks); resolved by `scripts/check-release-artifacts.sh` build-provenance gate.
  - next: LemonSqueezy E2E verification (six gates per `NEXT.md` step 4: checkout loads · test-mode payment · license generated · in-app activation · free/pro limit behavior · LAUNCH30 promo). On pass, product is commercially operational; demo recording (step 5) follows.
  - target: re-baseline at LS-verify session start
  - source: docs/LAUNCH.md, NEXT.md, docs/OPS.md
  - moved: 2026-05-06
- **R-SD-002** · P1 · next · Publish launch announcement (Reddit r/macapps + HN Show HN)
  - next: draft copy once R-SD-004 Track C2 clears + LemonSqueezy checkout URL is live
  - note: 2026-08-10 · re-anclada por la decisión free-until-further-notice (`docs/DECISIONS.md §2026-08-10`) — el announcement ya NO espera el checkout LS; espera la build en modo free + el copy de landing actualizado. El ángulo "Free for a limited time" ES parte del pitch del announcement.
  - depends_on: R-SD-004
  - source: docs/MARKETING.md
  - moved: 2026-04-23
- **R-SD-004** · P0 · active · Relaunch hardening — v1.2.0 reliability release + legal repositioning
  - gate: Big Sur (Santalucía / Oriol Villar) client delivery — founder decision 2026-07-27, Weekly Loop. Reopen the moment Big Sur ships. This item is NOT blocked: nothing external stops Track C2, the founder's attention is committed elsewhere and saying so out loud beats carrying an unworked P0. Two consecutive Weekly Loops (2026-07-19, 2026-07-27) named C2 as the next move and neither week moved it — the gate records why instead of letting the board keep asserting a priority that execution contradicts.
  - gate-resolved: 2026-08-03 — founder decision SD-F-002 «Aceptar (a) — ships» (ledger: `00_System/AUDITS/2026-08-03-audit-program-final-consolidation/FOUNDER-DECISIONS-RESPONSES-2026-08-03.md` §1.1). The client-facing Big Sur v1 delivery on 2026-07-30 satisfies the gate ("Big Sur" = the STUDIO client job, not macOS 11). **Track C2 (six Lemon Squeezy E2E gates, `NEXT.md` step 4) is the live next move.** The four remaining STUDIO client loose ends stay in STUDIO as quality/licensing residue and do NOT re-block this item. Unblocks R-SD-002 and R-GROW-003.
  - status 2026-08-03: trigger satisfied · **the six LS E2E gates remain UNEXECUTED** (product work, founder-in-the-loop) · v1.2.0 publicly distributed (release URLs verified 200 on 2026-08-03) but commercially unverified · signing/notarization decision (SD-F-009) still open — no build/release/signing until the founder answers it.
  - note: 2026-08-16 · SD-F-009 resolved (a) by the founder (OOO) — deferral kept for the FREE launch; the build hold is released; unsigned DMG + install instructions (xattr / «Open Anyway» on macOS 15) belong to the free-mode task in `00_System/TASKS.md`. Revisit signing at >100 downloads (today 45 DMG total) or install complaints.
  - note: 2026-08-10 · founder decision — **launch goes FREE until further notice** (activación por email; objetivo = tráfico/descargas/usuarios; "Free for a limited time" como marketing). **Track C se difiere**: las seis gates C2 dejan de bloquear el lanzamiento y corren cuando se active el cobro. Decisión completa: `docs/DECISIONS.md §2026-08-10`. Implementación: `00_System/TASKS.md § SUPER-DOWNLOADS`.
  - merged_from: R-SD-001 (LemonSqueezy submission + the six E2E gates now live here as Track C2)
  - note: 2026-08-17 (OOO-auto, worker + boss) · **Vimeo Tier-1 had been FAILING since 2026-08-04** (13 daily runs, macOS notification only — nobody away saw it): yt-dlp's anonymous `macos` OAuth bootstrap revoked upstream (#17271; fix #17272 on master only, latest stable still 2026.07.04). App now retries once via `player.vimeo.com` on that signature (`lib.rs`), health check probes the same way → T1 PASS again today. Instagram endpoint-format fallback built + wired (`instagram_fallback.rs`, 27 tests) — live verdict inconclusive from this network (IG anonymous endpoints 404/403/empty), tier stays best-effort. main `248b2d7`, pushed; ships with the free-mode build. Details: `docs/PLATFORM-HEALTH.md` §History 2026-08-17.
  - note: 2026-07-27 · repo-intel found the Instagram engine is FAILING right now — `docs/PLATFORM-HEALTH.md` Tier 2 = FAIL (HTTP 400, yt-dlp #13626/#16311), i.e. Track A's reliability claim is currently false for that platform. Named remedy: the endpoint-fallback technique in `instaloader/instaloader` (Vault note `instaloader-instaloader.md`, verdict EXTRACT). This does NOT lift the `gate:` above — it is context for when Big Sur ships, or the reason to lift it early if Instagram matters to the relaunch.
  - retitled: 2026-07-16 (was "Relaunch hardening — download reliability (v1.2.0), legal repositioning, payments resilience") — aligned to HEALTH.md next_milestone wording; payments resilience remains in scope, see scope/progress below.
  - added: 2026-07-16 (founder recommit)
  - scope: execution order B → A → C — Track B legal urgent items (GDPR/PostHog cookieless, copy sanitation, legal surfaces, sales reframing) → Track A v1.2.0 reliability release + robustness + health protocol v2 → Track C LemonSqueezy mitigations + E2E verification (C2 = the six gates in NEXT.md step 4)
  - progress 2026-07-16: **Track B SHIPPED to production** (landing: PostHog cookieless, Privacy rewrite, Terms-as-EULA, /copyright page, tiered platform claims, footer disclaimer; docs/README sanitized). **Track A code COMPLETE** — health protocol v2 live (CERTIFIED OPERATIONAL at baseline) + v1.2.0 reliability code committed (impersonation, wider auth retry, default-browser cookies, engine UI, version guard, onboarding terms), bundled yt-dlp → 2026.07.04, `npm run check` green, signed artifacts built. **Track C1 prepped** (LS compliance email draft + mitigations checklist in NEXT.md; local activation cache still pending — v1.2.x).
  - next: v1.2.0 PUBLISHED 2026-07-16 ✅ (release live, anonymous URLs 200, updater manifest serving) → post-release smoke test + C1 email + C2 E2E gates
  - source: docs/superpowers/specs/2026-07-16-relaunch-hardening-design.md
  - moved: 2026-07-16

## Shipped (last 30d)

- **R-SD-003** · P1 · shipped · Download reliability (360p fix)
  - outcome: SHIPPED in **v1.1.1** (2026-05-31). Root cause: bundled yt-dlp was `2026.01.29` (~4mo stale) → YouTube fell back to 360p. Refreshed to `2026.03.17` (YouTube restored to 4K), verified via `scripts/platform-health-check.sh`. Released to users. Verified: `find_ytdlp()` already prefers the bundled binary first (same as ffmpeg/ffprobe), so users always get the fresh bundled yt-dlp. Open: Facebook extraction degraded on stable yt-dlp (upstream); runtime yt-dlp self-update (implemented in code 2026-06-16, Unreleased — see Backlog) decouples extractor freshness from app releases.
  - moved: 2026-05-31
  - shipped: 2026-05-31
- **Platform-health monitor (automated)** — SHIPPED 2026-05-31. `launchd` agent runs `scripts/platform-health-notify.sh` daily at 10:00; notifies only when a platform's status *changes* (no daily spam). See `docs/PLATFORM-HEALTH.md`. Lifecycle truth (2026-08-03): the agent was DEAD 2026-07-02 → 2026-08-03 — the plist sat renamed `.plist.disabled`, zero automated runs in that window, one manual run 2026-07-16. Reloaded 2026-08-03 (audit Wave D; no kickstart); first scheduled run expected 2026-08-04 10:00; now registered in `00_System/AUTOMATIONS.md`.
- **One-click auto-update** — SHIPPED in **v1.1.1** (2026-05-31). Tauri updater + static `latest.json` on GitHub Releases; in-app "Update" banner → download/install/relaunch. Release pipeline: `scripts/make-release.sh`. Signing key rotated (password in `~/.secrets`). First updater-capable build; future releases self-update for users on v1.1.1+.

## Backlog (unranked)

- OG image asset (screenshot or custom design)
- Screenshots / assets / demo GIF for launch day polish (distinct from the demo video under R-SD-001)
- Batch URL paste feature
- Analytics telemetry opt-in (from Phase 2 deferred list)
- Windows/Linux expansion
- Browser extension (was Phase 8 in prior roadmap — legitimately future work)
- MacUpdate / AlternativeTo directory submissions
- Product Hunt launch (timing TBD)
- **Founder comp license via LemonSqueezy** — issue real comp license through Lemon dashboard (Product → Licenses → Issue License Key, comp/free) for `supermariomonteiro@gmail.com`, replace local `FOUNDER-MARIO-001` stop-gap (Session 201 inject, see `docs/SECURITY-NOTES.md`). Validates the full activation flow end-to-end with a real Lemon-tracked record. Depends on: Lemon product live (linked to R-SD-001 Phase 4 LS-verify). **Sequencing constraint** (surfaced 2026-08-03 from `docs/SECURITY-NOTES.md` prose, SD-F-014): replace the FOUNDER-MARIO-001 stop-gap before/with the license-hardening review below, so any revalidation layer doesn't lock the founder's injected keys out.
- **LinkedIn health-probe URL** — the `PROBES` entry in `scripts/platform-health-check.sh` is a self-admitted dead placeholder (`docs/PLATFORM-HEALTH.md` "Open item"); swap in any public LinkedIn post with native video. Only probed on `--cookies` runs. (Surfaced 2026-08-03 from PLATFORM-HEALTH prose, SD-F-014.)
- **`copy_to_clipboard` dead fallback** — `src/main.js` copy-link fallback invokes a Tauri command that doesn't exist in `lib.rs` (primary `navigator.clipboard.writeText` path works; the fallback would toast "Failed to copy link"). Remove the fallback or implement the command; then drop it from `KNOWN_MISSING` in `scripts/check-invoke-bindings.mjs`. (Discovered 2026-08-03 by the SD-F-007 regression checker.)
- **License hardening / anti-crack security review** — current freemium gate is bypassable in 60s via raw SQLite localStorage inject (`proLicenseKey` set → `isProUser()` returns true forever, no re-validation). Documented attack vector in `docs/SECURITY-NOTES.md`. Scope: pick + implement a hardening level appropriate for €29 one-time consumer app (recommended: periodic Lemon revalidation + signed cache, NOT full DRM). Pre-launch hardening item, not launch-blocker.
- **Runtime yt-dlp self-update** — ✅ shipping in **v1.2.0** (2026-07-16, R-SD-004 Track A). Pendings closed: (1) in the v1.2.0 build ✅; (3) Settings "Update engine" button wired to `update_ytdlp` + `get_ytdlp_version` ✅; (4) version-comparison guard implemented (`prune_stale_managed_ytdlp`) ✅. Still open: (2) runtime E2E verify of the self-update network path on a real app run — do at v1.2.0 smoke test.
- ~~**Cockpit state wiring** — after the 2026-06-16 frontmatter reconciliation (ROADMAP + HEALTH → parked), regenerate + publish `state.super-downloads.json` via the LifeOS engine (`/lifeos-publish`) so the cockpit stops rendering SD as an active Phase-4 project. The doc edits are the inputs; the shard regen is the engine-facing surface. (cross-department: 00_System/LIFEOS-COCKPIT + Vercel)~~ [killed: premise reversed on 2026-07-21 — SD was reopened 2026-07-16 and should render as active; the shard regenerates unattended (state.super-downloads.json refreshed daily by reconstruction-engine) so there is no wiring left to do]
- **Reconcile stale product audits** — `docs/APP-AUDIT.md` (14 items) + `docs/DIAGNOSTIC.md` are frozen at 2026-03-24/v1.1.0; many flagged items shipped across Phases 0–3 + v1.1.1. Add a "Reconciled" banner, strike shipped items, and lift the still-open debt into this backlog. Residue list re-verified 2026-08-03 (SD-F-012): the only genuinely open item is **landing CSP `img-src`**. Resolved since the list was written — history-persistence README contradiction (fixed 2026-08-03, SD-F-007: invoke key + catch logging + README re-word), hardcoded version P8 (dynamic `getVersion()` overwrite ships; only a pre-JS placeholder remains), stray `console.log`s T3 (0 exist).
- **Release-ops hardening (dept)** — (a) backfill local annotated git tags for v1.1.0/v1.1.1 (none exist; `git show <tag>` history hints don't resolve) and add a tag-push step to `scripts/make-release.sh`; (b) lift the release pipeline (`make-release.sh`, required `latest.json` asset, `check-release-artifacts.sh` provenance gate, Ed25519 key rotation) from MEMORY.md into `docs/OPS.md` (the authoritative ops doc per Doc Ownership).

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

Full phase-level detail (with per-task checkboxes) can be retrieved with `git show HEAD~:01_Projects/SUPER-DOWNLOADS/ROADMAP.md`; `PROGRESS.md` exists (historical pre-launch progress notes) and carries that record. *(Wording fixed 2026-08-03 — this line predated PROGRESS.md's creation; MA-SD-10.)*
