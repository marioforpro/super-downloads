#!/usr/bin/env bash
#
# platform-health-check.sh — v2. Daily smoke test + operational certification
# for the 7 supported platforms.
#
# Probes each platform with a known-good public URL using yt-dlp metadata
# extraction (no download). Reports PASS / FAIL / WARN per platform and the
# best available resolution, so we catch extractor breakage (the #1 cause of
# "video won't download" and "downloads at 360p") before users do.
#
# v2 adds:
#   - Platform tiers. Tier 1 (youtube, tiktok, vimeo) must work out of the box
#     and gates certification. Tier 2 (instagram, facebook, twitter, linkedin)
#     is best-effort / browser-login per product decision 2026-07-16 — failures
#     are reported and notified but do not block certification.
#   - Engine staleness gate: yt-dlp versions are dates (YYYY.MM.DD). An engine
#     older than 28 days cannot be certified (stale extractors are the #1 root
#     cause of breakage).
#   - CERTIFIED OPERATIONAL / DEGRADED final verdict (exit 0 only if certified).
#   - --download-probe: real end-to-end download of the YouTube probe (catches
#     pipeline breakage that metadata-only probing misses).
#   - Reports the managed self-update copy's version when present (that is what
#     users actually run after the app's weekly yt-dlp self-update).
#
# Usage:
#   scripts/platform-health-check.sh                   # bundled yt-dlp (what ships)
#   scripts/platform-health-check.sh --system          # system yt-dlp (Homebrew/pip)
#   scripts/platform-health-check.sh --cookies         # use Chrome cookies (auth platforms)
#   scripts/platform-health-check.sh --download-probe  # also do a real download probe
#
# NOTE: Homebrew/pip builds of yt-dlp usually lack curl_cffi (no impersonation
# support) and will fail on Facebook where the bundled standalone binary works.
# Always judge platform health against the bundled/managed binary.
#
# Exit code: 0 only if CERTIFIED OPERATIONAL. Protocol: docs/PLATFORM-HEALTH.md
#
set -uo pipefail

cd "$(dirname "$0")/.." || exit 1

# ---- Config: one known-good public URL per platform -------------------------
# Format: "platform|url". URLs rot over time — if a probe WARNs with
# "removed/private/404", swap the URL here. An extractor ERROR (not a 404)
# means the platform actually broke. See docs/PLATFORM-HEALTH.md.
PROBES=(
  "youtube|https://www.youtube.com/watch?v=aqz-KE-bpKQ"   # Big Buck Bunny (Blender, has up to 4K)
  "vimeo|https://vimeo.com/76979871"                       # Vimeo staff pick (stable)
  "twitter|https://x.com/SpaceX/status/1732824684683784516" # SpaceX Starship IFT-2 (1080p, no auth; verified 2026-07-16)
  "tiktok|https://www.tiktok.com/@tiktok/video/7106594312292453675"
  "linkedin|https://www.linkedin.com/posts/linkedin_activity-7000000000000000000"
  "instagram|https://www.instagram.com/p/CqzZ0HwI9bA/"
  "facebook|https://www.facebook.com/watch/?v=10153231379946729"
)

# Tier 1 gates certification: must extract out of the box (no cookies).
TIER1="youtube tiktok vimeo"
# Tier 2 is best-effort / browser-login (product decision 2026-07-16): reported
# and notified on change, but does not block certification.
TIER2="instagram facebook twitter linkedin"

# Platforms that normally require auth cookies to extract — a no-cookies WARN
# on these is expected, not an outage.
AUTH_PLATFORMS="linkedin instagram facebook"

# Platforms with NO stable public probe URL (everything requires login). These
# are only probed on a --cookies run; otherwise reported as SKIP.
AUTH_ONLY="linkedin"

# Platforms whose probe URL is known HD/4K. If best available height drops below
# 720p here, extraction is DEGRADED (the classic "downloads at 360p" symptom of a
# stale yt-dlp) — treat as FAIL even though metadata extraction "succeeded".
HD_PLATFORMS="youtube vimeo"
HD_MIN_HEIGHT=720

# Engine older than this cannot be certified (yt-dlp versions are dates).
MAX_ENGINE_AGE_DAYS=28

# ---- Flags -------------------------------------------------------------------
USE_SYSTEM=0
USE_COOKIES=0
DOWNLOAD_PROBE=0
for arg in "$@"; do
  case "$arg" in
    --system)         USE_SYSTEM=1 ;;
    --cookies)        USE_COOKIES=1 ;;
    --download-probe) DOWNLOAD_PROBE=1 ;;
    *) echo "Unknown flag: $arg" >&2; exit 2 ;;
  esac
done

# ---- Resolve yt-dlp ---------------------------------------------------------
ARCH="$(uname -m)"
case "$ARCH" in
  arm64)  YTDLP_ARCH="aarch64-apple-darwin" ;;
  x86_64) YTDLP_ARCH="x86_64-apple-darwin" ;;
  *)      YTDLP_ARCH="aarch64-apple-darwin" ;;
