# Platform Health Protocol v2 — Super Downloads

> Daily validation + **operational certification** that the supported platforms
> are extractable. Owner: founder. Tooling: `scripts/platform-health-check.sh`.
> Tracks roadmap items **R-SD-003** (download reliability) and **R-SD-004** (relaunch hardening).
> v2 introduced 2026-07-16 (tiers, certification verdict, staleness gate, download probe, cookie runs).

## Why this exists

Super Downloads depends on `yt-dlp` to extract video formats. Every platform
(especially YouTube) rotates its player/signature scheme frequently. When the
bundled `yt-dlp` falls behind, two user-visible failures appear:

1. **"Video won't download"** — the extractor breaks entirely (403, "unable to
   extract", "Cannot parse data").
2. **"Downloads at 360p"** — the extractor can no longer reach the high-res DASH
   formats and silently falls back to the only format left. No error, just bad
   quality — the dangerous one. The check catches it by asserting a minimum
   resolution on known-HD probe URLs.

Broken downloads are also the **commercial** risk: refunds → chargebacks →
payment-processor review (see `docs/superpowers/specs/2026-07-16-relaunch-hardening-design.md`).

## Platform tiers (product decision 2026-07-16)

| Tier | Platforms | Promise | Gates certification? |
|------|-----------|---------|----------------------|
| **Tier 1** | YouTube · TikTok · Vimeo | Works out of the box | **Yes** |
| **Tier 2** | Instagram · Facebook · X/Twitter · LinkedIn | Best-effort, may need the user's browser login | No (reported + notified only) |

The landing page and app copy mirror these tiers. LinkedIn is **auth-only**
(no stable public probe URL exists) — it is only probed on `--cookies` runs.

## Certification

A run ends in one of two verdicts (also the exit code: 0 = certified):

- **`CERTIFIED OPERATIONAL`** — all Tier-1 platforms PASS, the engine is fresh
  (≤ 28 days old — yt-dlp versions are dates), and, when run, the real-download
  probe passed. Tier-2 failures are annotated but do not block.
- **`DEGRADED (…)`** — a Tier-1 platform failed, the engine is stale, or the
  pipeline probe failed. The suffix names the cause.

## How to run

```bash
cd /Users/supermac/Desktop/DEV/01_Projects/SUPER-DOWNLOADS

./scripts/platform-health-check.sh                    # bundled engine (what ships) — default
./scripts/platform-health-check.sh --cookies          # + Chrome cookies (probes Tier-2 for real)
./scripts/platform-health-check.sh --download-probe   # + real end-to-end download (pipeline check)
./scripts/platform-health-check.sh --system           # compare against Homebrew/pip yt-dlp
```

**Important:** Homebrew/pip builds of yt-dlp usually lack `curl_cffi`
(impersonation) and will fail Facebook where the bundled standalone binary
works. Judge platform health only against the bundled/managed binary.

The header prints engine path, **version + age**, and the managed self-update
copy's version when present (that is what users actually run after the app's
weekly yt-dlp self-update, shipped in v1.2.0).

## Reading the results

| Verdict | Meaning | Action |
|---------|---------|--------|
| `PASS (best Np)` | Extraction OK, best height N | none |
| `FAIL (DEGRADED — best 360p…)` | HD platform capped low | stale engine → refresh + re-release |
| `FAIL (extractor)` | Platform extraction broke | see runbook below |
| `FAIL (auth/rate-limit)` | Blocked even with cookies | re-auth browser session; check IP rate-limit |
| `FAIL (PIPELINE…)` | Metadata OK but real download broke | worst signal — test the app's download path end-to-end |
| `WARN (test URL rotted)` | Probe video removed/private | swap the URL in `PROBES` (top of script) |
| `WARN (auth required)` | Auth platform, no cookies | expected on cookie-less runs |
| `SKIP (auth-only)` | No public probe URL exists (LinkedIn) | probed on `--cookies` runs only |
| `STALE ENGINE` (header) | Engine > 28 days old | refresh bundle + ship patch release |

## Runbook (escalation by symptom)

1. **Tier-1 FAIL** → refresh yt-dlp and re-probe (fix for ~90% of failures):
   download the latest `yt-dlp_macos` (universal) into
   `src-tauri/binaries/yt-dlp-{aarch64,x86_64}-apple-darwin`, re-run the check,
   and ship a patch release. Users on v1.2.0+ also receive the engine via the
   in-app weekly self-update, but the bundle must not rot (first-run experience).
