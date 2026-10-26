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
# Apple signing + notarization (dormant until the Developer account exists,
# SD-F-017): set APPLE_SIGNING_IDENTITY + notary credentials in ~/.secrets
# (docs/SIGNING.md) and this same command builds signed, notarized releases.
# Without them it builds unsigned, as before.
#
# Usage:
#   ./scripts/make-release.sh            # build + stage + latest.json (no publish)
#   ./scripts/make-release.sh --publish  # also create the GitHub release via gh
#   ./scripts/make-release.sh --no-build # skip build, just (re)stage + manifest
#   ./scripts/make-release.sh --sign-dry-run  # Apple Silicon only, ad-hoc signed
#       with the hardened runtime, verified, NOT staged: rehearses the signed
#       pipeline without an Apple account (dist/ is left untouched)
#
set -euo pipefail

cd "$(dirname "$0")/.."
ROOT="$(pwd)"
REPO="marioforpro/super-downloads"

BUILD=1
PUBLISH=0
DRY_SIGN=0
for arg in "$@"; do
  case "$arg" in
    --no-build) BUILD=0 ;;
    --publish)  PUBLISH=1 ;;
    --sign-dry-run) DRY_SIGN=1; export APPLE_SIGNING_IDENTITY="-" ;;
    *) echo "Unknown flag: $arg" >&2; exit 2 ;;
  esac
done

if [[ "$DRY_SIGN" == "1" && ( "$BUILD" == "0" || "$PUBLISH" == "1" ) ]]; then
  echo "FAIL: --sign-dry-run builds and stops; it cannot be combined with --no-build or --publish" >&2
  exit 2
fi

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

# ── Apple signing (only when APPLE_SIGNING_IDENTITY is set) ──────────────
SIGN=0; [[ -n "${APPLE_SIGNING_IDENTITY:-}" ]] && SIGN=1
TARGET_DIR="src-tauri/target"
TRIPLES=(aarch64-apple-darwin x86_64-apple-darwin)
CONFIG_ARGS=()
if [[ "$SIGN" == "1" ]]; then
  ./scripts/macos-sign.sh preflight
  # codesign refuses Finder/iCloud/Dropbox extended attributes («detritus»), and
  # the iCloud tree stamps them on every bundle built inside it: build elsewhere.
  export CARGO_TARGET_DIR="$HOME/Library/Caches/super-downloads/target"
  TARGET_DIR="$CARGO_TARGET_DIR"
  [[ "$DRY_SIGN" == "1" ]] && TRIPLES=(aarch64-apple-darwin)
  if [[ "$BUILD" == "1" ]]; then
    xattr -cr src-tauri/binaries/ src-tauri/icons src-tauri/ytdlp-plugins
    # Sign a copy of the engine, never the source one, and bundle that copy.
    ENGINE_STAGE="$TARGET_DIR/signed-engine"
    mkdir -p "$TARGET_DIR"; rm -rf "$ENGINE_STAGE"
    ditto --noextattr src-tauri/binaries/yt-dlp-onedir "$ENGINE_STAGE"
    ./scripts/macos-sign.sh nested "$ENGINE_STAGE"
    CONFIG_ARGS=(--config "{\"bundle\":{\"resources\":{\"binaries/yt-dlp-onedir/\":null,\"${ENGINE_STAGE}/\":\"yt-dlp-engine/\"}}}")
  fi
  echo "▶ Signing as: ${APPLE_SIGNING_IDENTITY} (Tauri signs + notarizes the app; DMGs below)"
else
  echo "▶ Unsigned build (no APPLE_SIGNING_IDENTITY)"
fi

# ── 1. Build ─────────────────────────────────────────────────────────────
if [[ "$BUILD" == "1" ]]; then
  for triple in "${TRIPLES[@]}"; do
    echo "▶ Building ${triple}…"
    npm run tauri build -- --target "$triple" --bundles app dmg ${CONFIG_ARGS[@]+"${CONFIG_ARGS[@]}"}
  done
fi

if [[ "$SIGN" == "1" && "$BUILD" == "1" ]]; then
  for triple in "${TRIPLES[@]}"; do
    bundle_dir="${TARGET_DIR}/${triple}/release/bundle"
    app="${bundle_dir}/macos/Super Downloads.app"
    "$app/Contents/Resources/yt-dlp-engine/yt-dlp_macos" --version >/dev/null \
      || { echo "FAIL: bundled engine does not start (${triple})" >&2; exit 1; }
    if [[ "$DRY_SIGN" == "1" ]]; then
      codesign --verify --deep --strict --verbose=2 "$app"
      flags="$(codesign -dv "$app" 2>&1)"
      [[ "$flags" == *runtime* ]] || { echo "FAIL: no hardened runtime" >&2; exit 1; }
    else
      ./scripts/macos-sign.sh dmg "$(ls "${bundle_dir}/dmg/"*.dmg | head -1)"
      ./scripts/macos-sign.sh verify "$app"
    fi
  done
  if [[ "$DRY_SIGN" == "1" ]]; then
    echo "✓ Sign dry run OK: ${TARGET_DIR}/aarch64-apple-darwin/release/bundle (not staged)"
    exit 0
  fi
fi

# ── 2. Stage artifacts ───────────────────────────────────────────────────
mkdir -p dist

stage() {
  local label="$1" triple="$2"
  local dmg_src bundle_dir tar_src sig_src
  bundle_dir="${TARGET_DIR}/${triple}/release/bundle"
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

stage aarch64 "aarch64-apple-darwin"
stage x64     "x86_64-apple-darwin"

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
  # Provenance gate: the DMGs must be newer than the last release-critical commit.
  ./scripts/check-release-artifacts.sh dist/Super-Downloads_aarch64.dmg dist/Super-Downloads_x64.dmg

  # Tag exactly the commit that was built, which must already be on GitHub.
  TARGET="$(git rev-parse HEAD)"
  git fetch -q origin
  if ! git merge-base --is-ancestor "$TARGET" origin/main; then
    echo "FAIL: HEAD ${TARGET:0:7} is not on origin/main — push it first" >&2
    exit 1
  fi

  # Release notes = this version's CHANGELOG section (the landing links to them).
  NOTES="$(awk -v v="## [${VERSION}]" 'index($0, v) == 1 { on = 1; next } on && /^## / { exit } on' CHANGELOG.md)"
  [[ -n "${NOTES//[[:space:]]/}" ]] || NOTES="See CHANGELOG.md"
  # Checksums so anyone can verify the unsigned DMGs (`shasum -a 256 <file>`).
  NOTES+=$'\n\n### Verify your download (SHA-256)\n'
  for a in "${ASSETS[@]}"; do
    [[ "$a" == *.dmg ]] && NOTES+=$'\n'"\`$(shasum -a 256 "$a" | cut -d' ' -f1)\`  $(basename "$a")"
  done

  echo "▶ Publishing ${TAG} (${TARGET:0:7}) to GitHub…"
  gh release create "$TAG" --repo "$REPO" --target "$TARGET" --title "Super Downloads ${TAG}" \
    --notes "$NOTES" "${ASSETS[@]}"
  git fetch -q --tags origin
  echo "✓ Published. Updater endpoint will serve dist/latest.json via releases/latest."
else
  echo "Artifacts ready in dist/. To publish:"
  echo "  gh release create $TAG --repo $REPO --title \"Super Downloads $TAG\" \\"
  echo "    --notes \"See CHANGELOG.md\" \\"
  for a in "${ASSETS[@]}"; do echo "    $a \\"; done
  echo
  echo "Or re-run: ./scripts/make-release.sh --no-build --publish"
fi