esac

BUNDLED="src-tauri/binaries/yt-dlp-${YTDLP_ARCH}"
MANAGED="$HOME/Library/Application Support/com.supermac.super-downloads/bin/yt-dlp"
if [[ "$USE_SYSTEM" == "1" ]]; then
  YTDLP="$(command -v yt-dlp || true)"
  SOURCE="system"
elif [[ -x "$BUNDLED" ]]; then
  YTDLP="$BUNDLED"
  SOURCE="bundled"
else
  YTDLP="$(command -v yt-dlp || true)"
  SOURCE="system (bundled not found)"
fi

if [[ -z "${YTDLP:-}" ]]; then
  echo "ERROR: yt-dlp not found (bundled or system)." >&2
  exit 3
fi

# ---- Header: surface engine version(s) (staleness = root cause of 360p) -----
VERSION="$("$YTDLP" --version 2>/dev/null || echo '?')"
MANAGED_VERSION=""
[[ -x "$MANAGED" ]] && MANAGED_VERSION="$("$MANAGED" --version 2>/dev/null || echo '?')"

# Engine age in days (version strings are YYYY.MM.DD dates).
ENGINE_AGE_DAYS=""
STALE=0
if [[ "$VERSION" =~ ^[0-9]{4}\.[0-9]{2}\.[0-9]{2} ]]; then
  ver_date="${VERSION//./-}"
  ver_epoch="$(date -j -f '%Y-%m-%d' "${ver_date:0:10}" '+%s' 2>/dev/null || echo '')"
  if [[ -n "$ver_epoch" ]]; then
    ENGINE_AGE_DAYS=$(( ( $(date '+%s') - ver_epoch ) / 86400 ))
    (( ENGINE_AGE_DAYS > MAX_ENGINE_AGE_DAYS )) && STALE=1
  fi
fi

echo "============================================================"
echo " Super Downloads — Platform Health Check (v2)"
echo " yt-dlp : $YTDLP"
echo " source : $SOURCE"
echo " version: $VERSION${ENGINE_AGE_DAYS:+ (${ENGINE_AGE_DAYS}d old)}$( ((STALE)) && echo ' — STALE ENGINE')"
[[ -n "$MANAGED_VERSION" ]] && echo " managed: $MANAGED_VERSION (self-update copy — what users run)"
echo " cookies: $([[ "$USE_COOKIES" == "1" ]] && echo 'chrome' || echo 'none')"
echo "============================================================"

COOKIE_ARGS=()
[[ "$USE_COOKIES" == "1" ]] && COOKIE_ARGS=(--cookies-from-browser chrome)

# Impersonation (curl_cffi) — the app (v1.2.0+) uses --impersonate chrome for
# Facebook, so the probe must mirror that. Standalone yt-dlp_macos builds have
# it; Homebrew/pip builds usually don't.
IMPERSONATE_ARGS=()
if "$YTDLP" --list-impersonate-targets 2>/dev/null | grep -q "curl_cffi" \
   && ! "$YTDLP" --list-impersonate-targets 2>/dev/null | grep -q "unavailable"; then
  IMPERSONATE_ARGS=(--impersonate chrome)
fi

UA="Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15"

TIER1_FAILS=0
TIER2_FAILS=0
WARNS=0
SKIPS=0

# Parse max height from yt-dlp JSON (python3 ships with macOS).
# NOTE: use `python3 -c` (not `python3 - <<HEREDOC`) so the piped JSON reaches
# sys.stdin instead of being shadowed by the heredoc-supplied script body.
max_height() {
  python3 -c '
import sys, json
try:
    d = json.load(sys.stdin)
except Exception:
    print(""); sys.exit(0)
hs = [f.get("height") for f in d.get("formats", []) if isinstance(f.get("height"), int)]
print(max(hs) if hs else (d.get("height") or ""))
' 2>/dev/null
}

record_fail() {
  local platform="$1"
  if echo "$TIER1" | grep -qw "$platform"; then
    TIER1_FAILS=$((TIER1_FAILS+1))
  else
    TIER2_FAILS=$((TIER2_FAILS+1))
  fi
}

