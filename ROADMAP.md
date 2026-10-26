# ROADMAP — Super Downloads

---
scope: project:SUPER-DOWNLOADS
prefix: SD
lifecycle: active
current_phase: Free app, private use + friends testing (v1.5.6 live) — work plan 2026-10-23 below
next_milestone: Developer ID signing + notarization in early January before the 26-Jan launch (R-SD-009, target 2027-01-20) — pipeline ready and rehearsed + fixes from tester reports
updated: 2026-10-26
---

> Forward-motion view for Super Downloads at LifeOS level.
> Schema contract: `docs/protocols/ROADMAP-SCHEMA.md`.
> Phase 0–3 history preserved below and in git. Day-to-day code-level detail (per-phase checkbox lists) is NOT tracked here any more — that was task-manager territory. This file tracks launch-level forward motion only.

## Roadmap

> **Work plan (2026-10-23).** Order = what moves the product most for the least founder time. Now → Next → Later.
> References behind each call are cited inline (yt-dlp release cadence, Apple notarization, competitor apps).

- **R-SD-014** · P1 · shipped · v1.5.6 — marketing consent unticked, share notice, dependency refresh
  - added: 2026-10-25
  - moved: 2026-10-26
  - scope: branch `release/v1.5.6` (built, not published): marketing consent unticked with its wording (Planet49) · one-time «share it» notice after the 10th download · Tauri 2.12 + cargo/npm refresh · turn on the welcome email (Resend) · release pipeline adds a VirusTotal upload + `VT_SCANNED` before publishing.
  - shipped: 2026-10-26 — founder yes (SD-F-018). Tag `9155d41`; latest.json 1.5.6 verified live, updater signature valid with the 1.5.x key, web SHA-256 = binary, `test-live-site` 29/29; Dependabot 21 → 1 (glib, Linux-only). Web «Free» with no deadline (SD-F-016, `af8b6d6`+`e649dee`); the in-app activation card still says «limited time» until the next app release. One-click unsubscribe + Super Guides P.S. for consented users built (`2ca232d`), off until Mario adds the Resend + Vercel env vars. Report `_done/2026-10-26-super-downloads-v156-signing-ps.md`.
