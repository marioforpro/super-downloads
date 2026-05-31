# Platform Health Protocol — Super Downloads

> Daily validation that the 7 supported platforms are still extractable.
> Owner: founder. Tooling: `scripts/platform-health-check.sh`.
> Tracks roadmap item **R-SD-003** (download reliability).

## Why this exists

Super Downloads depends on `yt-dlp` to extract video formats. Every platform
(especially YouTube) rotates its player/signature scheme frequently. When the
bundled `yt-dlp` falls behind, two user-visible failures appear:

1. **"Video won't download"** — the extractor breaks entirely (403, "unable to
   extract", "Cannot parse data").
2. **"Downloads at 360p"** — the extractor can no longer reach the high-res DASH
   formats and silently falls back to the only format left (YouTube format 18 =
   360p progressive). The video *looks* like it downloaded fine, just degraded.

Symptom #2 is the dangerous one: no error, just bad quality. The health check
catches it by asserting a minimum resolution on known-HD probe URLs.

### Root cause confirmed + fixed (2026-05-31)

Bundled `yt-dlp` was **2026.01.29** (~4 months stale). Probing a known-4K
YouTube video returned only heights `[27, 45, 90, 180, 360]` → max 360p. That is
exactly the founder-reported bug.

**Fix applied:** refreshed bundled `yt-dlp` → **2026.03.17** (latest stable).
YouTube probe now reports **2160p (4K)**; the 360p degradation is gone.
Confirmed by `scripts/platform-health-check.sh`. Still pending: ship a release so
end users receive the fresh binary (see ROADMAP R-SD-003 + the one-click
auto-update item). **Fix = keep yt-dlp fresh** (see Remediation).

> Note: Facebook still FAILs ("Cannot parse data") even on 2026.03.17 — FB
> extraction is genuinely degraded in current stable yt-dlp without cookies. This
> is a true red signal, not a stale-binary artifact. Re-test with `--cookies`.

## The 7 platforms

YouTube · Vimeo · Twitter/X · TikTok · LinkedIn · Instagram · Facebook
(canonical list — matches the landing page and `isValidURL()` in `src/main.js`).

## How to run (daily)

```bash
cd /Users/supermac/Desktop/DEV/SUPER-DOWNLOADS

# Tests the bundled yt-dlp (what ships to users) — the default, most representative.
./scripts/platform-health-check.sh

# Auth platforms (LinkedIn/Instagram/Facebook) need browser cookies:
./scripts/platform-health-check.sh --cookies

# Compare against your system (Homebrew/pip) yt-dlp:
./scripts/platform-health-check.sh --system
```

The header always prints the resolved `yt-dlp` path + **version** — glance at it
first; a stale version is the most common root cause.

## Reading the results

| Verdict | Meaning | Action |
|---------|---------|--------|
| `PASS (best Np)` | Extraction OK, best height N | none |
| `FAIL (DEGRADED — best 360p…)` | HD platform capped low | **update yt-dlp** (Remediation) |
| `FAIL (extractor)` | Platform extraction broke | update yt-dlp; if still broken, the platform changed — track upstream yt-dlp issue |
| `FAIL (auth/rate-limit)` | Blocked even with cookies | re-auth browser session; check IP rate-limit |
| `WARN (test URL rotted)` | Probe video removed/private | swap the URL in `PROBES` (top of script) |
| `WARN (auth required)` | Auth platform, no cookies | expected — re-run with `--cookies` to truly verify |

Exit code is **non-zero if any FAIL**, so the script is CI/automation-friendly.

## Remediation (when a FAIL fires)

1. **Refresh yt-dlp and re-probe** (the fix for ~90% of failures):
   ```bash
   # update the bundled binaries the app actually ships
   ./scripts/platform-health-check.sh --system   # confirm a fresh system yt-dlp passes first
   ```
   Then re-bundle: download the latest `yt-dlp_macos` per-arch into
   `src-tauri/binaries/yt-dlp-{aarch64,x86_64}-apple-darwin`, re-run the check,
   commit, and ship a patch release (users get the fresh extractor on update).
2. **If a fresh yt-dlp still FAILs a platform**, the platform changed and yt-dlp
   hasn't caught up — watch the relevant yt-dlp GitHub issue; ship once upstream fixes.
3. **Note the secondary bug**: `find_ytdlp()` in `src-tauri/src/lib.rs` prefers a
   *system* yt-dlp over the bundled one, so on dev machines staleness can hide.
   End users without yt-dlp rely on the bundle — keep the bundle fresh.

## Cadence & automation (future)

- **Now (manual):** run the script as part of the daily founder check; it takes ~30s.
- **Next (automated):** a `launchd` agent runs the check each morning and pushes a
  macOS notification only on FAIL (silent on all-pass). Plist uses absolute paths
  per the LifeOS launchd rule. Tracked as the *platform-health monitor* idea in
  `IDEAS.md` and the matching ROADMAP backlog item.

## Probe URL maintenance

Test URLs rot. When a probe `WARN`s with "test URL rotted", replace it in the
`PROBES` array at the top of `scripts/platform-health-check.sh` with a fresh,
public, durable video. Prefer official/institutional accounts (less likely to be
deleted). For HD platforms keep a video known to offer ≥1080p so the DEGRADED
assertion stays meaningful.