2. **Tier-1 FAIL persists on a fresh engine** → the platform changed and yt-dlp
   hasn't caught up. Find the upstream issue (github.com/yt-dlp/yt-dlp/issues),
   subscribe, and re-test daily. If it lasts > a few days, consider a landing
   status note.
3. **Tier-2 FAIL persistent > 2 weeks** → check the upstream yt-dlp issue; if
   the platform is hard-broken (e.g. Instagram HTTP 400, issues #13626/#16311),
   verify the landing copy still frames it honestly (best-effort tier) and note
   it in the FAQ if needed. No release urgency — Tier 2 does not gate.
4. **PIPELINE FAIL** → metadata extraction works but real downloads break
   (format selection, merge, ffmpeg). Reproduce in the app immediately; this is
   a release blocker.
5. **STALE ENGINE** → the bundle is > 28 days old. Refresh per step 1. The
   in-app self-update covers users, but certification is measured on the bundle.
6. **URL-rot WARN** → swap the probe URL (see maintenance below). Not an outage.

## Cadence & automation

A `launchd` agent runs the check **daily at 10:00**:

- Agent: `scripts/platform-health-notify.sh` (wrapper) via
  `scripts/com.superdownloads.health-check.plist` → installed at
  `~/Library/LaunchAgents/com.superdownloads.health-check.plist`.
- **v2 wrapper behavior:** the daily run uses `--cookies` (Tier-2 genuinely
  monitored). If cookie extraction fails in the launchd context (locked
  keychain), it degrades to a cookie-less pass so the Tier-1 signal is never
  lost. **Mondays** add `--download-probe` (weekly pipeline check).
- **Change-detection:** posts a macOS notification ONLY when the set of failing
  platforms *changes* (newly broken or recovered). State:
  `~/Library/Application Support/SuperDownloads/health-failing.txt`.
- Every run logs to `~/Library/Logs/super-downloads-health.log`.
- The plist uses absolute paths via the `/Users/supermac/Desktop/DEV` symlink
  (LifeOS launchd rule: no `$HOME`/`~`/shell vars in plists).

Manage it:
```bash
launchctl kickstart gui/$(id -u)/com.superdownloads.health-check   # run now
launchctl print     gui/$(id -u)/com.superdownloads.health-check   # status
launchctl bootout   gui/$(id -u)/com.superdownloads.health-check   # disable
# re-enable: launchctl bootstrap gui/$(id -u) ~/Library/LaunchAgents/com.superdownloads.health-check.plist
```

## Probe URL maintenance

Test URLs rot. When a probe `WARN`s with "test URL rotted", replace it in the
`PROBES` array at the top of `scripts/platform-health-check.sh` with a fresh,
public, durable video. Prefer official/institutional accounts. For HD platforms
keep a video known to offer ≥1080p so the DEGRADED assertion stays meaningful.
Review all URLs quarterly even without WARNs.

**Open item:** LinkedIn needs a real probe URL (current one is a dead
placeholder). Any public LinkedIn post with native video works; it will only be
probed on `--cookies` runs.

## Baseline — 2026-07-16 (v2 rollout)

Engine: bundled yt-dlp `2026.07.04` (12d old, curl_cffi/impersonation available).

| Platform | No cookies | With cookies | Note |
|----------|-----------|--------------|------|
| youtube | PASS 2160p | PASS 2160p | |
| tiktok | PASS 1920p | PASS 1920p | |
| vimeo | PASS 720p | PASS 720p | |
| twitter | PASS 1080p | PASS 1080p | new SpaceX probe URL |
| facebook | intermittent | **PASS 720p with `--impersonate chrome`** | probe now mirrors app behavior |
| instagram | WARN (auth) | **FAIL — broken upstream** (HTTP 400; yt-dlp #13626/#16311) | Tier-2, does not gate |
| linkedin | SKIP (auth-only) | WARN (probe URL is a dead placeholder) | needs real URL |
| pipeline | PASS (28MB real download) | — | weekly on Mondays |

Verdict at baseline: **CERTIFIED OPERATIONAL** (Tier 2 degraded: instagram).

### History

- **2026-05-31 — root cause confirmed + fixed:** bundled yt-dlp was `2026.01.29`
  (~4 months stale) → YouTube capped at 360p. Refreshed to `2026.03.17`,
  YouTube restored to 4K. Lesson institutionalized as the staleness gate.
- **2026-07-16 — v2:** bundle refreshed to `2026.07.04`; tiers + certification +
  cookie runs + download probe + Facebook impersonation probing introduced.
