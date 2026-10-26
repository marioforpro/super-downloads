#!/usr/bin/env bash
#
# macos-sign.sh — Developer ID signing + Apple notarization for Super Downloads.
#
# Dormant until the Apple Developer account exists (decision SD-F-017: January,
# before the 26-Jan launch). make-release.sh calls it only when
# APPLE_SIGNING_IDENTITY is set; without it, releases stay unsigned as today.
# Day-one guide: docs/SIGNING.md.
#
# What Tauri does on its own when the env below is set: signs the app and the
# ffmpeg/ffprobe sidecars with the hardened runtime, notarizes the .app and
# staples it (the updater .app.tar.gz is made after that, so it is notarized too).
# What Tauri does NOT do, and this script does:
#   nested [DIR]  sign every Mach-O inside the yt-dlp onedir engine (default: the
#             source copy Tauri bundles as Resources/yt-dlp-engine as plain files).
#             Notarization rejects any unsigned one. Removes _internal/Python.framework
#             first: upstream ships it as three real copies of the Python dylib (no
#             symlinks), which codesign refuses as an «ambiguous bundle»; the engine
#             loads _internal/Python and runs without it (tested 2026-10-26).
#   dmg FILE  sign + notarize + staple a DMG (so the disk image itself opens clean)
#   verify P  codesign --verify --deep --strict + spctl assessment of an .app/.dmg
#   preflight check identity in the keychain + notary credentials, change nothing
#
# Env (all in ~/.secrets once the account exists; `load-secrets` first):
#   APPLE_SIGNING_IDENTITY  "Developer ID Application: <Legal Name> (<TEAMID>)"
#                           Use "-" for a local dry run (ad-hoc + hardened runtime):
#                           checks the engine still runs under the runtime, no Apple.
#   Notary, preferred (App Store Connect API key, Team Key, role Developer):
#     APPLE_API_ISSUER  APPLE_API_KEY (key id)  APPLE_API_KEY_PATH (AuthKey_XXXX.p8)
#   Notary, alternative (Apple ID + app-specific password):
#     APPLE_ID  APPLE_PASSWORD  APPLE_TEAM_ID
#
set -euo pipefail
cd "$(dirname "$0")/.."

ENGINE_DIR="src-tauri/binaries/yt-dlp-onedir"
ENGINE_ENTITLEMENTS="src-tauri/entitlements/engine.plist"

die() { echo "FAIL: $*" >&2; exit 1; }

identity() {
  [[ -n "${APPLE_SIGNING_IDENTITY:-}" ]] || die "APPLE_SIGNING_IDENTITY is not set (see docs/SIGNING.md)"
  echo "$APPLE_SIGNING_IDENTITY"
}

# Timestamps need Apple's server; an ad-hoc dry run cannot have one.
ts_flag() { [[ "$(identity)" == "-" ]] && echo "--timestamp=none" || echo "--timestamp"; }

notary_args() {
  if [[ -n "${APPLE_API_ISSUER:-}" && -n "${APPLE_API_KEY:-}" && -n "${APPLE_API_KEY_PATH:-}" ]]; then
    [[ -f "$APPLE_API_KEY_PATH" ]] || die "APPLE_API_KEY_PATH does not exist: $APPLE_API_KEY_PATH"
    echo "--key $APPLE_API_KEY_PATH --key-id $APPLE_API_KEY --issuer $APPLE_API_ISSUER"
  elif [[ -n "${APPLE_ID:-}" && -n "${APPLE_PASSWORD:-}" && -n "${APPLE_TEAM_ID:-}" ]]; then
    echo "--apple-id $APPLE_ID --password $APPLE_PASSWORD --team-id $APPLE_TEAM_ID"
  else
    return 1
  fi
}

is_macho() { file -b "$1" | grep -q "Mach-O"; }

cmd_nested() {
  local dir="${1:-$ENGINE_DIR}" id ts ent n=0
  id="$(identity)"; ts="$(ts_flag)"; ent="$ENGINE_ENTITLEMENTS"
  [[ -x "$dir/yt-dlp_macos" ]] || die "engine missing: $dir/yt-dlp_macos"
  if [[ "$id" == "-" ]]; then
    # Ad-hoc signatures carry no Team ID, so library validation would refuse the
    # engine's own dylibs. A Developer ID run doesn't need this exception.
    ent="$(mktemp -t sd-engine-dry).plist"
    sed 's#<dict>#<dict><key>com.apple.security.cs.disable-library-validation</key><true/>#' \
      "$ENGINE_ENTITLEMENTS" > "$ent"
  fi
  if [[ -d "$dir/_internal/Python.framework" ]]; then
    [[ -f "$dir/_internal/Python" && ! -L "$dir/_internal/Python" ]] || die "unexpected engine layout: $dir/_internal/Python"
    rm -r "$dir/_internal/Python.framework"
  fi
  # Inside-out: libraries first, the engine executable last (with its entitlements).
  while IFS= read -r -d '' f; do
    is_macho "$f" || continue
    codesign --force --sign "$id" --options runtime "$ts" "$f"
    n=$((n + 1))
  done < <(find "$dir/_internal" -type f -print0)
  codesign --force --sign "$id" --options runtime "$ts" --entitlements "$ent" "$dir/yt-dlp_macos"
  "$dir/yt-dlp_macos" --version >/dev/null || die "signed engine does not start"
  echo "signed engine: $n libraries + yt-dlp_macos (identity: $id)"
}

cmd_dmg() {
  local dmg="$1" id args
  [[ -f "$dmg" ]] || die "no such DMG: $dmg"
  id="$(identity)"
  codesign --force --sign "$id" "$(ts_flag)" "$dmg"
  [[ "$id" == "-" ]] && { echo "dry run: DMG ad-hoc signed, not notarized"; return; }
  args="$(notary_args)" || die "no notary credentials (APPLE_API_* or APPLE_ID/APPLE_PASSWORD/APPLE_TEAM_ID)"
  # shellcheck disable=SC2086
  xcrun notarytool submit "$dmg" $args --wait --timeout 30m
  xcrun stapler staple "$dmg"
  xcrun stapler validate "$dmg"
}

cmd_verify() {
  local p="$1"
  [[ -e "$p" ]] || die "no such path: $p"
  if [[ "$p" == *.app ]]; then
    codesign --verify --deep --strict --verbose=2 "$p"
    spctl --assess --type execute --verbose=4 "$p"
  else
    codesign --verify --verbose=2 "$p"
    spctl --assess --type open --context context:primary-signature --verbose=4 "$p"
  fi
}

cmd_preflight() {
  local id
  id="$(identity)"
  if [[ "$id" != "-" ]]; then
    security find-identity -v -p codesigning | grep -F "$id" >/dev/null \
      || die "identity not in the keychain: $id (security find-identity -v -p codesigning)"
    notary_args >/dev/null || die "no notary credentials"
  fi
  [[ -f "$ENGINE_ENTITLEMENTS" ]] || die "missing $ENGINE_ENTITLEMENTS"
  echo "preflight OK (identity: $id)"
}

case "${1:-}" in
  nested)    cmd_nested "${2:-}" ;;
  dmg)       cmd_dmg "${2:?usage: macos-sign.sh dmg FILE.dmg}" ;;
  verify)    cmd_verify "${2:?usage: macos-sign.sh verify PATH}" ;;
  preflight) cmd_preflight ;;
  *) echo "usage: $0 nested [DIR] | dmg FILE.dmg | verify PATH | preflight" >&2; exit 2 ;;
esac
