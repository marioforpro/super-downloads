#!/usr/bin/env bash
#
# make-release.sh — build signed release artifacts + generate the updater
# manifest (latest.json) consumed by the in-app one-click "Update" button.
#
# Pipeline:
#   1. Build both arches with updater artifacts (.app.tar.gz + .sig)
#   2. Stage DMGs + updater artifacts into dist/ with space-free names
#   3. Generate dist/latest.json (the manifest the app polls)
#   4. Print the `gh release create` command (or run it with --publish)
#
# The app's updater endpoint (tauri.conf.json) points at:
#   github.com/marioforpro/super-downloads/releases/latest/download/latest.json
# so latest.json MUST be uploaded as a release asset alongside the artifacts.
#
# Signing: updater artifacts are signed with the Ed25519 key. Provide:
#   export TAURI_SIGNING_PRIVATE_KEY="$(cat src-tauri/.tauri-update-key)"
#   export TAURI_SIGNING_PRIVATE_KEY_PASSWORD="<password or empty>"
# (the script sets the key from the file automatically if the env var is unset).
#
# Usage:
#   ./scripts/make-release.sh            # build + stage + latest.json (no publish)
#   ./scripts/make-release.sh --publish  # also create the GitHub release via gh
#   ./scripts/make-release.sh --no-build # skip build, just (re)stage + manifest
#
set -euo pipefail

cd "$(dirname "$0")/.."
ROOT="$(pwd)"
REPO="marioforpro/super-downloads"

BUILD=1
PUBLISH=0
for arg in "$@"; do
  case "$arg" in
    --no-build) BUILD=0 ;;
    --publish)  PUBLISH=1 ;;
    *) echo "Unknown flag: $arg" >&2; exit 2 ;;
  esac
done

VERSION="$(python3 -c "import json;print(json.load(open('src-tauri/tauri.conf.json'))['version'])")"
TAG="v${VERSION}"
echo "▶ Release ${TAG}"

# ── Signing key ──────────────────────────────────────────────────────────
if [[ -z "${TAURI_SIGNING_PRIVATE_KEY:-}" ]]; then
  if [[ -f src-tauri/.tauri-update-key ]]; then
    export TAURI_SIGNING_PRIVATE_KEY="$(cat src-tauri/.tauri-update-key)"
  else
    echo "FAIL: no signing key (src-tauri/.tauri-update-key missing and TAURI_SIGNING_PRIVATE_KEY unset)" >&2
    exit 1
  fi
fi
export TAURI_SIGNING_PRIVATE_KEY_PASSWORD="${TAURI_SIGNING_PRIVATE_KEY_PASSWORD:-}"

# Use the rustup toolchain (has both aarch64 + x86_64 std). See docs/DEVELOPMENT.md.
export PATH="/opt/homebrew/opt/rustup/bin:$PATH"

# ── 1. Build ─────────────────────────────────────────────────────────────
if [[ "$BUILD" == "1" ]]; then
  echo "▶ Building aarch64…"
  npm run tauri build -- --target aarch64-apple-darwin --bundles app dmg
  echo "▶ Building x86_64…"
  npm run tauri build -- --target x86_64-apple-darwin --bundles app dmg
fi

# ── 2. Stage artifacts ───────────────────────────────────────────────────
mkdir -p dist
declare -A ARCH_DIR=( [aarch64]="aarch64-apple-darwin" [x64]="x86_64-apple-darwin" )

stage() {
  local label="$1" triple="$2"
  local dmg_src bundle_dir tar_src sig_src
  bundle_dir="src-tauri/target/${triple}/release/bundle"
  dmg_src="$(ls "${bundle_dir}/dmg/"*.dmg 2>/dev/null | head -1)"
  tar_src="$(ls "${bundle_dir}/macos/"*.app.tar.gz 2>/dev/null | head -1)"
  sig_src="$(ls "${bundle_dir}/macos/"*.app.tar.gz.sig 2>/dev/null | head -1)"

  [[ -f "$dmg_src" ]] || { echo "FAIL: missing DMG for ${label} (${bundle_dir}/dmg)"; exit 1; }
  [[ -f "$tar_src" ]] || { echo "FAIL: missing .app.tar.gz for ${label} — is createUpdaterArtifacts true + signing key set?"; exit 1; }
  [[ -f "$sig_src" ]] || { echo "FAIL: missing .sig for ${label}"; exit 1; }

  cp "$dmg_src" "dist/Super-Downloads_${label}.dmg"
  cp "$tar_src" "dist/Super-Downloads_${VERSION}_${label}.app.tar.gz"
  cp "$sig_src" "dist/Super-Downloads_${VERSION}_${label}.app.tar.gz.sig"
  echo "  staged ${label}: DMG + updater artifact + sig"
}

stage aarch64 "${ARCH_DIR[aarch64]}"
stage x64     "${ARCH_DIR[x64]}"

# ── 3. Generate latest.json ──────────────────────────────────────────────
SIG_AARCH64="$(cat "dist/Super-Downloads_${VERSION}_aarch64.app.tar.gz.sig")"
SIG_X64="$(cat "dist/Super-Downloads_${VERSION}_x64.app.tar.gz.sig")"
BASE="https://github.com/${REPO}/releases/download/${TAG}"

python3 - "$VERSION" "$TAG" "$SIG_AARCH64" "$SIG_X64" "$BASE" > dist/latest.json <<'PY'
import json, sys, datetime
version, tag, sig_aarch64, sig_x64, base = sys.argv[1:6]
manifest = {
    "version": version,
    "notes": f"Super Downloads {tag}",
    "pub_date": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
    "platforms": {
        "darwin-aarch64": {"signature": sig_aarch64, "url": f"{base}/Super-Downloads_{version}_aarch64.app.tar.gz"},
        "darwin-x86_64":  {"signature": sig_x64,      "url": f"{base}/Super-Downloads_{version}_x64.app.tar.gz"},
    },
}
print(json.dumps(manifest, indent=2))
PY
echo "  wrote dist/latest.json"

# ── 4. Publish ───────────────────────────────────────────────────────────
ASSETS=(
  "dist/Super-Downloads_aarch64.dmg"
  "dist/Super-Downloads_x64.dmg"
  "dist/Super-Downloads_${VERSION}_aarch64.app.tar.gz"
  "dist/Super-Downloads_${VERSION}_x64.app.tar.gz"
  "dist/latest.json"
)

echo
if [[ "$PUBLISH" == "1" ]]; then
  echo "▶ Publishing ${TAG} to GitHub…"
  gh release create "$TAG" --repo "$REPO" --title "Super Downloads ${TAG}" \
    --notes "See CHANGELOG.md" "${ASSETS[@]}"
  echo "✓ Published. Updater endpoint will serve dist/latest.json via releases/latest."
else
  echo "Artifacts ready in dist/. To publish:"
  echo "  gh release create $TAG --repo $REPO --title \"Super Downloads $TAG\" \\"
  echo "    --notes \"See CHANGELOG.md\" \\"
  for a in "${ASSETS[@]}"; do echo "    $a \\"; done
  echo
  echo "Or re-run: ./scripts/make-release.sh --no-build --publish"
fi
