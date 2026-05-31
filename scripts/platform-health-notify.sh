#!/usr/bin/env bash
#
# platform-health-notify.sh — launchd wrapper around platform-health-check.sh.
#
# Runs the daily health check and posts a macOS notification ONLY when the set
# of failing platforms CHANGES (a platform newly breaks, or recovers). This
# avoids daily spam for a persistently-broken platform while still alerting the
# moment something stops working — which is the whole point of the protocol.
#
# Every run is appended to ~/Library/Logs/super-downloads-health.log.
# Installed via scripts/com.superdownloads.health-check.plist (see docs/PLATFORM-HEALTH.md).
#
set -uo pipefail

cd "$(dirname "$0")/.." || exit 1

LOG="$HOME/Library/Logs/super-downloads-health.log"
STATE_DIR="$HOME/Library/Application Support/SuperDownloads"
STATE="$STATE_DIR/health-failing.txt"
mkdir -p "$(dirname "$LOG")" "$STATE_DIR"

ts="$(date '+%Y-%m-%d %H:%M:%S')"
OUT="$(./scripts/platform-health-check.sh 2>&1)"
RC=$?

{ echo "===== $ts (rc=$RC) ====="; echo "$OUT"; echo; } >> "$LOG"

# Current failing platforms: lines whose verdict column ($2) is exactly FAIL
# (excludes the "Result: … N FAIL" summary line), sorted+unique.
cur="$(printf '%s\n' "$OUT" | awk '$2=="FAIL"{print $1}' | sort -u)"
prev="$(cat "$STATE" 2>/dev/null || true)"
printf '%s\n' "$cur" > "$STATE"

notify() {
  /usr/bin/osascript -e "display notification \"$1\" with title \"Super Downloads — platform health\" sound name \"Basso\"" >/dev/null 2>&1 || true
}

if [ "$cur" != "$prev" ]; then
  if [ -n "$cur" ]; then
    list="$(printf '%s' "$cur" | paste -sd ', ' -)"
    notify "Platform check changed — now failing: ${list}"
  else
    notify "All 7 platforms healthy again ✓"
  fi
fi

exit "$RC"
