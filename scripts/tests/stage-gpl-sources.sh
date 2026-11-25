#!/usr/bin/env bash
# Test for scripts/stage-gpl-sources.sh (SD-03) with stubbed binaries and a stub fetcher.
set -uo pipefail
cd "$(dirname "$0")/../.."
SUT="$PWD/scripts/stage-gpl-sources.sh"
T="$(mktemp -d)"; trap 'rm -rf "$T"' EXIT
pass=0; failn=0
ok()  { pass=$((pass+1)); echo "  ok   $1"; }
bad() { failn=$((failn+1)); echo "  FAIL $1"; }

# fake_bin <path> <version-string> <gpl yes|no>
fake_bin() {
  local conf="--prefix=/x"; [[ "$3" == yes ]] && conf="$conf --enable-gpl --enable-version3"
  printf '#!/bin/sh\necho "ffmpeg version %s Copyright (c) 2000-2024"\necho "configuration: %s"\n' "$2" "$conf" > "$1"
  chmod +x "$1"
}
# stub fetcher: records the URL, writes a fake tarball
cat > "$T/fetch.sh" <<'F'
#!/bin/sh
echo "$1" >> "$FETCH_LOG"
echo "tarball for $1" > "$2"
F
chmod +x "$T/fetch.sh"
export SD_FETCH="$T/fetch.sh" FETCH_LOG="$T/fetch.log"

mkbins() {  # <dir> <arm version> <x86 version> <x86 gpl>
  mkdir -p "$1/ffmpeg-arm64-sources-2026-10-27"
  echo src > "$1/ffmpeg-arm64-sources-2026-10-27/ffmpeg.tar.xz"
  echo src > "$1/ffmpeg-arm64-sources-2026-10-27/x264.tar.gz"
  for t in ffmpeg ffprobe; do
    fake_bin "$1/$t-aarch64-apple-darwin" "$2" yes
    fake_bin "$1/$t-x86_64-apple-darwin" "$3" "$4"
  done
}

# 1. both GPL: both architectures get source + config
mkbins "$T/b1" 7.1.5 6.1.1-tessus yes
: > "$T/fetch.log"
out="$(bash "$SUT" "$T/b1" "$T/o1" 2>"$T/err1")"; rc=$?
[[ $rc -eq 0 ]] && ok "both GPL: exit 0" || { bad "both GPL: exit 0 (rc=$rc)"; cat "$T/err1"; }
grep -q 'ffmpeg-arm64-source-ffmpeg.tar.xz' <<<"$out" && ok "arm64 source listed" || bad "arm64 source listed"
grep -q 'ffmpeg-x86_64-source-ffmpeg-6.1.1.tar.xz' <<<"$out" && ok "x86_64 ffmpeg 6.1.1 tarball listed" || bad "x86_64 tarball listed"
grep -q 'ffmpeg-x86_64-source-build-config.txt' <<<"$out" && ok "x86_64 build config listed" || bad "x86_64 build config listed"
grep -q 'ffmpeg-arm64-source-build-config.txt' <<<"$out" && ok "arm64 build config listed" || bad "arm64 build config listed"
grep -qx 'https://ffmpeg.org/releases/ffmpeg-6.1.1.tar.xz' "$T/fetch.log" && ok "fetched the matching upstream URL" || bad "fetched URL"
[[ -s "$T/o1/ffmpeg-x86_64-source-ffmpeg-6.1.1.tar.xz" ]] && ok "tarball staged non-empty" || bad "tarball staged"
grep -q -- '--enable-gpl' "$T/o1/ffmpeg-x86_64-source-build-config.txt" && ok "config has -version output" || bad "config content"

# 2. x86_64 not GPL: no x86_64 source required or staged
mkbins "$T/b2" 7.1.5 6.1.1 no
out="$(bash "$SUT" "$T/b2" "$T/o2" 2>/dev/null)"; rc=$?
[[ $rc -eq 0 ]] && ! grep -q x86_64 <<<"$out" && ok "non-GPL x86_64 skipped" || bad "non-GPL x86_64 skipped"

# 3. fetch failure on a GPL binary: gate fails
printf '#!/bin/sh\nexit 1\n' > "$T/failfetch.sh"; chmod +x "$T/failfetch.sh"
SD_FETCH="$T/failfetch.sh" bash "$SUT" "$T/b1" "$T/o3" >/dev/null 2>&1 && bad "fetch failure fails the gate" || ok "fetch failure fails the gate"

# 4. arm64 GPL but sources dir missing: gate fails
mkbins "$T/b4" 7.1.5 6.1.1 yes; rm -rf "$T/b4"/ffmpeg-arm64-sources-*
bash "$SUT" "$T/b4" "$T/o4" >/dev/null 2>&1 && bad "missing arm64 sources fails" || ok "missing arm64 sources fails"

# 5. unparsable version on a GPL binary: gate fails
mkbins "$T/b5" 7.1.5 weird yes
bash "$SUT" "$T/b5" "$T/o5" >/dev/null 2>&1 && bad "unparsable version fails" || ok "unparsable version fails"

# 6. ffprobe version differing from ffmpeg: gate fails
mkbins "$T/b6" 7.1.5 6.1.1 yes; fake_bin "$T/b6/ffprobe-x86_64-apple-darwin" 6.0 yes
bash "$SUT" "$T/b6" "$T/o6" >/dev/null 2>&1 && bad "ffmpeg/ffprobe mismatch fails" || ok "ffmpeg/ffprobe mismatch fails"

echo "passed=$pass failed=$failn"
[[ $failn -eq 0 ]]
