#!/bin/bash
set -euo pipefail

APP_NAME="Super Downloads"
ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
DIST_DIR="${ROOT_DIR}/dist"

AARCH_APP="${ROOT_DIR}/src-tauri/target/aarch64-apple-darwin/release/bundle/macos/${APP_NAME}.app"
INTEL_APP="${ROOT_DIR}/src-tauri/target/x86_64-apple-darwin/release/bundle/macos/${APP_NAME}.app"

AARCH_DMG="${DIST_DIR}/SuperDownloads_v1.1_AppleSilicon.dmg"
INTEL_DMG="${DIST_DIR}/SuperDownloads_v1.1_Intel.dmg"

if [[ ! -d "${AARCH_APP}" ]]; then
  echo "Error: Apple Silicon app bundle missing:"
  echo "  ${AARCH_APP}"
  echo "Build it first with:"
  echo "  npm run tauri build -- --target aarch64-apple-darwin --bundles app"
  exit 1
fi

if [[ ! -d "${INTEL_APP}" ]]; then
  echo "Error: Intel app bundle missing:"
  echo "  ${INTEL_APP}"
  echo "Build it first with:"
  echo "  npm run tauri build -- --target x86_64-apple-darwin --bundles app"
  exit 1
fi

mkdir -p "${DIST_DIR}"

create_dmg() {
  local app_path="$1"
  local dmg_path="$2"
  local arch_label="$3"
  local temp_dir
  temp_dir="$(mktemp -d)"

  cp -R "${app_path}" "${temp_dir}/"
  cat > "${temp_dir}/INSTRUCTIONS.txt" <<'EOF'
Super Downloads v1.1 - Install Notes

INSTALL:
1) Open the .dmg file
2) Drag "Super Downloads.app" to Applications
3) Eject the mounted DMG

FIRST LAUNCH (unsigned app):
- In Applications, right-click "Super Downloads.app" and choose Open
- Click Open again in the warning dialog

IF MACOS BLOCKS IT:
- System Settings > Privacy & Security > Open Anyway

IF MACOS SAYS APP IS DAMAGED:
- Open Terminal and run:
  xattr -dr com.apple.quarantine "/Applications/Super Downloads.app"
- Then right-click the app and choose Open

SUPPORTED PLATFORMS:
YouTube, TikTok, X/Twitter, Vimeo, Instagram, Facebook, LinkedIn

NOTES:
- Some platforms (Facebook, LinkedIn) may require you to be logged
  in via Chrome for private/authenticated content
- Age-restricted YouTube videos will automatically attempt to
  use your Chrome cookies
EOF

  rm -f "${dmg_path}"
  hdiutil create \
    -volname "${APP_NAME} (${arch_label})" \
    -srcfolder "${temp_dir}" \
    -ov \
    -format UDZO \
    "${dmg_path}"

  rm -rf "${temp_dir}"
}

echo "Creating Apple Silicon DMG..."
create_dmg "${AARCH_APP}" "${AARCH_DMG}" "Apple Silicon"

echo "Creating Intel DMG..."
create_dmg "${INTEL_APP}" "${INTEL_DMG}" "Intel"

echo "Done:"
echo "  ${AARCH_DMG}"
echo "  ${INTEL_DMG}"
