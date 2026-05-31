#!/usr/bin/env bash
#
# platform-health-check.sh — Daily smoke test for the 7 supported platforms.
#
# Probes each platform with a known-good public URL using yt-dlp metadata
# extraction (no download). Reports PASS / FAIL / WARN per platform and the
# best available resolution, so we catch extractor breakage (the #1 cause of
# "video won't download" and "downloads at 360p") before users do.
#
# Usage:
#   scripts/platform-health-check.sh             # test the bundled yt-dlp (what ships)
#   scripts/platform-health-check.sh --system    # test the system yt-dlp (Homebrew/pip)
#   scripts/platform-health-check.sh --cookies    # use Chrome cookies (auth platforms)
#
# Exit code: 0 if all platforms PASS (WARN allowed), non-zero if any FAIL.
# Protocol + escalation: docs/PLATFORM-HEALTH.md
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
  "twitter|https://x.com/Twitter/status/1445078208190291973"
  "tiktok|https://www.tiktok.com/@tiktok/video/7106594312292453675"
  "linkedin|https://www.linkedin.com/posts/linkedin_activity-7000000000000000000"
  "instagram|https://www.instagram.com/p/CqzZ0HwI9bA/"
  "facebook|https://www.facebook.com/watch/?v=10153231379946729"
)

# Platforms that normally require auth cookies to extract — a no-cookies WARN
# on these is expected, not an outage. (X/Twitter has required login since 2023.)
AUTH_PLATFORMS="linkedin instagram facebook twitter"

# Platforms whose probe URL is known HD/4K. If best available height drops below
# 720p here, extraction is DEGRADED (the classic "downloads at 360p" symptom of a
# stale yt-dlp) — treat as FAIL even though metadata extraction "succeeded".
HD_PLATFORMS="youtube vimeo"
HD_MIN_HEIGHT=720

# ---- Resolve yt-dlp ---------------------------------------------------------
USE_SYSTEM=0
USE_COOKIES=0
for arg in "$@"; do
  case "$arg" in
    --system)  USE_SYSTEM=1 ;;
    --cookies) USE_COOKIES=1 ;;
    *) echo "Unknown flag: $arg" >&2; exit 2 ;;
  esac
done

ARCH="$(uname -m)"
case "$ARCH" in
  arm64)  YTDLP_ARCH="aarch64-apple-darwin" ;;
  x86_64) YTDLP_ARCH="x86_64-apple-darwin" ;;
  *)      YTDLP_ARCH="aarch64-apple-darwin" ;;
esac

BUNDLED="src-tauri/binaries/yt-dlp-${YTDLP_ARCH}"
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

# ---- Header: surface yt-dlp version (staleness = root cause of 360p) --------
VERSION="$("$YTDLP" --version 2>/dev/null || echo '?')"
echo "============================================================"
echo " Super Downloads — Platform Health Check"
echo " yt-dlp : $YTDLP"
echo " source : $SOURCE"
echo " version: $VERSION"
echo " cookies: $([[ "$USE_COOKIES" == "1" ]] && echo 'chrome' || echo 'none')"
echo "============================================================"

COOKIE_ARGS=()
[[ "$USE_COOKIES" == "1" ]] && COOKIE_ARGS=(--cookies-from-browser chrome)

UA="Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.0 Safari/605.1.15"

FAILS=0
WARNS=0

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

for entry in "${PROBES[@]}"; do
  platform="${entry%%|*}"
  url="${entry#*|}"
  printf " %-10s " "$platform"

  json="$("$YTDLP" -J --simulate --no-warnings --socket-timeout 30 \
           --user-agent "$UA" ${COOKIE_ARGS[@]+"${COOKIE_ARGS[@]}"} "$url" 2>/tmp/sd_hc_err)"
  rc=$?
  err="$(cat /tmp/sd_hc_err 2>/dev/null)"

  if [[ $rc -eq 0 && -n "$json" ]]; then
    h="$(printf '%s' "$json" | max_height)"
    if [[ -n "$h" ]] && echo "$HD_PLATFORMS" | grep -qw "$platform" && (( h < HD_MIN_HEIGHT )); then
      echo "FAIL  (DEGRADED — best ${h}p, expected >=${HD_MIN_HEIGHT}p; likely stale yt-dlp, run yt-dlp -U)"
      FAILS=$((FAILS+1))
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
  elif echo "$lc" | grep -qE "login|sign in|cookies|authenticat|rate.?limit|429"; then
    if echo "$AUTH_PLATFORMS" | grep -qw "$platform" && [[ "$USE_COOKIES" == "0" ]]; then
      echo "WARN  (auth required — re-run with --cookies)"
      WARNS=$((WARNS+1))
    else
      echo "FAIL  (auth/rate-limit) — $(echo "$err" | tail -1)"
      FAILS=$((FAILS+1))
    fi
  else
    # Real extractor breakage: unable to extract, nsig, unsupported, etc.
    echo "FAIL  (extractor) — $(echo "$err" | tail -1)"
    FAILS=$((FAILS+1))
  fi
done

rm -f /tmp/sd_hc_err
echo "============================================================"
echo " Result: $((${#PROBES[@]} - FAILS - WARNS)) PASS · $WARNS WARN · $FAILS FAIL"
echo "============================================================"

[[ $FAILS -gt 0 ]] && exit 1
exit 0
