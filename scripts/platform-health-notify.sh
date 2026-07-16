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

# v2: daily run uses Chrome cookies so Tier-2 (browser-login) platforms are
# genuinely monitored, and Mondays add a real-download pipeline probe.
ARGS=(--cookies)
[[ "$(date '+%u')" == "1" ]] && ARGS+=(--download-probe)

OUT="$(./scripts/platform-health-check.sh "${ARGS[@]}" 2>&1)"
RC=$?

# Cookie extraction can fail in a launchd context (e.g. keychain locked). If the
# run died before probing (no verdict line), degrade to a cookie-less run so the
# Tier-1 signal is never lost.
if ! printf '%s' "$OUT" | grep -q "Verdict:"; then
  OUT="[cookie run failed — degraded to no-cookies pass]
$(./scripts/platform-health-check.sh 2>&1)"
  RC=$?
fi

{ echo "===== $ts (rc=$RC) ====="; echo "$OUT"; echo; } >> "$LOG"

# Current failing platforms: v2 line format is "platform TIER VERDICT (...)",
# so the verdict is column $3 (excludes the "Result: … N FAIL" summary line).
cur="$(printf '%s\n' "$OUT" | awk '$3=="FAIL"{print $1}' | sort -u)"
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
    verdict="$(printf '%s\n' "$OUT" | awk -F': ' '/ Verdict:/{print $2; exit}')"
    notify "All platforms healthy again ✓ (${verdict:-certified})"
  fi
fi

exit "$RC"
