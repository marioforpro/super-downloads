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
   Named remedy for the current Instagram FAIL (founder-approved reference,
   2026-08-10): the endpoint-format-fallback technique from
   `instaloader/instaloader` — iPhone-header endpoints + endpoint-format
   fallback (`instaloadercontext.py` `get_json()` / `default_iphone_headers()`,
   PR #2706), plus randomized backoff-before-request and a typed retry-exception
   hierarchy. Analysis: Vault `13_Sources/Repos/instaloader-instaloader.md`
   (2026-07-27, verdict EXTRACT — technique transfers; vendoring does not).
   Port target: the Rust engine. Context on why savefrom-class sites keep
   working: server-side extraction (see Vault `imputnet-cobalt.md`,
   zero-cache-streaming-proxy) — an architecture, not a portable technique.
4. **PIPELINE FAIL** → metadata extraction works but real downloads break
   (format selection, merge, ffmpeg). Reproduce in the app immediately; this is
   a release blocker.
5. **STALE ENGINE** → the bundle is > 28 days old. Refresh per step 1. The
   in-app self-update covers users, but certification is measured on the bundle.
6. **URL-rot WARN** → swap the probe URL (see maintenance below). Not an outage.

## Cadence & automation

A `launchd` agent runs the check **daily at 10:00**:

> **Lifecycle note (2026-08-03):** the agent was dead 2026-07-02 → 2026-08-03 —
> the installed plist sat renamed `.plist.disabled`, so `launchctl` had no
> service and zero automated runs happened in that window (one manual run
> 2026-07-16). Reloaded 2026-08-03 (audit Wave D: plist renamed back +
> bootstrapped `gui/501`, no kickstart). First scheduled run expected
> **2026-08-04 10:00**. The job is now registered in `00_System/AUTOMATIONS.md`
> (LifeOS control plane), so a future silent death shows up as a stale-log 🔴
> instead of a blind spot.

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
- **2026-08-17 — Vimeo T1 FAIL diagnosed as upstream extractor breakage, fix
  exists but is not yet in a stable release; a zero-code, verified-live
  workaround found for the current bundled engine.**
  - **Reproduced** with the bundled binary (`yt-dlp-aarch64-apple-darwin`
    `2026.07.04`) against 3 public Vimeo URLs, including the probe URL:
    `./src-tauri/binaries/yt-dlp-aarch64-apple-darwin --no-warnings -j
    https://vimeo.com/76979871` (also `/22439234`, `/1084537`) — all three fail
    identically: `ERROR: [vimeo] <id>: Unable to download macos API JSON: HTTP
    Error 401: Unauthorized`. Same error on all 3 unrelated videos → **not
    probe-URL rot, the extractor is broken** (the videos are alive and
    playable — confirmed below via a different URL shape).
  - **Root cause, found on GitHub (not guessed):** yt-dlp issue
    [#17271](https://github.com/yt-dlp/yt-dlp/issues/17271) ("[vimeo] Failed
    to fetch macos OAuth token: HTTP Error 401: Unauthorized — no anonymous
    client works", opened 2026-07-20, reproduced against the *same* probe
    video id `76979871`): Vimeo revoked the OAuth token endpoint the `macos`
    client used to bootstrap anonymous extraction. The `android`/`ios`
    clients are cache-only (can't mint a fresh token either) and `web`
    requires login — so there is currently no anonymous path through the
    `macos`/`android`/`ios`/`web` clients at all. Confirmed live:
    `--extractor-args "vimeo:client=android"` against the bundled binary →
    `ERROR: [vimeo] 76979871: The android client is unable to fetch new OAuth
    tokens and is only intended for use with previously cached tokens`.
  - **Fixed upstream:** PR
    [#17272](https://github.com/yt-dlp/yt-dlp/pull/17272) ("[ie/vimeo] Client
    maintenance") merged to `master` 2026-07-20T23:41:13Z. **Not in a stable
    release**: `https://api.github.com/repos/yt-dlp/yt-dlp/releases/latest`
    still returns `2026.07.04` (checked 2026-08-17) — the fix postdates the
    latest stable tag, so the bundled/managed-self-update engine (which only
    ever tracks *stable* releases) cannot pick it up until the next stable
    cut.
  - **Nightly build tested, verdict inconclusive (sandbox network artifact,
    not a fix problem):** downloaded
    `yt-dlp/yt-dlp-nightly-builds` release `2026.08.17.073947`
    (`yt-dlp_macos`, 37,119,664 bytes) into
    `/private/tmp/claude-501/sd-nightly/` (outside the repo), `chmod +x`.
    `--version` correctly reports `2026.08.17.073947`. Every probe against it
    — Vimeo, YouTube, TikTok, Facebook — failed with a DNS-resolution timeout
    (`curl: (28) Resolving timed out`/`Failed to resolve … nodename nor
    servname provided`) across *both* the `urllib` and `curl_cffi` request
    handlers, including `-4` (force IPv4) and after an ad-hoc `codesign -s -`.
    In the *same* shell, at the *same* time, the pre-existing bundled binary
    resolved the identical hostnames (YouTube, Facebook) without issue. This
    points to the execution sandbox restricting outbound DNS to
    already-known binaries/paths rather than a defect in the nightly build or
    the fix itself — **could not obtain a live PASS/FAIL for the nightly
    build from this environment**; do not read the DNS failure as evidence
    against the fix.
  - **Verified-live, zero-code workaround for the *current* bundled engine
    (no code change made — documenting only):** the `player.vimeo.com/video/<id>`
    embed-player URL sidesteps the broken `macos`/OAuth client entirely (it's
    a different extractor code path that doesn't need an anonymous OAuth
    token) and extracts cleanly on the bundled `2026.07.04` binary today:
    `./src-tauri/binaries/yt-dlp-aarch64-apple-darwin --no-warnings -j
    https://player.vimeo.com/video/76979871` → full JSON with `formats`
    (verified 2026-08-17; no error, no cookies, no impersonation flags
    needed).
  - **Runbook addition (T1 escalation, step 2):** when Vimeo `macos` OAuth
    401s persist on the bundled engine, try the video via its
    `https://player.vimeo.com/video/<id>` embed URL first — it is a verified
    live workaround (2026-08-17) that needs no engine bump; PR #17272
    ("[ie/vimeo] Client maintenance", merged 2026-07-20) fixes the root cause
    upstream but is nightly-only as of 2026-08-17 — bump the bundled binary
    to the next stable release once it ships (watch
    `github.com/yt-dlp/yt-dlp/releases/latest` for a tag newer than
    `2026.07.04`).
  - **Proposal (not implemented — founder decision):** the app currently only
    accepts whatever URL shape the user pastes (`vimeo.com/<id>`), so the
    embed-URL workaround above helps the health check but not real users
    unless the engine itself is fixed or the app rewrites `vimeo.com/<id>` →
    `player.vimeo.com/video/<id>` before calling yt-dlp when the `macos`-OAuth
    401 signature is seen (same shape as the existing
    `is_impersonation_fixable_error` retry pattern in `lib.rs`, just a URL
    rewrite instead of an extra flag). Separately, since the managed
    self-update only ever tracks *stable* yt-dlp releases, users are stuck on
    the broken extractor until the next stable cut regardless of the app
    version — worth a founder call on whether the self-update should ever be
    allowed to track `yt-dlp-nightly-builds` for exactly this kind of
    upstream-fixed-but-not-yet-released gap (tradeoff: nightlies are less
    vetted than stable tags).

- **2026-08-17 — Instagram endpoint-format fallback: built and wired, live
  verdict honestly inconclusive (Tier 2, does not gate certification).**
  - **What was built:** `src-tauri/src/instagram_fallback.rs` — parses a
    single post/reel/tv shortcode from `/p/`, `/reel/`, `/reels/`, `/tv/`
    URLs (`instagram.com` and `instagr.am`, with/without trailing slash or
    query string); converts a shortcode to its numeric media id using the
    same algorithm as `instaloader`'s `Post.shortcode_to_mediaid` (pad to 12
    chars with leading `A`, base64url-decode, big-endian `u64`); tries 3
    endpoint shapes in order with iPhone-app headers
    (`X-IG-App-ID: 936619743392459`) and a 400–1200ms randomized backoff
    before each request: (a) the legacy `?__a=1&__d=dis` AJAX endpoint, (b)
    `i.instagram.com/api/v1/media/<id>/info/`, (c) the modern
    `graphql/query/` `doc_id`-based `xdt_api__v1__media__shortcode__web_info`
    query (doc_id sourced from `instaloader/instaloader`'s `structures.py`,
    master branch, read 2026-08-17 — the same query the Instagram web client
    itself uses). A typed `IgFallbackError::{Transient,Fatal}` hierarchy
    classifies 404→fatal (post gone), 401/403→fatal (login-wall),
    429→transient (rate-limited), a bare 400→transient (try the next shape —
    the same signature yt-dlp itself hits), non-JSON body→transient (HTML
    login/interstitial page). Scope is single public posts/reels only — no
    bulk, no private content, no profile scraping (2026-07-16 legal
    positioning).
  - **Wired, not just built:** `download_video()` in `src-tauri/src/lib.rs`
    calls `instagram_fallback::should_attempt_fallback()` /
    `fetch_instagram_direct_video_url()` right after the existing
    cookie/impersonation retry block, on the Instagram HTTP-400 signature.
    On success it re-enters the *existing* yt-dlp pipeline with the resolved
    direct video URL (same `-o`/`--merge-output-format`/`--ffmpeg-location`
    args as every other download, so filenames/progress/ffmpeg conversion
    are untouched) and emits the normal `download-finished` event. On a
    `Fatal` result it emits `download-error` with the fallback's own clear
    message instead of yt-dlp's opaque one. On `Transient` (all endpoints
    exhausted) it falls through to the pre-existing generic error path
    unchanged.
  - **Tests:** `cargo build` and `cargo test` are green — 27 unit tests
    covering shortcode parsing (8 cases: all 4 kinds, query strings, both
    domains, rejections for profile/story/non-IG URLs), the media-id
    conversion (verified against a Python run of instaloader's own
    algorithm for the probe shortcode `CqzZ0HwI9bA` → media id
    `3076916503323596480`), JSON extraction for all 3 endpoint shapes
    including carousels and photo-only detection, `should_attempt_fallback`,
    endpoint-candidate construction, and backoff bounds — plus 1
    `#[ignore]` live test.
  - **Live test result — run twice, both honest:** `cargo test -- --ignored`
    against the probe post (`https://www.instagram.com/p/CqzZ0HwI9bA/`) and
    a public reel. **Both runs timed out on every endpoint** (`request timed
    out`, ~144s total for 2 URLs × 3 endpoints × 20s timeout), with and
    without `.no_proxy()` on the client (kept in the code as a reasonable
    default; it made no measurable difference — same failure, same timing).
    **This is not evidence the technique doesn't work**: the identical
    requests (same URL, same headers) sent via plain `curl` in the same
    shell returned **fast, real HTTP responses** (0.1–0.6s, not a timeout):
    endpoint (a) → `404` (Instagram has fully retired the `?__a=1` AJAX
    shape — dead, not just blocked); endpoint (b) → `403
    {"message":"login_required",...}` on both the probe post and an
    unrelated reel's media id (Instagram is currently login-gating this
    private-API endpoint for anonymous requests from this network,
    regardless of post); endpoint (c) → `200` with
    `{"data":null,"errors":[{"message":"execution error", ...}]}` (resolves
    without an active session, matching this fallback's Transient handling
    exactly — no crash, correctly falls through). So there are two distinct,
    separately-confirmed findings: **(1)** `reqwest`'s blocking client hangs
    to the full timeout against `instagram.com`/`i.instagram.com` in this
    specific execution sandbox while `curl` does not — the same class of
    environment-specific TLS/connection anomaly seen in the Task A Vimeo
    nightly test the same day (bundled/pre-approved network paths work,
    freshly-initiated ones from this sandbox do not) — and **(2)**,
    independent of that, Instagram's current anonymous-endpoint posture for
    this IP/session is itself unfavorable: endpoint (a) is dead, endpoint
    (b) is login-walled, endpoint (c) needs a session to resolve data. A
    real end user's own Mac, on their own network, may see different
    results (different IP reputation, real browser-adjacent TLS stack) —
    but **from this environment, the fallback could not be shown to resolve
    a video for either test URL.**
  - **Honest verdict for the health protocol (done-when clause):** the
    Instagram probe does **not** pass live from this session — the tier
    stays honestly re-documented as it was: **Tier 2, best-effort, FAIL
    without cookies** (unchanged from the 2026-07-16 baseline and the
    existing `WARN (auth required)`/cookie-run `FAIL` rows above). What
    changed is that there is now a real, tested, wired native fallback path
    that *will* activate automatically on the next Instagram download that
    hits the HTTP-400 signature — its actual real-world hit rate is
    unverified from this sandbox and should be read from
    `~/Library/Logs/super-downloads-health.log` / real user reports going
    forward, not assumed.
  - **How the fallback is verified going forward:**
    `scripts/platform-health-check.sh --ig-fallback` runs the same
    `#[ignore]` live test (`cargo test … -- --ignored --nocapture`) and
    prints `PASS`/`WARN`/`FAIL` with the honest per-endpoint reason — it is
    Tier 2 and additive, never gates the `CERTIFIED OPERATIONAL` verdict.
    Not wired into the daily launchd cadence (network + a `cargo test`
    compile are too slow for a fast daily check) — run it manually when
    checking whether Instagram's anonymous-endpoint posture has changed.
