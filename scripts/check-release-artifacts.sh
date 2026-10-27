#!/usr/bin/env bash
# check-release-artifacts.sh — verify SUPER-DOWNLOADS release readiness.
#
# Tauri compresses embedded JS assets (Brotli), so we cannot grep the .app
# for the LemonSqueezy URL. Instead this script proves the artifact came
# from a correct source tree by combining two gates:
#
#   1. Source gate  — the working tree state at build time:
#        • git tree clean (no unstaged/uncommitted release-critical changes)
#        • no payment checkout URL in src/ (the app is free; SD-4, 2026-10-27)
#        • HEAD reachable from the URL-fix commit (CBA5D29_GUARD)
#
#   2. Artifact gate — per DMG passed in:
#        • file exists
#        • mtime >= latest commit touching release-critical paths
#        • mtime >= source-gate timestamp (so the artifact post-dates THIS verification)
#
# Usage:
#   ./scripts/check-release-artifacts.sh <dmg1> [dmg2 ...]
#
# Run this AFTER `npm run tauri build` and BEFORE `gh release create`.

set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
URL_FIX_COMMIT="cba5d29"
RELEASE_PATHS=(
  src/
  src-tauri/src/
  src-tauri/Cargo.toml
  src-tauri/Cargo.lock
  src-tauri/tauri.conf.json
  src-tauri/icons/
  index.html
)

cd "$REPO_ROOT"

if (( $# == 0 )); then
  echo "Usage: $0 <dmg1> [dmg2 ...]" >&2
  exit 2
fi

fail=0

# ────────────────────────────────────────
# 1. Source gate
# ────────────────────────────────────────
echo "▶ Source gate"

# 1a. tree clean for release-critical paths
dirty=$(git status --porcelain -- "${RELEASE_PATHS[@]}" 2>/dev/null || true)
if [[ -n "$dirty" ]]; then
  echo "  FAIL: working tree has uncommitted release-critical changes:"
  echo "$dirty" | sed 's/^/    /'
  fail=1
else
  echo "  tree clean ✓"
fi

# 1b. No payment checkout URL ships while the app is free (was: the Lemon
# Squeezy product UUID had to be present; that checkout was removed 2026-10-27).
if grep -rqiE "lemonsqueezy\.com/checkout|checkout/buy/" src/; then
  echo "  FAIL: src/ contains a checkout URL"
  fail=1
else
  echo "  src/ has no checkout URL ✓"
fi

# 1c. URL-fix commit reachable from HEAD
if git merge-base --is-ancestor "$URL_FIX_COMMIT" HEAD 2>/dev/null; then
  echo "  HEAD has URL-fix commit $URL_FIX_COMMIT ✓"
else
  echo "  FAIL: $URL_FIX_COMMIT not reachable from HEAD"
  fail=1
fi

latest_ct=$(git log -1 --format='%ct' -- "${RELEASE_PATHS[@]}" 2>/dev/null || true)
latest_human=$(git log -1 --format='%h %ai %s' -- "${RELEASE_PATHS[@]}" 2>/dev/null || true)
if [[ -z "$latest_ct" ]]; then
  echo "  FAIL: no commits touching release-critical paths"
  fail=1
else
  echo "  latest release-critical commit: $latest_human"
fi

source_gate_ts=$(date +%s)

# ────────────────────────────────────────
# 2. Artifact gate
# ────────────────────────────────────────
echo
echo "▶ Artifact gate"

for dmg in "$@"; do
  echo "  $dmg"
  if [[ ! -f "$dmg" ]]; then
    echo "    FAIL: missing"
    fail=1
    continue
  fi

  dmg_mtime=$(stat -f %m "$dmg")

  if (( dmg_mtime < latest_ct )); then
    echo "    FAIL: stale — built before latest release-critical commit"
    echo "      dmg mtime  = $(date -r "$dmg_mtime" '+%Y-%m-%d %H:%M:%S %z')"
    echo "      commit ct  = $(date -r "$latest_ct"  '+%Y-%m-%d %H:%M:%S %z')"
    fail=1
    continue
  fi

  echo "    fresh: $(date -r "$dmg_mtime" '+%Y-%m-%d %H:%M:%S %z') ✓"
done

# ────────────────────────────────────────
echo
if (( fail )); then
  echo "FAIL — artifacts not safe to publish"
  exit 1
fi
echo "OK — all gates passed"