for entry in "${PROBES[@]}"; do
  platform="${entry%%|*}"
  url="${entry#*|}"
  tier="$(echo "$TIER1" | grep -qw "$platform" && echo T1 || echo T2)"
  printf " %-10s %-3s" "$platform" "$tier"

  # Auth-only platforms have no public probe URL — only meaningful with cookies.
  if echo "$AUTH_ONLY" | grep -qw "$platform" && [[ "$USE_COOKIES" == "0" ]]; then
    echo "SKIP  (auth-only platform — probed on --cookies runs)"
    SKIPS=$((SKIPS+1))
    continue
  fi

  # Facebook blocks non-browser TLS fingerprints — probe it the way the app
  # downloads it (impersonation), when the engine supports it.
  EXTRA_ARGS=()
  [[ "$platform" == "facebook" && ${#IMPERSONATE_ARGS[@]} -gt 0 ]] && EXTRA_ARGS=("${IMPERSONATE_ARGS[@]}")

  json="$("$YTDLP" -J --simulate --no-warnings --socket-timeout 30 \
           --user-agent "$UA" ${COOKIE_ARGS[@]+"${COOKIE_ARGS[@]}"} \
           ${EXTRA_ARGS[@]+"${EXTRA_ARGS[@]}"} "$url" 2>/tmp/sd_hc_err)"
  rc=$?
  err="$(cat /tmp/sd_hc_err 2>/dev/null)"

  if [[ $rc -eq 0 && -n "$json" ]]; then
    h="$(printf '%s' "$json" | max_height)"
    if [[ -n "$h" ]] && echo "$HD_PLATFORMS" | grep -qw "$platform" && (( h < HD_MIN_HEIGHT )); then
      echo "FAIL  (DEGRADED — best ${h}p, expected >=${HD_MIN_HEIGHT}p; likely stale yt-dlp)"
      record_fail "$platform"
    elif [[ -n "$h" ]]; then
      echo "PASS  (best ${h}p)"
    else
      echo "PASS  (no height reported)"
    fi
    continue
  fi

  # Classify the failure.
  lc="$(printf '%s' "$err" | tr '[:upper:]' '[:lower:]')"
  if echo "$lc" | grep -qE "private|removed|unavailable|not found|404|deleted|no longer|no video could be found|no video formats|no media found"; then
    echo "WARN  (test URL has no downloadable video — swap it in PROBES)"
    WARNS=$((WARNS+1))
  elif echo "$lc" | grep -qE "login|sign in|cookies|authenticat|rate.?limit|429|empty media response"; then
    if echo "$AUTH_PLATFORMS" | grep -qw "$platform" && [[ "$USE_COOKIES" == "0" ]]; then
      echo "WARN  (auth required — re-run with --cookies)"
      WARNS=$((WARNS+1))
    else
      echo "FAIL  (auth/rate-limit) — $(echo "$err" | tail -1)"
      record_fail "$platform"
    fi
  else
    # Real extractor breakage: unable to extract, nsig, unsupported, etc.
    echo "FAIL  (extractor) — $(echo "$err" | tail -1)"
    record_fail "$platform"
  fi
done
rm -f /tmp/sd_hc_err

# ---- Optional real-download probe (pipeline check, YouTube Tier-1 probe) -----
PIPELINE_FAIL=0
if [[ "$DOWNLOAD_PROBE" == "1" ]]; then
  printf " %-10s %-3s" "pipeline" "T1"
  tmpdir="$(mktemp -d /tmp/sd-download-probe.XXXXXX)"
  yt_url="$(printf '%s\n' "${PROBES[@]}" | awk -F'|' '$1=="youtube"{print $2}')"
  "$YTDLP" -f "worst[height>=360][ext=mp4]/worst" --no-warnings --socket-timeout 30 \
      --user-agent "$UA" -o "$tmpdir/probe.%(ext)s" "$yt_url" >/dev/null 2>/tmp/sd_dp_err
  dprc=$?
  f="$(find "$tmpdir" -type f -size +100k | head -1)"
  if [[ $dprc -eq 0 && -n "$f" ]]; then
    echo "PASS  (real download OK: $(du -h "$f" | cut -f1 | tr -d ' '))"
  else
    echo "FAIL  (PIPELINE — metadata may pass but download/merge is broken) — $(tail -1 /tmp/sd_dp_err 2>/dev/null)"
    PIPELINE_FAIL=1
  fi
  rm -rf "$tmpdir" /tmp/sd_dp_err
fi

# ---- Verdict -----------------------------------------------------------------
echo "============================================================"
TOTAL=${#PROBES[@]}
PASSES=$(( TOTAL - TIER1_FAILS - TIER2_FAILS - WARNS - SKIPS ))
echo " Result: $PASSES PASS · $WARNS WARN · $((TIER1_FAILS+TIER2_FAILS)) FAIL · $SKIPS SKIP"
if (( TIER1_FAILS > 0 )); then
  echo " Verdict: DEGRADED — Tier 1 platform(s) failing. See runbook in docs/PLATFORM-HEALTH.md"
elif (( PIPELINE_FAIL > 0 )); then
  echo " Verdict: DEGRADED (pipeline) — real download failed despite metadata OK"
elif (( STALE )); then
  echo " Verdict: DEGRADED (stale engine) — yt-dlp is ${ENGINE_AGE_DAYS}d old (max ${MAX_ENGINE_AGE_DAYS}d). Refresh + re-release."
else
  echo " Verdict: CERTIFIED OPERATIONAL$( (( TIER2_FAILS > 0 )) && echo " (Tier 2 degraded: ${TIER2_FAILS} — best-effort, does not block)")"
fi
echo "============================================================"

if (( TIER1_FAILS > 0 || PIPELINE_FAIL > 0 || STALE )); then
  exit 1
fi
exit 0
