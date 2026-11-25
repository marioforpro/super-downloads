#!/usr/bin/env bash
# check-bundle-version.sh <VERSION> <bundle_dir> — refuse to stage a build
# whose version is not VERSION (SD-10). Checks, on the SOURCE artifacts:
#   • a DMG whose name contains _<VERSION>_
#   • the .app's CFBundleShortVersionString == VERSION
#   • the updater .app.tar.gz holds an app with that same version
# Prints the DMG path on success; exits 1 with a FAIL line otherwise.
set -euo pipefail

VERSION="${1:?usage: check-bundle-version.sh <VERSION> <bundle_dir>}"
BUNDLE="${2:?usage: check-bundle-version.sh <VERSION> <bundle_dir>}"

plist_version() {  # stdin: Info.plist
  python3 -c 'import plistlib,sys;print(plistlib.loads(sys.stdin.buffer.read()).get("CFBundleShortVersionString",""))'
}

dmg=""
for f in "$BUNDLE"/dmg/*_"${VERSION}"_*.dmg; do
  [[ -f "$f" ]] && { dmg="$f"; break; }
done
if [[ -z "$dmg" ]]; then
  echo "FAIL: no DMG named *_${VERSION}_* in ${BUNDLE}/dmg (stale build? rebuild without --no-build)" >&2
  exit 1
fi

app_ver="$(plist_version < "$BUNDLE/macos/Super Downloads.app/Contents/Info.plist" 2>/dev/null || true)"
if [[ "$app_ver" != "$VERSION" ]]; then
  echo "FAIL: .app version is '${app_ver:-unreadable}', expected ${VERSION} (stale build?)" >&2
  exit 1
fi

tar_ver="$(tar -xzOf "$BUNDLE/macos/Super Downloads.app.tar.gz" "Super Downloads.app/Contents/Info.plist" 2>/dev/null | plist_version 2>/dev/null || true)"
if [[ "$tar_ver" != "$VERSION" ]]; then
  echo "FAIL: updater .app.tar.gz version is '${tar_ver:-unreadable}', expected ${VERSION} (stale build?)" >&2
  exit 1
fi

echo "$dmg"
