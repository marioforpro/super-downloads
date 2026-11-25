#!/usr/bin/env bash
# Test for scripts/check-bundle-version.sh (SD-10) with fake bundle files.
set -uo pipefail
cd "$(dirname "$0")/../.."
SUT=scripts/check-bundle-version.sh
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
pass=0; failn=0
ok()  { pass=$((pass+1)); echo "  ok   $1"; }
bad() { failn=$((failn+1)); echo "  FAIL $1"; }

# make_bundle <dir> <dmg-version-in-name> <app-version> <tar-app-version>
make_bundle() {
  local d="$1" dv="$2" av="$3" tv="$4"
  mkdir -p "$d/dmg" "$d/macos/Super Downloads.app/Contents"
  : > "$d/dmg/Super Downloads_${dv}_aarch64.dmg"
  plist() { printf '<?xml version="1.0"?><plist version="1.0"><dict><key>CFBundleShortVersionString</key><string>%s</string></dict></plist>' "$1"; }
  plist "$av" > "$d/macos/Super Downloads.app/Contents/Info.plist"
  local tt; tt="$(mktemp -d)"; mkdir -p "$tt/Super Downloads.app/Contents"
  plist "$tv" > "$tt/Super Downloads.app/Contents/Info.plist"
  tar -czf "$d/macos/Super Downloads.app.tar.gz" -C "$tt" "Super Downloads.app"
  : > "$d/macos/Super Downloads.app.tar.gz.sig"
}

make_bundle "$T/good" 1.5.10 1.5.10 1.5.10
bash "$SUT" 1.5.10 "$T/good" >/dev/null 2>&1 && ok "matching bundle accepted" || bad "matching bundle accepted"

make_bundle "$T/olddmg" 1.5.9 1.5.10 1.5.10
bash "$SUT" 1.5.10 "$T/olddmg" >/dev/null 2>&1 && bad "old DMG name refused" || ok "old DMG name refused"

make_bundle "$T/oldapp" 1.5.10 1.5.9 1.5.10
bash "$SUT" 1.5.10 "$T/oldapp" >/dev/null 2>&1 && bad "old .app version refused" || ok "old .app version refused"

make_bundle "$T/oldtar" 1.5.10 1.5.10 1.5.9
bash "$SUT" 1.5.10 "$T/oldtar" >/dev/null 2>&1 && bad "old updater tarball refused" || ok "old updater tarball refused"

# 1.5.1 must not match a 1.5.10 name via prefix
make_bundle "$T/prefix" 1.5.1 1.5.10 1.5.10
bash "$SUT" 1.5.10 "$T/prefix" >/dev/null 2>&1 && bad "prefix version name refused" || ok "prefix version name refused"

echo "passed=$pass failed=$failn"
[[ $failn -eq 0 ]]