- **R-SD-013** · P1 · shipped · v1.5.5 — Vimeo + Instagram fixes, «Copy. Paste. Done.», anonymous daily active ping
  - added: 2026-10-25
  - moved: 2026-10-25
  - scope: Vimeo hidden-copy fix · Instagram fallback refreshed · «Copy. Paste. Done.» app + landing copy · `app_active` daily ping on by default with a Settings switch (docs/DECISIONS.md 2026-10-25) · SHA-256 on the landing.
  - shipped: 2026-10-25 — v1.5.5 published (founder's yes in chat); final round 29/29 live checks.
- **R-SD-012** · P1 · next · Go-to-market — big launch Tue 2027-01-26, after signing (plan: private, LifeOS `08_Growth/super-downloads/GTM-2026-10-25.md`)
  - added: 2026-10-25
  - moved: 2026-10-25
  - why: GTM brief 2026-10-25 (`08_Growth/GTM-BRIEF-2026-10-25.md`). Measured: 183 landing views/30 d (~75 % search), 39 DMGs since free mode → 3 email activations (2 founder tests): the unsigned install is where the funnel breaks, so the launch waits for R-SD-009. Portfolio order (orchestrator, 2026-10-25): SPC 10-nov · SG 17-nov · SP 24-nov · SD 26-ene.
  - scope: 5 channels (SEO/GEO pages · directories as the launch moment · r/macapps «App Pile» megathread + editor threads · partner emails to editor blogs/newsletters · brand Instagram + Threads via Buffer free) · 12 drafts in `08_Growth/super-downloads/drafts-2027-01-launch` (private)/` · STUDIO briefs S1–S5 (GTM §8). Product Hunt and Show HN out (personal maker account / personal post required).
  - open_decision: D1–D5 in GTM §10 (launch date, Apple identity, PH, Uneed 29,99 USD, SD→SG cross-promo) — founder.
  - depends_on: R-SD-009
  - target: 2027-01-26
  - next: Claude: AlternativeTo + Uneed listings ready to submit in November, SEO pages 1–3 + `llms.txt`, open STUDIO jobs S1–S4 · Mario: decide D1–D5, create the brand profiles (Instagram, Threads, Reddit) and connect Buffer.
- **R-SD-005** · P0 · shipped · Landing refresh — real app screenshots, OG image, editor-first design
  - added: 2026-10-23
  - moved: 2026-10-23
  - progress: 2026-10-23 · built locally, not deployed. New `web/src/pages/index.astro`: hero = real app window over an editing timeline, «Copy. Switch. Enter.» sequence, three feature rows with real screenshots (dark + light), platforms + trust facts, download, FAQ. `og.png` now exists (it was referenced but missing → every shared link had no preview). Geist as web typeface (BRAND.md had typography «pending»). JSON-LD `softwareVersion` 1.1 → 1.5.1. Screenshots come from the real frontend rendered in WebKit with staged data (`scripts/screenshots/`), thumbnails = Blender Studio open movies, CC BY 4.0, credited in the footer.
  - shipped: 2026-10-23 — live on superdownloads.app (home, og.png, shots 200). Founder check pending: share preview in WhatsApp/X.
  - progress: 2026-10-23 · polish pass from a 4-reviewer design review (visual, UX/conversion, copy, a11y/perf): one-story hero, chip-detected direct download, install guide after the click, mobile «send to your Mac», copy rewrite, expanded FAQ (JSON-LD = visible), WCAG AA fixes, self-hosted Geist. Open founder questions: what happens after the free period (FAQ deliberately silent), maker line wording.
  - next: 15-s hero loop (screen recording or HyperFrames) — founder taste call, still open. Done 2026-10-23: `/changelog` page (plain-language notes in `web/src/data/releases.js`, also backfilled into GitHub releases 1.2.0–1.5.2) · version + size on the download buttons (read at build time) · sitemap refreshed (+ /changelog, /copyright).
- **R-SD-006** · P0 · shipped · v1.5.2 maintenance release — fresh engine + small fixes
  - added: 2026-10-23
  - moved: 2026-10-23
  - why: the daily health check went **DEGRADED (stale engine)** on 2026-10-22 — bundled yt-dlp `2026.08.19` is 64 days old (gate 28 d); upstream is `2026.10.21`. All 7 platforms still PASS. Installed apps already refresh the engine weekly on their own (since v1.2.0), so this mostly matters for fresh installs and for the case where the self-update fails — but yt-dlp ships fixes every 2–4 weeks because sites change, so the bundled copy should not lag.
  - scope: bump bundled yt-dlp → latest stable · fix `url.contains("x.com")` also matching dropbox.com (known bug #6) · remove the dead `copy_to_clipboard` fallback · empty-state copy «Copy a video link and come back» (IDEA-0036) · live E2E before publishing · verify the real minimum macOS (Tauri 2 targets 10.15; `tauri.conf.json` and the JSON-LD still claim 10.13) and align the landing.
  - progress: 2026-10-23 · code done on `main`: bundled yt-dlp 2026.08.19 → 2026.10.21 (checksum-verified), host-based X/Twitter match, dead `copy_to_clipboard` fallback removed (`KNOWN_MISSING` now empty), empty-state copy, engine checksum (R-SD-007). `npm run check` green.
  - shipped: 2026-10-23 — live E2E 9/9 (YouTube, MP3, TikTok, X, Vimeo, Vimeo embed-restricted, Instagram, Facebook, LinkedIn) → published; URLs 200, `latest.json` 1.5.2 both arches, DMG carries engine 2026.10.21. Minimum-macOS check moved to the backlog.
- **R-SD-007** · P1 · shipped · Engine self-update verified by checksum
  - added: 2026-10-23
  - moved: 2026-10-23
  - correction 2026-10-23: the weekly background refresh **already existed** (v1.2.0: launch-time check, atomic install into `bin/engine-<tag>/`, bundled engine as fallback). The plan first said otherwise — wrong. What was missing was integrity: the downloaded zip was only size-checked (known bug #7).
  - progress: 2026-10-23 · the self-update now fetches the release's `SHA2-256SUMS` first and refuses a zip whose SHA-256 does not match (`expected_sha256` / `sha256_hex` in `lib.rs`, unit-tested; live `engine_self_update_live` PASS with 2026.10.21). Ships in v1.5.2.
  - shipped: 2026-10-23 — in v1.5.2 (published by the founder via `make-release.sh --publish`).
- **R-SD-008** · P1 · shipped · Tester feedback loop — «Report a problem» inside the app
  - added: 2026-10-23
  - moved: 2026-10-23
  - why: the active phase is friends testing, but the only channel is a support email nobody sees in the app. A bug report that already carries app version, engine version, macOS version and the failing URL + error saves a back-and-forth per bug.
  - scope: menu item on a failed row («Report this problem») + Settings link → prefilled `mailto:support@superdownloads.app` (no backend). Then work the known-bugs list in `NEXT.md` as reports arrive.
  - progress: 2026-10-23 · code done on `main`, not released: ⋯ menu on a failed row → «Report This Problem…»; Settings › App › Help → «Report a problem». Both open a prefilled email (link, friendly error, last 800 chars of the engine's raw error — now stored as `rawError` on the row, format, quality setting, app/engine/macOS+arch versions). New Rust command `get_system_info` (WebKit's user agent is frozen at 10.15.7). `npm run check` green; mail body verified by simulation, not yet clicked in the built app.
  - progress: 2026-10-23 · the same click also copies the report to the clipboard + a 6-s toast («paste it into Gmail or WhatsApp»): a tester on webmail has no mail app configured, so a bare `mailto:` would go nowhere silently. The copied text is marked as handled so copy & come back / Batch mode never treat it as a link.
  - progress: 2026-10-23 · verified in a WebKit harness over the real `src/` with a mocked Tauri API (30/30: menu item only on failed rows, mailto subject/body, clipboard copy, toast, copy & come back ignores the report, Settings generic report). Rides in v1.5.3 together with the redesigned welcome screen (founder feedback on the first-run card). Real-app clicks NOT done: driving the GUI while the founder used the Mac sent test keystrokes to WhatsApp (not sent) — never drive the GUI during a live session; use the harness.
  - shipped: 2026-10-23 — v1.5.3 published (DMGs 200, `latest.json` 1.5.3 both arches). Founder verified 2026-10-23 in the installed app: ⋯ › Report opens Apple Mail with the email ready to send. Known-bugs list in `NEXT.md` is worked as reports arrive.
- **R-SD-009** · P2 · next · Signing + notarization — decided yes, enrol in early January before the 26-Jan launch (SD-F-017)
  - added: 2026-10-23
  - moved: 2026-10-26
  - why: SD-F-009 (2026-08-16) said «revisit at >100 downloads or install complaints». DMG downloads across releases are now **113** (GitHub API, 2026-10-23). Unsigned builds make macOS 15 hide «Open Anyway» deep in System Settings, the single biggest drop-off for non-technical friends. Apple Developer Program = 99 USD/year; notarization is free with it (Apple: «Notarizing macOS software before distribution»).
  - decided: 2026-10-23 — founder: yes, in December (Christmas break). App Store is out (guideline 5.2.3 bans downloaders); Developer ID + notarization is an automated malware/signing check, not App Review.
  - target: 2027-01-20
  - retimed: 2026-10-26 — founder (SD-F-017): individual Apple account **yes, but later** — «pagaré lo de apple, pero no ahora; ahora quiero construir y prepararlo todo, no empezar a tener gastos». Enrol by 2027-01-05 (task in `00_System/TASKS.md` § Projects); until then zero spend, signing pipeline prepared to receive the certs. GitHub organisation per project = study first (task, target 2026-11-30), nothing moves before.
  - decide-first: 2026-10-25 — founder: «no personal info anywhere». Before enrolling, decide the public identity: an individual Apple account shows the founder's legal name as the developer in macOS install dialogs; showing «Super Downloads» needs an organization account (legal entity + D-U-N-S). Same move: GitHub repo to a `superdownloads` organization (name free on 2026-10-25; GitHub redirects old release/updater URLs) and new commits authored «Super Downloads»; rewriting the 154 existing commits is optional (irreversible, backup first). Only SD is public — SP/SPC repos are private, SG has none.
  - next: founder enrolls in the Apple Developer Program (the founder's Apple ID) and creates a Developer ID Application cert → Claude: sign every engine dylib/.so + hardened-runtime entitlements, `notarytool` + staple in `make-release.sh`, drop the xattr / «Open Anyway» install guide from landing + FAQ.
- **R-SD-010** · P1 · shipped · v1.5.4 — reliability + polish from the known-bugs list
  - added: 2026-10-23
  - moved: 2026-10-23
  - scope: stall watchdog (90 s without engine output → retryable «connection stalled» error, also bounds the Keychain-prompt cookie retry) · ⋯ menu survives re-renders · Settings focus management · real minimum macOS declared (Info.plist per arch) + landing aligned · row entry animation · release script hardening (tag = built commit, provenance gate, CHANGELOG notes).
  - progress: 2026-10-23 · code + docs done; `npm run check` green (36 Rust tests incl. 3 watchdog tests on a real process); WebKit UI check 39/39; landing build green.
  - shipped: 2026-10-23 — v1.5.4 published (E2E 9/9; DMGs 200; `latest.json` 1.5.4; first release through the hardened script: tag = built commit, CHANGELOG notes).
- **R-SD-011** · P1 · active · Demo film — «Links become clips» (landing, social, in-app)
  - added: 2026-10-23
  - moved: 2026-10-23
  - why: founder decision 2026-10-23 — the app is a lead magnet; a short, modern demo that teaches the flow is what gets shared and what friends/clients see first.
  - progress: 2026-10-23 · built in HyperFrames from the REAL app UI (`marketing/demo/tools/extract-ui.cjs` renders `src/` in WebKit; scenes generated from one template for 16:9 and 9:16). 24 s on a 120 BPM grid: copy in the browser → capsule flies into the app → Enter → download + «Converting to H.264» punch-in → Batch rain of 3 links → drag onto an editing timeline where «Spring» (Blender, CC BY 4.0) plays → platforms roll → end card «Free download · superdownloads.app» (no personal credit; checked on the v8 frames 2026-10-25). STUDIO motion QA run on every render (81 → intended-only fails). Landing `#demo` section live (muted loop while visible, «Play with sound», no autoplay with reduced motion). Settings › Help › Watch demo (unreleased, next app version). Vertical 1080×1920 rendered.
  - progress: 2026-10-23 · v7 after founder review of v5 («no dissolves, motion blur, continuous ease, playhead behind clips, monitor speed, better sound, better compression, drop "Made by"») + three critic subagents (motion 6.2 → fixes, sound 5/10 → 47 cues cut to 23, brand 7/10 → copy). Every hand-off is now a solid morph or a single shared pan; one never-parked camera per scene; real 180° shutter blur (make-slow ×8 → 240 fps → `shutter.sh`); footage at 1× (new 6 s continuous Spring shot); playhead on its own top layer; end card «Super Downloads / The video downloader built for editors. / Free download · superdownloads.app», all four Blender films credited. Sound from owned NAS packs, git-ignored copies (public repo). Landing encode CRF 23 (7.3 MB mp4 / 6.2 MB webm, SSIM 0.98) staged in the STUDIO job (`clients/super-downloads/demo-film/motion/demo/renders/landing_v7/`), not yet swapped in. Workshop moved to STUDIO 2026-10-23 (founder OK; the product-repo guard blocks workshops).
  - progress: 2026-10-23 · v8 live on the landing (founder's Suno track, Pro plan, commercial use; versioned files `*-v8.*` because the `/demo/*` cache served the silent v5; open on the light ground, «Downloads in H.264.» after Enter, end-card type without zoom). Workshop lives in STUDIO `clients/super-downloads/demo-film`. Social posting on hold by founder decision 2026-10-23.
  - next: founder decides when to post the 16:9 / 9:16 cuts · «Watch demo» in Settings ships with the next app release · STUDIO job close waits on PENDING SD-F-015.

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
- **R-SD-002** · P2 · dropped · Publish launch announcement (Reddit r/macapps + HN Show HN)
  - dropped: 2026-10-25
  - dropped_reason: merged into R-SD-012 (GTM plan). The old form no longer works: r/macapps rule 8 limits non-qualifying developers to the monthly megathread, and Show HN asks for a personal, handwritten post with no email gate.
  - merged_into: R-SD-012
  - deferral: 2026-10-03 (on hold) — founder: no announcement for now; the app is for Mario + friends to test first, promotion decided later (`docs/DECISIONS.md` 2026-10-03). Draft kept current for v1.4.0 in `docs/ANNOUNCEMENT-DRAFT.md`.
  - note: 2026-10-03 (was «unblocked») — v1.3.0 free-mode build published and the landing is live with free-mode copy; the announcement can go out (`docs/ANNOUNCEMENT-DRAFT.md`). Founder posts.
  - next: none while on hold — when promotion is decided, start from the checklist in `docs/ANNOUNCEMENT-DRAFT.md` §3. [Historical: copy DRAFTED (OOO-auto 2026-10-02) → `docs/ANNOUNCEMENT-DRAFT.md` (r/macapps + Show HN + pre-post checklist); post only after the free-mode build ships and `web/src/pages/index.astro` drops «5 downloads per day» / «€29 Pro»]
  - note: 2026-08-10 · re-anclada por la decisión free-until-further-notice (`docs/DECISIONS.md §2026-08-10`) — el announcement ya NO espera el checkout LS; espera la build en modo free + el copy de landing actualizado. El ángulo "Free for a limited time" ES parte del pitch del announcement.
  - depends_on: R-SD-004
  - source: docs/MARKETING.md
  - moved: 2026-10-03
- **R-SD-004** · P0 · shipped · Relaunch hardening — v1.2.0 reliability release + legal repositioning
  - note: 2026-10-03 · **v1.3.0 FREE mode shipped** (founder decision 2026-08-10): no daily limit, license gate hidden (LemonSqueezy code intact behind `FREE_MODE`), email activation via `register_activation` → `web/api/activate.js` → Airtable workspace SUPER DOWNLOADS › `SD · Users`; release published, updater serves 1.3.0 (both arches), activation verified E2E. Track C2 (LemonSqueezy) stays deferred.
  - gate: Big Sur (Santalucía / Oriol Villar) client delivery — founder decision 2026-07-27, Weekly Loop. Reopen the moment Big Sur ships. This item is NOT blocked: nothing external stops Track C2, the founder's attention is committed elsewhere and saying so out loud beats carrying an unworked P0. Two consecutive Weekly Loops (2026-07-19, 2026-07-27) named C2 as the next move and neither week moved it — the gate records why instead of letting the board keep asserting a priority that execution contradicts.
  - gate-resolved: 2026-08-03 — founder decision SD-F-002 «Aceptar (a) — ships» (ledger: `00_System/AUDITS/2026-08-03-audit-program-final-consolidation/FOUNDER-DECISIONS-RESPONSES-2026-08-03.md` §1.1). The client-facing Big Sur v1 delivery on 2026-07-30 satisfies the gate ("Big Sur" = the STUDIO client job, not macOS 11). **Track C2 (six Lemon Squeezy E2E gates, `NEXT.md` step 4) is the live next move.** The four remaining STUDIO client loose ends stay in STUDIO as quality/licensing residue and do NOT re-block this item. Unblocks R-SD-002 and R-GROW-003.
  - status 2026-08-03: trigger satisfied · **the six LS E2E gates remain UNEXECUTED** (product work, founder-in-the-loop) · v1.2.0 publicly distributed (release URLs verified 200 on 2026-08-03) but commercially unverified · signing/notarization decision (SD-F-009) still open — no build/release/signing until the founder answers it.
  - note: 2026-08-16 · SD-F-009 resolved (a) by the founder (OOO) — deferral kept for the FREE launch; the build hold is released; unsigned DMG + install instructions (xattr / «Open Anyway» on macOS 15) belong to the free-mode task in `00_System/TASKS.md`. Revisit signing at >100 downloads (today 45 DMG total) or install complaints.
  - note: 2026-08-10 · founder decision — **launch goes FREE until further notice** (activación por email; objetivo = tráfico/descargas/usuarios; "Free for a limited time" como marketing). **Track C se difiere**: las seis gates C2 dejan de bloquear el lanzamiento y corren cuando se active el cobro. Decisión completa: `docs/DECISIONS.md §2026-08-10`. Implementación: `00_System/TASKS.md § SUPER-DOWNLOADS`.
  - note: 2026-10-03 · **Track C CLOSED** — founder: no LemonSqueezy account, the app is free. The six C2 gates and the four LS mitigations are void (`docs/DECISIONS.md` 2026-10-03). v1.4.0 (onedir engine ~7s→0.2s per call, launch-freeze fix, redesigned Settings, QA pass with live E2E, Vimeo 3-step flow) committed, awaiting build.
  - merged_from: R-SD-001 (LemonSqueezy submission + the six E2E gates now live here as Track C2)
  - note: 2026-08-17 (OOO-auto, worker + boss) · **Vimeo Tier-1 had been FAILING since 2026-08-04** (13 daily runs, macOS notification only — nobody away saw it): yt-dlp's anonymous `macos` OAuth bootstrap revoked upstream (#17271; fix #17272 on master only, latest stable still 2026.07.04). App now retries once via `player.vimeo.com` on that signature (`lib.rs`), health check probes the same way → T1 PASS again today. Instagram endpoint-format fallback built + wired (`instagram_fallback.rs`, 27 tests) — live verdict inconclusive from this network (IG anonymous endpoints 404/403/empty), tier stays best-effort. main `248b2d7`, pushed; ships with the free-mode build. Details: `docs/PLATFORM-HEALTH.md` §History 2026-08-17.
  - note: 2026-07-27 · repo-intel found the Instagram engine is FAILING right now — `docs/PLATFORM-HEALTH.md` Tier 2 = FAIL (HTTP 400, yt-dlp #13626/#16311), i.e. Track A's reliability claim is currently false for that platform. Named remedy: the endpoint-fallback technique in `instaloader/instaloader` (Vault note `instaloader-instaloader.md`, verdict EXTRACT). This does NOT lift the `gate:` above — it is context for when Big Sur ships, or the reason to lift it early if Instagram matters to the relaunch.
  - retitled: 2026-07-16 (was "Relaunch hardening — download reliability (v1.2.0), legal repositioning, payments resilience") — aligned to HEALTH.md next_milestone wording; payments resilience remains in scope, see scope/progress below.
  - added: 2026-07-16 (founder recommit)
  - scope: execution order B → A → C — Track B legal urgent items (GDPR/PostHog cookieless, copy sanitation, legal surfaces, sales reframing) → Track A v1.2.0 reliability release + robustness + health protocol v2 → Track C LemonSqueezy mitigations + E2E verification (C2 = the six gates in NEXT.md step 4)
  - progress 2026-07-16: **Track B SHIPPED to production** (landing: PostHog cookieless, Privacy rewrite, Terms-as-EULA, /copyright page, tiered platform claims, footer disclaimer; docs/README sanitized). **Track A code COMPLETE** — health protocol v2 live (CERTIFIED OPERATIONAL at baseline) + v1.2.0 reliability code committed (impersonation, wider auth retry, default-browser cookies, engine UI, version guard, onboarding terms), bundled yt-dlp → 2026.07.04, `npm run check` green, signed artifacts built. **Track C1 prepped** (LS compliance email draft + mitigations checklist in NEXT.md; local activation cache still pending — v1.2.x).
  - next: v1.2.0 PUBLISHED 2026-07-16 ✅ (release live, anonymous URLs 200, updater manifest serving) → post-release smoke test + C1 email + C2 E2E gates
  - source: docs/superpowers/specs/2026-07-16-relaunch-hardening-design.md
  - moved: 2026-10-03
  - shipped: 2026-10-03 — Track B (legal, 2026-07-16) + Track A (reliability: v1.2.0 → v1.4.0 onedir engine, QA pass with live E2E) done; Track C closed (no payments provider). Follow-up lives in NEXT.md (friends-testing phase).

## Shipped (last 30d)

- **R-SD-003** · P1 · shipped · Download reliability (360p fix)
  - outcome: SHIPPED in **v1.1.1** (2026-05-31). Root cause: bundled yt-dlp was `2026.01.29` (~4mo stale) → YouTube fell back to 360p. Refreshed to `2026.03.17` (YouTube restored to 4K), verified via `scripts/platform-health-check.sh`. Released to users. Verified: `find_ytdlp()` already prefers the bundled binary first (same as ffmpeg/ffprobe), so users always get the fresh bundled yt-dlp. Open: Facebook extraction degraded on stable yt-dlp (upstream); runtime yt-dlp self-update (implemented in code 2026-06-16, Unreleased — see Backlog) decouples extractor freshness from app releases.
  - moved: 2026-05-31
  - shipped: 2026-05-31
- **Platform-health monitor (automated)** — SHIPPED 2026-05-31. `launchd` agent runs `scripts/platform-health-notify.sh` daily at 10:00; notifies only when a platform's status *changes* (no daily spam). See `docs/PLATFORM-HEALTH.md`. Lifecycle truth (2026-08-03): the agent was DEAD 2026-07-02 → 2026-08-03 — the plist sat renamed `.plist.disabled`, zero automated runs in that window, one manual run 2026-07-16. Reloaded 2026-08-03 (audit Wave D; no kickstart); first scheduled run expected 2026-08-04 10:00; now registered in `00_System/AUTOMATIONS.md`.
- **One-click auto-update** — SHIPPED in **v1.1.1** (2026-05-31). Tauri updater + static `latest.json` on GitHub Releases; in-app "Update" banner → download/install/relaunch. Release pipeline: `scripts/make-release.sh`. Signing key rotated (password in `~/.secrets`). First updater-capable build; future releases self-update for users on v1.1.1+.

## Backlog (unranked)

- ~~OG image asset~~ — done 2026-10-23 (R-SD-005)
- ~~Screenshots / assets~~ — done 2026-10-23 (R-SD-005); demo GIF/loop still open there
- ~~Batch URL paste feature~~ — shipped v1.5.0 (paste several links + Batch mode)
- Analytics telemetry opt-in (from Phase 2 deferred list)
- Windows/Linux expansion
- Browser extension (was Phase 8 in prior roadmap — legitimately future work)
- ~~Verify the real minimum macOS~~ — done 2026-10-23 (measured with `otool` on the built bundles): **Intel 10.15** (engine), **Apple Silicon 12** (ffmpeg/ffprobe arm64). App declares it (`minimumSystemVersion` 10.15 + `src-tauri/Info.plist` `LSMinimumSystemVersionByArchitecture`); landing download buttons, FAQ and JSON-LD aligned. Ships in v1.5.4.
- ~~MacUpdate / AlternativeTo directory submissions~~ — now in R-SD-012 (GTM §4 channel 2)
- ~~Product Hunt launch~~ — out of R-SD-012: Product Hunt forbids company accounts, so the maker would be a personal profile (GTM §10 D3)
- **Founder comp license via LemonSqueezy** — **void 2026-10-03** (no LemonSqueezy account; app is free). ~~— issue real comp license through Lemon dashboard (Product → Licenses → Issue License Key, comp/free) for the founder's email, replace local `FOUNDER-MARIO-001` stop-gap (Session 201 inject, see `docs/SECURITY-NOTES.md`). Validates the full activation flow end-to-end with a real Lemon-tracked record. Depends on: Lemon product live (linked to R-SD-001 Phase 4 LS-verify). **Sequencing constraint** (surfaced 2026-08-03 from `docs/SECURITY-NOTES.md` prose, SD-F-014): replace the FOUNDER-MARIO-001 stop-gap before/with the license-hardening review below, so any revalidation layer doesn't lock the founder's injected keys out.~~
- ~~**LinkedIn health-probe URL**~~ — done 2026-09-27 (yt-dlp's own MathWorks test post; also the E2E LinkedIn case, PASS 2026-10-23).
- ~~**Instagram fallback is stale**~~ — fixed 2026-10-25 (ships in v1.5.5): the first endpoint (`__a=1`) now answers a 404 page and the second a 403, and the code treated both as final, so it never reached GraphQL — which works anonymously (200 with the video). GraphQL now goes first and a 404/403 on one shape no longer stops the others; 3 new unit tests, live test resolves the reel. Was: «unavailable or deleted» for a live public reel that yt-dlp downloads fine.
- ~~**`copy_to_clipboard` dead fallback**~~ — removed in v1.5.2 (`KNOWN_MISSING` empty). Was: `src/main.js` copy-link fallback invokes a Tauri command that doesn't exist in `lib.rs` (primary `navigator.clipboard.writeText` path works; the fallback would toast "Failed to copy link"). Remove the fallback or implement the command; then drop it from `KNOWN_MISSING` in `scripts/check-invoke-bindings.mjs`. (Discovered 2026-08-03 by the SD-F-007 regression checker.)
- **License hardening / anti-crack security review** — **void while the app is free** (2026-10-03). ~~— current freemium gate is bypassable in 60s via raw SQLite localStorage inject (`proLicenseKey` set → `isProUser()` returns true forever, no re-validation). Documented attack vector in `docs/SECURITY-NOTES.md`. Scope: pick + implement a hardening level appropriate for €29 one-time consumer app (recommended: periodic Lemon revalidation + signed cache, NOT full DRM). Pre-launch hardening item, not launch-blocker.~~
- **Runtime yt-dlp self-update** — ✅ shipping in **v1.2.0** (2026-07-16, R-SD-004 Track A). Pendings closed: (1) in the v1.2.0 build ✅; (3) Settings "Update engine" button wired to `update_ytdlp` + `get_ytdlp_version` ✅; (4) version-comparison guard implemented (`prune_stale_managed_ytdlp`) ✅. (2) runtime verify of the self-update network path — done 2026-10-23 (`engine_self_update_live` PASS with 2026.10.21, R-SD-007).
- ~~**Cockpit state wiring** — after the 2026-06-16 frontmatter reconciliation (ROADMAP + HEALTH → parked), regenerate + publish `state.super-downloads.json` via the LifeOS engine (`/lifeos-publish`) so the cockpit stops rendering SD as an active Phase-4 project. The doc edits are the inputs; the shard regen is the engine-facing surface. (cross-department: 00_System/LIFEOS-COCKPIT + Vercel)~~ [killed: premise reversed on 2026-07-21 — SD was reopened 2026-07-16 and should render as active; the shard regenerates unattended (state.super-downloads.json refreshed daily by reconstruction-engine) so there is no wiring left to do]
- ~~**Reconcile stale product audits**~~ — done 2026-10-23: reconciled banners on `docs/APP-AUDIT.md` + `docs/DIAGNOSTIC.md`; the last item (app CSP `img-src`, not landing) closed won't-fix (`docs/DECISIONS.md`). Was: `docs/APP-AUDIT.md` (14 items) + `docs/DIAGNOSTIC.md` are frozen at 2026-03-24/v1.1.0; many flagged items shipped across Phases 0–3 + v1.1.1. Add a "Reconciled" banner, strike shipped items, and lift the still-open debt into this backlog. Residue list re-verified 2026-08-03 (SD-F-012): the only genuinely open item is **landing CSP `img-src`**. Resolved since the list was written — history-persistence README contradiction (fixed 2026-08-03, SD-F-007: invoke key + catch logging + README re-word), hardcoded version P8 (dynamic `getVersion()` overwrite ships; only a pre-JS placeholder remains), stray `console.log`s T3 (0 exist).
- ~~**Release-ops hardening**~~ — done 2026-10-23: tags v1.1.0…v1.5.3 exist (gh creates them; now fetched locally); `make-release.sh --publish` tags the built commit (`--target`), refuses an unpushed HEAD, runs the provenance gate itself and uses the CHANGELOG section as release notes; pipeline documented in `docs/OPS.md` § Release pipeline.

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
